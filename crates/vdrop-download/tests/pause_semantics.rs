//! Duraklatmanin **transferi sonlandirdigini** ve `.part` dosyasini
//! koruyarak devam ettirilebilir biraktigini dogrular.
//!
//! Neden bu davranis: duraklatilmis bir indirme yerinde bekleseydi, hem
//! es zamanlilik yuvasini isgal ederdi (limit 3 iken 3 indirmeyi duraklatan
//! kullanici siradakileri baslatamazdi) hem de bosta duran bir HTTP
//! baglantisini acik tutardi.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;

use vdrop_download::{
    start_download, start_download_with_control, ControlSignal, DownloadEvent, DownloadOptions,
};

/// Yavas sunucu: govdeyi parca parca, aralarla gonderir. Boylece duraklatma
/// sinyali transfer ortasinda yakalanabilir - hizli bir sunucuda indirme
/// biter ve duraklatmayi test edemezdik.
async fn spawn_slow_server(
    payload: &'static [u8],
    chunk: usize,
    gap: Duration,
) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            tokio::spawn(async move {
                serve(&mut socket, payload, chunk, gap).await;
            });
        }
    });
    addr
}

async fn serve(socket: &mut TcpStream, payload: &'static [u8], chunk: usize, gap: Duration) {
    let mut buf = vec![0u8; 4096];
    let Ok(n) = socket.read(&mut buf).await else {
        return;
    };
    let req = String::from_utf8_lossy(&buf[..n]);
    let start = req
        .lines()
        .find(|l| l.to_lowercase().starts_with("range:"))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|r| r.trim_end_matches('\r').split('-').next())
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let body = &payload[start.min(payload.len())..];
    let head = if start > 0 {
        format!(
            "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
            body.len()
        )
    } else {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
            body.len()
        )
    };
    if socket.write_all(head.as_bytes()).await.is_err() {
        return;
    }
    for part in body.chunks(chunk) {
        if socket.write_all(part).await.is_err() {
            return;
        }
        let _ = socket.flush().await;
        tokio::time::sleep(gap).await;
    }
    let _ = socket.shutdown().await;
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vdrop-pause-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn part_of(dest: &Path) -> PathBuf {
    dest.with_file_name(format!(
        "{}.part",
        dest.file_name().unwrap().to_string_lossy()
    ))
}

/// 15 parca x 60 ms ~= 900 ms'lik bir transfer. Duraklatma sinyalinin
/// rahatca ortaya dusebilecegi kadar genis, testi yavaslatmayacak kadar kisa.
const CHUNK_GAP: Duration = Duration::from_millis(60);

static PAYLOAD: &[u8] = &[7u8; 60_000];

/// Govdenin **ilk baytlari diske dustukten sonra** donen bekleme.
///
/// Onceden buranin yerinde sabit bir `sleep(120ms)` vardi ve testler
/// tek baslarina gecip workspace kosusunda kiriliyordu: makine yuklendiginde
/// baglanti kurulumu o pencereyi asiyor, duraklatma sinyali ilk govde
/// parcasindan once yakalaniyordu. Indirme dongusu kontrol sinyalini parcayi
/// yazmadan **once** okudugu icin transfer `downloaded = 0` ile duraklıyor,
/// "duraklamadan once veri akmis olmali" iddiasi patliyordu. Duvar saati
/// yerine diske dusen ilk bayta senkronlanmak bu yarisi tumuyle kaldirir.
async fn wait_for_first_bytes(part: &Path) -> u64 {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Ok(meta) = tokio::fs::metadata(part).await {
            if meta.len() > 0 {
                return meta.len();
            }
        }
        assert!(
            Instant::now() < deadline,
            "ilk baytlar 10 saniyede diske dusmedi: {}",
            part.display()
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn pause_ends_the_transfer_and_keeps_partial_data() {
    let addr = spawn_slow_server(PAYLOAD, 4_000, CHUNK_GAP).await;
    let dir = scratch("ends");
    let dest = dir.join("clip.mp4");

    let (control, control_rx) = watch::channel(ControlSignal::Run);
    let mut events = start_download_with_control(
        reqwest::Client::new(),
        DownloadOptions::new(format!("http://{addr}/f"), dest.clone()),
        control_rx,
    );

    // Bir miktar veri aksin, sonra duraklat.
    wait_for_first_bytes(&part_of(&dest)).await;
    control.send(ControlSignal::Pause).unwrap();

    let paused_at;
    let deadline = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            ev = events.recv() => match ev {
                Some(DownloadEvent::Paused { downloaded_bytes }) => {
                    paused_at = Some(downloaded_bytes);
                    break;
                }
                Some(DownloadEvent::Completed { .. }) => panic!("duraklatma yerine tamamlandi"),
                Some(DownloadEvent::Failed { message }) => panic!("basarisiz: {message}"),
                Some(_) => continue,
                None => panic!("olay akisi Paused yayinlamadan kapandi"),
            },
            _ = &mut deadline => panic!("Paused olayi 10 saniyede gelmedi"),
        }
    }

    let downloaded = paused_at.expect("Paused olayi bayt sayisi tasimali");
    assert!(downloaded > 0, "duraklamadan once veri akmis olmali");
    assert!(
        downloaded < PAYLOAD.len() as u64,
        "test anlamli olsun diye duraklama bitisten once olmali"
    );

    // ASIL NOKTA: gorev sonlanmali. Olay kanali kapaniyorsa gorev bitmistir
    // ve tuttugu es zamanlilik izni serbest kalmistir.
    let closed = tokio::time::timeout(Duration::from_secs(3), events.recv()).await;
    assert!(
        closed.expect("kanal makul surede kapanmali").is_none(),
        "duraklatma gorevi sonlandirmali, yerinde beklememeli"
    );

    // Yarim veri korunmali; nihai ad olusmamali.
    let part = part_of(&dest);
    assert!(part.exists(), ".part dosyasi duraklamada korunmali");
    assert_eq!(
        std::fs::metadata(&part).unwrap().len(),
        downloaded,
        "diskteki bayt sayisi bildirilenle ayni olmali"
    );
    assert!(!dest.exists(), "yarim indirme nihai adi almamali");

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn resuming_after_pause_produces_the_complete_file() {
    let addr = spawn_slow_server(PAYLOAD, 4_000, CHUNK_GAP).await;
    let dir = scratch("resume");
    let dest = dir.join("clip.mp4");
    let url = format!("http://{addr}/f");

    // 1. tur: basla ve duraklat.
    let (control, control_rx) = watch::channel(ControlSignal::Run);
    let mut events = start_download_with_control(
        reqwest::Client::new(),
        DownloadOptions::new(url.clone(), dest.clone()),
        control_rx,
    );
    wait_for_first_bytes(&part_of(&dest)).await;
    control.send(ControlSignal::Pause).unwrap();
    while let Some(ev) = events.recv().await {
        if matches!(ev, DownloadEvent::Paused { .. }) {
            break;
        }
    }
    let partial = std::fs::metadata(part_of(&dest)).unwrap().len();
    assert!(partial > 0 && partial < PAYLOAD.len() as u64);

    // 2. tur: devam ettirme = yeniden baslatma. Uygulamanin `resume_download`
    // komutu tam olarak bunu yapar; motor `.part` dosyasini gorup Range ile
    // kaldigi yerden surdurur.
    let mut handle = start_download(
        reqwest::Client::new(),
        DownloadOptions::new(url, dest.clone()),
    );
    let done = tokio::time::timeout(Duration::from_secs(20), async {
        while let Some(ev) = handle.events.recv().await {
            match ev {
                DownloadEvent::Completed { total_bytes, .. } => return Ok(total_bytes),
                DownloadEvent::Failed { message } => return Err(message),
                _ => {}
            }
        }
        Err("akis erken kapandi".into())
    })
    .await
    .expect("devam ettirme 20 saniyede bitmeli")
    .expect("devam ettirme basarili olmali");

    assert_eq!(done, PAYLOAD.len() as u64);
    let written = std::fs::read(&dest).unwrap();
    assert_eq!(
        written, PAYLOAD,
        "duraklatilip devam ettirilen dosya bayt bayt orijinalle ayni olmali"
    );
    assert!(!part_of(&dest).exists(), ".part temizlenmeli");

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn cancel_still_reports_cancelled_not_paused() {
    // Duraklatma yolu eklenirken iptal yolunun bozulmadigini garanti eder.
    let addr = spawn_slow_server(PAYLOAD, 4_000, CHUNK_GAP).await;
    let dir = scratch("cancel");
    let dest = dir.join("clip.mp4");

    let (control, control_rx) = watch::channel(ControlSignal::Run);
    let mut events = start_download_with_control(
        reqwest::Client::new(),
        DownloadOptions::new(format!("http://{addr}/f"), dest.clone()),
        control_rx,
    );
    wait_for_first_bytes(&part_of(&dest)).await;
    control.send(ControlSignal::Cancel).unwrap();

    let mut saw_cancelled = false;
    while let Some(ev) = events.recv().await {
        match ev {
            DownloadEvent::Cancelled => {
                saw_cancelled = true;
                break;
            }
            DownloadEvent::Paused { .. } => panic!("iptal, duraklatma olarak raporlandi"),
            _ => {}
        }
    }
    assert!(saw_cancelled, "iptal Cancelled olayi yayinlamali");

    std::fs::remove_dir_all(&dir).ok();
}

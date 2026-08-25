//! Bant genisligi sinirinin gercek bir indirmede uygulandigini dogrular.
//!
//! Birim testleri kovanin matematigini olcuyor; buradaki soru baska:
//! limitleyici indirme dongusunde **gercekten cagriliyor mu**. Kova dogru
//! olup da cagri yeri unutulsaydi ayar arayuzde durur, hicbir sey yapmazdi.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use vdrop_download::{start_download, DownloadEvent, DownloadOptions, RateLimiter};

/// Govdeyi tek seferde, beklemeden gonderen sunucu: olctugumuz gecikme
/// yalnizca limitleyiciden gelsin.
async fn spawn_fast_server(payload: &'static [u8]) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let _ = drain_request(&mut socket).await;
                let head = format!(
                    "HTTP/1.1 200 OK
Content-Length: {}
Connection: close

",
                    payload.len()
                );
                let _ = socket.write_all(head.as_bytes()).await;
                let _ = socket.write_all(payload).await;
                let _ = socket.shutdown().await;
            });
        }
    });
    addr
}

async fn drain_request(socket: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = vec![0u8; 4096];
    let _ = socket.read(&mut buf).await?;
    Ok(())
}

static PAYLOAD: &[u8] = &[9u8; 30_000];

async fn download_with(limiter: Option<Arc<RateLimiter>>) -> (Duration, u64) {
    let addr = spawn_fast_server(PAYLOAD).await;
    let dir = std::env::temp_dir().join(format!(
        "vdrop-rate-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("clip.bin");

    let mut opts = DownloadOptions::new(format!("http://{addr}/f"), dest.clone());
    if let Some(l) = limiter {
        opts = opts.with_rate_limiter(l);
    }

    let start = Instant::now();
    let mut handle = start_download(reqwest::Client::new(), opts);
    let mut total = 0;
    while let Some(ev) = handle.events.recv().await {
        match ev {
            DownloadEvent::Completed { total_bytes, .. } => {
                total = total_bytes;
                break;
            }
            DownloadEvent::Failed { message } => panic!("indirme basarisiz: {message}"),
            _ => {}
        }
    }
    let elapsed = start.elapsed();
    std::fs::remove_dir_all(&dir).ok();
    (elapsed, total)
}

#[tokio::test]
async fn an_unlimited_download_is_not_slowed_down() {
    let (elapsed, total) = download_with(None).await;
    assert_eq!(total, PAYLOAD.len() as u64);
    assert!(
        elapsed < Duration::from_secs(2),
        "sinirsiz indirme yerel sunucudan aninda bitmeliydi, {elapsed:?} surdu"
    );
}

#[tokio::test]
async fn the_limit_actually_paces_a_real_download() {
    // 30.000 bayt, 10.000 bayt/sn. Kova dolu basladigi icin ilk 10.000
    // bedava; kalan 20.000 icin ~2 saniye beklenir.
    let limiter = Arc::new(RateLimiter::new(10_000));
    let (elapsed, total) = download_with(Some(limiter)).await;

    assert_eq!(total, PAYLOAD.len() as u64, "sinirlama veri kaybettirmemeli");
    assert!(
        elapsed >= Duration::from_millis(1_800),
        "sinir uygulanmamis gorunuyor: {elapsed:?} (en az ~2 sn beklenirdi)"
    );
    assert!(
        elapsed < Duration::from_secs(6),
        "sinir gereginden fazla yavaslatiyor: {elapsed:?}"
    );
}

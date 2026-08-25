//! Gercek ag uzerinde uctan uca indirme testi.
//!
//! Birim testler sahte bir yerel HTTP sunucusu kullanir; hizli ve
//! deterministiktir ama gercek bir CDN'in davranisini kanitlamaz. Bu test
//! gercek bir sunucuya baglanir ve su zinciri dogrular:
//!
//!     istek -> Range ile devam -> .part dosyasi -> yeniden adlandirma
//!
//! `#[ignore]` ile isaretli: agsiz bir ortamda (CI konteyneri, ucak modu)
//! `cargo test` kirmizi olmamali. Elle calistirmak icin:
//!
//!     cargo test -p vdrop-download --test live_download -- --ignored --nocapture

use std::path::PathBuf;
use std::time::Duration;

use vdrop_download::{
    safe_join, start_download, unique_destination, DownloadEvent, DownloadOptions,
};

/// ~1 MB, Range destekli, uzun sureli kararli bir test varligi.
const TEST_URL: &str =
    "https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4";

fn scratch_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("vdrop-live-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn downloads_a_real_file_end_to_end() {
    let dir = scratch_dir("full");

    // Uretimdeki yolu birebir izle: guvenilmeyen ad -> safe_join -> benzersiz ad.
    let dest = unique_destination(&safe_join(&dir, "Buyuk Tavsan: 360p.mp4").unwrap());
    assert_eq!(
        dest.file_name().unwrap(),
        "Buyuk Tavsan_ 360p.mp4",
        "iki nokta ust uste Windows'ta gecersiz, sanitize edilmeliydi"
    );

    let client = reqwest::Client::new();
    let mut handle = start_download(client, DownloadOptions::new(TEST_URL, dest.clone()));

    let mut saw_progress = false;
    let mut total_from_event = None;

    let outcome = tokio::time::timeout(Duration::from_secs(120), async {
        while let Some(ev) = handle.events.recv().await {
            match ev {
                DownloadEvent::Progress { .. } => saw_progress = true,
                DownloadEvent::Completed { total_bytes, .. } => {
                    total_from_event = Some(total_bytes);
                    return Ok(());
                }
                DownloadEvent::Failed { message } => return Err(message),
                _ => {}
            }
        }
        Err("olay akisi tamamlanmadan kapandi".to_string())
    })
    .await
    .expect("indirme 120 saniyede bitmeliydi");

    outcome.expect("indirme basarisiz oldu");

    let on_disk = std::fs::metadata(&dest).expect("hedef dosya olusmali").len();
    assert!(on_disk > 900_000, "dosya beklenenden kucuk: {on_disk} bayt");
    assert_eq!(total_from_event, Some(on_disk), "bildirilen boyut diske uymali");
    assert!(saw_progress, "en az bir ilerleme olayi gelmeliydi");

    // .part dosyasi temizlenmis olmali: yarim dosya birakmiyoruz.
    let part = dest.with_file_name(format!(
        "{}.part",
        dest.file_name().unwrap().to_string_lossy()
    ));
    assert!(!part.exists(), ".part dosyasi tamamlaninca silinmeliydi");

    // Gercekten bir MP4 mi? Ilk kutu "ftyp" olmali.
    let head = std::fs::read(&dest).unwrap();
    assert_eq!(&head[4..8], b"ftyp", "MP4 imzasi bulunamadi");

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn resumes_a_partially_downloaded_real_file() {
    let dir = scratch_dir("resume");
    let dest = dir.join("resume-test.mp4");

    // Once dosyanin tamamini al: karsilastirma icin dogru referans.
    let client = reqwest::Client::new();
    let reference = client
        .get(TEST_URL)
        .send()
        .await
        .expect("referans istegi")
        .bytes()
        .await
        .expect("referans govdesi");

    // Simdi yarim kalmis bir indirme taklit et: ilk 300 KB'i .part olarak yaz.
    let part = dest.with_file_name("resume-test.mp4.part");
    std::fs::write(&part, &reference[..300_000]).unwrap();

    let mut handle = start_download(
        reqwest::Client::new(),
        DownloadOptions::new(TEST_URL, dest.clone()),
    );

    let result = tokio::time::timeout(Duration::from_secs(120), async {
        while let Some(ev) = handle.events.recv().await {
            match ev {
                DownloadEvent::Completed { .. } => return Ok(()),
                DownloadEvent::Failed { message } => return Err(message),
                _ => {}
            }
        }
        Err("olay akisi erken kapandi".to_string())
    })
    .await
    .expect("devam ettirme 120 saniyede bitmeliydi");

    result.expect("devam ettirme basarisiz oldu");

    let written = std::fs::read(&dest).unwrap();
    assert_eq!(
        written.len(),
        reference.len(),
        "devam ettirilen dosya tam boyutta olmali"
    );
    assert_eq!(
        written, reference,
        "devam ettirilen dosya bayt bayt orijinalle ayni olmali \
         (Range ofseti kayarsa burasi patlar)"
    );

    std::fs::remove_dir_all(&dir).ok();
}

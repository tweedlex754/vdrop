//! Varyant seciminin gercekten ise yaradigini kanitlar.
//!
//! Bu, HLS kalite secimindeki **en riskli varsayim**: `-map 0:p:N` gercekten
//! N numarali varyanti mi indiriyor, yoksa FFmpeg yine varsayilanini mi
//! seciyor? Ayristirici dogru calissa bile burasi yanlissa kullanici 1080p
//! secip 240p indirir - ve bunu ancak dosyayi acinca anlar.
//!
//! Test, manifestteki **varsayilan olmayan** bir varyanti secer (varsayilan
//! ilk siradaki 1280x720'dir) ve inen dosyanin cozunurlugunu olcer. Boylece
//! "zaten varsayilani indirmis olabilir" ihtimali eleniyor.
//!
//!     cargo test -p vdrop-media --test live_variant -- --ignored --nocapture

use std::path::Path;
use std::time::Duration;

use tokio::sync::watch;
use vdrop_media::{
    probe_duration_seconds, start_stream_download, ControlSignal, DownloadEvent, Ffmpeg,
    StreamDownloadOptions, StreamSelector,
};

const HLS_URL: &str = "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8";

/// Manifest sirasi: 0=720p, 1=320x184, 2=512x288, 3=848x480, 4=1920x1080.
/// En kucugu seciyoruz: hem varsayilan degil (kanit icin sart) hem de en
/// hizli inen (test suresi icin).
const VARIANT: u32 = 1;
const EXPECTED_WIDTH: i64 = 320;
const EXPECTED_HEIGHT: i64 = 184;

fn probe_dimensions(ffmpeg: &Ffmpeg, path: &Path) -> Option<(i64, i64)> {
    let ffprobe = ffmpeg.ffprobe.as_ref()?;
    let out = std::process::Command::new(ffprobe)
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0",
        ])
        .arg(path)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().next()?.trim();
    let (w, h) = line.split_once(',')?;
    Some((w.trim().parse().ok()?, h.trim().parse().ok()?))
}

fn has_audio(ffmpeg: &Ffmpeg, path: &Path) -> bool {
    let Some(ffprobe) = ffmpeg.ffprobe.as_ref() else {
        return true; // olcemiyorsak iddiada bulunmayalim
    };
    let Ok(out) = std::process::Command::new(ffprobe)
        .args([
            "-v", "error", "-select_streams", "a", "-show_entries", "stream=index", "-of", "csv=p=0",
        ])
        .arg(path)
        .output()
    else {
        return true;
    };
    !String::from_utf8_lossy(&out.stdout).trim().is_empty()
}

#[tokio::test]
#[ignore = "ag ve FFmpeg gerektirir"]
async fn downloads_the_selected_variant_not_the_default() {
    let Some(ff) = Ffmpeg::discover(None) else {
        eprintln!("FFmpeg kurulu degil - test atlaniyor");
        return;
    };

    let dir = std::env::temp_dir().join(format!("vdrop-variant-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("variant.mp4");

    let duration = probe_duration_seconds(&ff, HLS_URL).await;
    let (_control_tx, control_rx) = watch::channel(ControlSignal::Run);

    let mut events = start_stream_download(
        ff.clone(),
        StreamDownloadOptions {
            url: HLS_URL.to_string(),
            destination: dest.clone(),
            duration_seconds: duration,
            headers: Vec::new(),
            selector: Some(StreamSelector::Program(VARIANT)),
            subtitle: false,
        },
        control_rx,
    );

    let outcome = tokio::time::timeout(Duration::from_secs(300), async {
        while let Some(ev) = events.recv().await {
            match ev {
                DownloadEvent::Completed { total_bytes, .. } => return Ok(total_bytes),
                DownloadEvent::Failed { message } => return Err(message),
                _ => {}
            }
        }
        Err("olay akisi tamamlanmadan kapandi".to_string())
    })
    .await
    .expect("varyant indirmesi 5 dakikada bitmeliydi");

    let bytes = outcome.expect("indirme basarili olmali");
    assert!(bytes > 100_000, "dosya beklenenden kucuk: {bytes} bayt");

    let (w, h) = probe_dimensions(&ff, &dest).expect("cikti cozunurlugu okunabilmeli");
    assert_eq!(
        (w, h),
        (EXPECTED_WIDTH, EXPECTED_HEIGHT),
        "secilen varyant yerine baska bir cozunurluk indi. \
         Varsayilan (ilk varyant) 1280x720 - bu deger cikiyorsa -map 0:p:N \
         calismiyor demektir."
    );

    assert!(
        has_audio(&ff, &dest),
        "program secimi sesi de getirmeliydi; yalnizca video akisi \
         secilseydi dosya sessiz olurdu"
    );

    std::fs::remove_dir_all(&dir).ok();
}

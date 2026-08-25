//! DASH temsil seciminin gercekten ise yaradigini kanitlar.
//!
//! HLS'teki `live_variant` testinin DASH karsiligi - ve **ayni riski**
//! kapatir: `-map 0:v:N` gercekten N numarali temsili mi indiriyor?
//!
//! Iki mekanizma farkli oldugu icin HLS testinin gecmesi buranin da
//! calistigini gostermez:
//!   - HLS master  -> her varyant ayri bir program (`-map 0:p:N`)
//!   - DASH        -> tek programin ayri akislari (`-map 0:v:N`)
//!
//! Test varsayilan OLMAYAN bir temsili secer (varsayilan ilk siradaki
//! 1024x576'dir) ve inen dosyanin cozunurlugunu olcer.
//!
//!     cargo test -p vdrop-media --test live_dash -- --ignored --nocapture

use std::path::Path;
use std::time::Duration;

use tokio::sync::watch;
use vdrop_media::{
    probe_duration_seconds, start_stream_download, ControlSignal, DownloadEvent, Ffmpeg,
    StreamDownloadOptions, StreamSelector,
};

const MPD_URL: &str = "https://dash.akamaized.net/akamai/bbb_30fps/bbb_30fps.mpd";

/// Belge sirasi: 0=1024x576, 1=1280x720, 2=1920x1080, 3=320x180, ...
/// 3'u seciyoruz: varsayilan degil (kanit icin sart) ve en kucugu.
const REPRESENTATION: u32 = 3;
const EXPECTED: (i64, i64) = (320, 180);

fn dimensions(ff: &Ffmpeg, path: &Path) -> Option<(i64, i64)> {
    let ffprobe = ff.ffprobe.as_ref()?;
    let out = std::process::Command::new(ffprobe)
        .args([
            "-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height", "-of",
            "csv=p=0",
        ])
        .arg(path)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let (w, h) = text.lines().next()?.trim().split_once(',')?;
    Some((w.trim().parse().ok()?, h.trim().parse().ok()?))
}

fn has_audio(ff: &Ffmpeg, path: &Path) -> bool {
    let Some(ffprobe) = ff.ffprobe.as_ref() else {
        return true;
    };
    let Ok(out) = std::process::Command::new(ffprobe)
        .args([
            "-v", "error", "-select_streams", "a", "-show_entries", "stream=index", "-of",
            "csv=p=0",
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
async fn downloads_the_selected_dash_representation() {
    let Some(ff) = Ffmpeg::discover(None) else {
        eprintln!("FFmpeg kurulu degil - test atlaniyor");
        return;
    };

    let dir = std::env::temp_dir().join(format!("vdrop-dash-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("dash.mp4");

    let duration = probe_duration_seconds(&ff, MPD_URL).await;
    let (_tx, rx) = watch::channel(ControlSignal::Run);

    let mut events = start_stream_download(
        ff.clone(),
        StreamDownloadOptions {
            url: MPD_URL.to_string(),
            destination: dest.clone(),
            duration_seconds: duration,
            headers: Vec::new(),
            selector: Some(StreamSelector::VideoStream(REPRESENTATION)),
            subtitle: false,
        },
        rx,
    );

    let outcome = tokio::time::timeout(Duration::from_secs(420), async {
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
    .expect("DASH indirmesi 7 dakikada bitmeliydi");

    let bytes = outcome.expect("indirme basarili olmali");
    assert!(bytes > 100_000, "dosya beklenenden kucuk: {bytes} bayt");

    assert_eq!(
        dimensions(&ff, &dest).expect("cozunurluk okunabilmeli"),
        EXPECTED,
        "secilen temsil yerine baskasi indi. Varsayilan (ilk temsil) \
         1024x576 - bu deger cikiyorsa -map 0:v:N calismiyor demektir."
    );
    assert!(
        has_audio(&ff, &dest),
        "DASH'te ses ayri AdaptationSet'tedir; -map 0:a:0 olmadan dosya \
         sessiz kalirdi"
    );

    std::fs::remove_dir_all(&dir).ok();
}

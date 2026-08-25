//! Gercek bir HLS akisina karsi FFmpeg boru hattini dogrular.
//!
//! Kapsam: FFmpeg bulma -> ffprobe ile sure -> segment indirmeye baslama ->
//! ilerleme olaylari -> iptal -> yarim dosyanin temizlenmesi.
//!
//! Test bilincli olarak akisin **tamamini indirmez**: birkac saniye sonra
//! iptal eder. Amac mekanizmayi kanitlamak, 60 MB veri cekmek degil. Ustelik
//! iptal yolu boylece asil senaryosuyla test edilmis olur.
//!
//!     cargo test -p vdrop-media --test live_stream -- --ignored --nocapture

use std::time::Duration;

use tokio::sync::watch;
use vdrop_media::{
    is_stream_manifest, probe_duration_seconds, start_stream_download, ControlSignal,
    DownloadEvent, Ffmpeg, StreamDownloadOptions,
};

/// Mux'un herkese acik HLS test akisi (Big Buck Bunny, coklu bitrate).
const HLS_URL: &str = "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8";

fn require_ffmpeg() -> Option<Ffmpeg> {
    match Ffmpeg::discover(None) {
        Some(ff) => Some(ff),
        None => {
            eprintln!("FFmpeg kurulu degil - test atlaniyor");
            None
        }
    }
}

#[tokio::test]
#[ignore = "ag ve FFmpeg gerektirir"]
async fn probes_duration_of_a_real_hls_manifest() {
    let Some(ff) = require_ffmpeg() else { return };
    assert!(is_stream_manifest(HLS_URL));

    let duration = probe_duration_seconds(&ff, HLS_URL).await;
    let seconds = duration.expect("ffprobe gercek bir manifestten sure okuyabilmeli");
    assert!(
        seconds > 60.0,
        "beklenen sure birkac dakika, okunan: {seconds}"
    );
}

#[tokio::test]
#[ignore = "ag ve FFmpeg gerektirir"]
async fn downloads_hls_segments_then_cancels_cleanly() {
    let Some(ff) = require_ffmpeg() else { return };

    let dir = std::env::temp_dir().join(format!("vdrop-hls-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("stream.mp4");

    let duration = probe_duration_seconds(&ff, HLS_URL).await;
    let (control_tx, control_rx) = watch::channel(ControlSignal::Run);

    let mut events = start_stream_download(
        ff,
        StreamDownloadOptions {
            url: HLS_URL.to_string(),
            destination: dest.clone(),
            duration_seconds: duration,
            headers: Vec::new(),
            selector: None,
            subtitle: false,
        },
        control_rx,
    );

    let mut started = false;
    let mut bytes_seen = 0u64;
    let mut cancelled = false;

    let deadline = tokio::time::sleep(Duration::from_secs(90));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            ev = events.recv() => {
                let Some(ev) = ev else { break };
                match ev {
                    DownloadEvent::Started { .. } => started = true,
                    DownloadEvent::Progress { downloaded_bytes, .. } => {
                        bytes_seen = downloaded_bytes;
                        // Segmentlerin gercekten aktigini gorduk: simdi iptal
                        // yolunu test et.
                        if bytes_seen > 200_000 {
                            control_tx.send(ControlSignal::Cancel).unwrap();
                        }
                    }
                    DownloadEvent::Cancelled => { cancelled = true; break; }
                    DownloadEvent::Failed { message } => panic!("akis basarisiz: {message}"),
                    DownloadEvent::Completed { .. } => break, // beklenmedik ama hata degil
                    _ => {}
                }
            }
            _ = &mut deadline => panic!("90 saniyede hicbir sey olmadi"),
        }
    }

    assert!(started, "Started olayi gelmeliydi");
    assert!(
        bytes_seen > 200_000,
        "FFmpeg segmentleri yazmaliydi, gorulen: {bytes_seen} bayt"
    );
    assert!(cancelled, "iptal sinyali Cancelled olayina donmeliydi");

    // Yarim kalan mp4 oynatilamaz; kullaniciya bozuk dosya birakmiyoruz.
    assert!(
        !dest.exists(),
        "iptal edilen akisin yarim ciktisi silinmeliydi"
    );

    std::fs::remove_dir_all(&dir).ok();
}

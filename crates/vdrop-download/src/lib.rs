//! vdrop-download: native, dependency-free (no Python/yt-dlp) resumable HTTP
//! download engine for VDrop.
//!
//! Implements the core of PRD section 9 (Native Download Engine):
//! - HTTP(S) GET with Range-based resume
//! - pause / resume / cancel via a control channel
//! - progress events (bytes downloaded, total, speed, ETA) via mpsc
//! - retry with exponential backoff on transient network errors
//! - atomic-ish writes: data is written to `<file>.part` and renamed on completion
//!
//! Segmented/multipart and HLS/DASH downloading build on top of this module
//! (see `docs/ARCHITECTURE.md`, section J) but are out of scope for this
//! first working slice.

pub mod paths;
pub use paths::{
    safe_join, sanitize_filename, unique_destination, unique_destination_with, PathError,
};

pub mod rate;

use std::sync::Arc;

pub use rate::RateLimiter;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use thiserror::Error;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::{mpsc, watch};

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("server does not support resuming this download")]
    ResumeUnsupported,
    #[error("download was cancelled")]
    Cancelled,
    /// Kullanici duraklatti. Hata degil, kontrollu bir cikis: `.part` dosyasi
    /// diskte kalir ve `Range` ile kaldigi yerden devam ettirilebilir.
    #[error("download was paused")]
    Paused,
    #[error("server returned an unexpected status: {0}")]
    BadStatus(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSignal {
    Run,
    Pause,
    Cancel,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DownloadEvent {
    Started { total_bytes: Option<u64> },
    Progress {
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
        speed_bps: f64,
        eta_seconds: Option<u64>,
    },
    Paused { downloaded_bytes: u64 },
    Retrying { attempt: u32, delay_ms: u64 },
    Completed { path: PathBuf, total_bytes: u64 },
    Failed { message: String },
    Cancelled,
}

pub struct DownloadHandle {
    pub control: watch::Sender<ControlSignal>,
    pub events: mpsc::Receiver<DownloadEvent>,
}

pub struct DownloadOptions {
    pub url: String,
    pub destination: PathBuf,
    pub max_retries: u32,
    /// Optional extra headers (e.g. cookies for authenticated media, per PRD §77).
    pub headers: Vec<(String, String)>,
    /// Bant genisligi siniri. **Paylasilan**: ayni `Arc` tum indirmelere
    /// verilir, cunku kullanicinin istedigi toplam hiz sinirdir. `None`
    /// sinirsiz demektir.
    pub rate_limiter: Option<Arc<RateLimiter>>,
}

impl DownloadOptions {
    pub fn new(url: impl Into<String>, destination: impl Into<PathBuf>) -> Self {
        Self {
            url: url.into(),
            destination: destination.into(),
            max_retries: 5,
            headers: Vec::new(),
            rate_limiter: None,
        }
    }

    /// Indirmeyi paylasilan bir hiz sinirina baglar.
    pub fn with_rate_limiter(mut self, limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }
}

/// Starts a resumable download. Returns a handle exposing a control channel
/// (pause/resume/cancel) and an event stream (progress/state).
pub fn start_download(client: reqwest::Client, opts: DownloadOptions) -> DownloadHandle {
    let (control_tx, control_rx) = watch::channel(ControlSignal::Run);
    let events = start_download_with_control(client, opts, control_rx);

    DownloadHandle {
        control: control_tx,
        events,
    }
}

/// Kontrol kanalini disaridan alan varyant.
///
/// Cagiran taraf gonderici ucu kendisi tuttugu icin indirme **daha
/// baslamadan** (ornegin es zamanlilik kuyrugunda beklerken) iptal
/// edilebilir. `start_download` bunun uzerine kurulu ince bir sarmalayicidir.
pub fn start_download_with_control(
    client: reqwest::Client,
    opts: DownloadOptions,
    control: watch::Receiver<ControlSignal>,
) -> mpsc::Receiver<DownloadEvent> {
    let (event_tx, event_rx) = mpsc::channel(64);
    tokio::spawn(run_download(client, opts, control, event_tx));
    event_rx
}

async fn run_download(
    client: reqwest::Client,
    opts: DownloadOptions,
    mut control: watch::Receiver<ControlSignal>,
    events: mpsc::Sender<DownloadEvent>,
) {
    let part_path = part_path(&opts.destination);
    let mut attempt = 0u32;

    loop {
        match try_download_once(&client, &opts, &part_path, &mut control, &events).await {
            Ok(total) => {
                // finalize: <file>.part -> <file>
                if let Err(e) = fs::rename(&part_path, &opts.destination).await {
                    let _ = events
                        .send(DownloadEvent::Failed {
                            message: format!("could not finalize file: {e}"),
                        })
                        .await;
                    return;
                }
                let _ = events
                    .send(DownloadEvent::Completed {
                        path: opts.destination.clone(),
                        total_bytes: total,
                    })
                    .await;
                return;
            }
            Err(DownloadError::Cancelled) => {
                let _ = events.send(DownloadEvent::Cancelled).await;
                return;
            }
            Err(DownloadError::Paused) => {
                // Duraklatma gorevi sonlandirir: baglanti kapanir, es
                // zamanlilik yuvasi serbest kalir. Devam etme, transferi
                // yeniden baslatir ve motor diskteki `.part` dosyasini
                // gorup Range ile kaldigi yerden surdurur.
                let downloaded = fs::metadata(&part_path)
                    .await
                    .map(|m| m.len())
                    .unwrap_or(0);
                let _ = events
                    .send(DownloadEvent::Paused {
                        downloaded_bytes: downloaded,
                    })
                    .await;
                return;
            }
            Err(e) if is_transient(&e) && attempt < opts.max_retries => {
                attempt += 1;
                let delay = backoff_delay(attempt);
                let _ = events
                    .send(DownloadEvent::Retrying {
                        attempt,
                        delay_ms: delay.as_millis() as u64,
                    })
                    .await;
                tokio::time::sleep(delay).await;
                continue;
            }
            Err(e) => {
                let _ = events
                    .send(DownloadEvent::Failed {
                        message: e.to_string(),
                    })
                    .await;
                return;
            }
        }
    }
}

/// Yalnizca ag/io hatalari yeniden denenir. `Cancelled` ve `Paused`
/// kullanici kararidir; onlari yeniden denemek kullaniciyla tartismak olurdu.
fn is_transient(e: &DownloadError) -> bool {
    matches!(e, DownloadError::Network(_) | DownloadError::Io(_))
}

/// Exponential backoff: 1s, 2s, 4s, 8s, 16s (capped), per PRD §34.
fn backoff_delay(attempt: u32) -> Duration {
    let secs = 1u64.wrapping_shl(attempt.saturating_sub(1).min(4));
    Duration::from_secs(secs.min(16))
}

fn part_path(dest: &Path) -> PathBuf {
    let mut p = dest.to_path_buf();
    let name = p
        .file_name()
        .map(|n| format!("{}.part", n.to_string_lossy()))
        .unwrap_or_else(|| "download.part".to_string());
    p.set_file_name(name);
    p
}

async fn try_download_once(
    client: &reqwest::Client,
    opts: &DownloadOptions,
    part_path: &Path,
    control: &mut watch::Receiver<ControlSignal>,
    events: &mpsc::Sender<DownloadEvent>,
) -> Result<u64, DownloadError> {
    // How much have we already got on disk? (resume support, PRD §9)
    let mut already: u64 = match fs::metadata(part_path).await {
        Ok(m) => m.len(),
        Err(_) => 0,
    };

    let mut req = client.get(&opts.url);
    for (k, v) in &opts.headers {
        req = req.header(k, v);
    }
    if already > 0 {
        req = req.header("Range", format!("bytes={already}-"));
    }

    let resp = req.send().await?;
    let status = resp.status();

    let (mut file, resuming) = if status.as_u16() == 206 {
        // Server honored our Range request.
        let f = OpenOptions::new().append(true).open(part_path).await?;
        (f, true)
    } else if status.is_success() {
        // Server ignored/doesn't support Range: start from scratch.
        already = 0;
        let f = File::create(part_path).await?;
        (f, false)
    } else {
        return Err(DownloadError::BadStatus(status.as_u16()));
    };
    let _ = resuming;

    let content_length = resp.content_length();
    let total_bytes = match (content_length, status.as_u16() == 206) {
        (Some(len), true) => Some(already + len),
        (Some(len), false) => Some(len),
        (None, _) => None,
    };

    let _ = events
        .send(DownloadEvent::Started { total_bytes })
        .await;

    let mut downloaded = already;
    let mut stream = resp.bytes_stream();
    let mut last_report = Instant::now();
    let mut bytes_since_report: u64 = 0;
    let start = Instant::now();

    while let Some(chunk) = stream.next().await {
        // Cooperative pause/cancel check between chunks.
        let signal = *control.borrow();
        match signal {
            ControlSignal::Cancel => return Err(DownloadError::Cancelled),
            // Duraklatmada baglantiyi acik tutup beklemiyoruz. Iki sebep:
            //  1. Bekleyen gorev es zamanlilik yuvasini isgal ederdi; limit 3
            //     iken 3 indirmeyi duraklatan kullanici siradakileri
            //     baslatamazdi - yani ozelligin var olma sebebi calismazdi.
            //  2. Uzun duraklamalarda sunucu bosta duran baglantiyi zaten
            //     dusurur ve bu, yeniden deneme sayacini bosa yakar.
            // Bunun yerine temiz cikiyoruz; `.part` diskte kaliyor.
            ControlSignal::Pause => {
                file.flush().await?;
                return Err(DownloadError::Paused);
            }
            ControlSignal::Run => {}
        }

        let chunk = chunk?;

        // Bedelini yazmadan ONCE ode. Boylece bekleme diske degil okumaya
        // yansiyor: yavaslattigimiz an TCP penceresi daraliyor ve sunucu
        // gercekten yavasliyor. Yazdiktan sonra beklemek ayni toplam hizi
        // verirdi ama bellekte veri biriktirirdi.
        if let Some(limiter) = &opts.rate_limiter {
            limiter.acquire(chunk.len()).await;
        }

        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        bytes_since_report += chunk.len() as u64;

        if last_report.elapsed() >= Duration::from_millis(400) {
            let elapsed = last_report.elapsed().as_secs_f64().max(0.001);
            let speed_bps = bytes_since_report as f64 / elapsed;
            let eta_seconds = total_bytes.and_then(|t| {
                if speed_bps > 0.0 && t > downloaded {
                    Some(((t - downloaded) as f64 / speed_bps) as u64)
                } else {
                    None
                }
            });
            let _ = events
                .send(DownloadEvent::Progress {
                    downloaded_bytes: downloaded,
                    total_bytes,
                    speed_bps,
                    eta_seconds,
                })
                .await;
            last_report = Instant::now();
            bytes_since_report = 0;
        }
    }

    file.flush().await?;
    // Seek is only needed if a caller reused the handle; kept for clarity/future segments.
    let _ = file.seek(std::io::SeekFrom::End(0)).await;

    // Son bir ilerleme olayi: 400 ms'lik yayin araligi, hizli biten kucuk
    // dosyalarda tek bir olay bile uretmiyordu - kullanici %0 gorup birden
    // "tamamlandi"ya atliyordu ve veritabanina hic bayt yazilmiyordu.
    // Buradaki hiz tum oturumun ortalamasidir, anlik degil.
    let elapsed = start.elapsed().as_secs_f64().max(0.001);
    let _ = events
        .send(DownloadEvent::Progress {
            downloaded_bytes: downloaded,
            total_bytes: total_bytes.or(Some(downloaded)),
            speed_bps: (downloaded.saturating_sub(already)) as f64 / elapsed,
            eta_seconds: Some(0),
        })
        .await;

    Ok(downloaded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    use tokio::io::AsyncReadExt;
    use tokio::net::{TcpListener, TcpStream};

    /// Minimal single-request HTTP/1.1 server used only for tests: serves a
    /// fixed in-memory payload and honors byte-range requests, so we can
    /// verify pause/resume/full-download behavior with zero external
    /// network access (this sandbox only allows crates.io/npm-style hosts).
    async fn spawn_mock_server(payload: &'static [u8]) -> std::net::SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            loop {
                let (mut socket, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(_) => return,
                };
                tokio::spawn(async move {
                    handle_conn(&mut socket, payload).await;
                });
            }
        });
        addr
    }

    async fn handle_conn(socket: &mut TcpStream, payload: &'static [u8]) {
        let mut buf = vec![0u8; 4096];
        let n = match socket.read(&mut buf).await {
            Ok(n) if n > 0 => n,
            _ => return,
        };
        let req = String::from_utf8_lossy(&buf[..n]);
        let range = req
            .lines()
            .find(|l| l.to_lowercase().starts_with("range:"))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|r| r.trim_end_matches("\r").split('-').next())
            .and_then(|s| s.parse::<usize>().ok());

        let mut out = Vec::new();
        if let Some(start) = range {
            let body = &payload[start.min(payload.len())..];
            write!(
                out,
                "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            out.extend_from_slice(body);
        } else {
            write!(
                out,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nAccept-Ranges: bytes\r\nConnection: close\r\n\r\n",
                payload.len()
            )
            .unwrap();
            out.extend_from_slice(payload);
        }
        let _ = socket.write_all(&out).await;
        let _ = socket.shutdown().await;
    }

    #[tokio::test]
    async fn downloads_full_file() {
        static PAYLOAD: &[u8] = b"VDrop native download engine test payload - 0123456789";
        let addr = spawn_mock_server(PAYLOAD).await;

        let dir = tempdir();
        let dest = dir.join("video.mp4");
        let client = reqwest::Client::new();
        let opts = DownloadOptions::new(format!("http://{addr}/file"), dest.clone());

        let mut handle = start_download(client, opts);
        let mut completed_len = None;
        while let Some(ev) = handle.events.recv().await {
            if let DownloadEvent::Completed { total_bytes, .. } = ev {
                completed_len = Some(total_bytes);
                break;
            }
            if let DownloadEvent::Failed { message } = ev {
                panic!("download failed: {message}");
            }
        }

        assert_eq!(completed_len, Some(PAYLOAD.len() as u64));
        let written = std::fs::read(dest).unwrap();
        assert_eq!(written, PAYLOAD);
    }

    #[tokio::test]
    async fn resumes_partial_file() {
        static PAYLOAD: &[u8] = b"resume-me-please-this-is-a-longer-payload-for-range-testing";
        let addr = spawn_mock_server(PAYLOAD).await;

        let dir = tempdir();
        let dest = dir.join("video.mp4");
        let part = super::part_path(&dest);
        // Pre-seed a partial ".part" file to simulate a previously interrupted download.
        std::fs::write(&part, &PAYLOAD[..10]).unwrap();

        let client = reqwest::Client::new();
        let opts = DownloadOptions::new(format!("http://{addr}/file"), dest.clone());
        let mut handle = start_download(client, opts);

        loop {
            match handle.events.recv().await {
                Some(DownloadEvent::Completed { .. }) => break,
                Some(DownloadEvent::Failed { message }) => panic!("failed: {message}"),
                Some(_) => continue,
                None => panic!("event stream closed early"),
            }
        }

        let written = std::fs::read(dest).unwrap();
        assert_eq!(written, PAYLOAD, "resumed download must equal full payload");
    }

    fn tempdir() -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("vdrop-test-{}", uuid_like()));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn uuid_like() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        format!(
            "{}-{:?}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
            std::thread::current().id()
        )
    }
}

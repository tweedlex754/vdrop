//! vdrop-ytdlp: yt-dlp ile site-ozel cikarim.
//!
//! ## Neden yt-dlp
//!
//! Projenin ilk tasarimi site-ozel extractor'lari sandboxli bir JS calisma
//! zamaniyla kendi icinde yazmayi ongoruyordu (`docs/ARCHITECTURE.md` K2).
//! Bu, yt-dlp'nin yillardir yaptigi isi bastan yapmak demek: yuzlerce site,
//! her biri kendi imza/token/cipher mantigiyla, ve hepsi surekli degisiyor.
//!
//! Daha durust cozum: o isi zaten yapan araca devretmek.
//!
//! ## Opsiyonel bilesen
//!
//! yt-dlp **zorunlu degil**. Kurulu degilse VDrop eskisi gibi calisir:
//! dogrudan baglantilar, HLS/DASH manifestleri ve sayfa cikarimi. Kuruluysa
//! kapsam yuzlerce siteye acilir. Ayni FFmpeg gibi: bulunursa yetenek acilir,
//! bulunmazsa uygulama calismaya devam eder.
//!
//! ## Neden indirmeyi de yt-dlp yapiyor
//!
//! `-J` ciktisi format URL'leri verir ve bunlari kendi motorumuzla
//! indirebilirdik. Ama:
//!
//!   - YouTube gibi sitelerde en yuksek kalite **ayri video + ayri ses**tir;
//!     birlestirme gerekir.
//!   - Bircok format URL'i kisa omurlu imzali adreslerdir; duraklatip bir
//!     saat sonra devam etmek 403 verir. yt-dlp gerektiginde yeniden cozer.
//!   - Cerez, referer, throttling ve yeniden deneme mantigi zaten oradadir.
//!
//! Duraklatma/devam etme yine bizim modelimizle calisir: duraklatma sureci
//! sonlandirir, `.part` diskte kalir, devam etme `--continue` ile yeniden
//! baslatir - HTTP motorumuzdaki semantigin aynisi.

pub mod provider;
pub use provider::YtDlpProvider;

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use serde::Deserialize;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{mpsc, watch};

// ---------------------------------------------------------------------------
// Alt surecler ve konsol penceresi
// ---------------------------------------------------------------------------

/// `CREATE_NO_WINDOW`: alt surec kendi konsolunu acmasin.
///
/// Uygulama GUI olarak derleniyor (`windows_subsystem = "windows"`), ama bu
/// yalnizca **kendi** surecini baglar. Baslattigi her alt surec (ffmpeg,
/// ffprobe, yt-dlp) Windows'ta kendi konsol penceresini aciyordu: kullanici
/// her cozumlemede ve her indirmede ekranda parlayan siyah bir pencere
/// goruyordu. Ilerleme zaten borulardan okunup arayuze yayinlandigi icin o
/// pencerenin hicbir islevi yok.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Async (tokio) surecler icin konsolu bastirir. Windows disinda islemsizdir.
fn quiet_async(cmd: &mut Command) -> &mut Command {
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

/// Bloklayan (std) surecler icin ayni sey - surum yoklamalari boyle yapiliyor.
fn quiet_sync(cmd: &mut std::process::Command) -> &mut std::process::Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

pub use vdrop_download::{ControlSignal, DownloadEvent};

#[derive(Debug, Error)]
pub enum YtDlpError {
    #[error("yt-dlp bulunamadi")]
    Missing,
    #[error("yt-dlp calistirilamadi: {0}")]
    Spawn(String),
    #[error("bu adres yt-dlp tarafindan desteklenmiyor")]
    Unsupported,
    #[error("yt-dlp hata verdi: {0}")]
    Failed(String),
    #[error("yt-dlp ciktisi cozumlenemedi: {0}")]
    Parse(String),
    #[error("io hatasi: {0}")]
    Io(#[from] std::io::Error),
}

/// Sistemde bulunan yt-dlp.
#[derive(Debug, Clone)]
pub struct YtDlp {
    pub path: PathBuf,
    pub version: String,
}

impl YtDlp {
    /// Once uygulamanin kendi `bin/` klasorunde, sonra PATH'te arar.
    ///
    /// Uygulama-yerel dizine oncelik vermek bilincli: VDrop yt-dlp'yi kendisi
    /// indirip guncelleyebilir (Ayarlar > Bilesenler) ve o surum, sistemdeki
    /// eski bir kurulumun golgesinde kalmamali.
    pub fn discover(app_bin_dir: Option<&Path>) -> Option<Self> {
        let exe = if cfg!(windows) { "yt-dlp.exe" } else { "yt-dlp" };
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(dir) = app_bin_dir {
            candidates.push(dir.join(exe));
        }
        candidates.push(PathBuf::from(exe));

        for cand in candidates {
            if let Some(version) = probe_version(&cand) {
                return Some(Self {
                    path: cand,
                    version,
                });
            }
        }
        None
    }
}

fn probe_version(path: &Path) -> Option<String> {
    let mut cmd = std::process::Command::new(path);
    let out = quiet_sync(&mut cmd)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let v = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!v.is_empty()).then_some(v)
}

// ---------------------------------------------------------------------------
// Cozumleme (-J)
// ---------------------------------------------------------------------------

/// yt-dlp'nin `-J` ciktisindan ihtiyacimiz olan alanlar.
///
/// Sema genis; yalnizca kullandiklarimizi tanimliyoruz. Bilinmeyen alanlar
/// sessizce yok sayilir, boylece yt-dlp guncellenip yeni alanlar ekledigi
/// zaman cozumleme bozulmaz.
#[derive(Debug, Deserialize)]
pub struct Info {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub uploader: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub upload_date: Option<String>,
    #[serde(default)]
    pub extractor: Option<String>,
    #[serde(default)]
    pub webpage_url: Option<String>,
    #[serde(default)]
    pub formats: Vec<Format>,
    /// Playlist ciktisinda tekil video yerine `entries` gelir.
    #[serde(default)]
    pub entries: Option<Vec<Info>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Format {
    pub format_id: String,
    #[serde(default)]
    pub ext: Option<String>,
    #[serde(default)]
    pub vcodec: Option<String>,
    #[serde(default)]
    pub acodec: Option<String>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub fps: Option<f32>,
    /// Toplam bit hizi (kbps). Bazi formatlarda yalnizca vbr/abr gelir.
    #[serde(default)]
    pub tbr: Option<f64>,
    #[serde(default)]
    pub vbr: Option<f64>,
    #[serde(default)]
    pub abr: Option<f64>,
    #[serde(default)]
    pub filesize: Option<u64>,
    #[serde(default)]
    pub filesize_approx: Option<u64>,
    #[serde(default)]
    pub format_note: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
}

impl Format {
    /// Alan **yoksa** "bilinmiyor" demektir, "yok" degil.
    ///
    /// Bazi cikaricilar kodek alanlarini hic doldurmaz (archive.org gibi);
    /// yt-dlp onlari "unknown" diye listeler ve sorunsuz indirir. Eksik alani
    /// "goruntu yok" saymak tum formatlari eliyor ve saglayiciyi devre disi
    /// birakiyordu. Yalnizca acikca "none" diyen alan yoklugu ifade eder.
    pub fn has_video(&self) -> bool {
        !matches!(self.vcodec.as_deref(), Some("none") | Some(""))
    }

    pub fn has_audio(&self) -> bool {
        !matches!(self.acodec.as_deref(), Some("none") | Some(""))
    }

    pub fn bitrate_kbps(&self) -> Option<u32> {
        self.tbr
            .or(self.vbr)
            .or(self.abr)
            .map(|b| b.round() as u32)
            .filter(|b| *b > 0)
    }

    pub fn size_bytes(&self) -> Option<u64> {
        self.filesize.or(self.filesize_approx)
    }

    pub fn resolution(&self) -> Option<String> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some(format!("{w}x{h}")),
            // Bazi formatlarda yalnizca yukseklik gelir.
            (None, Some(h)) => Some(format!("{h}p")),
            _ => None,
        }
    }

    /// Insan okur kodek adi: "avc1.640028" -> "H.264".
    pub fn codec_label(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(v) = self.vcodec.as_deref().filter(|v| *v != "none" && !v.is_empty()) {
            parts.push(pretty_codec(v));
        }
        if let Some(a) = self.acodec.as_deref().filter(|a| *a != "none" && !a.is_empty()) {
            parts.push(pretty_codec(a));
        }
        (!parts.is_empty()).then(|| parts.join(" · "))
    }
}

fn pretty_codec(code: &str) -> String {
    let family = code.split(['.', '-']).next().unwrap_or(code).to_lowercase();
    match family.as_str() {
        "avc1" | "avc3" | "h264" => "H.264",
        "hev1" | "hvc1" | "h265" => "H.265",
        "av01" => "AV1",
        "vp9" | "vp09" => "VP9",
        "vp8" | "vp08" => "VP8",
        "mp4a" | "aac" => "AAC",
        "opus" => "Opus",
        "mp3" => "MP3",
        "vorbis" => "Vorbis",
        "flac" => "FLAC",
        "ec3" | "eac3" => "E-AC-3",
        _ => return code.to_string(),
    }
    .to_string()
}

/// `yt-dlp -J` calistirip cikarim sonucunu dondurur.
pub async fn resolve(ytdlp: &YtDlp, url: &str) -> Result<Info, YtDlpError> {
    let mut cmd = Command::new(&ytdlp.path);
    let out = quiet_async(&mut cmd)
        .args([
            "-J",
            "--no-warnings",
            "--no-progress",
            // Playlist adresleri tek videoya indirgenir: kullanici bir
            // baglanti yapistirdi, 200 videoluk bir kuyruk beklemiyor.
            "--no-playlist",
            // Cikarim asamasinda hicbir sey indirilmemeli.
            "--skip-download",
            "--socket-timeout",
            "20",
        ])
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| YtDlpError::Spawn(e.to_string()))?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        // "Unsupported URL" ozel bir durum: baska bir saglayici deneyebilsin
        // diye "desteklenmiyor" olarak isaretliyoruz, genel hata olarak degil.
        if err.contains("Unsupported URL") || err.contains("is not a valid URL") {
            return Err(YtDlpError::Unsupported);
        }
        return Err(YtDlpError::Failed(first_meaningful_error(&err)));
    }

    let text = String::from_utf8_lossy(&out.stdout);
    let mut info: Info =
        serde_json::from_str(&text).map_err(|e| YtDlpError::Parse(e.to_string()))?;

    // `--no-playlist`e ragmen bazi cikaricılar playlist dondurur; ilk girdiyi
    // aliyoruz - kullanicinin yapistirdigi adres oydu.
    if info.formats.is_empty() {
        if let Some(first) = info.entries.as_mut().and_then(|e| e.drain(..).next()) {
            info = first;
        }
    }

    if info.formats.is_empty() {
        return Err(YtDlpError::Parse("format listesi bos".into()));
    }
    Ok(info)
}

/// yt-dlp stderr'i cok satirlidir; kullaniciya gosterilecek satiri secer.
fn first_meaningful_error(stderr: &str) -> String {
    let line = stderr
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .find(|l| l.starts_with("ERROR:"))
        .or_else(|| stderr.lines().map(str::trim).rev().find(|l| !l.is_empty()))
        .unwrap_or("bilinmeyen hata");
    line.trim_start_matches("ERROR:")
        .trim()
        .chars()
        .take(300)
        .collect()
}

// ---------------------------------------------------------------------------
// Indirme
// ---------------------------------------------------------------------------

pub struct DownloadOptions {
    pub url: String,
    /// yt-dlp format kimligi. `None` ise yt-dlp kendi en iyisini secer.
    pub format_id: Option<String>,
    pub destination: PathBuf,
    /// Bayt/sn siniri. `None` ya da `0` sinirsiz.
    ///
    /// Kendi HTTP motorumuzun paylasilan kovasini buraya tasiyamiyoruz:
    /// indirmeyi ayri bir surec yapiyor. Bunun yerine yt-dlp'nin kendi
    /// `--limit-rate` bayragini kullaniyoruz. Sonuc olarak ayni anda hem
    /// yt-dlp hem duz HTTP indirmesi kosuyorsa toplam hiz siniri asabilir;
    /// her motor kendi payini uygular.
    pub rate_limit_bytes: Option<u64>,
}

/// Ilerlemenin makine okunur gelmesi icin sablon. Alanlar bosalabilir
/// (`NA`), o yuzden ayristirici hosgorulu olmali.
const PROGRESS_TEMPLATE: &str =
    "VDROP %(progress.downloaded_bytes)s %(progress.total_bytes)s %(progress.total_bytes_estimate)s %(progress.speed)s %(progress.eta)s";

pub fn start_download(
    ytdlp: YtDlp,
    opts: DownloadOptions,
    control: watch::Receiver<ControlSignal>,
) -> mpsc::Receiver<DownloadEvent> {
    let (tx, rx) = mpsc::channel(64);
    tokio::spawn(async move {
        if let Err(e) = run(ytdlp, opts, control, tx.clone()).await {
            let _ = tx
                .send(DownloadEvent::Failed {
                    message: e.to_string(),
                })
                .await;
        }
    });
    rx
}

async fn run(
    ytdlp: YtDlp,
    opts: DownloadOptions,
    mut control: watch::Receiver<ControlSignal>,
    events: mpsc::Sender<DownloadEvent>,
) -> Result<(), YtDlpError> {
    let mut cmd = Command::new(&ytdlp.path);
    quiet_async(&mut cmd);
    if let Some(limit) = opts.rate_limit_bytes.filter(|v| *v > 0) {
        cmd.arg("--limit-rate").arg(limit.to_string());
    }
    cmd.args([
        "--no-warnings",
        "--no-playlist",
        "--newline",
        // Yarim kalan indirmeyi surdur: duraklatma sureci sonlandirdigi icin
        // "devam et" bunun uzerine kurulu.
        "--continue",
        "--progress-template",
    ])
    .arg(PROGRESS_TEMPLATE);

    if let Some(id) = &opts.format_id {
        // Ayri video+ses formatlari icin `+bestaudio` yedegi: kullanici
        // yalnizca goruntu tasiyan bir format sectiyse ses de eklenir,
        // yoksa sessiz dosya cikardi. `/` alternatif operatorudur.
        cmd.arg("-f").arg(format!("{id}+bestaudio/{id}"));
    }

    cmd.arg("-o")
        .arg(&opts.destination)
        .arg(&opts.url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| YtDlpError::Spawn(e.to_string()))?;
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");

    let err_handle = tokio::spawn(async move {
        let mut buf = String::new();
        let mut lines = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if buf.len() < 4000 {
                buf.push_str(&line);
                buf.push('\n');
            }
        }
        buf
    });

    let _ = events.send(DownloadEvent::Started { total_bytes: None }).await;

    let mut reader = BufReader::new(stdout).lines();
    let mut last_report = Instant::now();
    let mut downloaded = 0u64;
    let mut stopped: Option<ControlSignal> = None;

    loop {
        tokio::select! {
            line = reader.next_line() => {
                let Ok(Some(line)) = line else { break };
                let Some(progress) = parse_progress(&line) else { continue };
                downloaded = progress.downloaded;

                if last_report.elapsed() >= Duration::from_millis(400) {
                    let _ = events
                        .send(DownloadEvent::Progress {
                            downloaded_bytes: progress.downloaded,
                            total_bytes: progress.total,
                            speed_bps: progress.speed.unwrap_or(0.0),
                            eta_seconds: progress.eta,
                        })
                        .await;
                    last_report = Instant::now();
                }
            }
            changed = control.changed() => {
                if changed.is_err() { continue; }
                let signal = *control.borrow();
                if matches!(signal, ControlSignal::Cancel | ControlSignal::Pause) {
                    stopped = Some(signal);
                    let _ = child.start_kill();
                    break;
                }
            }
        }
    }

    let status = child.wait().await?;
    let stderr_text = err_handle.await.unwrap_or_default();

    match stopped {
        Some(ControlSignal::Pause) => {
            // `.part` dosyasi diskte kaliyor; "devam et" `--continue` ile
            // kaldigi yerden surdurur.
            let _ = events
                .send(DownloadEvent::Paused {
                    downloaded_bytes: downloaded,
                })
                .await;
            return Ok(());
        }
        Some(ControlSignal::Cancel) => {
            cleanup_partials(&opts.destination).await;
            let _ = events.send(DownloadEvent::Cancelled).await;
            return Ok(());
        }
        _ => {}
    }

    if !status.success() {
        return Err(YtDlpError::Failed(first_meaningful_error(&stderr_text)));
    }

    let final_size = tokio::fs::metadata(&opts.destination)
        .await
        .map(|m| m.len())
        .unwrap_or(downloaded);

    let _ = events
        .send(DownloadEvent::Completed {
            path: opts.destination.clone(),
            total_bytes: final_size,
        })
        .await;
    Ok(())
}

/// Iptal edilen indirmenin yarim dosyalarini temizler.
///
/// yt-dlp birlestirme yaparken birden cok gecici dosya birakabilir
/// (`ad.f137.mp4`, `ad.part`), bu yuzden yalnizca hedefi silmek yetmez.
async fn cleanup_partials(dest: &Path) {
    tokio::fs::remove_file(dest).await.ok();
    let Some(dir) = dest.parent() else { return };
    let Some(stem) = dest.file_stem().map(|s| s.to_string_lossy().to_string()) else {
        return;
    };
    let Ok(mut entries) = tokio::fs::read_dir(dir).await else {
        return;
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        let name = entry.file_name().to_string_lossy().to_string();
        // Yalnizca bu indirmeye ait gecici dosyalar: ayni govde + gecici uzanti.
        let temporary = name.ends_with(".part")
            || name.ends_with(".ytdl")
            || name.contains(".part-Frag");
        if temporary && name.starts_with(&stem) {
            tokio::fs::remove_file(entry.path()).await.ok();
        }
    }
}

#[derive(Debug, PartialEq)]
struct Progress {
    downloaded: u64,
    total: Option<u64>,
    speed: Option<f64>,
    eta: Option<u64>,
}

/// `VDROP <indirilen> <toplam> <tahmini_toplam> <hiz> <kalan>` satirini okur.
///
/// yt-dlp bilinmeyen alanlar icin "NA" yazar; bazi surumlerde alan tamamen
/// bos gelebilir. Ayristirici bunlarin hepsinde `None` uretir, hata degil.
fn parse_progress(line: &str) -> Option<Progress> {
    let rest = line.trim().strip_prefix("VDROP ")?;
    let mut parts = rest.split_whitespace();

    let downloaded = number(parts.next())? as u64;
    let total = number(parts.next()).map(|v| v as u64);
    let estimate = number(parts.next()).map(|v| v as u64);
    let speed = number(parts.next());
    let eta = number(parts.next()).map(|v| v as u64);

    Some(Progress {
        downloaded,
        // Gercek toplam yoksa tahmine duseriz: ilerleme cubugu yine de
        // anlamli bir yuzde gosterebilsin.
        total: total.or(estimate),
        speed,
        eta,
    })
}

fn number(token: Option<&str>) -> Option<f64> {
    let t = token?.trim();
    if t.is_empty() || t == "NA" || t == "None" || t == "-" {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_full_progress_line() {
        let p = parse_progress("VDROP 1048576 10485760 10485760 524288.0 18").unwrap();
        assert_eq!(p.downloaded, 1_048_576);
        assert_eq!(p.total, Some(10_485_760));
        assert_eq!(p.speed, Some(524_288.0));
        assert_eq!(p.eta, Some(18));
    }

    #[test]
    fn falls_back_to_the_estimate_when_the_total_is_unknown() {
        // Canli/parcali indirmelerde gercek toplam bilinmez ama tahmin gelir.
        let p = parse_progress("VDROP 500 NA 4096 1000.0 NA").unwrap();
        assert_eq!(p.total, Some(4096));
        assert_eq!(p.eta, None);
    }

    #[test]
    fn tolerates_missing_and_na_fields() {
        let p = parse_progress("VDROP 100 NA NA NA NA").unwrap();
        assert_eq!(p.downloaded, 100);
        assert_eq!(p.total, None);
        assert_eq!(p.speed, None);
        assert_eq!(p.eta, None);
    }

    #[test]
    fn ignores_lines_that_are_not_progress() {
        // yt-dlp stdout'a bilgi satirlari da yazar; onlari yutmaliyiz.
        assert!(parse_progress("[youtube] Extracting URL: https://...").is_none());
        assert!(parse_progress("[download] Destination: video.mp4").is_none());
        assert!(parse_progress("").is_none());
        assert!(parse_progress("VDROP").is_none());
    }

    #[test]
    fn rejects_nonsense_numbers() {
        assert!(parse_progress("VDROP abc 1 1 1 1").is_none());
        // Negatif bayt sayisi anlamsiz; alan yok sayilir.
        let p = parse_progress("VDROP 10 -5 NA NA NA").unwrap();
        assert_eq!(p.total, None);
    }

    #[test]
    fn classifies_video_audio_and_muxed_formats() {
        let muxed = Format {
            format_id: "18".into(),
            ext: Some("mp4".into()),
            vcodec: Some("avc1.42001E".into()),
            acodec: Some("mp4a.40.2".into()),
            width: Some(640),
            height: Some(360),
            fps: Some(30.0),
            tbr: Some(500.0),
            vbr: None,
            abr: None,
            filesize: Some(1000),
            filesize_approx: None,
            format_note: Some("360p".into()),
            protocol: Some("https".into()),
        };
        assert!(muxed.has_video() && muxed.has_audio());
        assert_eq!(muxed.codec_label().as_deref(), Some("H.264 · AAC"));
        assert_eq!(muxed.resolution().as_deref(), Some("640x360"));

        let video_only = Format {
            acodec: Some("none".into()),
            ..muxed.clone()
        };
        assert!(video_only.has_video() && !video_only.has_audio());

        let audio_only = Format {
            vcodec: Some("none".into()),
            width: None,
            height: None,
            ..muxed.clone()
        };
        assert!(!audio_only.has_video() && audio_only.has_audio());
        assert_eq!(audio_only.resolution(), None);
    }

    #[test]
    fn formats_without_codec_fields_are_still_downloadable() {
        // Bazi cikaricilar (archive.org gibi) kodek alanlarini hic
        // doldurmaz; yt-dlp bunlari "unknown" diye listeler ve indirir.
        // Eksik alan "ses/goruntu yok" DEMEK DEGILDIR - oyle sayilinca
        // saglayici tum formatlari eliyor, "yt-dlp indirilebilir bir format
        // bildirmedi" deyip zincir genel sayfa cikarimina dusuyordu:
        // kullanici 3 kalite yerine 1 tane goruyordu.
        let unknown = Format {
            format_id: "1".into(),
            ext: Some("mp4".into()),
            vcodec: None,
            acodec: None,
            width: Some(640),
            height: Some(360),
            fps: None,
            tbr: None,
            vbr: None,
            abr: None,
            filesize: Some(61_878_609),
            filesize_approx: None,
            format_note: None,
            protocol: Some("https".into()),
        };
        assert!(
            unknown.has_video() && unknown.has_audio(),
            "kodegi bilinmeyen format elenmemeli"
        );

        // Acikca "none" diyen alan hala yok demektir: yt-dlp ses-yok /
        // goruntu-yok formatlari boyle isaretler ve bu ayrim korunmali.
        let silent = Format {
            acodec: Some("none".into()),
            ..unknown.clone()
        };
        assert!(silent.has_video() && !silent.has_audio());
    }

    #[test]
    fn height_only_formats_still_report_a_quality() {
        let f = Format {
            format_id: "x".into(),
            ext: None,
            vcodec: Some("vp9".into()),
            acodec: None,
            width: None,
            height: Some(1080),
            fps: None,
            tbr: None,
            vbr: None,
            abr: None,
            filesize: None,
            filesize_approx: None,
            format_note: None,
            protocol: None,
        };
        assert_eq!(f.resolution().as_deref(), Some("1080p"));
        assert_eq!(f.codec_label().as_deref(), Some("VP9"));
    }

    #[test]
    fn bitrate_falls_back_through_tbr_vbr_abr() {
        let base = Format {
            format_id: "x".into(),
            ext: None,
            vcodec: None,
            acodec: None,
            width: None,
            height: None,
            fps: None,
            tbr: None,
            vbr: None,
            abr: None,
            filesize: None,
            filesize_approx: None,
            format_note: None,
            protocol: None,
        };
        assert_eq!(base.bitrate_kbps(), None);
        assert_eq!(
            Format { tbr: Some(1234.6), ..base.clone() }.bitrate_kbps(),
            Some(1235)
        );
        assert_eq!(
            Format { vbr: Some(800.0), ..base.clone() }.bitrate_kbps(),
            Some(800)
        );
        assert_eq!(
            Format { abr: Some(128.0), ..base.clone() }.bitrate_kbps(),
            Some(128)
        );
        // Sifir bit hizi bilgi tasimaz.
        assert_eq!(Format { tbr: Some(0.0), ..base }.bitrate_kbps(), None);
    }

    #[test]
    fn size_prefers_the_exact_value_over_the_estimate() {
        let f = Format {
            format_id: "x".into(),
            ext: None,
            vcodec: None,
            acodec: None,
            width: None,
            height: None,
            fps: None,
            tbr: None,
            vbr: None,
            abr: None,
            filesize: Some(100),
            filesize_approx: Some(999),
            format_note: None,
            protocol: None,
        };
        assert_eq!(f.size_bytes(), Some(100));
        assert_eq!(
            Format { filesize: None, ..f }.size_bytes(),
            Some(999)
        );
    }

    #[test]
    fn error_extraction_prefers_the_error_line() {
        let stderr = "[youtube] Downloading webpage\nERROR: Video unavailable\nsome trailing noise";
        assert_eq!(first_meaningful_error(stderr), "Video unavailable");
        assert_eq!(first_meaningful_error("   \n\n"), "bilinmeyen hata");
    }

    #[test]
    fn deserialises_a_realistic_info_payload() {
        // yt-dlp semasi genis; bilinmeyen alanlar cozumlemeyi bozmamali.
        let json = r#"{
            "id": "abc",
            "title": "Ornek Video",
            "uploader": "Bir Kanal",
            "duration": 212.0,
            "thumbnail": "https://x.com/t.jpg",
            "extractor": "youtube",
            "bilinmeyen_alan": {"ic": 1},
            "formats": [
                {"format_id": "137", "ext": "mp4", "vcodec": "avc1.640028",
                 "acodec": "none", "width": 1920, "height": 1080, "fps": 30,
                 "tbr": 4500.0, "filesize": 123456789, "protocol": "https"},
                {"format_id": "140", "ext": "m4a", "vcodec": "none",
                 "acodec": "mp4a.40.2", "abr": 128.0, "filesize_approx": 3400000}
            ]
        }"#;
        let info: Info = serde_json::from_str(json).unwrap();
        assert_eq!(info.title.as_deref(), Some("Ornek Video"));
        assert_eq!(info.formats.len(), 2);
        assert!(info.formats[0].has_video() && !info.formats[0].has_audio());
        assert_eq!(info.formats[1].bitrate_kbps(), Some(128));
    }
}

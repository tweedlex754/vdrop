//! vdrop-media: HLS / DASH akislari ve FFmpeg boru hatti.
//!
//! `docs/ARCHITECTURE.md` bolum L'de planlanan ama yazilmamis olan katman.
//!
//! ## Neden FFmpeg'e delege ediyoruz
//!
//! `vdrop-download` duz bir HTTP govdesini indirir. Bir `.m3u8` linkinde ise
//! govde sadece birkac kilobayt metindir: asil video binlerce ayri `.ts`
//! segmentindedir. Bunlari elle indirmek AES-128 anahtar rotasyonu,
//! discontinuity isaretleri, varyant/bitrate secimi ve PTS yeniden zamanlama
//! demektir. FFmpeg bunu `-c copy` ile **yeniden kodlama yapmadan** cozer:
//! CPU maliyeti sifira yakin, kalite kaybi yok.
//!
//! ## Duraklatma neden yok
//!
//! FFmpeg bir alt surectir; Windows'ta POSIX `SIGSTOP` karsiligi guvenli bir
//! duraklatma yok. Bu yuzden HLS/DASH indirmeleri **iptal edilebilir ama
//! duraklatilamaz**; arayuz bunu `can_pause = false` ile bilir ve Duraklat
//! dugmesini gostermez. Yarim kalan cikti silinir (bozuk mp4 birakmayiz).

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

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
pub enum MediaError {
    #[error("FFmpeg bulunamadi. Ayarlar > Bilesenler bolumunden kurabilirsiniz.")]
    FfmpegMissing,
    #[error("FFmpeg calistirilamadi: {0}")]
    Spawn(String),
    #[error("FFmpeg hata koduyla cikti ({code}): {detail}")]
    Failed { code: i32, detail: String },
    #[error("io hatasi: {0}")]
    Io(#[from] std::io::Error),
}

/// Sistemde FFmpeg/FFprobe'un nerede oldugunu tutar.
#[derive(Debug, Clone)]
pub struct Ffmpeg {
    pub ffmpeg: PathBuf,
    pub ffprobe: Option<PathBuf>,
    pub version: String,
}

impl Ffmpeg {
    /// FFmpeg'i once uygulamanin kendi `bin/` klasorunde, sonra PATH'te arar.
    ///
    /// Uygulama-yerel dizini once bakmamizin sebebi: ileride VDrop kendi
    /// FFmpeg'ini indirip yonetecek (bolum P, bilesen versiyonlama). O zaman
    /// kullanicinin sistemdeki eski surumu bizimkini golgelememeli.
    pub fn discover(app_bin_dir: Option<&Path>) -> Option<Self> {
        let exe = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };
        let probe_exe = if cfg!(windows) { "ffprobe.exe" } else { "ffprobe" };

        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(dir) = app_bin_dir {
            candidates.push(dir.join(exe));
        }
        candidates.push(PathBuf::from(exe)); // PATH uzerinden

        for cand in candidates {
            if let Some(version) = probe_version(&cand) {
                let ffprobe = cand
                    .parent()
                    .map(|p| p.join(probe_exe))
                    .filter(|p| p.exists())
                    .or_else(|| {
                        probe_version(Path::new(probe_exe)).map(|_| PathBuf::from(probe_exe))
                    });
                return Some(Self {
                    ffmpeg: cand,
                    ffprobe,
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
        .arg("-version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    // Ilk satir: "ffmpeg version N-125875-g5d4d3bdc61 Copyright ..."
    text.lines()
        .next()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
}

/// Bir URL'in segment tabanli akis (HLS/DASH) olup olmadigini soyler.
/// Uzantiya ve yaygin sorgu parametrelerine bakar; ag erisimi yapmaz.
pub fn is_stream_manifest(url: &str) -> bool {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
    path.ends_with(".m3u8") || path.ends_with(".m3u") || path.ends_with(".mpd")
}

/// FFprobe ile sure ogrenir. Progress yuzdesi icin gerekli: FFmpeg bize
/// "kac saniye islendi" der, "yuzde kac bitti" demez.
pub async fn probe_duration_seconds(ff: &Ffmpeg, url: &str) -> Option<f64> {
    let probe = ff.ffprobe.as_ref()?;
    let mut cmd = Command::new(probe);
    let out = quiet_async(&mut cmd)
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .ok()?;
    String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().ok()
}

pub struct StreamDownloadOptions {
    pub url: String,
    pub destination: PathBuf,
    /// Bilinen sure (saniye). `None` ise ilerleme yuzdesi hesaplanamaz,
    /// sadece yazilan bayt gosterilir.
    pub duration_seconds: Option<f64>,
    pub headers: Vec<(String, String)>,
    /// Kullanicinin sectigi kalite. `None` ise FFmpeg kendi secer.
    pub selector: Option<StreamSelector>,
    /// Bu bir altyazi izi mi?
    ///
    /// Ayri bir bayrak, cunku cikti argumanlari tamamen degisiyor: video
    /// boru hatti `-c copy -bsf:a aac_adtstoasc -movflags +faststart`
    /// kullaniyor ve bunlarin ucu de altyazida yanlis - ses filtresi
    /// uygulanacak ses yok, faststart mp4'e ozgu. Altyazi izinin kendi
    /// playlist adresi indirildigi icin `-map` de gerekmiyor.
    pub subtitle: bool,
}

/// Kalite seciminin FFmpeg'e nasil anlatilacagi.
///
/// Iki bicim var cunku FFmpeg iki manifest turunu farkli acar:
///
/// - **HLS master playlist** -> her varyant ayri bir *program*
/// - **DASH manifest**       -> tum temsiller tek programin ayri *akislari*
///
/// Bu ayrimi bir enum ile tasimak, "indeks" gibi anlamsiz bir sayiyi
/// dogru komuta cevirme sorumlulugunu tek yerde tutar. Iki hali karistirmak
/// sessizce yanlis kaliteyi indirmek demektir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamSelector {
    /// HLS: `-map 0:p:N` — programin tum akislari (ses dahil).
    Program(u32),
    /// DASH: `-map 0:v:N -map 0:a:0` — ses ayri bir AdaptationSet'te
    /// oldugu icin acikca eklenmeli, yoksa dosya sessiz olur.
    VideoStream(u32),
}

impl StreamSelector {
    /// Secimi FFmpeg arguman listesine cevirir.
    pub fn to_args(self) -> Vec<String> {
        match self {
            Self::Program(n) => vec!["-map".into(), format!("0:p:{n}")],
            Self::VideoStream(n) => vec![
                "-map".into(),
                format!("0:v:{n}"),
                // `?` eki: ses akisi yoksa FFmpeg hata vermek yerine
                // sessizce gecsin (yalnizca goruntu iceren manifestler var).
                "-map".into(),
                "0:a:0?".into(),
            ],
        }
    }
}

/// Bir HLS/DASH akisini indirir. `vdrop_download::DownloadEvent` yayar; boylece
/// arayuz duz HTTP indirmeleriyle akis indirmeleri arasinda ayrim yapmak
/// zorunda kalmaz - tek bir olay modeli.
pub fn start_stream_download(
    ff: Ffmpeg,
    opts: StreamDownloadOptions,
    control: watch::Receiver<ControlSignal>,
) -> mpsc::Receiver<DownloadEvent> {
    let (tx, rx) = mpsc::channel(64);
    tokio::spawn(async move {
        if let Err(e) = run_stream(ff, opts, control, tx.clone()).await {
            let _ = tx
                .send(DownloadEvent::Failed {
                    message: e.to_string(),
                })
                .await;
        }
    });
    rx
}

async fn run_stream(
    ff: Ffmpeg,
    opts: StreamDownloadOptions,
    mut control: watch::Receiver<ControlSignal>,
    events: mpsc::Sender<DownloadEvent>,
) -> Result<(), MediaError> {
    let mut cmd = Command::new(&ff.ffmpeg);
    quiet_async(&mut cmd);

    // Ozel basliklar (cerez, referer) gerekiyorsa FFmpeg'e CRLF ile ayrilmis
    // tek bir -headers argumani olarak verilir.
    if !opts.headers.is_empty() {
        let joined = opts
            .headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}\r\n"))
            .collect::<String>();
        cmd.arg("-headers").arg(joined);
    }

    cmd.args([
        "-hide_banner",
        "-nostdin",
        "-loglevel", "error",
        // Kesintiye ugrayan segmentlerde yeniden dene.
        "-reconnect", "1",
        "-reconnect_streamed", "1",
        "-reconnect_delay_max", "10",
    ])
    .arg("-i")
    .arg(&opts.url);

    // Kalite secimi. `-map` bir CIKTI secenegi oldugu icin `-i`'den sonra
    // gelmek zorunda.
    if let Some(selector) = opts.selector {
        cmd.args(selector.to_args());
    }

    if opts.subtitle {
        // Altyazi: WebVTT segmentleri tek bir SRT dosyasinda birlestirilir.
        // Burada yeniden kodlama ISTIYORUZ - `copy` deseydik cikti gecerli
        // bir SRT olmazdi. Video boru hattinin diger bayraklari (aac
        // filtresi, faststart) burada anlamsiz, o yuzden hic verilmiyor.
        cmd.args(["-c:s", "srt"]);
    } else {
        cmd.args([
            // Veri ve altyazi akislarini ALMA.
            //
            // `-map 0:p:N` programin TUM akislarini alir. Kick/Amazon IVS
            // yayinlarinda her programda bir `timed_id3` veri akisi var ve
            // onun zaman damgalari geri atliyor; mp4 muxer'i paketi
            // reddedince tum cikti dusuyordu:
            //   "non monotonically increasing dts to muxer in stream 2"
            //   "[out#0/mp4] Task finished with error: Invalid argument"
            // Kullanici bunu yarim kalmis bir dosya ve -22 hatasi olarak
            // goruyordu. Bu boru hatti goruntu+ses uretiyor; veri akisinin
            // ciktida isi yok. Altyazi da mp4'e ancak mov_text olarak
            // girer, HLS'ten gelen WebVTT ayni sekilde patlatirdi.
            "-dn", "-sn",
            // Yeniden kodlama YOK: segmentleri oldugu gibi kapsayiciya tasi.
            "-c", "copy",
            // HLS'ten gelen AAC akislari mp4'e girerken bu filtreyi ister.
            "-bsf:a", "aac_adtstoasc",
            // Oynatici dosyanin tamamini beklemeden acabilsin.
            "-movflags", "+faststart",
        ]);
    }

    cmd.args([
        // Makine okunur ilerleme: stdout'a key=value satirlari.
        "-progress", "pipe:1",
        "-nostats",
        "-y",
    ])
    .arg(&opts.destination)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| MediaError::Spawn(e.to_string()))?;

    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");

    // stderr'i ayri bir gorevde topla: hata olursa kullaniciya gercek sebebi
    // gosterelim ("Server returned 403" gibi), sadece cikis kodunu degil.
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

    let total_us = opts.duration_seconds.map(|d| (d * 1_000_000.0) as u64);
    let _ = events
        .send(DownloadEvent::Started {
            // Toplam bayt onceden bilinemez; ilk progress olayinda tahmin edilir.
            total_bytes: None,
        })
        .await;

    let mut reader = BufReader::new(stdout).lines();
    let mut written: u64 = 0;
    let mut out_time_us: u64 = 0;
    let mut last_report = Instant::now();
    let mut last_bytes: u64 = 0;
    let mut cancelled = false;

    loop {
        tokio::select! {
            line = reader.next_line() => {
                let Ok(Some(line)) = line else { break };
                let Some((key, value)) = line.split_once('=') else { continue };
                match key {
                    "total_size" => written = value.trim().parse().unwrap_or(written),
                    "out_time_us" | "out_time_ms" => {
                        // FFmpeg surumune gore anahtar degisir; ikisi de mikrosaniye
                        // yayar (out_time_ms yanlis adlandirilmis bir alandir).
                        out_time_us = value.trim().parse().unwrap_or(out_time_us);
                    }
                    "progress" if value.trim() == "end" => break,
                    _ => {}
                }

                if last_report.elapsed() >= Duration::from_millis(500) {
                    let secs = last_report.elapsed().as_secs_f64().max(0.001);
                    let speed_bps = (written.saturating_sub(last_bytes)) as f64 / secs;

                    // Islenen sure oranindan toplam boyutu tahmin et: yuzde
                    // cubugu icin yeterince iyi, dosya bitince gercek deger yazilir.
                    let (total_bytes, eta_seconds) = match (total_us, out_time_us) {
                        (Some(total), done) if total > 0 && done > 0 => {
                            let frac = (done as f64 / total as f64).clamp(0.001, 1.0);
                            let est = (written as f64 / frac) as u64;
                            let remaining_us = total.saturating_sub(done);
                            let eta = if speed_bps > 0.0 && est > written {
                                Some(((est - written) as f64 / speed_bps) as u64)
                            } else {
                                Some(remaining_us / 1_000_000)
                            };
                            (Some(est), eta)
                        }
                        _ => (None, None),
                    };

                    let _ = events
                        .send(DownloadEvent::Progress {
                            downloaded_bytes: written,
                            total_bytes,
                            speed_bps,
                            eta_seconds,
                        })
                        .await;
                    last_report = Instant::now();
                    last_bytes = written;
                }
            }
            changed = control.changed() => {
                if changed.is_err() { continue; }
                if *control.borrow() == ControlSignal::Cancel {
                    cancelled = true;
                    let _ = child.start_kill();
                    break;
                }
            }
        }
    }

    let status = child.wait().await?;
    let stderr_text = err_handle.await.unwrap_or_default();

    if cancelled {
        // Yarim kalan mp4 oynatilamaz; kullaniciya bozuk dosya birakmayalim.
        tokio::fs::remove_file(&opts.destination).await.ok();
        let _ = events.send(DownloadEvent::Cancelled).await;
        return Ok(());
    }

    if !status.success() {
        tokio::fs::remove_file(&opts.destination).await.ok();
        return Err(MediaError::Failed {
            code: status.code().unwrap_or(-1),
            detail: first_meaningful_error(&stderr_text),
        });
    }

    let final_size = tokio::fs::metadata(&opts.destination)
        .await
        .map(|m| m.len())
        .unwrap_or(written);

    let _ = events
        .send(DownloadEvent::Completed {
            path: opts.destination.clone(),
            total_bytes: final_size,
        })
        .await;
    Ok(())
}

/// FFmpeg stderr'i cok satirli olabilir; kullaniciya gosterilecek en anlamli
/// satiri secer (genelde son satir asil sebeptir).
fn first_meaningful_error(stderr: &str) -> String {
    let line = stderr
        .lines()
        .rev()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("bilinmeyen hata");
    line.chars().take(300).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_stream_manifests() {
        assert!(is_stream_manifest("https://x.com/master.m3u8"));
        assert!(is_stream_manifest("https://x.com/manifest.mpd?token=abc"));
        assert!(is_stream_manifest("https://X.com/PLAYLIST.M3U8"));
        assert!(!is_stream_manifest("https://x.com/video.mp4"));
        assert!(!is_stream_manifest("https://x.com/m3u8-hakkinda.html"));
    }

    #[test]
    fn error_extraction_picks_last_real_line() {
        let stderr = "some noise\n\nServer returned 403 Forbidden\n\n";
        assert_eq!(
            first_meaningful_error(stderr),
            "Server returned 403 Forbidden"
        );
        assert_eq!(first_meaningful_error("   \n\n"), "bilinmeyen hata");
    }

    #[test]
    fn selector_produces_the_right_ffmpeg_arguments() {
        assert_eq!(
            StreamSelector::Program(4).to_args(),
            vec!["-map".to_string(), "0:p:4".to_string()]
        );
        // DASH'te ses ayri bir AdaptationSet'te: acikca eklenmezse dosya
        // sessiz olur. `?` eki, sessiz manifestlerde hata vermesin diye.
        assert_eq!(
            StreamSelector::VideoStream(3).to_args(),
            vec![
                "-map".to_string(),
                "0:v:3".to_string(),
                "-map".to_string(),
                "0:a:0?".to_string()
            ]
        );
    }

    #[test]
    fn discover_finds_ffmpeg_when_present() {
        // Bu makinede FFmpeg kurulu; yoksa test atlanir (CI'da kurulu olmayabilir).
        match Ffmpeg::discover(None) {
            Some(ff) => {
                assert!(ff.version.to_lowercase().contains("ffmpeg"));
            }
            None => eprintln!("FFmpeg bulunamadi, test atlandi"),
        }
    }
}

// VDrop masaustu kabugu (Tauri 2).
//
// Bu dosya sadece "tel baglantisi" yapar: arayuzden gelen IPC komutlarini,
// bagimsiz olarak derlenip test edilen cekirdek crate'lere yonlendirir.
//
//   vdrop-download   resumable HTTP motoru
//   vdrop-media      HLS/DASH akislari (FFmpeg)
//   vdrop-providers  URL -> MediaInfo cozumleme
//   vdrop-storage    SQLite kaliciligi
//
// Tasarim kurallari:
//   1. Is mantigi burada degil, crate'lerde yasar; burasi ince kalmali.
//   2. Guvenilmeyen her metin (baslik, dosya adi) diske dokunmadan once
//      `vdrop_download::safe_join` uzerinden gecer.
//   3. Her durum degisikligi hem SQLite'a yazilir hem arayuze yayinlanir;
//      arayuz yeniden acildiginda listeyi veritabanindan kurar.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod concurrency;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::Serialize;
use tauri::{Emitter, Manager, State};
use tokio::sync::{watch, Mutex};
use uuid::Uuid;

use clipboard::ClipboardWatcher;
use concurrency::ConcurrencyGate;
use vdrop_download::{
    safe_join, start_download_with_control, unique_destination_with, ControlSignal, DownloadEvent,
    DownloadOptions, RateLimiter,
};
use vdrop_media::{is_stream_manifest, Ffmpeg, StreamDownloadOptions, StreamSelector};
use vdrop_providers::{MediaInfo, ProviderRegistry};
use vdrop_ytdlp::{YtDlp, YtDlpProvider};
use vdrop_storage::{DownloadRecord, HistoryRecord, LibraryItem, NewDownload, Storage};

// ---------------------------------------------------------------------------
// Ayar anahtarlari ve varsayilanlari
// ---------------------------------------------------------------------------

const KEY_DOWNLOAD_FOLDER: &str = "download_folder";
const KEY_MAX_CONCURRENT: &str = "max_concurrent";
const KEY_THEME: &str = "theme";
const KEY_LANGUAGE: &str = "language";
const KEY_NOTIFICATIONS: &str = "notifications";
const KEY_CLIPBOARD_WATCH: &str = "clipboard_watch";
/// Toplam indirme hizi siniri, **KB/sn**. `0` sinirsiz demektir.
const KEY_BANDWIDTH_LIMIT: &str = "bandwidth_limit_kbps";

/// Ayardaki KB/sn metnini bayt/sn'ye cevirir.
///
/// Cozulemeyen ya da negatif her deger **sinirsiz** sayilir: bozuk bir ayar
/// yuzunden indirmelerin 1 bayt/sn'ye dusmesi, sinirin hic uygulanmamasindan
/// cok daha kotu bir hata olurdu.
fn bandwidth_bytes_per_sec(value: &str) -> u64 {
    value
        .trim()
        .parse::<u64>()
        .ok()
        .map(|kb| kb.saturating_mul(1024))
        .unwrap_or(0)
}

fn default_settings(download_dir: &str) -> Vec<(&'static str, String)> {
    vec![
        (KEY_THEME, "system".into()),
        (KEY_LANGUAGE, "tr".into()),
        (KEY_DOWNLOAD_FOLDER, download_dir.to_string()),
        (KEY_MAX_CONCURRENT, "3".into()),
        (KEY_NOTIFICATIONS, "on".into()),
        (KEY_CLIPBOARD_WATCH, "off".into()),
        (KEY_BANDWIDTH_LIMIT, "0".into()),
        ("auto_open_folder", "off".into()),
    ]
}

/// Saglayici kimligi + indeksten FFmpeg secicisini kurar.
///
/// Ayrimi burada yapmak sart: ayni sayi HLS'te program, DASH'te video akis
/// indeksi anlamina gelir ve karistirmak sessizce yanlis kaliteyi indirir.
///
/// HLS altyazilari buraya hic ugramaz: orada izin kendi playlist adresi
/// indirildigi icin secilecek bir akis yoktur, `variant_index` bos gelir.
fn build_selector(provider_id: &str, index: Option<u32>) -> Option<StreamSelector> {
    let index = index?;
    Some(match provider_id {
        "dash" => StreamSelector::VideoStream(index),
        _ => StreamSelector::Program(index),
    })
}

/// Bu indirme FFmpeg gerektiren bolumlu bir akis mi?
///
/// Yalnizca uzantiya bakmak yetmez: `WebProvider` artik `Content-Type`
/// uzerinden uzantisiz manifestleri de cozebiliyor (imzali CDN adresleri
/// gibi). Boyle bir adresi duz HTTP olarak indirseydik diske birkac
/// kilobaytlik manifest METNI yazilirdi - kullanici bunu ancak dosyayi
/// acinca anlardi.
fn is_stream_download(url: &str, container: Option<&str>, variant_index: Option<u32>) -> bool {
    is_stream_manifest(url)
        || variant_index.is_some()
        || matches!(
            container.map(str::to_lowercase).as_deref(),
            Some("m3u8") | Some("m3u") | Some("mpd") | Some("hls-or-dash")
        )
}

/// Ayarlar SQLite'ta TEXT tutulur. Eski surumlerden "true"/"1" gelme
/// ihtimaline karsi hepsini kabul ediyoruz; taninmayan her sey kapalidir.
fn is_on(value: &str) -> bool {
    matches!(value, "on" | "true" | "1")
}

// ---------------------------------------------------------------------------
// Uygulama durumu
// ---------------------------------------------------------------------------

struct AppState {
    storage: Mutex<Storage>,
    providers: ProviderRegistry,
    http: reqwest::Client,
    /// Aktif indirmelerin kontrol kanallari. Kuyrukta bekleyenler de burada
    /// olur; boylece daha baslamamis bir indirme de iptal edilebilir.
    controls: Mutex<HashMap<String, watch::Sender<ControlSignal>>>,
    ffmpeg: Option<Ffmpeg>,
    /// Opsiyonel: kuruluysa yuzlerce siteye cikarim acilir, degilse
    /// uygulama dogrudan baglantilar ve manifestlerle calismaya devam eder.
    ytdlp: Option<YtDlp>,
    gate: ConcurrencyGate,
    /// Bant genisligi siniri. Tum indirmeler ayni kovayi paylasir: kullanici
    /// "toplam su kadari gecmesin" der, "her indirme ayri ayri" demez.
    rate: Arc<RateLimiter>,
    /// Acilista kurulur; ayar degistikce acilip kapanir.
    clipboard: std::sync::OnceLock<Arc<ClipboardWatcher>>,
}

impl AppState {
    async fn setting(&self, key: &str) -> Option<String> {
        self.storage.lock().await.get_setting(key).ok().flatten()
    }
}

// ---------------------------------------------------------------------------
// Arayuze donen tipler
// ---------------------------------------------------------------------------

/// Arayuze giden hata: **cevrilebilir bir kod** + istege bagli teknik detay.
///
/// Onceden buradan duz metin gidiyordu ve kullanici
/// "network error while probing media: sunucu 500 dondu" goruyordu: yarisi
/// Ingilizce, yarisi Turkce, hicbiri dil ayarina bagli degil. Cumleyi
/// arayuzun kurmasi gerekiyor, cunku dili yalnizca o biliyor.
///
/// `detail` bilerek cevrilmiyor: HTTP durum kodu ya da ayristirici mesaji
/// gibi teknik izler tek dilde (Ingilizce) kalir - hata raporlarinda
/// aranabilir olmalari cevrilmis olmalarindan daha degerli.
#[derive(Serialize, Clone)]
struct AppError {
    code: &'static str,
    detail: Option<String>,
}

impl AppError {
    fn new(code: &'static str) -> Self {
        Self { code, detail: None }
    }

    fn with_detail(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: Some(detail.into()),
        }
    }
}

/// Henuz siniflandirilmamis hatalar icin kopru.
///
/// Cok sayida `?` zinciri `String` uretiyor. Hepsini birden koda cevirmek
/// yerine bunlar "internal" olarak gecip detayi tasiyor: arayuz taniyamadigi
/// kodda detayi gosteriyor, yani hicbir bilgi kaybolmuyor. Bir hata yeterince
/// sik gorulurse kendi kodunu almasi kolay.
impl From<String> for AppError {
    fn from(detail: String) -> Self {
        AppError::with_detail("internal", detail)
    }
}

impl From<vdrop_providers::ProviderError> for AppError {
    fn from(e: vdrop_providers::ProviderError) -> Self {
        use vdrop_providers::ProviderError as P;
        match e {
            P::Unsupported => AppError::new("unsupported"),
            P::DrmProtected => AppError::new("drm"),
            P::NoMedia => AppError::new("no_media"),
            P::Network(detail) => AppError::with_detail("network", detail),
            P::Parse(detail) => AppError::with_detail("parse", detail),
        }
    }
}

#[derive(Serialize, Clone)]
struct AnalyzeResult {
    media: MediaInfo,
    /// Hangi saglayici cozdu. Arayuz buna gore indirme cagrisini kurar
    /// (yt-dlp formatlari format kimligiyle indirilir).
    provider_id: String,
    /// HLS/DASH ise arayuz "Duraklat" dugmesini gizler ve FFmpeg uyarisi verir.
    is_stream: bool,
    ffmpeg_available: bool,
}

#[derive(Serialize, Clone)]
struct AppInfo {
    version: String,
    /// Isletim sistemi: "windows" | "macos" | "linux".
    ///
    /// Arayuz bilesen kurulum ipuclarini buna gore seciyor. Tarayicidan
    /// `navigator.platform` okumak yerine kabugun kendi bildigi degeri
    /// gonderiyoruz: sniffing yanilir, `std::env::consts::OS` yanilmaz.
    os: String,
    ffmpeg_version: Option<String>,
    ytdlp_version: Option<String>,
    default_download_dir: String,
    max_concurrent: usize,
}

/// Tek bir olay adi altinda id + olay gonderiyoruz. Yedi ayri olay adina
/// yedi ayri dinleyici baglamak yerine arayuz tek bir yerden abone olur.
#[derive(Serialize, Clone)]
struct DownloadEventPayload {
    id: String,
    event: DownloadEvent,
}

// ---------------------------------------------------------------------------
// Komutlar: cozumleme
// ---------------------------------------------------------------------------

#[tauri::command]
async fn analyze_url(
    url: String,
    state: State<'_, Arc<AppState>>,
) -> Result<AnalyzeResult, AppError> {
    let url = url.trim().to_string();
    if url.is_empty() {
        return Err(AppError::new("empty_url"));
    }
    let resolved = state.providers.resolve_detailed(&url).await?;
    let media = resolved.media;

    // Arayuz buna gore "duraklatilamaz" uyarisini gosterir ve Duraklat
    // dugmesini gizler; `create_download` ile ayni kurali kullanmali.
    let is_stream = media
        .streams
        .first()
        .map(|s| is_stream_download(&s.url, s.container.as_deref(), s.variant_index))
        .unwrap_or(false);

    Ok(AnalyzeResult {
        media,
        provider_id: resolved.provider_id.to_string(),
        is_stream,
        ffmpeg_available: state.ffmpeg.is_some(),
    })
}

// ---------------------------------------------------------------------------
// Komutlar: indirme yasam dongusu
// ---------------------------------------------------------------------------

// Clippy "cok fazla arguman" diyor; burada gecerli degil. Uyarinin gerekcesi
// konumsal cagrilarda argumanlarin yerinin karistirilmasidir - ama Tauri
// komutlarinda arayuz argumanlari ADLA gonderir (`{ url, suggestedName, ... }`),
// sirayla degil. Tek bir yapiya sarmak IPC cagrisini `{ request: {...} }`
// haline getirir ve okunurlugu artirmadan bir katman ekler.
#[allow(clippy::too_many_arguments)]
#[tauri::command]
async fn create_download(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    url: String,
    suggested_name: String,
    title: Option<String>,
    folder: Option<String>,
    thumbnail_url: Option<String>,
    // HLS kalite secimi. Duz indirmelerde ve tek renditionlu akislarda yok.
    variant_index: Option<u32>,
    // Cozumlemede bulunan kapsayici ("mp4", "m3u8", ...). Uzantisiz
    // adreslerde akis tespiti icin tek ipucu budur.
    container: Option<String>,
    // yt-dlp format kimligi. Doluysa indirme yt-dlp'ye devredilir.
    format_id: Option<String>,
) -> Result<DownloadRecord, AppError> {
    let via_ytdlp = format_id.is_some();
    if via_ytdlp && state.ytdlp.is_none() {
        return Err(AppError::new("ytdlp_missing"));
    }
    let stream = !via_ytdlp && is_stream_download(&url, container.as_deref(), variant_index);
    if stream && state.ffmpeg.is_none() {
        return Err(AppError::new("ffmpeg_missing"));
    }

    // Hedef klasor: cagrida verilmisse o, yoksa ayarlardaki, o da yoksa
    // isletim sisteminin indirilenler klasoru.
    let folder = match folder {
        Some(f) if !f.trim().is_empty() => f,
        _ => state
            .setting(KEY_DOWNLOAD_FOLDER)
            .await
            .unwrap_or_else(|| default_download_dir(&app)),
    };
    let folder_path = PathBuf::from(&folder);
    std::fs::create_dir_all(&folder_path)
        .map_err(|e| format!("Hedef klasor olusturulamadi ({folder}): {e}"))?;

    // GUVENLIK: `suggested_name` uzaktan gelen veriden turemis olabilir.
    // safe_join hem sanitize eder hem sonucun klasor icinde kaldigini dogrular.
    let base = safe_join(&folder_path, &suggested_name).map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();
    // Altyazi ayri bir tur: FFmpeg boru hatti ayni ama cikti argumanlari
    // farkli. Ayri bir SUTUN yerine `kind`e yazmak, devam ettirmeyi bedavaya
    // getiriyor - `kind` zaten veritabaninda ve uygulama yeniden acildiginda
    // kaydin ne oldugu oradan okunuyor.
    let subtitle = container
        .as_deref()
        .map(str::to_lowercase)
        .is_some_and(|c| c == "srt" || c == "vtt");
    let kind = if via_ytdlp {
        "ytdlp"
    } else if subtitle {
        "subtitle"
    } else if stream {
        "stream"
    } else {
        "http"
    };

    // Hangi saglayici cozdu? Bu, `variant_index`in ANLAMINI belirler:
    // HLS'te program indeksi, DASH'te video akis indeksi. Kalici olarak
    // saklaniyor ki devam ettirilen bir indirme de dogru bicimi kullansin.
    let provider_id = match container.as_deref().map(str::to_lowercase).as_deref() {
        _ if via_ytdlp => "yt-dlp",
        Some("mpd") => "dash",
        Some("m3u8") | Some("m3u") | Some("hls-or-dash") => "hls",
        _ if stream => "hls",
        _ => "web",
    };
    let selector = build_selector(provider_id, variant_index);

    // Ad rezervasyonu ve kayit ekleme AYNI kilit altinda: aksi halde iki es
    // zamanli `create_download` cagrisi, kontrol ile ekleme arasinda
    // birbirini gecip ayni adi secebilir ve ikisi de ayni `.part` dosyasina
    // yazar. Dosya sistemi tek basina yetmez, cunku kuyrukta bekleyen bir
    // indirmenin `.part` dosyasi henuz yoktur.
    let dest = {
        let storage = state.storage.lock().await;
        let dest = unique_destination_with(&base, |candidate| {
            storage
                .destination_reserved(&candidate.to_string_lossy())
                .unwrap_or(false)
        });
        storage
            .insert_download(&NewDownload {
                id: &id,
                url: &url,
                title: title.as_deref(),
                destination_path: &dest.to_string_lossy(),
                kind,
                thumbnail_url: thumbnail_url.as_deref(),
                provider_id: Some(provider_id),
                variant_index: variant_index.map(i64::from),
                format_id: format_id.as_deref(),
            })
            .map_err(|e| e.to_string())?;
        dest
    };

    spawn_transfer(
        app.clone(),
        state.inner().clone(),
        Transfer {
            id: id.clone(),
            url,
            dest,
            kind: kind.to_string(),
            selector,
            format_id,
        },
    )
    .await;

    let storage = state.storage.lock().await;
    storage
        .get_download(&id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| AppError::new("record_missing"))
}

/// Bir transferi baslatmak icin gereken her sey.
///
/// `create_download` ve `resume_download` ayni gorevi kurar; alanlari
/// konumsal argumanlar yerine adlariyla vermek, iki cagri yerinin sessizce
/// ayrisma ihtimalini ortadan kaldirir.
struct Transfer {
    id: String,
    url: String,
    dest: PathBuf,
    /// "http" | "stream" | "ytdlp"
    kind: String,
    /// HLS/DASH kalite secimi.
    selector: Option<StreamSelector>,
    /// yt-dlp format kimligi.
    format_id: Option<String>,
}

/// Kontrol kanalini kurar, kuyruk izni bekleyen bir gorev baslatir ve olay
/// akisini hem veritabanina hem arayuze pompalar.
async fn spawn_transfer(app: tauri::AppHandle, state: Arc<AppState>, transfer: Transfer) {
    let Transfer {
        id,
        url,
        dest,
        kind,
        selector,
        format_id,
    } = transfer;
    let (control_tx, control_rx) = watch::channel(ControlSignal::Run);
    state
        .controls
        .lock()
        .await
        .insert(id.clone(), control_tx.clone());

    tokio::spawn(async move {
        // Es zamanlilik kuyrugu: slot bosalana kadar bekle.
        let _permit = state.gate.acquire().await;

        // Beklerken iptal edilmis olabilir.
        if *control_rx.borrow() == ControlSignal::Cancel {
            finish(&app, &state, &id, "cancelled", None, &url, &dest, None).await;
            emit(&app, &id, DownloadEvent::Cancelled);
            return;
        }

        let events = if kind == "ytdlp" {
            let Some(ytdlp) = state.ytdlp.clone() else {
                let msg = "yt-dlp bulunamadi.".to_string();
                finish(&app, &state, &id, "failed", Some(&msg), &url, &dest, None).await;
                emit(&app, &id, DownloadEvent::Failed { message: msg });
                return;
            };
            vdrop_ytdlp::start_download(
                ytdlp,
                vdrop_ytdlp::DownloadOptions {
                    url: url.clone(),
                    format_id: format_id.clone(),
                    destination: dest.clone(),
                    rate_limit_bytes: Some(state.rate.rate()),
                },
                control_rx.clone(),
            )
        } else if kind == "stream" || kind == "subtitle" {
            let ff = match state.ffmpeg.clone() {
                Some(ff) => ff,
                None => {
                    let msg = "FFmpeg bulunamadi.".to_string();
                    finish(&app, &state, &id, "failed", Some(&msg), &url, &dest, None).await;
                    emit(&app, &id, DownloadEvent::Failed { message: msg });
                    return;
                }
            };
            // Sure bilgisi olmadan yuzde hesaplanamaz; ffprobe ile ogreniyoruz.
            let duration = vdrop_media::probe_duration_seconds(&ff, &url).await;
            vdrop_media::start_stream_download(
                ff,
                StreamDownloadOptions {
                    url: url.clone(),
                    destination: dest.clone(),
                    duration_seconds: duration,
                    headers: Vec::new(),
                    selector,
                    subtitle: kind == "subtitle",
                },
                control_rx.clone(),
            )
        } else {
            start_download_with_control(
                state.http.clone(),
                DownloadOptions::new(url.clone(), dest.clone())
                    .with_rate_limiter(state.rate.clone()),
                control_rx.clone(),
            )
        };

        pump_events(app, state, id, url, dest, events).await;
    });
}

/// Motor olaylarini tuketir: veritabanini gunceller ve arayuze yayinlar.
async fn pump_events(
    app: tauri::AppHandle,
    state: Arc<AppState>,
    id: String,
    url: String,
    dest: PathBuf,
    mut events: tokio::sync::mpsc::Receiver<DownloadEvent>,
) {
    // Ilerleme olaylari saniyede ~2 kez gelir. Her birini diske yazmak gereksiz;
    // arayuze hepsini yayinlariz ama veritabanina her 4. olayi yazariz.
    // Sonlanma olaylari her zaman yazilir.
    const DB_WRITE_EVERY: u32 = 4;
    let mut tick: u32 = 0;

    while let Some(ev) = events.recv().await {
        match &ev {
            DownloadEvent::Started { total_bytes } => {
                let storage = state.storage.lock().await;
                storage.set_status(&id, "downloading", None).ok();
                storage
                    .update_progress(&id, 0, total_bytes.map(|v| v as i64))
                    .ok();
            }
            DownloadEvent::Progress {
                downloaded_bytes,
                total_bytes,
                ..
            } => {
                tick = tick.wrapping_add(1);
                if tick.is_multiple_of(DB_WRITE_EVERY) {
                    let storage = state.storage.lock().await;
                    storage
                        .update_progress(
                            &id,
                            *downloaded_bytes as i64,
                            total_bytes.map(|v| v as i64),
                        )
                        .ok();
                    storage.set_status(&id, "downloading", None).ok();
                }
            }
            DownloadEvent::Paused { downloaded_bytes } => {
                let storage = state.storage.lock().await;
                storage
                    .update_progress(&id, *downloaded_bytes as i64, None)
                    .ok();
                storage.set_status(&id, "paused", None).ok();
            }
            DownloadEvent::Retrying { .. } => {
                let storage = state.storage.lock().await;
                storage.set_status(&id, "retrying", None).ok();
            }
            DownloadEvent::Completed { total_bytes, .. } => {
                {
                    let storage = state.storage.lock().await;
                    storage
                        .update_progress(&id, *total_bytes as i64, Some(*total_bytes as i64))
                        .ok();
                }
                finish(
                    &app,
                    &state,
                    &id,
                    "completed",
                    None,
                    &url,
                    &dest,
                    Some(*total_bytes as i64),
                )
                .await;
            }
            DownloadEvent::Failed { message } => {
                finish(&app, &state, &id, "failed", Some(message), &url, &dest, None).await;
            }
            DownloadEvent::Cancelled => {
                finish(&app, &state, &id, "cancelled", None, &url, &dest, None).await;
            }
        }
        emit(&app, &id, ev);
    }

    // Olay akisi kapandi: kontrol kanalini birak.
    state.controls.lock().await.remove(&id);
}

/// Sonlanma islemleri: durum yaz, gecmise ekle, tamamlandiysa kutuphaneye al.
#[allow(clippy::too_many_arguments)]
async fn finish(
    app: &tauri::AppHandle,
    state: &Arc<AppState>,
    id: &str,
    status: &str,
    error: Option<&str>,
    url: &str,
    dest: &PathBuf,
    total_bytes: Option<i64>,
) {
    let storage = state.storage.lock().await;
    storage.set_status(id, status, error).ok();

    let title = storage
        .get_download(id)
        .ok()
        .flatten()
        .and_then(|r| r.title)
        .unwrap_or_else(|| {
            dest.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        });

    storage
        .add_history(
            &Uuid::new_v4().to_string(),
            Some(id),
            url,
            Some(&title),
            status,
            Some(&dest.to_string_lossy()),
            total_bytes,
        )
        .ok();

    let notifications_on = storage
        .get_setting(KEY_NOTIFICATIONS)
        .ok()
        .flatten()
        .map(|v| is_on(&v))
        .unwrap_or(true);

    if status == "completed" {
        let size = total_bytes.or_else(|| std::fs::metadata(dest).ok().map(|m| m.len() as i64));
        storage
            .add_library_item(id, Some(&title), &dest.to_string_lossy(), size)
            .ok();
    }

    // Kilidi birak: bildirim gostermek isletim sistemine gider ve yavas
    // olabilir; veritabani kilidini o sure boyunca tutmanin anlami yok.
    drop(storage);

    if notifications_on {
        notify_finished(app, status, &title);
    }
}

/// Indirme bitince sistem bildirimi. Yalnizca sonuclanmis durumlar icin;
/// duraklatma gibi kullanicinin kendi yaptigi seyler bildirilmez - kendi
/// tikladigi seyi ona haber vermek gurultudur.
fn notify_finished(app: &tauri::AppHandle, status: &str, title: &str) {
    use tauri_plugin_notification::NotificationExt;

    let body = match status {
        "completed" => format!("Indirildi: {title}"),
        "failed" => format!("Basarisiz: {title}"),
        _ => return,
    };

    // Bildirim gosterilemezse indirme yine de basarilidir; bu bir yan etki,
    // hata yolu degil. Yine de sessizce yutmuyoruz: Windows'ta toast
    // bildirimleri uygulamanin Baslat menusunde kayitli olmasini ister
    // (yani kurulmus olmasini), bu yuzden tasinabilir/gelistirme
    // calistirmalarinda basarisiz olur. Sebebi gorunur olsun.
    if let Err(e) = app.notification().builder().title("VDrop").body(body).show() {
        eprintln!("[vdrop] bildirim gosterilemedi: {e}");
    }
}

fn emit(app: &tauri::AppHandle, id: &str, event: DownloadEvent) {
    let _ = app.emit(
        "download:event",
        DownloadEventPayload {
            id: id.to_string(),
            event,
        },
    );
}

#[tauri::command]
async fn pause_download(id: String, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    set_control(&state, &id, ControlSignal::Pause).await
}

#[tauri::command]
async fn cancel_download(id: String, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    set_control(&state, &id, ControlSignal::Cancel).await
}

/// Devam ettir (ve "yeniden dene").
///
/// Duraklatma transferi tamamen sonlandirir - baglanti kapanir, es zamanlilik
/// yuvasi serbest kalir - ama `.part` dosyasi diskte kalir. Dolayisiyla devam
/// ettirme her zaman **yeniden baslatmadir**; motor `.part` dosyasini gorup
/// Range istegiyle kaldigi yerden surdurur.
///
/// Ayni yol uygulama kapanip acildiginda da isler: o durumda da kontrol
/// kanali yoktur, sadece diskteki `.part` vardir. Tek kod yolu, iki senaryo.
#[tauri::command]
async fn resume_download(
    app: tauri::AppHandle,
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let record = {
        let storage = state.storage.lock().await;
        storage
            .get_download(&id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{id} kimlikli indirme bulunamadi."))?
    };

    // Zaten kosan bir indirmeyi yeniden baslatmak, ayni `.part` dosyasina
    // yazan iki gorev demek olurdu - dosya bozulurdu.
    if !matches!(
        record.status.as_str(),
        "paused" | "failed" | "cancelled" | "queued"
    ) {
        return Err(format!(
            "Bu indirme zaten calisiyor (durum: {}).",
            record.status
        ));
    }

    // Sonlanmis gorevden kalmis olabilecek kanali temizle.
    state.controls.lock().await.remove(&id);

    spawn_transfer(
        app,
        state.inner().clone(),
        Transfer {
            id: record.id.clone(),
            url: record.url.clone(),
            dest: PathBuf::from(&record.destination_path),
            kind: record.kind.clone(),
            // Kullanicinin sectigi kaliteyi koru: devam ettirilen bir indirme
            // varsayilana dusmemeli. Secici, kaydedilen saglayici kimligine
            // gore yeniden kuruluyor.
            selector: build_selector(
                record.provider_id.as_deref().unwrap_or("hls"),
                record.variant_index.map(|v| v as u32),
            ),
            format_id: record.format_id.clone(),
        },
    )
    .await;
    Ok(())
}

async fn set_control(
    state: &State<'_, Arc<AppState>>,
    id: &str,
    signal: ControlSignal,
) -> Result<(), String> {
    let controls = state.controls.lock().await;
    match controls.get(id) {
        Some(tx) => tx.send(signal).map_err(|e| e.to_string()),
        None => Err(format!("{id} kimlikli aktif indirme yok.")),
    }
}

#[tauri::command]
async fn list_downloads(state: State<'_, Arc<AppState>>) -> Result<Vec<DownloadRecord>, String> {
    state
        .storage
        .lock()
        .await
        .list_downloads()
        .map_err(|e| e.to_string())
}

/// Listeden kaldirir. `delete_file` isaretliyse diskteki dosyayi (ve varsa
/// yarim kalan `.part` dosyasini) da siler.
#[tauri::command]
async fn remove_download(
    id: String,
    delete_file: bool,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    // Once durdur: silinen bir kaydin arka planda yazmaya devam etmesi
    // "hayalet dosya" birakirdi.
    let _ = set_control(&state, &id, ControlSignal::Cancel).await;

    let storage = state.storage.lock().await;
    if delete_file {
        if let Ok(Some(rec)) = storage.get_download(&id) {
            let path = PathBuf::from(&rec.destination_path);
            std::fs::remove_file(&path).ok();
            let part = path.with_file_name(format!(
                "{}.part",
                path.file_name().unwrap_or_default().to_string_lossy()
            ));
            std::fs::remove_file(part).ok();
        }
        storage.delete_library_item(&id).ok();
    }
    storage.delete_download(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_finished(state: State<'_, Arc<AppState>>) -> Result<usize, String> {
    state
        .storage
        .lock()
        .await
        .clear_finished_downloads()
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Komutlar: gecmis ve kutuphane
// ---------------------------------------------------------------------------

#[tauri::command]
async fn list_history(
    limit: Option<i64>,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<HistoryRecord>, String> {
    state
        .storage
        .lock()
        .await
        .list_history(limit.unwrap_or(200))
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_history(state: State<'_, Arc<AppState>>) -> Result<usize, String> {
    state
        .storage
        .lock()
        .await
        .clear_history()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_library(state: State<'_, Arc<AppState>>) -> Result<Vec<LibraryItem>, String> {
    state
        .storage
        .lock()
        .await
        .list_library()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_library_item(
    id: String,
    delete_file: bool,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let storage = state.storage.lock().await;
    if delete_file {
        if let Ok(items) = storage.list_library() {
            if let Some(item) = items.iter().find(|i| i.id == id) {
                std::fs::remove_file(&item.file_path).ok();
            }
        }
    }
    storage.delete_library_item(&id).map_err(|e| e.to_string())
}

/// Dosyanin diskte hala var olup olmadigini soyler. Kutuphane, kullanicinin
/// Explorer'dan sildigi dosyalari "eksik" olarak isaretleyebilsin diye.
#[tauri::command]
fn paths_exist(paths: Vec<String>) -> HashMap<String, bool> {
    paths
        .into_iter()
        .map(|p| {
            let exists = std::path::Path::new(&p).exists();
            (p, exists)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Komutlar: ayarlar ve sistem
// ---------------------------------------------------------------------------

#[tauri::command]
async fn get_settings(state: State<'_, Arc<AppState>>) -> Result<HashMap<String, String>, String> {
    state
        .storage
        .lock()
        .await
        .all_settings()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_setting(
    key: String,
    value: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    {
        let storage = state.storage.lock().await;
        storage.set_setting(&key, &value).map_err(|e| e.to_string())?;
    }
    // Bazi ayarlar aninda yururluge girmeli; yeniden baslatma gerekmesin.
    match key.as_str() {
        KEY_MAX_CONCURRENT => {
            if let Ok(n) = value.parse::<usize>() {
                state.gate.set_limit(n).await;
            }
        }
        KEY_BANDWIDTH_LIMIT => {
            // Calisan indirmeler yeniden baslamadan yeni sinira uyar:
            // limitleyici hizi her parcada yeniden okur.
            state.rate.set_rate(bandwidth_bytes_per_sec(&value));
        }
        KEY_CLIPBOARD_WATCH => {
            if let Some(watcher) = state.clipboard.get() {
                watcher.set_enabled(is_on(&value));
            }
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
async fn select_download_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_folder(move |folder| {
        let _ = tx.send(folder.map(|f| f.to_string()));
    });
    rx.await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn app_info(app: tauri::AppHandle, state: State<'_, Arc<AppState>>) -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        os: std::env::consts::OS.to_string(),
        ffmpeg_version: state.ffmpeg.as_ref().map(|f| f.version.clone()),
        ytdlp_version: state.ytdlp.as_ref().map(|y| y.version.clone()),
        default_download_dir: state
            .setting(KEY_DOWNLOAD_FOLDER)
            .await
            .unwrap_or_else(|| default_download_dir(&app)),
        max_concurrent: state.gate.limit().await,
    })
}

/// Dosyayi sistemin varsayilan uygulamasiyla acar.
#[tauri::command]
fn open_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string())
}

/// Dosyayi iceren klasoru acip dosyayi secili gosterir.
#[tauri::command]
fn reveal_path(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .reveal_item_in_dir(path)
        .map_err(|e| e.to_string())
}

fn default_download_dir(app: &tauri::AppHandle) -> String {
    app.path()
        .download_dir()
        .or_else(|_| app.path().home_dir())
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

// ---------------------------------------------------------------------------
// Giris noktasi
// ---------------------------------------------------------------------------

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Veri klasoru. `VDROP_DATA_DIR` doluysa isletim sisteminin
            // klasoru yerine o kullanilir.
            //
            // Neden bir ortam degiskeni: uctan uca testler uygulamayi gercek
            // .exe olarak calistiriyor ve Windows'ta `app_data_dir()`
            // SHGetKnownFolderPath'e dayandigi icin %APPDATA%'yi degistirmek
            // ise yaramiyordu - test kosulari kullanicinin gercek
            // veritabanina yaziyordu. Uygulamanin kendi davranisi degismez;
            // degisken bos oldugunda yol eskisi gibi cozumlenir.
            let data_dir = match std::env::var_os("VDROP_DATA_DIR") {
                Some(dir) if !dir.is_empty() => {
                    let dir = std::path::PathBuf::from(dir);
                    println!("[vdrop] veri klasoru gecersiz kilindi: {}", dir.display());
                    dir
                }
                _ => app
                    .path()
                    .app_data_dir()
                    .expect("uygulama veri klasoru cozumlenemedi"),
            };
            std::fs::create_dir_all(&data_dir).ok();
            let storage = Storage::open(data_dir.join("vdrop.sqlite3"))
                .expect("VDrop veritabani acilamadi");

            // Onceki oturum cokmus olabilir: "downloading" kalmis kayitlari
            // devam ettirilebilir bir duruma cek.
            storage.reconcile_interrupted().ok();

            // Ilk calistirmada varsayilan ayarlari yaz (mevcut degerleri ezmeden).
            let dl_dir = default_download_dir(&handle);
            let existing = storage.all_settings().unwrap_or_default();
            for (key, value) in default_settings(&dl_dir) {
                if !existing.contains_key(key) {
                    storage.set_setting(key, &value).ok();
                }
            }

            let bandwidth = storage
                .get_setting(KEY_BANDWIDTH_LIMIT)
                .ok()
                .flatten()
                .map(|v| bandwidth_bytes_per_sec(&v))
                .unwrap_or(0);

            let max_concurrent = storage
                .get_setting(KEY_MAX_CONCURRENT)
                .ok()
                .flatten()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(3);

            // FFmpeg'i once uygulamanin kendi bin/ klasorunde, sonra PATH'te ara.
            let app_bin = data_dir.join("bin");
            let ffmpeg = Ffmpeg::discover(Some(&app_bin));
            match &ffmpeg {
                Some(ff) => println!("[vdrop] FFmpeg: {}", ff.version),
                None => println!("[vdrop] FFmpeg bulunamadi - HLS/DASH indirmeleri devre disi"),
            }

            let ytdlp = YtDlp::discover(Some(&app_bin));
            match &ytdlp {
                Some(y) => println!("[vdrop] yt-dlp: {}", y.version),
                None => println!(
                    "[vdrop] yt-dlp bulunamadi - site-ozel cikarim devre disi \
                     (dogrudan baglantilar ve manifestler calisir)"
                ),
            }

            let clipboard_on = storage
                .get_setting(KEY_CLIPBOARD_WATCH)
                .ok()
                .flatten()
                .map(|v| is_on(&v))
                .unwrap_or(false);

            // HTTP istemci politikasi (User-Agent, baglanti zaman asimi,
            // yonlendirme siniri) tek yerde: vdrop_providers::default_client.
            let http = vdrop_providers::default_client();

            let state = Arc::new(AppState {
                storage: Mutex::new(storage),
                // Saglayicilar indirme motoruyla ayni HTTP istemcisini
                // paylasir: ikinci bir baglanti havuzu ve ikinci bir TLS
                // oturum onbellegi acmanin anlami yok. Ayrica manifest
                // istegi ile segment istegi ayni User-Agent'i tasir - bazi
                // CDN'ler bunu tutarsiz bulup ikincisini reddediyor.
                providers: {
                    let mut reg = ProviderRegistry::with_client(http.clone());
                    // yt-dlp EN ONE kaydediliyor: kuruluysa en yetenekli
                    // cikaricidir. Tanimadigi bir adres icin "desteklenmiyor"
                    // der ve zincir genel sayfa cikarimina duser.
                    if let Some(y) = ytdlp.clone() {
                        reg.register_first(Box::new(YtDlpProvider::new(y)));
                    }
                    reg
                },
                http,
                controls: Mutex::new(HashMap::new()),
                ffmpeg,
                ytdlp,
                gate: ConcurrencyGate::new(max_concurrent),
                rate: Arc::new(RateLimiter::new(bandwidth)),
                clipboard: std::sync::OnceLock::new(),
            });

            // Izleyici gorev, kendisini tutan state'ten once baslatilamaz
            // (AppHandle gerekiyor), o yuzden OnceLock ile sonradan takiliyor.
            let _ = state
                .clipboard
                .set(ClipboardWatcher::start(handle.clone(), clipboard_on));

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            analyze_url,
            create_download,
            pause_download,
            resume_download,
            cancel_download,
            list_downloads,
            remove_download,
            clear_finished,
            list_history,
            clear_history,
            list_library,
            remove_library_item,
            paths_exist,
            get_settings,
            set_setting,
            select_download_folder,
            app_info,
            open_path,
            reveal_path,
        ])
        .run(tauri::generate_context!())
        .expect("VDrop calistirilirken hata olustu");
}

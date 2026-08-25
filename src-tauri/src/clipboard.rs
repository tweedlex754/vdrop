//! Pano izleyici.
//!
//! Ayarlardaki "Panoyu izle" acikken, kullanicinin kopyaladigi medya
//! baglantilarini yakalar ve arayuze bildirir.
//!
//! ## Neden otomatik cozumlemiyoruz
//!
//! Yakalanan baglantiya **hicbir ag istegi atilmaz**. Arayuz sadece bir
//! bildirim serdi gosterir; istek ancak kullanici "Cozumle"ye basinca gider.
//!
//! Sebep gizlilik: pano her seyi tasir - is yerinin ic ag adresleri, imzali
//! S3 baglantilari, parola sifirlama linkleri. Kopyalanan her seye sessizce
//! istek atan bir program, kullanicinin haberi olmadan onun adina konusur.
//!
//! ## Neden yalnizca medya uzantilari
//!
//! Her http baglantisinda bildirim cikarmak, ozelligi bir dakikada kapatilir
//! hale getirirdi. Filtre dar tutuldu: bilinen medya uzantilari ve akis
//! manifestleri. Yanlis pozitif yerine yanlis negatifi tercih ediyoruz -
//! kacirilan bir link elle yapistirilabilir, her kopyalamada ziplayan bir
//! pencere ise ozelligi olduren seydir.

use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use tauri::Emitter;
use tokio::sync::watch;

/// Pano yoklama araligi. 1.2 sn: kopyalama ile bildirim arasinda insanin
/// "aninda" saydigi gecikme, ama CPU'yu mesgul edecek kadar sik degil.
const POLL_INTERVAL: Duration = Duration::from_millis(1200);

/// Bir baglantinin medya olup olmadigi bu uzantilardan anlasilir.
const MEDIA_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "webm", "mov", "avi", "flv", "m4v", "mpg", "mpeg", "ts", "m2ts", "mp3", "m4a",
    "aac", "opus", "ogg", "oga", "flac", "wav", "wma", "m3u8", "m3u", "mpd",
];

#[derive(Serialize, Clone)]
pub struct ClipboardLink {
    pub url: String,
    /// Kullaniciya gosterilecek kisa ad (dosya adi). Uzun bir CDN URL'ini
    /// oldugu gibi gostermek serdi okunmaz yapardi.
    pub label: String,
    pub is_stream: bool,
}

/// Izleyicinin acik/kapali durumunu tasiyan tutamac.
pub struct ClipboardWatcher {
    enabled: watch::Sender<bool>,
}

impl ClipboardWatcher {
    /// Izleyiciyi baslatir. Gorev uygulama boyunca yasar ama `enabled`
    /// kapaliyken hicbir sey yapmaz - surekli gorev olusturup yok etmek
    /// yerine tek bir gorevi kapida bekletmek daha basit ve sizinti riski yok.
    pub fn start(app: tauri::AppHandle, initially_enabled: bool) -> Arc<Self> {
        let (tx, rx) = watch::channel(initially_enabled);
        let watcher = Arc::new(Self { enabled: tx });
        // `tokio::spawn` DEGIL: bu fonksiyon Tauri'nin `setup()` blogundan,
        // yani ana thread'de ve bir Tokio runtime baglami DISINDA cagriliyor.
        // `tokio::spawn` orada "there is no reactor running" diye panik eder.
        // `tauri::async_runtime::spawn` Tauri'nin kendi runtime tutamacini
        // kullanir ve her iki baglamda da calisir.
        tauri::async_runtime::spawn(run(app, rx));
        watcher
    }

    pub fn set_enabled(&self, enabled: bool) {
        let _ = self.enabled.send(enabled);
    }
}

async fn run(app: tauri::AppHandle, mut enabled: watch::Receiver<bool>) {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    // Acilista panoda ne varsa "gorulmis" say. Aksi halde uygulama her
    // acildiginda, saatler once kopyalanmis bir link icin bildirim cikardi.
    let mut last_seen = app.clipboard().read_text().unwrap_or_default();

    loop {
        if !*enabled.borrow() {
            // Kapaliyken uyuyoruz; acildiginda `changed()` bizi uyandirir.
            if enabled.changed().await.is_err() {
                return;
            }
            // Kapaliyken kopyalananlar gecmise karisir: acilir acilmaz eski
            // bir link icin bildirim cikarmayalim.
            last_seen = app.clipboard().read_text().unwrap_or_default();
            continue;
        }

        tokio::time::sleep(POLL_INTERVAL).await;

        let Ok(text) = app.clipboard().read_text() else {
            continue;
        };
        if text == last_seen {
            continue;
        }
        last_seen = text.clone();

        if let Some(link) = classify(&text) {
            let _ = app.emit("clipboard:link", link);
        }
    }
}

/// Pano metnini bir medya baglantisina cevirir; degilse `None`.
pub fn classify(text: &str) -> Option<ClipboardLink> {
    let url = text.trim();

    // Pano cogu zaman metin blogu tasir. Tek satirlik, tek kelimelik bir
    // baglanti degilse ilgilenmiyoruz - bir makalenin icinden URL avlamak
    // bu ozelligin isi degil.
    if url.contains(char::is_whitespace) || url.len() > 2048 {
        return None;
    }
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return None;
    }

    let path = url.split(['?', '#']).next().unwrap_or(url);
    let ext = path.rsplit('.').next()?.to_lowercase();
    if !MEDIA_EXTENSIONS.contains(&ext.as_str()) {
        return None;
    }

    let label = path
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(path)
        .to_string();

    Some(ClipboardLink {
        url: url.to_string(),
        label,
        is_stream: matches!(ext.as_str(), "m3u8" | "m3u" | "mpd"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_direct_media_links() {
        let link = classify("https://cdn.example.com/videos/tavsan.mp4").unwrap();
        assert_eq!(link.label, "tavsan.mp4");
        assert!(!link.is_stream);

        let link = classify("https://x.com/live/master.m3u8?token=abc").unwrap();
        assert_eq!(link.label, "master.m3u8");
        assert!(link.is_stream, "manifestler akis olarak isaretlenmeli");
    }

    #[test]
    fn ignores_non_media_links() {
        assert!(classify("https://example.com/makale").is_none());
        assert!(classify("https://example.com/index.html").is_none());
        assert!(classify("https://youtube.com/watch?v=abc").is_none());
    }

    #[test]
    fn ignores_non_urls_and_text_blocks() {
        assert!(classify("merhaba dunya").is_none());
        assert!(classify("ftp://example.com/v.mp4").is_none());
        assert!(classify("").is_none());
        // Icinde link gecen bir metin blogu: avlanmiyoruz.
        assert!(classify("su videoyu izle https://x.com/a.mp4 guzelmis").is_none());
    }

    #[test]
    fn trims_surrounding_whitespace_from_a_lone_link() {
        // Kopyalarken basa/sona bosluk gelmesi cok yaygin.
        let link = classify("  https://cdn.example.com/a.mp4\n").unwrap();
        assert_eq!(link.url, "https://cdn.example.com/a.mp4");
    }

    #[test]
    fn rejects_absurdly_long_input() {
        let long = format!("https://x.com/{}.mp4", "a".repeat(3000));
        assert!(classify(&long).is_none());
    }
}

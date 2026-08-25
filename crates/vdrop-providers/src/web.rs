//! Genel HTTP saglayicisi: bir adresin ne oldugunu **sorarak** ogrenir.
//!
//! Onceki davranis adresi uzantisindan tahmin etmekti. Bu iki yerde
//! yetersiz kaliyordu:
//!
//! 1. **Uzantisiz dogrudan baglantilar** (imzali CDN adresleri, yonlendiren
//!    kisa linkler) medya sayilmiyordu.
//! 2. **Sayfa adresleri** hic ele alinmiyordu - oysa insanlar dogal olarak
//!    sayfa adresini yapistirir, dosya adresini degil.
//!
//! Cozum: bir istek at, `Content-Type`'a bak.
//!   - `video/*`, `audio/*`, manifest tipleri  -> dogrudan medya
//!   - `text/html`                              -> sayfayi ayristir
//!
//! Yan fayda: `Content-Length` ile **gercek dosya boyutu** ogrenilir, yani
//! kullanici indirmeden once ne kadar yer kaplayacagini gorur.

use async_trait::async_trait;

use crate::extract::extract_media;
use crate::hls::absolutize;
use crate::{
    HlsProvider, MediaInfo, Provider, ProviderError, StreamKind, StreamOption,
};

/// HTML govdesi icin ust sinir. 8 MB'tan buyuk bir "sayfa" ya bir hata ya da
/// bir tuzaktir; sinirsiz okumak bellegi tuketebilir.
const MAX_HTML_BYTES: usize = 8 * 1024 * 1024;

pub struct WebProvider {
    client: reqwest::Client,
    hls: HlsProvider,
}

impl WebProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            hls: HlsProvider::new(client.clone()),
            client,
        }
    }

    /// Adresin ne oldugunu ogrenir. Govdeyi yalnizca HTML ise okur.
    async fn probe(&self, url: &str) -> Result<Probe, ProviderError> {
        let resp = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(ProviderError::Network(format!(
                "server returned {}",
                status.as_u16()
            )));
        }

        // Yonlendirmelerden sonraki gercek adres; goreli URL'leri buna gore
        // cozmeliyiz, kullanicinin yapistirdigina gore degil.
        let final_url = resp.url().to_string();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.split(';').next().unwrap_or(v).trim().to_lowercase());
        let content_length = resp.content_length();

        let is_html = content_type
            .as_deref()
            .map(|t| t.contains("html") || t.contains("xhtml"))
            .unwrap_or(false);

        let body = if is_html {
            Some(read_capped(resp).await?)
        } else {
            // Govdeyi okumuyoruz: baglanti burada dusuyor, bayt inmiyor.
            None
        };

        Ok(Probe {
            final_url,
            content_type,
            content_length,
            body,
        })
    }
}

struct Probe {
    final_url: String,
    content_type: Option<String>,
    content_length: Option<u64>,
    body: Option<String>,
}

async fn read_capped(mut resp: reqwest::Response) -> Result<String, ProviderError> {
    let mut buf: Vec<u8> = Vec::new();
    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| ProviderError::Network(e.to_string()))?
    {
        buf.extend_from_slice(&chunk);
        if buf.len() >= MAX_HTML_BYTES {
            buf.truncate(MAX_HTML_BYTES);
            break;
        }
    }
    // Sayfa gecersiz UTF-8 tasiyabilir (eski kodlamalar); metaetiketleri
    // okuyabilmek icin kayipli donusum yeterli.
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

fn is_media_type(content_type: Option<&str>) -> bool {
    let Some(t) = content_type else { return false };
    t.starts_with("video/")
        || t.starts_with("audio/")
        || t.contains("mpegurl")
        || t.contains("dash+xml")
        || t == "application/octet-stream"
}

fn container_from(content_type: Option<&str>, url: &str) -> Option<String> {
    // Once adresteki uzantiya bakariz: `application/octet-stream` gibi bir
    // tip hicbir sey soylemez, uzanti soyler.
    let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
    if let Some(ext) = path.rsplit('.').next() {
        if crate::extract::PLAYABLE_EXTENSIONS.contains(&ext) {
            return Some(ext.to_string());
        }
    }

    // HTML'deki `type` oznitelikleri parametre tasir:
    // `video/ogg; codecs="theora, vorbis"`. Parametreleri atmazsak kapsayici
    // adi olarak bu curufun tamami ekrana basilir.
    let base = content_type?
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_lowercase();

    match base.as_str() {
        t if t.contains("mpegurl") => Some("m3u8".into()),
        t if t.contains("dash+xml") => Some("mpd".into()),
        t => t.split('/').nth(1).filter(|s| !s.is_empty()).map(str::to_string),
    }
}

fn filename_from(url: &str) -> String {
    url.split(['?', '#'])
        .next()
        .unwrap_or(url)
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("media")
        .to_string()
}

#[async_trait]
impl Provider for WebProvider {
    fn id(&self) -> &'static str {
        "web"
    }

    fn matches(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let probe = self.probe(url).await?;

        // --- Dogrudan medya --------------------------------------------
        if is_media_type(probe.content_type.as_deref()) {
            let container = container_from(probe.content_type.as_deref(), &probe.final_url);

            // Manifest ciktiysa HLS saglayicisina devret: kullanici kalite
            // secebilsin. Uzantisiz ama `application/x-mpegurl` donen
            // adresler icin tek yol budur.
            if matches!(container.as_deref(), Some("m3u8")) {
                return self.hls.resolve(&probe.final_url).await;
            }

            let is_audio = probe
                .content_type
                .as_deref()
                .map(|t| t.starts_with("audio/"))
                .unwrap_or(false);

            return Ok(MediaInfo {
                title: filename_from(&probe.final_url),
                streams: vec![StreamOption {
                    id: "direct".into(),
                    kind: if is_audio {
                        StreamKind::Audio
                    } else {
                        StreamKind::Muxed
                    },
                    url: probe.final_url.clone(),
                    container,
                    codec: None,
                    resolution: None,
                    fps: None,
                    bitrate_kbps: None,
                    language: None,
                    // Sunucunun bildirdigi gercek boyut - tahmin degil.
                    label: None,
                    estimated_size_bytes: probe.content_length,
                    variant_index: None,
                }],
                ..Default::default()
            });
        }

        // --- HTML sayfasi -----------------------------------------------
        let Some(body) = probe.body else {
            return Err(ProviderError::Unsupported);
        };
        let page = extract_media(&body, &probe.final_url);

        if page.candidates.is_empty() {
            // Metin degil TIP donuyoruz: bu durumda kullaniciya ne
            // soylenecegini arayuz kendi dilinde kuruyor. Onceden buraya
            // gomulu Turkce bir cumle vardi ve Ingilizce arayuzde de
            // aynen cikiyordu.
            return Err(ProviderError::NoMedia);
        }

        // En guvenilir aday bir HLS manifestiyse, kalite listesi icin HLS
        // saglayicisina devret - ama sayfadan ogrendigimiz baslik ve kapak
        // daha iyidir, onlari koruyoruz.
        let best = &page.candidates[0];
        if best.url.to_lowercase().contains(".m3u8") {
            if let Ok(mut info) = self.hls.resolve(&best.url).await {
                info.title = page.title.clone().unwrap_or(info.title);
                info.thumbnail_url = page.thumbnail.clone();
                info.uploader = page.uploader.clone();
                if info.duration_seconds.is_none() {
                    info.duration_seconds = page.duration_seconds;
                }
                return Ok(info);
            }
            // HLS cozumlemesi basarisizsa asagidaki genel yol devreye girer.
        }

        // Ilk adayin gercek boyutunu ogren: kullanici indirmeden once ne
        // kadar yer kaplayacagini gormeli. Yalnizca bir aday icin sorguyoruz;
        // her aday icin istek atmak sayfa basina onlarca istek demek olurdu.
        let head_size = match self.probe(&best.url).await {
            Ok(p) if is_media_type(p.content_type.as_deref()) => p.content_length,
            _ => None,
        };

        let streams = page
            .candidates
            .iter()
            .enumerate()
            .map(|(i, c)| StreamOption {
                id: format!("web-{i}"),
                kind: if matches!(c.source, crate::extract::Source::AudioElement) {
                    StreamKind::Audio
                } else {
                    StreamKind::Muxed
                },
                url: absolutize(&probe.final_url, &c.url),
                container: container_from(c.mime.as_deref(), &c.url),
                codec: None,
                // Yayincinin dosya adindaki kalite etiketi. Yoksa uydurmuyoruz.
                resolution: c.resolution_hint.clone(),
                fps: None,
                bitrate_kbps: None,
                language: None,
                label: None,
                estimated_size_bytes: if i == 0 { head_size } else { None },
                variant_index: None,
            })
            .collect();

        Ok(MediaInfo {
            title: page.title.unwrap_or_else(|| filename_from(&probe.final_url)),
            uploader: page.uploader,
            thumbnail_url: page.thumbnail,
            duration_seconds: page.duration_seconds,
            streams,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_media_content_types() {
        assert!(is_media_type(Some("video/mp4")));
        assert!(is_media_type(Some("audio/mpeg")));
        assert!(is_media_type(Some("application/x-mpegurl")));
        assert!(is_media_type(Some("application/dash+xml")));
        assert!(is_media_type(Some("application/octet-stream")));
        assert!(!is_media_type(Some("text/html")));
        assert!(!is_media_type(Some("application/json")));
        assert!(!is_media_type(None));
    }

    #[test]
    fn container_prefers_the_url_extension_over_a_vague_type() {
        // octet-stream hicbir sey soylemez; uzanti soyler.
        assert_eq!(
            container_from(Some("application/octet-stream"), "https://x.com/a.mkv"),
            Some("mkv".into())
        );
        // Uzanti yoksa tipe duseriz.
        assert_eq!(
            container_from(Some("video/mp4"), "https://x.com/indir?id=7"),
            Some("mp4".into())
        );
        assert_eq!(
            container_from(Some("application/x-mpegurl"), "https://x.com/canli"),
            Some("m3u8".into())
        );
    }

    #[test]
    fn mime_parameters_are_not_mistaken_for_a_container() {
        // Wikimedia gercek bir ornek: <source type="video/ogg; codecs=...">
        assert_eq!(
            container_from(Some("video/ogg; codecs=\"theora, vorbis\""), "https://x.com/indir"),
            Some("ogg".into())
        );
        assert_eq!(
            container_from(Some("video/webm; codecs=\"vp9, opus\""), "https://x.com/indir"),
            Some("webm".into())
        );
    }

    #[test]
    fn known_extensions_win_over_the_declared_type() {
        // `.ogv` gercek uzanti; tip `video/ogg` deseydi "ogg" derdik ve
        // dosya adi yanlis uzantiyla kaydedilirdi.
        assert_eq!(
            container_from(Some("video/ogg"), "https://x.com/film.ogv?utm=1"),
            Some("ogv".into())
        );
    }

    #[test]
    fn filename_extraction_survives_query_strings() {
        assert_eq!(filename_from("https://x.com/a/b/video.mp4?t=1"), "video.mp4");
        assert_eq!(filename_from("https://x.com/"), "media");
    }
}

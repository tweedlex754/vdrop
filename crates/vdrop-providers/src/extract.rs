//! Bir HTML sayfasindan medya adaylarini cikarir.
//!
//! Site-ozel kod yazmadan genis bir kapsam acan gozlem: **cogu sayfa
//! videosunu zaten kendisi ilan eder.** Sosyal medya onizlemesi (Open Graph)
//! ve arama motoru zenginlestirmesi (JSON-LD `VideoObject`) icin bu
//! bilgiyi koymak zorundadirlar. Biz de sayfanin kendi beyanini okuyoruz.
//!
//! Kapsam disi: JavaScript ile calisma zamaninda kurulan oynaticilar. Onlar
//! icin sandboxli bir JS calisma zamani gerekir (bkz. `docs/ARCHITECTURE.md`
//! bolum K) - ayri ve buyuk bir is.
//!
//! Bu modul **ag erisimi yapmaz**; girdi metin, cikti aday listesi. Boylece
//! tum kenar durumlar sabit ornekle test edilebilir.

use crate::hls::absolutize;
use crate::html::{decode_entities, find_tags, json_ld_blocks, text_of_first};

/// Adayin nereden geldigi. Guven sirasi bu enumun sirasidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    /// `og:video:secure_url` — yayincinin acikca isaret ettigi https adres.
    OpenGraphSecure,
    /// `og:video` / `og:video:url`
    OpenGraph,
    /// JSON-LD `VideoObject.contentUrl`
    JsonLd,
    /// `<video src>`
    VideoElement,
    /// `<video><source src>`
    SourceElement,
    /// `<audio src>` / `<audio><source>`
    AudioElement,
    /// `twitter:player:stream`
    TwitterPlayer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub url: String,
    pub source: Source,
    /// `type="video/mp4"` gibi bir ipucu varsa.
    pub mime: Option<String>,
    /// Dosya adindan okunan kalite etiketi ("480p", "1920x1080").
    ///
    /// Uydurma degil: yayincinin dosyayi kendi adlandirdigi sekil. Ayni
    /// sayfada `...480p.vp9.webm` ve `...240p.vp9.webm` varsa, bu ipucu
    /// olmadan format listesindeki iki satir da "Dosya - WEBM" der ve
    /// kullanici hangisini sectigini bilemez.
    pub resolution_hint: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageMedia {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub uploader: Option<String>,
    pub duration_seconds: Option<f64>,
    pub candidates: Vec<Candidate>,
}

/// Oynatilabilir gorunen uzantilar. Sayfalarda `og:video` bazen bir
/// **oynatici sayfasina** isaret eder (`/embed/xyz`), gercek dosyaya degil;
/// bu durumda onu aday saymayiz - indirmeye kalksak HTML inerdi.
pub const PLAYABLE_EXTENSIONS: &[&str] = &[
    // video
    "mp4", "m4v", "mkv", "webm", "mov", "avi", "flv", "ts", "m2ts", "mpg", "mpeg", "3gp", "ogv",
    "wmv", //
    // akis manifestleri
    "m3u8", "m3u", "mpd", //
    // ses
    "mp3", "m4a", "aac", "opus", "ogg", "oga", "flac", "wav", "wma",
];

fn looks_playable(url: &str, mime: Option<&str>) -> bool {
    if let Some(m) = mime {
        let m = m.to_lowercase();
        if m.starts_with("video/")
            || m.starts_with("audio/")
            || m.contains("mpegurl")
            || m.contains("dash+xml")
        {
            return true;
        }
        // `text/html` diyen bir og:video, bir oynatici sayfasidir.
        if m.starts_with("text/") {
            return false;
        }
    }
    let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
    match path.rsplit('.').next() {
        Some(ext) => PLAYABLE_EXTENSIONS.contains(&ext),
        None => false,
    }
}

/// Sayfadan baslik, kapak ve medya adaylarini cikarir.
pub fn extract_media(html: &str, page_url: &str) -> PageMedia {
    let mut out = PageMedia::default();
    let mut candidates: Vec<Candidate> = Vec::new();

    // --- Open Graph / Twitter meta etiketleri ---------------------------
    let mut og_video_type: Option<String> = None;
    for tag in find_tags(html, "meta") {
        // Bazi sayfalar `property`, bazilari `name` kullanir.
        let key = tag
            .attr("property")
            .or_else(|| tag.attr("name"))
            .unwrap_or("")
            .to_lowercase();
        let Some(content) = tag.attr("content").filter(|c| !c.trim().is_empty()) else {
            continue;
        };
        let content = content.trim();

        match key.as_str() {
            "og:title" | "twitter:title" => {
                out.title.get_or_insert_with(|| content.to_string());
            }
            "og:image" | "twitter:image" => {
                out.thumbnail
                    .get_or_insert_with(|| absolutize(page_url, content));
            }
            "og:site_name" | "author" => {
                out.uploader.get_or_insert_with(|| content.to_string());
            }
            "og:video:type" => og_video_type = Some(content.to_string()),
            "og:video:duration" | "video:duration" => {
                out.duration_seconds = content.parse().ok();
            }
            "og:video:secure_url" => candidates.push(Candidate {
                url: absolutize(page_url, content),
                source: Source::OpenGraphSecure,
                mime: None,
                resolution_hint: resolution_hint(content),
            }),
            "og:video" | "og:video:url" => candidates.push(Candidate {
                url: absolutize(page_url, content),
                source: Source::OpenGraph,
                mime: None,
                resolution_hint: resolution_hint(content),
            }),
            "twitter:player:stream" => candidates.push(Candidate {
                url: absolutize(page_url, content),
                source: Source::TwitterPlayer,
                mime: None,
                resolution_hint: resolution_hint(content),
            }),
            "og:audio" | "og:audio:secure_url" => candidates.push(Candidate {
                url: absolutize(page_url, content),
                source: Source::AudioElement,
                mime: None,
                resolution_hint: None,
            }),
            _ => {}
        }
    }

    // og:video:type tum og:video adaylarina uygulanir (ayri bir etikettir).
    if let Some(mime) = og_video_type {
        for c in candidates.iter_mut() {
            if matches!(c.source, Source::OpenGraph | Source::OpenGraphSecure) {
                c.mime = Some(mime.clone());
            }
        }
    }

    // --- JSON-LD VideoObject --------------------------------------------
    for block in json_ld_blocks(html) {
        if let Some(url) = json_string_field(&block, "contentUrl") {
            candidates.push(Candidate {
                url: absolutize(page_url, &url),
                source: Source::JsonLd,
                mime: None,
                resolution_hint: resolution_hint(&url),
            });
        }
        if out.thumbnail.is_none() {
            if let Some(t) = json_string_field(&block, "thumbnailUrl") {
                out.thumbnail = Some(absolutize(page_url, &t));
            }
        }
        if out.title.is_none() {
            out.title = json_string_field(&block, "name");
        }
        if out.duration_seconds.is_none() {
            out.duration_seconds = json_string_field(&block, "duration")
                .as_deref()
                .and_then(parse_iso8601_duration);
        }
    }

    // --- <video> ve <source> --------------------------------------------
    for tag in find_tags(html, "video") {
        if let Some(src) = tag.attr("src").filter(|s| !s.trim().is_empty()) {
            candidates.push(Candidate {
                url: absolutize(page_url, src.trim()),
                source: Source::VideoElement,
                mime: tag.attr("type").map(str::to_string),
                resolution_hint: resolution_hint(src),
            });
        }
        if out.thumbnail.is_none() {
            if let Some(poster) = tag.attr("poster").filter(|s| !s.trim().is_empty()) {
                out.thumbnail = Some(absolutize(page_url, poster.trim()));
            }
        }
    }

    for tag in find_tags(html, "source") {
        if let Some(src) = tag.attr("src").filter(|s| !s.trim().is_empty()) {
            candidates.push(Candidate {
                url: absolutize(page_url, src.trim()),
                source: Source::SourceElement,
                mime: tag.attr("type").map(str::to_string),
                resolution_hint: resolution_hint(src),
            });
        }
    }

    for tag in find_tags(html, "audio") {
        if let Some(src) = tag.attr("src").filter(|s| !s.trim().is_empty()) {
            candidates.push(Candidate {
                url: absolutize(page_url, src.trim()),
                source: Source::AudioElement,
                mime: tag.attr("type").map(str::to_string),
                resolution_hint: None,
            });
        }
    }

    // --- Ayikla, sirala, tekrarlari at ----------------------------------
    candidates.retain(|c| {
        (c.url.starts_with("http://") || c.url.starts_with("https://"))
            && looks_playable(&c.url, c.mime.as_deref())
    });

    // Guven sirasina gore sirala; ayni URL birden cok yerde gecerse en
    // guvenilir kaynagi koru. `sort_by_key` kararlidir (stable): ayni
    // kaynaktan gelen adaylar sayfadaki sirasini korur, yani yayincinin
    // ilk yazdigi secenek listede de once gorunur.
    candidates.sort_by_key(|c| c.source);
    let mut seen = Vec::new();
    candidates.retain(|c| {
        if seen.contains(&c.url) {
            false
        } else {
            seen.push(c.url.clone());
            true
        }
    });

    if out.title.is_none() {
        out.title = text_of_first(html, "title");
    }
    out.candidates = candidates;
    out
}

/// Dosya adindan kalite etiketi okur: `480p`, `1080P`, `1920x1080`.
///
/// Yalnizca yol kismina bakar; sorgu parametrelerindeki sayilar
/// (`?t=1080`) yanlis eslesme uretmesin diye.
fn resolution_hint(url: &str) -> Option<String> {
    let path = url.split(['?', '#']).next().unwrap_or(url);

    // Once WxH: daha kesin oldugu icin oncelikli.
    for token in path.split(|c: char| !c.is_ascii_alphanumeric()) {
        if let Some((w, h)) = token.split_once(['x', 'X']) {
            if is_dimension(w) && is_dimension(h) {
                return Some(format!("{w}x{h}"));
            }
        }
    }

    for token in path.split(|c: char| !c.is_ascii_alphanumeric()) {
        let lower = token.to_ascii_lowercase();
        let Some(digits) = lower.strip_suffix('p') else {
            continue;
        };
        if is_dimension(digits) {
            return Some(format!("{digits}p"));
        }
    }
    None
}

/// Makul bir piksel olcusu mu? Alt sinir bir video icin anlamsiz kucuk
/// sayilari (surum numaralari gibi), ust sinir tarih/kimlik benzeri uzun
/// sayilari eler.
fn is_dimension(text: &str) -> bool {
    if text.len() < 3 || text.len() > 4 || !text.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(text.parse::<u32>(), Ok(n) if (120..=8192).contains(&n))
}

/// JSON metninden bir dize alani ceker.
///
/// Tam bir JSON ayristiricisi yerine hedefli tarama: JSON-LD bloklari bazen
/// birden cok nesne ya da `@graph` dizisi tasir; bize lazim olan tek bir
/// alanin ilk gecerli degeri.
fn json_string_field(json: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\"");
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    if !after.starts_with('"') {
        return None;
    }

    // Kacisli tirnaklari dogru gec.
    let bytes = after.as_bytes();
    let mut i = 1usize;
    let mut value = String::new();
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if i + 1 < bytes.len() => {
                // JSON kacislari: \/ ve \" en sik gorunenler.
                let next = bytes[i + 1] as char;
                value.push(match next {
                    'n' => '\n',
                    't' => '\t',
                    other => other,
                });
                i += 2;
            }
            b'"' => break,
            _ => {
                let ch = after[i..].chars().next()?;
                value.push(ch);
                i += ch.len_utf8();
            }
        }
    }
    let value = decode_entities(&value);
    (!value.trim().is_empty()).then_some(value)
}

/// ISO 8601 sure (`PT1H2M30S`) -> saniye. Schema.org bu bicimi kullanir.
fn parse_iso8601_duration(input: &str) -> Option<f64> {
    let rest = input.strip_prefix("PT")?;
    let mut total = 0.0f64;
    let mut number = String::new();
    let mut saw_unit = false;

    for c in rest.chars() {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            continue;
        }
        let value: f64 = number.parse().ok()?;
        number.clear();
        total += match c.to_ascii_uppercase() {
            'H' => value * 3600.0,
            'M' => value * 60.0,
            'S' => value,
            _ => return None,
        };
        saw_unit = true;
    }
    saw_unit.then_some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_open_graph_video() {
        let html = r#"<html><head>
            <title>Sayfa basligi</title>
            <meta property="og:title" content="Gercek baslik">
            <meta property="og:image" content="/kapak.jpg">
            <meta property="og:video" content="https://cdn.x.com/v.mp4">
        </head></html>"#;
        let m = extract_media(html, "https://x.com/haber/1");

        assert_eq!(m.title.as_deref(), Some("Gercek baslik"), "og:title <title>'i yenmeli");
        assert_eq!(m.thumbnail.as_deref(), Some("https://x.com/kapak.jpg"));
        assert_eq!(m.candidates.len(), 1);
        assert_eq!(m.candidates[0].url, "https://cdn.x.com/v.mp4");
    }

    #[test]
    fn secure_url_outranks_plain_og_video() {
        let html = r#"
            <meta property="og:video" content="http://cdn.x.com/v.mp4">
            <meta property="og:video:secure_url" content="https://cdn.x.com/v-hd.mp4">"#;
        let m = extract_media(html, "https://x.com/a");
        assert_eq!(m.candidates[0].source, Source::OpenGraphSecure);
        assert_eq!(m.candidates[0].url, "https://cdn.x.com/v-hd.mp4");
    }

    #[test]
    fn ignores_player_pages_masquerading_as_video() {
        // og:video bir oynatici SAYFASINA isaret ediyor; indirsek HTML inerdi.
        let html = r#"
            <meta property="og:video" content="https://x.com/embed/abc123">
            <meta property="og:video:type" content="text/html">"#;
        let m = extract_media(html, "https://x.com/a");
        assert!(
            m.candidates.is_empty(),
            "text/html tipindeki og:video aday olmamali"
        );
    }

    #[test]
    fn ignores_extensionless_non_media_urls() {
        let html = r#"<meta property="og:video" content="https://x.com/player/abc">"#;
        let m = extract_media(html, "https://x.com/a");
        assert!(m.candidates.is_empty());
    }

    #[test]
    fn extracts_video_and_source_elements_with_relative_urls() {
        let html = r#"<video poster="kapak.jpg" src="videolar/ana.mp4">
              <source src="videolar/yedek.webm" type="video/webm">
            </video>"#;
        let m = extract_media(html, "https://x.com/sayfa/index.html");

        let urls: Vec<&str> = m.candidates.iter().map(|c| c.url.as_str()).collect();
        assert!(urls.contains(&"https://x.com/sayfa/videolar/ana.mp4"));
        assert!(urls.contains(&"https://x.com/sayfa/videolar/yedek.webm"));
        assert_eq!(m.thumbnail.as_deref(), Some("https://x.com/sayfa/kapak.jpg"));

        // <video src>, <source src>'ten once gelmeli.
        assert_eq!(m.candidates[0].source, Source::VideoElement);
    }

    #[test]
    fn extracts_json_ld_video_object() {
        let html = r#"<script type="application/ld+json">
        {"@context":"https://schema.org","@type":"VideoObject",
         "name":"JSON-LD baslik",
         "duration":"PT1M45S",
         "thumbnailUrl":"https://x.com/t.jpg",
         "contentUrl":"https://cdn.x.com/ld.mp4"}
        </script>"#;
        let m = extract_media(html, "https://x.com/a");

        assert_eq!(m.candidates.len(), 1);
        assert_eq!(m.candidates[0].url, "https://cdn.x.com/ld.mp4");
        assert_eq!(m.title.as_deref(), Some("JSON-LD baslik"));
        assert_eq!(m.thumbnail.as_deref(), Some("https://x.com/t.jpg"));
        assert_eq!(m.duration_seconds, Some(105.0));
    }

    #[test]
    fn json_ld_handles_escaped_slashes_in_urls() {
        // Bircok CMS URL'leri "https:\/\/..." diye kacisla yazar.
        let html = r#"<script type="application/ld+json">
        {"contentUrl":"https:\/\/cdn.x.com\/kacis.mp4"}
        </script>"#;
        let m = extract_media(html, "https://x.com/a");
        assert_eq!(m.candidates[0].url, "https://cdn.x.com/kacis.mp4");
    }

    #[test]
    fn deduplicates_the_same_url_from_several_sources() {
        let html = r#"
            <meta property="og:video" content="https://cdn.x.com/v.mp4">
            <video src="https://cdn.x.com/v.mp4"></video>"#;
        let m = extract_media(html, "https://x.com/a");
        assert_eq!(m.candidates.len(), 1, "ayni URL bir kez sayilmali");
        assert_eq!(
            m.candidates[0].source,
            Source::OpenGraph,
            "daha guvenilir kaynak korunmali"
        );
    }

    #[test]
    fn hls_manifest_in_a_page_is_a_valid_candidate() {
        let html = r#"<meta property="og:video" content="https://cdn.x.com/live/master.m3u8">"#;
        let m = extract_media(html, "https://x.com/a");
        assert_eq!(m.candidates.len(), 1);
        assert!(m.candidates[0].url.ends_with(".m3u8"));
    }

    #[test]
    fn iso_duration_parsing() {
        assert_eq!(parse_iso8601_duration("PT1H2M30S"), Some(3750.0));
        assert_eq!(parse_iso8601_duration("PT45S"), Some(45.0));
        assert_eq!(parse_iso8601_duration("PT10M"), Some(600.0));
        assert_eq!(parse_iso8601_duration("bozuk"), None);
        assert_eq!(parse_iso8601_duration("PT"), None);
    }

    #[test]
    fn reads_the_quality_label_from_the_publishers_own_filename() {
        let html = r#"<video>
            <source src="https://x.com/v/film.480p.vp9.webm" type="video/webm">
            <source src="https://x.com/v/film.240p.vp9.webm" type="video/webm">
            <source src="https://x.com/v/film.1920x1080.mp4" type="video/mp4">
        </video>"#;
        let m = extract_media(html, "https://x.com/a");
        let hints: Vec<Option<&str>> = m
            .candidates
            .iter()
            .map(|c| c.resolution_hint.as_deref())
            .collect();
        assert!(hints.contains(&Some("480p")));
        assert!(hints.contains(&Some("240p")));
        assert!(hints.contains(&Some("1920x1080")));
    }

    #[test]
    fn quality_hint_ignores_numbers_that_are_not_dimensions() {
        // Surum numarasi, tarih ve kimlik gibi sayilar kalite sanilmamali.
        assert_eq!(resolution_hint("https://x.com/v/film-v2.mp4"), None);
        assert_eq!(resolution_hint("https://x.com/2026/01/film.mp4"), None);
        assert_eq!(resolution_hint("https://x.com/v/12345678.mp4"), None);
        // Sorgudaki sayi da eslesmemeli.
        assert_eq!(resolution_hint("https://x.com/v/film.mp4?t=1080"), None);
        // Gercek etiketler eslesmeli.
        assert_eq!(resolution_hint("https://x.com/a/720p/f.mp4").as_deref(), Some("720p"));
    }

    #[test]
    fn a_page_with_nothing_yields_nothing() {
        let html = "<html><head><title>Sadece metin</title></head><body>merhaba</body></html>";
        let m = extract_media(html, "https://x.com/a");
        assert!(m.candidates.is_empty());
        assert_eq!(m.title.as_deref(), Some("Sadece metin"));
    }

    #[test]
    fn relative_protocol_urls_are_resolved() {
        let html = r#"<meta property="og:video" content="//cdn.x.com/v.mp4">"#;
        let m = extract_media(html, "https://sayfa.com/a");
        assert_eq!(m.candidates[0].url, "https://cdn.x.com/v.mp4");
    }
}

//! `kick-video.download` aracisi - zincirin **son caresi**.
//!
//! # Neden en sonda
//!
//! Bu saglayici cozumlenen adresi ucuncu bir tarafa gonderir. yt-dlp kuruluysa
//! Kick zaten aracisiz cozuluyor (kick:vod / kick:clips / kick:live), o yuzden
//! buraya ancak diger her sey basarisiz olunca gelinir: normal kullanimda hic
//! cagrilmaz, bozuk bir durumda emniyet agi olur.
//!
//! # Servisin calisma mantigi
//!
//! Sitenin kendi arayuzu su adimlari izliyor (agi izleyerek cikarildi):
//!
//! 1. Adresten kanal adi ayiklanir: `kick.com/{kanal}/videos/{uuid}`
//! 2. Kanal kimligi **Kick'in kendi genel API'sinden** cozulur:
//!    `kick.com/api/v2/channels/{kanal}/info` -> `.id`
//! 3. Servisin arka ucu cagrilir:
//!    `GET /api/get-kick-video2?url={kodlanmis}&channelId={id}`
//! 4. 404 / 403 / 429 "beklenen basarisizlik" sayilir.
//!
//! # Yanit semasi bilerek varsayilmiyor
//!
//! Yazildigi sirada servis **hicbir** Kick VOD'unu cozemiyordu (uc VOD, iki
//! kanal, hem kendi arayuzunden hem dogrudan cagriyla: `404 Failed to resolve
//! Kick video`). Basarili bir yanit hic gorulemedi, dolayisiyla alan adlari
//! bilinmiyor.
//!
//! Cozum: tiplenmis bir yapiya baglamak yerine donen JSON'da **geziyoruz** ve
//! oynatilabilir bir adres ariyoruz. Alanlar baska adlar tasisa da calisir;
//! tek varsayim "yanitin bir yerinde medya adresi vardir".

use async_trait::async_trait;
use serde_json::Value;

use crate::{HlsProvider, MediaInfo, Provider, ProviderError, StreamKind, StreamOption};

/// Yanit govdesi icin ust sinir: aracinin ne dondurecegini bilmiyoruz.
const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;

pub struct KickDownloadProvider {
    client: reqwest::Client,
    hls: HlsProvider,
    /// Aracinin koku. Testlerde yerel bir sunucuya yonlendirilir.
    base: String,
    /// Kick API koku. Ayri alan, cunku kanal kimligi aracidan degil
    /// **Kick'in kendisinden** cozuluyor - testin ikisini de sahtelemesi
    /// gerekiyor, yoksa bu saglayicinin HTTP yolu hic kosulmamis kalirdi.
    kick_api: String,
}

impl KickDownloadProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self::with_bases(client, "https://kick-video.download", "https://kick.com")
    }

    pub fn with_bases(
        client: reqwest::Client,
        base: impl Into<String>,
        kick_api: impl Into<String>,
    ) -> Self {
        Self {
            hls: HlsProvider::new(client.clone()),
            client,
            base: base.into(),
            kick_api: kick_api.into(),
        }
    }

    async fn channel_id(&self, channel: &str) -> Option<u64> {
        let url = format!("{}/api/v2/channels/{channel}/info", self.kick_api);
        let body = self.client.get(url).send().await.ok()?.text().await.ok()?;
        let json: Value = serde_json::from_str(&body).ok()?;
        json.get("id")?.as_u64()
    }
}

/// `kick.com/{kanal}/videos/{uuid}` adresinden kanal ve video kimligini ayiklar.
///
/// `kick.com/video/{uuid}` bilerek **kabul edilmiyor**: aracinin ucu o bicime
/// `422 No Kick VOD found` doner, yani gondermek bosuna bir istek olurdu.
pub fn parse_vod_url(url: &str) -> Option<(String, String)> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let rest = rest.strip_prefix("www.").unwrap_or(rest);
    let rest = rest.strip_prefix("kick.com/")?;
    let path = rest.split(['?', '#']).next()?;

    let mut parts = path.split('/').filter(|s| !s.is_empty());
    let channel = parts.next()?;
    if parts.next()? != "videos" {
        return None;
    }
    let video = parts.next()?;
    if parts.next().is_some() || channel.is_empty() || video.is_empty() {
        return None;
    }
    Some((channel.to_string(), video.to_string()))
}

/// Bir dizenin oynatilabilir medya adresi olup olmadigina bakar, turunu doner.
fn media_kind(value: &str) -> Option<&'static str> {
    if !(value.starts_with("http://") || value.starts_with("https://")) {
        return None;
    }
    let path = value.split(['?', '#']).next().unwrap_or(value).to_lowercase();
    for ext in ["m3u8", "mp4", "ts", "mkv", "webm", "m4a", "mp3"] {
        if path.ends_with(&format!(".{ext}")) {
            return Some(match ext {
                "m3u8" => "m3u8",
                "mp3" | "m4a" => "audio",
                other => other,
            });
        }
    }
    None
}

/// JSON'in **tamamini** gezip medya adaylarini toplar.
///
/// Sema bilinmedigi icin alan adlarina guvenmiyoruz.
fn collect_media(value: &Value, out: &mut Vec<(&'static str, String)>) {
    match value {
        Value::String(s) => {
            if let Some(kind) = media_kind(s) {
                if !out.iter().any(|(_, u)| u == s) {
                    out.push((kind, s.clone()));
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_media(item, out);
            }
        }
        Value::Object(map) => {
            for item in map.values() {
                collect_media(item, out);
            }
        }
        _ => {}
    }
}

/// Yanittaki kucuk resmi arar.
///
/// Once `thumbnail` adli alani deniyoruz, cunku gercek yanitta gorsel orada
/// duruyor: `{"video":{"thumbnail":{"src":"...720.webp"}}}`. Alan adina
/// bakmadan "ilk resim adresini al" deseydik `srcSet` gibi coklu-adres
/// dizeleri ya da site logosu gibi alakasiz gorseller de yakalanabilirdi.
fn find_thumbnail(value: &Value) -> Option<String> {
    fn image_url(v: &Value) -> Option<String> {
        let s = v.as_str()?;
        if !(s.starts_with("http://") || s.starts_with("https://")) {
            return None;
        }
        // Bosluk iceren degerler `srcSet` gibi coklu-adres listeleridir;
        // tek bir adres degil.
        if s.contains(' ') {
            return None;
        }
        let path = s.split(['?', '#']).next().unwrap_or(s).to_lowercase();
        ["jpg", "jpeg", "png", "webp", "avif", "gif"]
            .iter()
            .any(|ext| path.ends_with(&format!(".{ext}")))
            .then(|| s.to_string())
    }

    match value {
        Value::Object(map) => {
            for key in ["thumbnail", "thumb", "poster", "image"] {
                if let Some(found) = map.get(key) {
                    // Alan ya dogrudan adres ya da `{ src: ... }` nesnesi.
                    if let Some(url) = image_url(found) {
                        return Some(url);
                    }
                    if let Some(url) = found.get("src").and_then(image_url) {
                        return Some(url);
                    }
                }
            }
            map.values().find_map(find_thumbnail)
        }
        Value::Array(items) => items.iter().find_map(find_thumbnail),
        _ => None,
    }
}

/// Yanitta bir yerde duran basligi arar.
fn find_title(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => {
            for key in ["title", "session_title", "video_title", "name"] {
                if let Some(Value::String(s)) = map.get(key) {
                    if !s.trim().is_empty() {
                        return Some(s.clone());
                    }
                }
            }
            map.values().find_map(find_title)
        }
        Value::Array(items) => items.iter().find_map(find_title),
        _ => None,
    }
}

#[async_trait]
impl Provider for KickDownloadProvider {
    fn id(&self) -> &'static str {
        "kick-video.download"
    }

    fn matches(&self, url: &str) -> bool {
        parse_vod_url(url).is_some()
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let Some((channel, _video)) = parse_vod_url(url) else {
            return Err(ProviderError::Unsupported);
        };

        // Kanal kimligi cozulemezse bos gonderiyoruz: sitenin kendi kodu da
        // bos deger gecerek ayni seyi yapiyor.
        let channel_id = self
            .channel_id(&channel)
            .await
            .map(|id| id.to_string())
            .unwrap_or_default();

        let endpoint = reqwest::Url::parse_with_params(
            &format!("{}/api/get-kick-video2", self.base),
            &[("url", url), ("channelId", channel_id.as_str())],
        )
        .map_err(|e| ProviderError::Parse(format!("could not build request url: {e}")))?;

        let resp = self
            .client
            .get(endpoint)
            .header("Referer", "https://kick-video.download/")
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;
        let body = &body[..body.len().min(MAX_BODY_BYTES)];

        // Sitenin kendi kodu 404/403/429 durumlarini beklenen basarisizlik
        // sayiyor. Bizim icin de oyle: "bu aracinin elinden gelmiyor" bir
        // ariza degil, zincirin dogru cevabi Unsupported.
        if !status.is_success() {
            return Err(ProviderError::Unsupported);
        }

        let json: Value = serde_json::from_str(body)
            .map_err(|e| ProviderError::Parse(format!("intermediary returned non-JSON: {e}")))?;

        let mut found = Vec::new();
        collect_media(&json, &mut found);
        if found.is_empty() {
            return Err(ProviderError::Unsupported);
        }

        // Manifest varsa HLS saglayicisina devret: aracinin tek adres
        // dondurmesi, kullanicinin kalite secmesinden vazgecmemizi gerektirmez.
        if let Some((_, manifest)) = found.iter().find(|(kind, _)| *kind == "m3u8") {
            if let Ok(mut info) = self.hls.resolve(manifest).await {
                // HLS saglayicisi manifestten yalnizca kalite listesi ve sure
                // cikarabilir; baslik ve kucuk resim manifestte YOK, aracinin
                // yanitinda var. Devrederken onlari tasimazsak kullanici
                // adres benzeri bir baslik ve bos bir gorsel kutusu gorur.
                if let Some(title) = find_title(&json) {
                    info.title = title;
                }
                info.thumbnail_url = find_thumbnail(&json);
                info.uploader = Some(channel.clone());
                return Ok(info);
            }
        }

        let title = find_title(&json).unwrap_or_else(|| "Kick VOD".to_string());
        let streams = found
            .iter()
            .enumerate()
            .map(|(i, (kind, media_url))| StreamOption {
                id: format!("kickdl-{i}"),
                kind: if *kind == "audio" {
                    StreamKind::Audio
                } else {
                    StreamKind::Muxed
                },
                url: media_url.clone(),
                container: Some(if *kind == "audio" {
                    "m4a".to_string()
                } else {
                    (*kind).to_string()
                }),
                codec: None,
                resolution: None,
                fps: None,
                bitrate_kbps: None,
                language: None,
                label: None,
                estimated_size_bytes: None,
                variant_index: None,
            })
            .collect();

        Ok(MediaInfo {
            title,
            uploader: Some(channel),
            description: None,
            upload_date: None,
            duration_seconds: None,
            thumbnail_url: find_thumbnail(&json),
            streams,
            is_playlist: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_the_url_shape_the_service_understands() {
        assert_eq!(
            parse_vod_url("https://kick.com/xqc/videos/b6fd5deb-9ac1-4091-87fb-758fdacfa003"),
            Some(("xqc".into(), "b6fd5deb-9ac1-4091-87fb-758fdacfa003".into()))
        );
        assert!(parse_vod_url("https://www.kick.com/adinross/videos/abc?t=53").is_some());

        // Bu bicimi GONDERMIYORUZ: servis `422 No Kick VOD found` donuyor
        // (olculdu). Bosuna bir istek atmanin anlami yok.
        assert_eq!(
            parse_vod_url("https://kick.com/video/b6fd5deb-9ac1-4091-87fb-758fdacfa003"),
            None
        );
        // Klipler de o ucun kapsaminda degil (yine 422).
        assert_eq!(parse_vod_url("https://kick.com/adinross/clips/clip_01M0"), None);
        assert_eq!(parse_vod_url("https://kick.com/xqc"), None);
        assert_eq!(parse_vod_url("https://example.com/xqc/videos/abc"), None);
        assert_eq!(parse_vod_url("https://kick.com/xqc/videos/abc/extra"), None);
    }

    #[test]
    fn the_real_error_payloads_yield_no_media() {
        // Servisin GERCEKTEN dondurdugu iki govde (bu saglayici yazilirken
        // olculdu). Ikisi de "medya yok" demeli, cokmemeli.
        for body in [
            r#"{"error":"Failed to resolve Kick video","output":null}"#,
            r#"{"error":"No Kick VOD found for url: https://kick.com/video/abc"}"#,
        ] {
            let json: Value = serde_json::from_str(body).unwrap();
            let mut found = Vec::new();
            collect_media(&json, &mut found);
            assert!(found.is_empty(), "hata govdesinden medya cikmamali: {body}");
        }
    }

    #[test]
    fn handles_the_real_response_the_service_returns() {
        // Servisin GERCEKTEN dondurdugu govde (kisaltildi). Yazildigi sirada
        // hicbir VOD cozulemiyordu ve sema bilinmiyordu; sonradan calisan bir
        // ornek yakalanip buraya sabitlendi.
        //
        // Iki sey ayni anda dogrulanmis oluyor: gezici ayristirici gercek
        // yanitta da isini goruyor, VE dondurulen adres bir master playlist
        // oldugu icin kalite listesi HLS saglayicisina devredilebiliyor -
        // kullanici tek secenek yerine gercek kaliteleri goruyor.
        let json: Value = serde_json::from_str(
            r#"{"video":{"id":"01a03471","title":"sznt bekleme odasi","is_live":true,
                 "language":"tr","channel":{"id":31445122,"slug":"videoyun"},
                 "thumbnail":{"src":"https://images.kick.com/video_thumbnails/a/b/720.webp",
                 "srcSet":"https://images.kick.com/video_thumbnails/a/b/1080.webp 1920w"}},
                "source":"https://stream.kick.com/3c81/ivs/v1/196/DED/media/hls/master.m3u8"}"#,
        )
        .unwrap();

        let mut found = Vec::new();
        collect_media(&json, &mut found);
        assert_eq!(
            found,
            vec![(
                "m3u8",
                "https://stream.kick.com/3c81/ivs/v1/196/DED/media/hls/master.m3u8".to_string()
            )],
            "yalnizca master playlist alinmali; kucuk resimler medya degildir"
        );
        assert_eq!(find_title(&json).as_deref(), Some("sznt bekleme odasi"));

        // Kucuk resim de tasinmali: HLS manifestinde gorsel yok, tek kaynak
        // aracinin yaniti. Tasimazsak kullanici bos bir gorsel kutusu gorur.
        assert_eq!(
            find_thumbnail(&json).as_deref(),
            Some("https://images.kick.com/video_thumbnails/a/b/720.webp"),
            "coklu-adres tasiyan srcSet degil, tek adresli src alinmali"
        );
    }

    #[test]
    fn a_payload_without_any_image_yields_no_thumbnail() {
        let json: Value =
            serde_json::from_str(r#"{"source":"https://x/master.m3u8"}"#).unwrap();
        assert_eq!(find_thumbnail(&json), None);
    }

    #[test]
    fn unrelated_urls_are_not_mistaken_for_thumbnails() {
        // Alan adina bakmasaydik medya adresi ya da site logosu gorsel
        // sanilabilirdi.
        let json: Value = serde_json::from_str(
            r#"{"logo":"https://cdn/site-logo.png","source":"https://x/master.m3u8"}"#,
        )
        .unwrap();
        assert_eq!(find_thumbnail(&json), None);
    }

    #[test]
    fn finds_media_wherever_the_schema_hides_it() {
        // Basarili yanitin semasi bilinmiyor, o yuzden alan adlarina degil
        // degerlere bakiyoruz: ne kadar derine gomulu olursa olsun bulunmali.
        let json: Value = serde_json::from_str(
            r#"{"output":{"data":[{"deep":{"src":"https://cdn.example/v/master.m3u8"}}]},
                "poster":"https://cdn.example/thumb.jpg"}"#,
        )
        .unwrap();
        let mut found = Vec::new();
        collect_media(&json, &mut found);
        assert_eq!(found, vec![("m3u8", "https://cdn.example/v/master.m3u8".to_string())]);
    }

    #[test]
    fn keeps_every_candidate_and_drops_duplicates() {
        let json: Value = serde_json::from_str(
            r#"{"a":"https://x/1080.mp4","b":"https://x/720.mp4","c":"https://x/1080.mp4",
                "d":"https://x/audio.m4a","e":"not a url","f":"https://x/page.html"}"#,
        )
        .unwrap();
        let mut found = Vec::new();
        collect_media(&json, &mut found);
        assert_eq!(
            found,
            vec![
                ("mp4", "https://x/1080.mp4".to_string()),
                ("mp4", "https://x/720.mp4".to_string()),
                ("audio", "https://x/audio.m4a".to_string()),
            ],
            "tekrarlar elenmeli, medya olmayanlar alinmamali"
        );
    }

    #[test]
    fn query_strings_do_not_hide_the_extension() {
        // Imzali CDN adresleri neredeyse her zaman sorgu parametresi tasir.
        let json: Value =
            serde_json::from_str(r#"{"u":"https://cdn.example/v/master.m3u8?token=abc&exp=1"}"#)
                .unwrap();
        let mut found = Vec::new();
        collect_media(&json, &mut found);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, "m3u8");
    }

    #[test]
    fn digs_the_title_out_of_a_nested_payload() {
        let json: Value = serde_json::from_str(
            r#"{"output":{"video":{"session_title":"Yayin kaydi"}}}"#,
        )
        .unwrap();
        assert_eq!(find_title(&json).as_deref(), Some("Yayin kaydi"));

        // Bos baslik baslik degildir: "" gosteren bir kart isimsiz gorunurdu.
        let empty: Value = serde_json::from_str(r#"{"title":"   "}"#).unwrap();
        assert_eq!(find_title(&empty), None);
    }
}

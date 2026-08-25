//! vdrop-providers: the Provider/Extractor/Resolver architecture (PRD §6-8).
//!
//! A `Provider` turns a pasted URL into `MediaInfo` (title, thumbnail,
//! available streams, etc.) without VDrop's core ever needing to know about
//! individual websites. Site-specific providers (YouTube, Vimeo, ...) are
//! meant to ship as updatable "Provider Packs" (PRD §7-8) — sandboxed units
//! that can be revised independently of the app, instead of being baked
//! into the native core. That sandboxed-JS runtime is a separate, larger
//! subsystem (see docs/ARCHITECTURE.md, section K) and is intentionally not
//! implemented here; this crate ships the trait boundary plus one concrete,
//! fully-working provider: direct HTTP(S) media links and generic HLS/DASH
//! manifests, which already covers a large class of real download links
//! (CDN links, most `.m3u8`/`.mpd` streams, self-hosted media, etc.).

pub mod dash;
pub mod extract;
pub mod hls;
pub mod kickdl;
pub mod html;
pub mod web;

pub use kickdl::KickDownloadProvider;
pub use web::WebProvider;

/// VDrop'un kendini tanittigi dize.
///
/// **Kimlik gizlemiyoruz.** Bir tarayici gibi gorunmek daha cok kapi acardi
/// ama dogru degil; ustelik bazi siteler (Wikimedia acikca) aciklayici bir
/// User-Agent istiyor ve UA'siz istekleri 403 ile reddediyor - yani durust
/// olmak ayni zamanda calisan yol.
pub const USER_AGENT: &str = concat!("VDrop/", env!("CARGO_PKG_VERSION"));

/// Uygulamanin her yerde kullandigi HTTP istemcisi.
///
/// **Bilincli olarak global bir `timeout` yok.** Bu istemciyi indirme motoru
/// da paylasiyor; toplam istek suresine sinir koymak, saatlerce suren buyuk
/// bir indirmeyi ortasindan keserdi. Saglayicilar kendi isteklerine
/// `.timeout(...)` ekler - orada takilan bir manifest istegi cozumlemeyi
/// sonsuza kadar bekletmemeli.
pub fn default_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(std::time::Duration::from_secs(15))
        // Sinirsiz yonlendirme bir donguye girebilir; 10 her gercek senaryoya yeter.
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .unwrap_or_default()
}

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("this provider cannot handle the given URL")]
    Unsupported,
    #[error("network error while probing media: {0}")]
    Network(String),
    #[error("this media appears to use DRM protection and cannot be downloaded by VDrop")]
    DrmProtected,
    #[error("could not parse media metadata: {0}")]
    Parse(String),
    /// Sayfa okundu ama icinde indirilebilir medya yok.
    ///
    /// `Parse`ten ayri bir varyant, cunku kullaniciya soylenecek sey farkli:
    /// "bir sey bozuldu" degil, "burada indirilecek bir sey goremedim, belki
    /// dogrudan medya baglantisini dene". Arayuz bunu ayri bir metinle
    /// karsilayabilsin diye dizeye degil tipe yazildi.
    #[error("no downloadable media found on the page")]
    NoMedia,
}

/// Hangi hata kullaniciya soylenmeye deger?
///
/// Zincirdeki her saglayici basarisiz olursa elimizde birden fazla hata
/// kalir ve bunlarin hepsi dogrudur. Kullaniciya en **somut** olani
/// soylemek gerekir: "bu saglayici bu adresi tanimiyor" (Unsupported) bir
/// bilgi tasimazken "sunucu 404 dondu" (Network) sorunu isaret eder.
///
/// DRM en tepede, cunku o bir arizanin degil bir sinirin adidir: baska bir
/// saglayici denemenin faydasi yoktur ve kullanicinin bilmesi gereken sey
/// tam olarak budur.
fn rank(e: &ProviderError) -> u8 {
    match e {
        ProviderError::DrmProtected => 4,
        ProviderError::Network(_) => 3,
        ProviderError::Parse(_) => 2,
        ProviderError::NoMedia => 1,
        ProviderError::Unsupported => 0,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamKind {
    Video,
    Audio,
    Muxed,
    Subtitle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOption {
    pub id: String,
    pub kind: StreamKind,
    pub url: String,
    pub container: Option<String>,
    pub codec: Option<String>,
    pub resolution: Option<String>,
    pub fps: Option<f32>,
    pub bitrate_kbps: Option<u32>,
    pub language: Option<String>,
    /// Yayincinin bu secenege verdigi ad (`English (Forced)`, `1080` ...).
    ///
    /// Neden ayri bir alan: iki altyazi izi de `en` olabilir - biri tam
    /// ceviri, digeri yalnizca yabanci replikleri gosteren "forced" iz.
    /// Yalnizca dil kodunu gostermek ikisini ayirt edilemez kilardi ve
    /// kullanici yanlis olani indirdigini ancak oynatirken anlardi.
    /// Dil kodunu bu adla birlestirmek de olurdu ama o zaman `language`
    /// alani bazen kod bazen serbest metin tasirdi.
    #[serde(default)]
    pub label: Option<String>,
    pub estimated_size_bytes: Option<u64>,
    /// HLS master playlist'teki program indeksi.
    ///
    /// Varyantin kendi playlist URL'ini indirmek yerine master URL'i +
    /// bu indeksi kullaniyoruz (`ffmpeg -map 0:p:N`). Sebebi: bazi
    /// yayinlarda ses ayri bir renditiondadir ve varyant playlist'i tek
    /// basina sessiz bir video verir. Program secimi sesi de getirir.
    #[serde(default)]
    pub variant_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaInfo {
    pub title: String,
    pub uploader: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<f64>,
    pub description: Option<String>,
    pub upload_date: Option<String>,
    pub streams: Vec<StreamOption>,
    pub is_playlist: bool,
}

/// The core contract every VDrop provider implements (PRD §6).
#[async_trait]
pub trait Provider: Send + Sync {
    /// Stable id, e.g. "direct-http", "youtube". Shown in Settings → Providers.
    fn id(&self) -> &'static str;

    /// Cheap, offline check: does this provider *plausibly* handle the URL?
    fn matches(&self, url: &str) -> bool;

    /// Resolve a URL into downloadable media info. May hit the network.
    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError>;
}

/// Cozumleme sonucu ve onu ureten saglayici.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub provider_id: &'static str,
    pub media: MediaInfo,
}

/// Tries each registered provider in order and returns the first match's
/// resolved `MediaInfo`. This is the extension point Provider Packs plug
/// into (PRD §7): site-specific providers register themselves ahead of the
/// generic fallback.
pub struct ProviderRegistry {
    providers: Vec<Box<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self { providers: Vec::new() }
    }

    pub fn register(&mut self, provider: Box<dyn Provider>) {
        self.providers.push(provider);
    }

    /// Saglayiciyi zincirin **basina** ekler.
    ///
    /// Sira yetenek sirasidir: en ozel/yetenekli olan once denenir. yt-dlp
    /// gibi calisma zamaninda bulunan opsiyonel saglayicilar bu yolla one
    /// alinir.
    pub fn register_first(&mut self, provider: Box<dyn Provider>) {
        self.providers.insert(0, provider);
    }

    /// Varsayilan saglayici seti, kendi HTTP istemcisiyle.
    pub fn with_defaults() -> Self {
        Self::with_client(default_client())
    }

    /// Varsayilan saglayici seti, disaridan verilen istemciyle.
    ///
    /// Uygulama zaten bir `reqwest::Client` tutuyor; onu paylasmak ikinci
    /// bir baglanti havuzu ve ikinci bir TLS oturum onbellegi acmayi onler.
    pub fn with_client(client: reqwest::Client) -> Self {
        let mut reg = Self::new();
        // Sira onemli, ozelden genele:
        //  1. HLS  - `.m3u8` icin kalite listesi uretir
        //  2. DASH - `.mpd` icin kalite listesi uretir
        //  3. Web  - her seyi kabul eder; adrese SORARAK ne oldugunu bulur
        //            (dogrudan medya mi, sayfa mi) ve sayfalardan medya cikarir
        //
        // `DirectMediaProvider` artik kayitli degil: yaptigi is (uzantidan
        // tahmin) `WebProvider`in kapsaminda ve o, tahmin yerine gercek
        // `Content-Type`a bakiyor. Tip disari acik kalmaya devam ediyor;
        // agsiz bir baglamda hala kullanilabilir.
        reg.register(Box::new(HlsProvider::new(client.clone())));
        reg.register(Box::new(DashProvider::new(client.clone())));
        reg.register(Box::new(WebProvider::new(client.clone())));
        // EN SONDA: cozumlenen adresi ucuncu bir tarafa gonderir.
        // Yalnizca yukaridakilerin hepsi basarisiz olursa devreye girer,
        // yani normal kullanimda hicbir adres disari cikmaz.
        reg.register(Box::new(KickDownloadProvider::new(client)));
        reg
    }

    /// Saglayicilari sirayla dener.
    ///
    /// Bir saglayici `Unsupported` donerse bu bir **yetenek eksigidir**, bir
    /// ariza degil - siradaki denenir. Ornegin yt-dlp tanimadigi bir adres
    /// icin boyle der ve genel sayfa cikarimi devreye girer.
    ///
    /// **Hicbir hata zinciri kesmez.** `Network` dahil: bir saglayicinin
    /// aldigi 404, digerlerinin de basarisiz olacagi anlamina gelmiyor.
    /// Hepsi basarisiz olursa en somut hata raporlanir (bkz. `rank`), yani
    /// kullaniciya hala dogru hikaye anlatilir - ama once cozulme sansi
    /// tuketilir.
    pub async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        Ok(self.resolve_detailed(url).await?.media)
    }

    /// Cozumlemeyi, **hangi saglayicinin** yaptigi bilgisiyle birlikte
    /// dondurur. Indirme katmani buna gore davranir: yt-dlp formatlari
    /// yt-dlp ile, HLS varyantlari FFmpeg program secimiyle iner.
    pub async fn resolve_detailed(&self, url: &str) -> Result<Resolved, ProviderError> {
        let mut best: Option<ProviderError> = None;

        for provider in &self.providers {
            if !provider.matches(url) {
                continue;
            }
            match provider.resolve(url).await {
                Ok(media) => {
                    return Ok(Resolved {
                        provider_id: provider.id(),
                        media,
                    })
                }
                Err(e) => {
                    // HICBIR hata zinciri kesmiyor.
                    //
                    // Onceden `Network` hemen yayiliyordu; gerekce "sunucu
                    // 404 verdiyse bunu baska bir saglayiciyla ortmek
                    // kullaniciya yanlis hikaye anlatir"di. O gerekce iki
                    // ayri seyi karistiriyordu: HANGI hatayi raporladigimiz
                    // ile DENEMEYI birakip birakmadigimiz. Sonraki saglayici
                    // gercekten cozerse ortada ortulen bir sey yok - sorun
                    // cozulmustur.
                    //
                    // Somut sonucu suydu: yt-dlp gecici bir 404 alinca zincir
                    // orada duruyor, `web` ve son care saglayici hic
                    // calismiyordu. Kullanici, baska bir yoldan inebilecek
                    // bir videoyu indiremiyordu.
                    if best.as_ref().map(rank).unwrap_or(0) < rank(&e) {
                        best = Some(e);
                    }
                }
            }
        }

        Err(best.unwrap_or(ProviderError::Unsupported))
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// The one provider that needs no site-specific logic at all: a URL that
/// already points straight at a media file, or at an HLS/DASH manifest.
/// This alone lets VDrop download from any site that exposes direct file
/// links (a very common case: CDN-hosted video, podcast feeds, self-hosted
/// media, most `.m3u8`/`.mpd` players, etc.) without any provider pack.
pub struct DirectMediaProvider;

impl Default for DirectMediaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectMediaProvider {
    pub fn new() -> Self {
        Self
    }

    fn guess_container(url: &str) -> Option<String> {
        let path = url.split(['?', '#']).next().unwrap_or(url);
        let ext = path.rsplit('.').next()?.to_lowercase();
        const KNOWN: &[&str] = &[
            "mp4", "mkv", "webm", "mov", "m4a", "mp3", "aac", "opus", "ogg", "flac", "wav",
            "m3u8", "mpd",
        ];
        KNOWN.contains(&ext.as_str()).then_some(ext)
    }
}

#[async_trait]
impl Provider for DirectMediaProvider {
    fn id(&self) -> &'static str {
        "direct-http"
    }

    fn matches(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let container = Self::guess_container(url);
        let is_manifest = matches!(container.as_deref(), Some("m3u8") | Some("mpd"));

        let title = url
            .split('/')
            .next_back()
            .unwrap_or("media")
            .split(['?', '#'])
            .next()
            .unwrap_or("media")
            .to_string();

        let kind = if matches!(
            container.as_deref(),
            Some("mp3") | Some("m4a") | Some("aac") | Some("opus") | Some("ogg") | Some("flac") | Some("wav")
        ) {
            StreamKind::Audio
        } else {
            StreamKind::Muxed
        };

        Ok(MediaInfo {
            title,
            uploader: None,
            thumbnail_url: None,
            duration_seconds: None,
            description: None,
            upload_date: None,
            is_playlist: false,
            streams: vec![StreamOption {
                id: "direct".to_string(),
                kind,
                url: url.to_string(),
                container: container.or_else(|| is_manifest.then(|| "hls-or-dash".to_string())),
                codec: None,
                resolution: None,
                fps: None,
                bitrate_kbps: None,
                language: None,
                label: None,
                estimated_size_bytes: None,
                variant_index: None,
            }],
        })
    }
}

/// HLS master playlist'lerini kalite listesine cevirir.
///
/// Ayristirma mantigi `hls` modulunde (ag erisimi yok, saf ve test edilebilir);
/// burasi yalnizca manifesti indirip o mantigi uygular.
pub struct HlsProvider {
    client: reqwest::Client,
}

impl HlsProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    async fn fetch(&self, url: &str) -> Result<String, ProviderError> {
        let resp = self
            .client
            .get(url)
            // Manifest istegi kucuktur; takilirsa cozumlemeyi sonsuza kadar
            // bekletmesin. Istemci genelinde degil, istek basina sinir.
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(ProviderError::Network(format!(
                "server returned {}",
                resp.status().as_u16()
            )));
        }
        resp.text()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))
    }
}

#[async_trait]
impl Provider for HlsProvider {
    fn id(&self) -> &'static str {
        "hls"
    }

    fn matches(&self, url: &str) -> bool {
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return false;
        }
        let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
        path.ends_with(".m3u8") || path.ends_with(".m3u")
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let text = self.fetch(url).await?;

        let title = url
            .split(['?', '#'])
            .next()
            .unwrap_or(url)
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("stream")
            .to_string();

        if !hls::is_master_playlist(&text) {
            // Dogrudan bir medya playlist'i: tek kalite var, secim yok.
            let duration = hls::total_duration_seconds(&text);
            return Ok(MediaInfo {
                title,
                duration_seconds: duration,
                streams: vec![StreamOption {
                    id: "hls".to_string(),
                    kind: StreamKind::Muxed,
                    url: url.to_string(),
                    container: Some("m3u8".to_string()),
                    codec: None,
                    resolution: None,
                    fps: None,
                    bitrate_kbps: None,
                    language: None,
                    label: None,
                    estimated_size_bytes: None,
                    variant_index: None,
                }],
                ..Default::default()
            });
        }

        let variants = hls::parse_master(&text, url);
        if variants.is_empty() {
            return Err(ProviderError::Parse(
                "master playlist parsed but it declared no variants".into(),
            ));
        }

        // Sure icin tek bir varyantin medya playlist'ini cekiyoruz: sure tum
        // varyantlarda ayni oldugu icin bir istek yeter. En dusuk bant
        // genisliklisini seciyoruz - en kucuk dosya, en hizli cevap.
        let probe_target = variants
            .iter()
            .min_by_key(|v| v.bandwidth_bps.unwrap_or(u64::MAX))
            .unwrap();
        let duration = match self.fetch(&probe_target.url).await {
            Ok(media) => hls::total_duration_seconds(&media),
            // Sure ogrenilemezse cozumleme basarisiz sayilmaz: kullanici
            // yine de kalite secebilir, sadece boyut tahmini gorunmez.
            Err(_) => None,
        };

        let mut streams: Vec<StreamOption> = variants
            .iter()
            .map(|v| StreamOption {
                id: format!("hls-{}", v.index),
                kind: StreamKind::Muxed,
                // DIKKAT: varyantin kendi URL'i degil, master URL'i.
                // Secim `variant_index` ile FFmpeg'e program olarak verilir;
                // gerekcesi `hls` modulunun basinda.
                url: url.to_string(),
                container: Some("m3u8".to_string()),
                codec: v.codecs.clone(),
                resolution: v.resolution(),
                fps: v.frame_rate,
                bitrate_kbps: v.bitrate_kbps(),
                language: None,
                // Yayincinin adi (`NAME="1080"`) simdiye kadar ayristirilip
                // atiliyordu; cozunurluk bilinmeyen yayinlarda tek ipucu bu.
                label: v.name.clone(),
                estimated_size_bytes: duration.and_then(|d| v.estimated_bytes(d)),
                variant_index: Some(v.index),
            })
            .collect();

        // En yuksek kalite basta: kullanicilarin cogunlugu en iyisini ister
        // ve varsayilan secim ilk satirdir.
        streams.sort_by(|a, b| {
            b.bitrate_kbps
                .unwrap_or(0)
                .cmp(&a.bitrate_kbps.unwrap_or(0))
        });

        // Altyazilar siralamanin DISINDA, sona ekleniyor. Bit hizina gore
        // siralanan bir listeye karisirlarsa "1080p, altyazi, 720p" gibi
        // anlamsiz bir sira cikardi; ustelik varsayilan secim ilk satir
        // oldugu icin kullanici farkinda olmadan altyazi indirebilirdi.
        for (i, track) in hls::parse_subtitles(&text, url).into_iter().enumerate() {
            streams.push(StreamOption {
                id: format!("hls-sub-{i}"),
                kind: StreamKind::Subtitle,
                // Varyantlarin aksine izin KENDI adresi kullaniliyor: altyazi
                // rendition'i tek basina eksiksiz, program secmeye gerek yok.
                url: track.url,
                // Cikti bicimi: FFmpeg WebVTT segmentlerini SRT'ye cevirir.
                // SRT'yi her oynatici acar, .vtt'yi bircogu acmaz.
                container: Some("srt".to_string()),
                codec: None,
                resolution: None,
                fps: None,
                bitrate_kbps: None,
                language: track.language,
                label: track.name.map(|name| {
                    if track.forced {
                        // "Forced" izler yalnizca yabanci replikleri gosterir.
                        // Ad zaten bunu soyluyorsa tekrar etmiyoruz.
                        if name.to_lowercase().contains("forced") {
                            name
                        } else {
                            format!("{name} (forced)")
                        }
                    } else {
                        name
                    }
                }),
                estimated_size_bytes: None,
                variant_index: None,
            });
        }

        Ok(MediaInfo {
            title,
            duration_seconds: duration,
            streams,
            ..Default::default()
        })
    }
}

/// DASH manifestlerini kalite listesine cevirir.
///
/// `HlsProvider` ile ayni sekli izler ama secici anlami farklidir: burada
/// `variant_index` bir **video akis indeksi**dir, program indeksi degil
/// (bkz. `dash` modulunun basi). Bu ayrimi `provider_id` tasir - indirme
/// katmani ondan hangi `-map` bicimini kullanacagini bilir.
pub struct DashProvider {
    client: reqwest::Client,
}

impl DashProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Provider for DashProvider {
    fn id(&self) -> &'static str {
        "dash"
    }

    fn matches(&self, url: &str) -> bool {
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return false;
        }
        let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
        path.ends_with(".mpd")
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let resp = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(ProviderError::Network(format!(
                "server returned {}",
                resp.status().as_u16()
            )));
        }
        let text = resp
            .text()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let manifest = dash::parse(&text);
        if manifest.representations.is_empty() {
            return Err(ProviderError::Parse(
                "the DASH manifest declared no video representation".into(),
            ));
        }

        let title = url
            .split(['?', '#'])
            .next()
            .unwrap_or(url)
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("stream")
            .to_string();

        let duration = manifest.duration_seconds;
        let mut streams: Vec<StreamOption> = manifest
            .representations
            .iter()
            .map(|r| StreamOption {
                id: format!("dash-{}", r.video_stream_index),
                kind: StreamKind::Muxed,
                url: url.to_string(),
                container: Some("mpd".to_string()),
                codec: r.codecs.clone(),
                resolution: r.resolution(),
                fps: r.frame_rate,
                bitrate_kbps: r.bitrate_kbps(),
                language: None,
                label: None,
                estimated_size_bytes: duration.and_then(|d| r.estimated_bytes(d)),
                variant_index: Some(r.video_stream_index),
            })
            .collect();

        // En yuksek kalite basta: varsayilan secim ilk satirdir.
        streams.sort_by(|a, b| {
            b.bitrate_kbps
                .unwrap_or(0)
                .cmp(&a.bitrate_kbps.unwrap_or(0))
        });

        Ok(MediaInfo {
            title,
            duration_seconds: duration,
            streams,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `DirectMediaProvider` agsiz calisan yedek yoldur: adresi yalnizca
    /// uzantisindan degerlendirir. Kayitli saglayici listesinde degil (bkz.
    /// `with_client`), ama davranisi bozulmamali - agin olmadigi bir
    /// baglamda tek secenek odur.
    #[tokio::test]
    async fn direct_provider_guesses_from_the_extension_without_network() {
        let provider = DirectMediaProvider::new();
        assert!(provider.matches("https://cdn.example.com/videos/sample.mp4"));

        let info = provider
            .resolve("https://cdn.example.com/videos/sample.mp4")
            .await
            .unwrap();
        assert_eq!(info.title, "sample.mp4");
        assert_eq!(info.streams.len(), 1);
        assert_eq!(info.streams[0].container.as_deref(), Some("mp4"));

        let audio = provider
            .resolve("https://cdn.example.com/podcast/bolum-3.mp3")
            .await
            .unwrap();
        assert!(matches!(audio.streams[0].kind, StreamKind::Audio));
    }

    /// Zincir davranisini olcmek icin ag gerektirmeyen saglayici.
    struct Fake {
        id: &'static str,
        fail: Option<u8>,
    }

    #[async_trait]
    impl Provider for Fake {
        fn id(&self) -> &'static str {
            self.id
        }
        fn matches(&self, _url: &str) -> bool {
            true
        }
        async fn resolve(&self, _url: &str) -> Result<MediaInfo, ProviderError> {
            match self.fail {
                None => Ok(MediaInfo {
                    title: self.id.to_string(),
                    ..Default::default()
                }),
                Some(0) => Err(ProviderError::Unsupported),
                Some(1) => Err(ProviderError::NoMedia),
                Some(2) => Err(ProviderError::Parse("bozuk".into())),
                Some(3) => Err(ProviderError::Network("server returned 404".into())),
                _ => Err(ProviderError::DrmProtected),
            }
        }
    }

    fn chain(providers: Vec<Fake>) -> ProviderRegistry {
        let mut reg = ProviderRegistry::new();
        for p in providers {
            reg.register(Box::new(p));
        }
        reg
    }

    #[tokio::test]
    async fn a_network_failure_no_longer_stops_the_chain() {
        // GERCEK VAKA: yt-dlp bir Kick VOD'u icin gecici 404 aldi. Eskiden
        // `Network` hemen yayiliyordu, yani `web` ve son care saglayici hic
        // calismiyordu - baska bir yoldan inebilecek video inmiyordu.
        let reg = chain(vec![
            Fake { id: "yt-dlp", fail: Some(3) },
            Fake { id: "son-care", fail: None },
        ]);

        let resolved = reg
            .resolve_detailed("https://kick.com/x/videos/abc")
            .await
            .expect("sonraki saglayici cozebilmeliydi");
        assert_eq!(resolved.provider_id, "son-care");
    }

    #[tokio::test]
    async fn reports_the_most_concrete_failure_when_everything_fails() {
        // Hepsi basarisizsa elimizde birden fazla dogru hata kalir.
        // "Bu saglayici bu adresi tanimiyor" bilgi tasimaz; "sunucu 404
        // dondu" sorunu isaret eder.
        let reg = chain(vec![
            Fake { id: "a", fail: Some(0) },
            Fake { id: "b", fail: Some(3) },
            Fake { id: "c", fail: Some(0) },
        ]);

        match reg.resolve_detailed("https://x.com/v").await {
            Err(ProviderError::Network(msg)) => assert!(msg.contains("404")),
            other => panic!("Network bekleniyordu: {other:?}"),
        }
    }

    #[tokio::test]
    async fn drm_outranks_every_other_failure() {
        // DRM bir ariza degil, bir sinir: baska saglayici denemenin faydasi
        // yok ve kullanicinin bilmesi gereken tam olarak budur.
        let reg = chain(vec![
            Fake { id: "a", fail: Some(3) },
            Fake { id: "b", fail: Some(4) },
            Fake { id: "c", fail: Some(2) },
        ]);

        assert!(matches!(
            reg.resolve_detailed("https://x.com/v").await,
            Err(ProviderError::DrmProtected)
        ));
    }

    #[tokio::test]
    async fn the_first_success_wins_and_later_providers_are_not_called() {
        let reg = chain(vec![
            Fake { id: "ilk", fail: None },
            Fake { id: "ikinci", fail: None },
        ]);
        let resolved = reg.resolve_detailed("https://x.com/v").await.unwrap();
        assert_eq!(resolved.provider_id, "ilk");
    }

    #[tokio::test]
    async fn registry_order_puts_hls_before_the_general_provider() {
        let reg = ProviderRegistry::with_defaults();
        // Ag istegi atmadan yalnizca eslesme sirasini dogruluyoruz.
        let ids: Vec<_> = reg.providers.iter().map(|p| p.id()).collect();
        assert_eq!(ids, vec!["hls", "dash", "web", "kick-video.download"]);

        // Araci EN SONDA olmali. Bu bir siralama tercihi degil, bir gizlilik
        // kurali: ondan once gelen her saglayici basarisiz olmadikca
        // cozumlenen adres ucuncu bir tarafa gitmez. Yukari tasinirsa bu
        // test kirilir ve neden kirildigini soyler.
        assert_eq!(
            ids.last(),
            Some(&"kick-video.download"),
            "araci saglayici zincirin sonunda kalmali"
        );
    }

    #[tokio::test]
    async fn unsupported_scheme_is_rejected() {
        let reg = ProviderRegistry::with_defaults();
        let err = reg.resolve("ftp://example.com/video.mp4").await.unwrap_err();
        assert!(matches!(err, ProviderError::Unsupported));
    }
}

//! `vdrop_providers::Provider` uyarlamasi.

use async_trait::async_trait;
use vdrop_providers::{MediaInfo, Provider, ProviderError, StreamKind, StreamOption};

use crate::{resolve, Format, YtDlp, YtDlpError};

pub struct YtDlpProvider {
    ytdlp: YtDlp,
}

impl YtDlpProvider {
    pub fn new(ytdlp: YtDlp) -> Self {
        Self { ytdlp }
    }
}

fn to_provider_error(e: YtDlpError) -> ProviderError {
    match e {
        // "Desteklenmiyor" bir yetenek eksigidir, bir arıza degil: kayit
        // zinciri bir sonraki saglayiciyi denesin diye boyle isaretliyoruz.
        YtDlpError::Unsupported | YtDlpError::Missing => ProviderError::Unsupported,
        YtDlpError::Parse(m) => ProviderError::Parse(m),
        other => ProviderError::Network(other.to_string()),
    }
}

#[async_trait]
impl Provider for YtDlpProvider {
    fn id(&self) -> &'static str {
        "yt-dlp"
    }

    fn matches(&self, url: &str) -> bool {
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return false;
        }
        // Dogrudan medya dosyalari ve manifestler icin yt-dlp cagirmiyoruz:
        // bir alt surec baslatmak gereksiz ve yavas olurdu, ustelik kendi
        // motorumuz o adresleri zaten daha iyi ele aliyor (devam ettirme,
        // duraklatma, kalite listesi).
        let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
        const DIRECT: &[&str] = &[
            ".mp4", ".mkv", ".webm", ".mov", ".m4v", ".avi", ".flv", ".ts", ".ogv", ".mp3", ".m4a",
            ".aac", ".opus", ".ogg", ".flac", ".wav", ".m3u8", ".m3u", ".mpd",
        ];
        !DIRECT.iter().any(|ext| path.ends_with(ext))
    }

    async fn resolve(&self, url: &str) -> Result<MediaInfo, ProviderError> {
        let info = resolve(&self.ytdlp, url).await.map_err(to_provider_error)?;

        // Akislar SAYFA adresini tasir, format URL'ini degil: indirmeyi yine
        // yt-dlp yaptigi icin ona sayfa adresi + format kimligi lazim. Ayrica
        // format URL'leri kisa omurludur; bir saat sonra devam ettirilen bir
        // indirme onlarla 403 alirdi.
        let page_url = info.webpage_url.clone().unwrap_or_else(|| url.to_string());
        let mut streams: Vec<StreamOption> = info
            .formats
            .iter()
            .filter(|f| f.has_video() || f.has_audio())
            .map(|f| to_stream(f, &page_url))
            .collect();

        if streams.is_empty() {
            return Err(ProviderError::Parse(
                "yt-dlp reported no downloadable format".into(),
            ));
        }

        // Once video, sonra ses; her grup kendi icinde kaliteden dusuge.
        // Kullanicilarin cogu en iyi goruntuyu ister ve varsayilan secim
        // ilk satirdir.
        streams.sort_by(|a, b| {
            let rank = |s: &StreamOption| match s.kind {
                StreamKind::Audio => 1,
                _ => 0,
            };
            rank(a)
                .cmp(&rank(b))
                .then_with(|| height_of(b).cmp(&height_of(a)))
                .then_with(|| b.bitrate_kbps.unwrap_or(0).cmp(&a.bitrate_kbps.unwrap_or(0)))
        });

        Ok(MediaInfo {
            title: info
                .title
                .filter(|t| !t.trim().is_empty())
                .unwrap_or_else(|| "video".to_string()),
            uploader: info.uploader.or(info.channel),
            thumbnail_url: info.thumbnail,
            duration_seconds: info.duration,
            description: info.description,
            upload_date: info.upload_date,
            streams,
            is_playlist: false,
        })
    }
}

/// Siralama icin piksel yuksekligi. "1920x1080" ya da "1080p" bicimlerini
/// anlar; bilinmiyorsa 0 (en sona duser).
fn height_of(s: &StreamOption) -> u32 {
    let Some(res) = s.resolution.as_deref() else {
        return 0;
    };
    if let Some((_, h)) = res.split_once(['x', 'X']) {
        return h.parse().unwrap_or(0);
    }
    res.trim_end_matches(['p', 'P']).parse().unwrap_or(0)
}

fn to_stream(f: &Format, page_url: &str) -> StreamOption {
    let kind = match (f.has_video(), f.has_audio()) {
        (true, true) => StreamKind::Muxed,
        (true, false) => StreamKind::Video,
        _ => StreamKind::Audio,
    };

    StreamOption {
        // Kimlik = yt-dlp format kimligi. Indirme katmani bunu `-f` olarak
        // geri verir, o yuzden birebir korunmali.
        id: f.format_id.clone(),
        kind,
        url: page_url.to_string(),
        container: f.ext.clone(),
        codec: f.codec_label(),
        resolution: f.resolution(),
        fps: f.fps,
        bitrate_kbps: f.bitrate_kbps(),
        language: None,
        label: None,
        estimated_size_bytes: f.size_bytes(),
        variant_index: None,
    }
}

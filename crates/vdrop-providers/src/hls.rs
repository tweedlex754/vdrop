//! HLS master playlist ayristirici.
//!
//! Bir `.m3u8` baglantisi genelde bir **master playlist**'tir: icinde asil
//! video yoktur, farkli kalitelerdeki varyantlarin listesi vardir. VDrop
//! bunu ayristirmadan kullaniciya tek bir "Akis" secenegi gosteriyordu -
//! oysa uygulamanin vaadi kaliteyi kullanicinin secmesi.
//!
//! ## Varyanti nasil indiriyoruz
//!
//! Kullanici "720p" sectiginde, o varyantin kendi playlist URL'ini FFmpeg'e
//! vermek **yanlis** olurdu: bazi yayinlarda ses ayri bir renditiondadir
//! (`#EXT-X-MEDIA:TYPE=AUDIO`) ve varyant playlist'i yalnizca goruntu tasir.
//! Sonuc sessiz bir video olurdu.
//!
//! Bunun yerine master URL'i FFmpeg'e verip varyanti **program indeksiyle**
//! seciyoruz (`-map 0:p:N`). FFmpeg master playlist'teki her varyanti bir
//! program olarak acar, manifest sirasiyla, ve `-map 0:p:N` o programin tum
//! akislarini (ses dahil) alir.
//!
//! ## Neden elle ayristiriyoruz
//!
//! Format listesi icin bant genisligi, cozunurluk ve kodek lazim; bunlar
//! yalnizca manifestte var. FFprobe da verebilirdi ama her cozumlemede bir
//! alt surec baslatmak, FFmpeg'i kurulu olmayan kullanicida da format
//! listesini yok ederdi. Manifest ayristirmak birkac satirlik is.

use serde::{Deserialize, Serialize};

/// Master playlist'teki tek bir kalite secenegi.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    /// FFmpeg'e verilecek program indeksi. Manifestteki sirayla ayni.
    pub index: u32,
    /// Varyantin kendi playlist URL'i (mutlak hale getirilmis).
    /// Su an indirmede kullanilmiyor (yukaridaki ses gerekcesi), ama
    /// hata ayiklama ve ileride segment indirici icin lazim.
    pub url: String,
    pub bandwidth_bps: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codecs: Option<String>,
    pub frame_rate: Option<f32>,
    /// Yayincinin verdigi ad (`NAME="1080"` gibi). Varsa cozunurluk yerine
    /// bunu gostermek daha dogru olabilir.
    pub name: Option<String>,
}

impl Variant {
    /// "1920x1080" biciminde cozunurluk.
    pub fn resolution(&self) -> Option<String> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some(format!("{w}x{h}")),
            _ => None,
        }
    }

    pub fn bitrate_kbps(&self) -> Option<u32> {
        self.bandwidth_bps.map(|b| (b / 1000) as u32)
    }

    /// Bilinen sureye gore kaba boyut tahmini.
    /// HLS'te gercek boyut ancak tum segmentler indirilince bilinir; bu
    /// tahmin kullanicinin "1080p mi 480p mi" kararini vermesine yeter.
    pub fn estimated_bytes(&self, duration_seconds: f64) -> Option<u64> {
        let bw = self.bandwidth_bps? as f64;
        if duration_seconds <= 0.0 {
            return None;
        }
        Some((bw * duration_seconds / 8.0) as u64)
    }
}

/// Metin bir master playlist mi (varyant listesi), yoksa medya playlist'i mi
/// (segment listesi)?
pub fn is_master_playlist(text: &str) -> bool {
    text.lines()
        .any(|l| l.trim_start().starts_with("#EXT-X-STREAM-INF"))
}

/// Medya playlist'indeki `#EXTINF` surelerini toplar.
///
/// VOD icin gercek toplam suredir; canli yayinda (`#EXT-X-ENDLIST` yok)
/// yalnizca elimizdeki pencerenin uzunlugudur, o yuzden `None` doneriz -
/// canli bir yayin icin "10 dakika" demek yaniltici olurdu.
pub fn total_duration_seconds(media_playlist: &str) -> Option<f64> {
    let mut total = 0.0f64;
    let mut saw_segment = false;
    let mut has_endlist = false;

    for line in media_playlist.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("#EXTINF:") {
            // "#EXTINF:9.009,baslik" -> ilk virgule kadar
            let value = rest.split(',').next().unwrap_or(rest).trim();
            if let Ok(seconds) = value.parse::<f64>() {
                total += seconds;
                saw_segment = true;
            }
        } else if line == "#EXT-X-ENDLIST" {
            has_endlist = true;
        }
    }

    (saw_segment && has_endlist).then_some(total)
}

/// Master playlist'i varyantlara ayristirir.
///
/// `base_url` goreli URI'lari mutlak hale getirmek icin kullanilir
/// ("url_0/x.m3u8" gibi girdiler cok yaygindir).
/// Master playlist'teki bir altyazi izi (`#EXT-X-MEDIA:TYPE=SUBTITLES`).
///
/// Varyantlardan ayri bir tip, cunku ayri bir sey: varyant bir kalite
/// secenegidir, altyazi izi bir dil secenegidir. Ikisini tek listede
/// birlestirmek "1080p mi Turkce mi" gibi anlamsiz bir secim uretirdi.
#[derive(Debug, Clone, PartialEq)]
pub struct SubtitleTrack {
    /// Izin kendi playlist adresi (mutlaklastirilmis).
    ///
    /// Bu adres dogrudan indirilebilir oldugu icin altyaziyi cekerken
    /// master playlist + `-map 0:s:N` yoluna hic gerek kalmiyor.
    pub url: String,
    /// BCP-47 dil etiketi (`en`, `fr`, `tr`...).
    pub language: Option<String>,
    /// Yayincinin verdigi ad (`English`, `Francais`...).
    pub name: Option<String>,
    /// "Forced" izler yalnizca yabanci dildeki replikleri gosterir; tam
    /// bir ceviri degildir. Ayirt edilmezse kullanici sagir bir altyazi
    /// indirdigini ancak oynatirken anlar.
    pub forced: bool,
}

/// Master playlist'teki altyazi izlerini cikarir.
///
/// `parse_master`ten ayri bir gecis: o fonksiyon `#EXT-X-STREAM-INF`
/// satirlarini ve onlari izleyen URI satirlarini isliyor, altyazilar ise
/// kendi URI'sini oznitelik olarak tasiyor. Ayni donguye sikistirmak iki
/// durumu da bulanistirirdi.
pub fn parse_subtitles(text: &str, base_url: &str) -> Vec<SubtitleTrack> {
    let mut out = Vec::new();

    for raw in text.lines() {
        let Some(rest) = raw.trim().strip_prefix("#EXT-X-MEDIA:") else {
            continue;
        };
        let attrs = Attributes::parse(rest);

        if !attrs
            .get("TYPE")
            .is_some_and(|t| t.eq_ignore_ascii_case("SUBTITLES"))
        {
            continue;
        }

        // URI'si olmayan bir iz indirilemez; listede gostermek kullaniciya
        // tiklayinca hicbir sey yapmayan bir secenek vermek olurdu.
        let Some(uri) = attrs.get("URI").filter(|u| !u.is_empty()) else {
            continue;
        };

        out.push(SubtitleTrack {
            url: absolutize(base_url, uri),
            language: attrs
                .get("LANGUAGE")
                .map(str::to_string)
                .filter(|s| !s.is_empty()),
            name: attrs
                .get("NAME")
                .map(str::to_string)
                .filter(|s| !s.is_empty()),
            forced: attrs
                .get("FORCED")
                .is_some_and(|v| v.eq_ignore_ascii_case("YES")),
        });
    }

    out
}

pub fn parse_master(text: &str, base_url: &str) -> Vec<Variant> {
    let mut variants = Vec::new();
    let mut pending: Option<Attributes> = None;
    let mut index = 0u32;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(attrs) = line.strip_prefix("#EXT-X-STREAM-INF:") {
            pending = Some(Attributes::parse(attrs));
            continue;
        }

        if line.starts_with('#') {
            continue;
        }

        // Yorum olmayan bir satir: bir onceki STREAM-INF'in URI'si.
        let Some(attrs) = pending.take() else {
            continue;
        };

        let (width, height) = attrs
            .get("RESOLUTION")
            .and_then(parse_resolution)
            .map(|(w, h)| (Some(w), Some(h)))
            .unwrap_or((None, None));

        variants.push(Variant {
            index,
            url: absolutize(base_url, line),
            // AVERAGE-BANDWIDTH varsa onu tercih ederiz: BANDWIDTH tepe
            // degeridir ve boyut tahminini sisirir.
            bandwidth_bps: attrs
                .get("AVERAGE-BANDWIDTH")
                .or_else(|| attrs.get("BANDWIDTH"))
                .and_then(|v| v.parse().ok()),
            width,
            height,
            codecs: attrs.get("CODECS").map(str::to_string),
            frame_rate: attrs.get("FRAME-RATE").and_then(|v| v.parse().ok()),
            name: attrs.get("NAME").map(str::to_string),
        });
        index += 1;
    }

    variants
}

fn parse_resolution(value: &str) -> Option<(u32, u32)> {
    let (w, h) = value.split_once('x')?;
    Some((w.trim().parse().ok()?, h.trim().parse().ok()?))
}

/// Goreli bir URI'yi manifest adresine gore mutlak hale getirir.
///
/// Tam bir URL kutuphanesi yerine elle yaziyoruz cunku ihtiyacimiz dar:
/// mutlak URL, kok-goreli yol, ve ayni klasordeki goreli yol. Buna karsilik
/// tum bagimlilik agacindan kurtuluyoruz.
pub fn absolutize(base: &str, uri: &str) -> String {
    if uri.starts_with("http://") || uri.starts_with("https://") {
        return uri.to_string();
    }

    // Sorgu ve fragmani at: taban yol icin anlamsizlar.
    let base_clean = base.split(['?', '#']).next().unwrap_or(base);

    if let Some(rest) = uri.strip_prefix("//") {
        let scheme = if base_clean.starts_with("http://") { "http" } else { "https" };
        return format!("{scheme}://{rest}");
    }

    // Kok-goreli: "/a/b.m3u8" -> sema + host ile birlestir.
    if uri.starts_with('/') {
        let after_scheme = base_clean.find("://").map(|i| i + 3).unwrap_or(0);
        let host_end = base_clean[after_scheme..]
            .find('/')
            .map(|i| after_scheme + i)
            .unwrap_or(base_clean.len());
        return format!("{}{}", &base_clean[..host_end], uri);
    }

    // Ayni klasor: son "/"e kadar olan kismi al.
    match base_clean.rfind('/') {
        Some(i) => format!("{}{}", &base_clean[..=i], uri),
        None => uri.to_string(),
    }
}

/// `#EXT-X-STREAM-INF` oznitelik listesi.
///
/// Basit bir `split(',')` **calismaz**: `CODECS="mp4a.40.2,avc1.64001f"`
/// tirnak icinde virgul tasir. Bu, HLS ayristiricilarda en sik yapilan
/// hatadir ve sessizce yanlis kodek/bant genisligi uretir.
struct Attributes(Vec<(String, String)>);

impl Attributes {
    fn parse(input: &str) -> Self {
        let mut out = Vec::new();
        let mut key = String::new();
        let mut value = String::new();
        let mut in_value = false;
        let mut in_quotes = false;

        for c in input.chars() {
            match c {
                '"' => in_quotes = !in_quotes,
                '=' if !in_value && !in_quotes => in_value = true,
                ',' if !in_quotes => {
                    if !key.trim().is_empty() {
                        out.push((key.trim().to_string(), value.trim().to_string()));
                    }
                    key.clear();
                    value.clear();
                    in_value = false;
                }
                _ => {
                    if in_value {
                        value.push(c);
                    } else {
                        key.push(c);
                    }
                }
            }
        }
        if !key.trim().is_empty() {
            out.push((key.trim().to_string(), value.trim().to_string()));
        }
        Self(out)
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mux'un herkese acik test yayinindan alinmis gercek bir master
    /// playlist. Uydurma bir ornek yerine gercegini kullaniyoruz.
    const MUX_MASTER: &str = r#"#EXTM3U
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=2149280,CODECS="mp4a.40.2,avc1.64001f",RESOLUTION=1280x720,NAME="720"
url_0/193039199_mp4_h264_aac_hd_7.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=246440,CODECS="mp4a.40.5,avc1.42000d",RESOLUTION=320x184,NAME="240"
url_2/193039199_mp4_h264_aac_ld_7.m3u8
#EXT-X-STREAM-INF:PROGRAM-ID=1,BANDWIDTH=6221600,CODECS="mp4a.40.2,avc1.640028",RESOLUTION=1920x1080,NAME="1080"
url_8/193039199_mp4_h264_aac_fhd_7.m3u8
"#;

    /// Apple'in herkese acik bipbop test yayinindan alinmis gercek
    /// `EXT-X-MEDIA` satirlari. Uydurma bir ornek, gercek manifestlerdeki
    /// oznitelik siralamasini ve kacislari yansitmazdi.
    const APPLE_SUBS: &str = r#"#EXTM3U
#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="aud1",NAME="English",LANGUAGE="en",URI="a1/prog_index.m3u8"
#EXT-X-MEDIA:TYPE=SUBTITLES,GROUP-ID="subs",NAME="English",DEFAULT=YES,AUTOSELECT=YES,FORCED=NO,LANGUAGE="en",CHARACTERISTICS="public.accessibility.transcribes-spoken-dialog",URI="subtitles/eng/prog_index.m3u8"
#EXT-X-MEDIA:TYPE=SUBTITLES,GROUP-ID="subs",NAME="English (Forced)",DEFAULT=NO,AUTOSELECT=NO,FORCED=YES,LANGUAGE="en",URI="subtitles/eng_forced/prog_index.m3u8"
#EXT-X-STREAM-INF:BANDWIDTH=2149280,RESOLUTION=1280x720,SUBTITLES="subs"
v1/prog_index.m3u8
"#;

    #[test]
    fn reads_subtitle_tracks_out_of_a_master_playlist() {
        let subs = parse_subtitles(
            APPLE_SUBS,
            "https://devstreaming-cdn.apple.com/videos/bipbop_16x9/variant.m3u8",
        );

        assert_eq!(subs.len(), 2, "yalnizca SUBTITLES izleri alinmali");
        assert_eq!(
            subs[0],
            SubtitleTrack {
                url: "https://devstreaming-cdn.apple.com/videos/bipbop_16x9/subtitles/eng/prog_index.m3u8".into(),
                language: Some("en".into()),
                name: Some("English".into()),
                forced: false,
            }
        );
        assert!(subs[1].forced, "FORCED=YES ayirt edilmeli");
        assert_eq!(subs[1].name.as_deref(), Some("English (Forced)"));
    }

    #[test]
    fn ignores_audio_renditions_and_uriless_tracks() {
        // TYPE=AUDIO ayni satir bicimini kullanir; altyazi listesine
        // sizmamali. URI'si olmayan iz de indirilemez.
        let text = concat!(
            "#EXTM3U\n",
            "#EXT-X-MEDIA:TYPE=AUDIO,NAME=\"English\",LANGUAGE=\"en\",URI=\"a1/index.m3u8\"\n",
            "#EXT-X-MEDIA:TYPE=SUBTITLES,NAME=\"Yok\",LANGUAGE=\"tr\"\n",
            "#EXT-X-MEDIA:TYPE=CLOSED-CAPTIONS,NAME=\"CC1\",LANGUAGE=\"en\"\n"
        );
        assert!(parse_subtitles(text, "https://x.com/a.m3u8").is_empty());
    }

    #[test]
    fn a_playlist_without_subtitles_yields_none() {
        // Sik durum: yayinlarin cogunda altyazi yok. Bos liste donmeli,
        // panik degil.
        assert!(parse_subtitles(MUX_MASTER, "https://x.com/a.m3u8").is_empty());
        assert!(parse_subtitles("", "https://x.com/a.m3u8").is_empty());
    }

    #[test]
    fn detects_master_versus_media_playlist() {
        assert!(is_master_playlist(MUX_MASTER));
        assert!(!is_master_playlist(
            "#EXTM3U\n#EXTINF:9.009,\nsegment0.ts\n#EXT-X-ENDLIST\n"
        ));
    }

    #[test]
    fn parses_every_variant_in_order() {
        let v = parse_master(MUX_MASTER, "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8");
        assert_eq!(v.len(), 3);

        // Indeks sirasi FFmpeg'in program sirasiyla ayni olmali; yanlis
        // olursa kullanici 1080p secip 240p indirir.
        assert_eq!(v[0].index, 0);
        assert_eq!(v[1].index, 1);
        assert_eq!(v[2].index, 2);

        assert_eq!(v[0].resolution().as_deref(), Some("1280x720"));
        assert_eq!(v[1].resolution().as_deref(), Some("320x184"));
        assert_eq!(v[2].resolution().as_deref(), Some("1920x1080"));
    }

    #[test]
    fn quoted_commas_inside_codecs_do_not_split_attributes() {
        // Bu, HLS ayristiricilarda en sik yapilan hata: naif bir
        // split(',') CODECS'i ikiye boler ve RESOLUTION'i kaybeder.
        let v = parse_master(MUX_MASTER, "https://x.dev/a.m3u8");
        assert_eq!(v[0].codecs.as_deref(), Some("mp4a.40.2,avc1.64001f"));
        assert_eq!(
            v[0].bandwidth_bps,
            Some(2_149_280),
            "tirnak icindeki virgul bant genisligini bozmamali"
        );
        assert_eq!(v[0].name.as_deref(), Some("720"));
    }

    #[test]
    fn resolves_relative_variant_uris() {
        let v = parse_master(MUX_MASTER, "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8");
        assert_eq!(
            v[0].url,
            "https://test-streams.mux.dev/x36xhzz/url_0/193039199_mp4_h264_aac_hd_7.m3u8"
        );
    }

    #[test]
    fn absolutize_handles_every_uri_shape() {
        let base = "https://cdn.example.com/live/2026/master.m3u8?token=abc";
        assert_eq!(
            absolutize(base, "https://other.com/x.m3u8"),
            "https://other.com/x.m3u8",
            "mutlak URL dokunulmadan gecmeli"
        );
        assert_eq!(
            absolutize(base, "v/720.m3u8"),
            "https://cdn.example.com/live/2026/v/720.m3u8",
            "goreli yol manifestin klasorune gore cozulmeli"
        );
        assert_eq!(
            absolutize(base, "/hls/720.m3u8"),
            "https://cdn.example.com/hls/720.m3u8",
            "kok-goreli yol host'a gore cozulmeli"
        );
        assert_eq!(
            absolutize(base, "//cdn2.example.com/x.m3u8"),
            "https://cdn2.example.com/x.m3u8",
            "sema-goreli yol tabanin semasini almali"
        );
    }

    #[test]
    fn prefers_average_bandwidth_when_present() {
        // BANDWIDTH tepe degeridir; AVERAGE-BANDWIDTH boyut tahmini icin
        // cok daha dogrudur.
        let text = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=9000000,AVERAGE-BANDWIDTH=4000000,RESOLUTION=1920x1080\nv.m3u8\n";
        let v = parse_master(text, "https://x.com/a.m3u8");
        assert_eq!(v[0].bandwidth_bps, Some(4_000_000));
    }

    #[test]
    fn sums_vod_duration_but_not_live_window() {
        let vod = "#EXTM3U\n#EXTINF:9.009,\na.ts\n#EXTINF:9.009,\nb.ts\n#EXTINF:2.0,\nc.ts\n#EXT-X-ENDLIST\n";
        let total = total_duration_seconds(vod).expect("VOD suresi hesaplanmali");
        assert!((total - 20.018).abs() < 0.001, "toplam: {total}");

        // Canli yayinda ENDLIST yok: elimizdeki pencere toplam sure degildir.
        let live = "#EXTM3U\n#EXTINF:9.009,\na.ts\n#EXTINF:9.009,\nb.ts\n";
        assert_eq!(
            total_duration_seconds(live),
            None,
            "canli yayinda sure bildirilmemeli"
        );
    }

    #[test]
    fn estimates_size_from_bandwidth_and_duration() {
        let v = &parse_master(MUX_MASTER, "https://x.com/a.m3u8")[2]; // 1080p
        // 6221600 bit/sn * 60 sn / 8 = ~46.7 MB
        let est = v.estimated_bytes(60.0).unwrap();
        assert!(
            (46_000_000..48_000_000).contains(&est),
            "tahmin makul araligin disinda: {est}"
        );
        assert_eq!(v.estimated_bytes(0.0), None, "sure yoksa tahmin de yok");
    }

    #[test]
    fn malformed_input_yields_no_variants_instead_of_panicking() {
        assert!(parse_master("", "https://x.com/a.m3u8").is_empty());
        assert!(parse_master("merhaba dunya", "https://x.com/a.m3u8").is_empty());
        // URI'si eksik bir STREAM-INF: satir atlanir, cokme olmaz.
        assert!(parse_master("#EXT-X-STREAM-INF:BANDWIDTH=1\n", "https://x.com/a.m3u8").is_empty());
    }
}

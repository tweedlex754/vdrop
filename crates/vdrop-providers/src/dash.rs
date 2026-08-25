//! DASH manifest (`.mpd`) ayristirici.
//!
//! HLS ile ayni amac - kaliteyi kullanici secsin - ama **mekanizma farkli.**
//!
//! FFmpeg bir HLS master playlist'indeki her varyanti ayri bir *program*
//! olarak acar (`-map 0:p:N`). DASH'te ise tum temsiller **tek programin
//! ayri akislaridir**; secim video akis indeksiyle yapilir:
//!
//! ```text
//! ffmpeg -i manifest.mpd -map 0:v:3 -map 0:a:0 -c copy cikti.mp4
//! ```
//!
//! Ses ayri bir `AdaptationSet`'tedir, o yuzden `-map 0:a:0` sart: yalnizca
//! video akisini secmek sessiz bir dosya uretir.
//!
//! ## Ayristirma yaklasimi
//!
//! MPD bir XML belgesidir ama bize lazim olan yalnizca `<Representation>`
//! ozniteliklerdir. `html::find_tags` bir etiket tarayicidir ve XML'de de
//! calisir; tam bir XML ayristiricisi (namespace cozumleme, sema dogrulama)
//! bu is icin fazlasiyla agir olurdu.
//!
//! Tek incelik: `find_tags` duz calisir, yani belgedeki TUM
//! `<Representation>`lari dondurur - ses ve altyazi olanlar dahil. Bu yuzden
//! belgeyi once `<AdaptationSet>` bloklarina boluyoruz.

use serde::{Deserialize, Serialize};

use crate::html::find_tags;

/// Video `AdaptationSet` icindeki tek bir kalite.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Representation {
    /// FFmpeg'e verilecek **video akis** indeksi. Belgedeki video
    /// temsillerinin sirasiyla ayni.
    pub video_stream_index: u32,
    pub id: Option<String>,
    pub bandwidth_bps: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codecs: Option<String>,
    pub frame_rate: Option<f32>,
}

impl Representation {
    pub fn resolution(&self) -> Option<String> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some(format!("{w}x{h}")),
            _ => None,
        }
    }

    pub fn bitrate_kbps(&self) -> Option<u32> {
        self.bandwidth_bps.map(|b| (b / 1000) as u32)
    }

    pub fn estimated_bytes(&self, duration_seconds: f64) -> Option<u64> {
        let bw = self.bandwidth_bps? as f64;
        if duration_seconds <= 0.0 {
            return None;
        }
        Some((bw * duration_seconds / 8.0) as u64)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Manifest {
    pub duration_seconds: Option<f64>,
    pub representations: Vec<Representation>,
    /// `type="dynamic"` ise canli yayindir: toplam sure yoktur.
    pub is_live: bool,
}

pub fn looks_like_mpd(text: &str) -> bool {
    let head: String = text.chars().take(2000).collect();
    head.contains("<MPD") || head.contains(":MPD")
}

/// MPD belgesini video kalitelerine ayristirir.
pub fn parse(text: &str) -> Manifest {
    let is_live = mpd_attr(text, "type")
        .map(|v| v.eq_ignore_ascii_case("dynamic"))
        .unwrap_or(false);

    // Canli yayinda `mediaPresentationDuration` ya yoktur ya da o ana kadarki
    // pencereyi anlatir; onu toplam sure diye sunmak yaniltici olurdu.
    let duration_seconds = if is_live {
        None
    } else {
        mpd_attr(text, "mediaPresentationDuration")
            .as_deref()
            .and_then(parse_iso8601_duration)
    };

    let mut representations = Vec::new();
    let mut index = 0u32;

    for block in adaptation_sets(text) {
        if !is_video_set(&block) {
            continue;
        }
        for tag in find_tags(&block, "Representation") {
            representations.push(Representation {
                video_stream_index: index,
                id: tag.attr("id").map(str::to_string),
                bandwidth_bps: tag.attr("bandwidth").and_then(|v| v.parse().ok()),
                width: tag.attr("width").and_then(|v| v.parse().ok()),
                height: tag.attr("height").and_then(|v| v.parse().ok()),
                codecs: tag.attr("codecs").map(str::to_string),
                frame_rate: tag.attr("frameRate").and_then(parse_frame_rate),
            });
            index += 1;
        }
    }

    Manifest {
        duration_seconds,
        representations,
        is_live,
    }
}

/// Kok `<MPD ...>` etiketinin bir ozniteligi.
fn mpd_attr(text: &str, key: &str) -> Option<String> {
    find_tags(text, "MPD")
        .first()
        .and_then(|t| t.attr(key).map(str::to_string))
}

/// Belgeyi `<AdaptationSet>` bloklarina boler.
///
/// `find_tags` duz calistigi icin bu bolme sart: aksi halde ses ve altyazi
/// temsilleri de video kalitesi sanilir ve kullaniciya "1920x1080" yerine
/// bos satirlar gosterilirdi.
fn adaptation_sets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut from = 0usize;

    while let Some(pos) = text[from..].find("<AdaptationSet") {
        let start = from + pos;
        let end = match text[start..].find("</AdaptationSet") {
            Some(e) => start + e,
            // Kapanis etiketi yoksa (bozuk manifest) belgenin sonuna kadar al.
            None => text.len(),
        };
        out.push(text[start..end].to_string());
        from = end.max(start + 1);
    }
    out
}

fn is_video_set(block: &str) -> bool {
    let head: String = block.chars().take(600).collect();
    let lower = head.to_lowercase();
    // `contentType="video"` ya da `mimeType="video/mp4"`.
    lower.contains("contenttype=\"video\"")
        || lower.contains("mimetype=\"video/")
        || lower.contains("contenttype='video'")
        || lower.contains("mimetype='video/")
}

/// DASH kare hizi ya duz sayidir ("30") ya da kesirdir ("30000/1001").
fn parse_frame_rate(value: &str) -> Option<f32> {
    if let Some((num, den)) = value.split_once('/') {
        let n: f32 = num.trim().parse().ok()?;
        let d: f32 = den.trim().parse().ok()?;
        if d == 0.0 {
            return None;
        }
        return Some(n / d);
    }
    value.trim().parse().ok()
}

/// ISO 8601 sure (`PT10M34.5S`) -> saniye.
pub fn parse_iso8601_duration(input: &str) -> Option<f64> {
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

    /// dash.akamaized.net'teki gercek test manifestinin kisaltilmis hali.
    const AKAMAI: &str = r#"<MPD mediaPresentationDuration="PT634.566S" type="static" xmlns="urn:mpeg:dash:schema:mpd:2011">
 <BaseURL>./</BaseURL>
 <Period>
  <AdaptationSet mimeType="video/mp4" contentType="video" par="16:9">
   <SegmentTemplate duration="120" timescale="30" media="$RepresentationID$/$Number$.m4v"/>
   <Representation id="bbb_1024x576" codecs="avc1.64001f" bandwidth="3134488" width="1024" height="576" frameRate="30"/>
   <Representation id="bbb_1920x1080" codecs="avc1.640028" bandwidth="9914554" width="1920" height="1080" frameRate="30"/>
   <Representation id="bbb_320x180" codecs="avc1.64000d" bandwidth="254320" width="320" height="180" frameRate="30"/>
  </AdaptationSet>
  <AdaptationSet mimeType="audio/mp4" contentType="audio">
   <Representation id="bbb_a64k" codecs="mp4a.40.5" bandwidth="67071" audioSamplingRate="48000"/>
  </AdaptationSet>
 </Period>
</MPD>"#;

    #[test]
    fn detects_an_mpd_document() {
        assert!(looks_like_mpd(AKAMAI));
        assert!(!looks_like_mpd("#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=1\n"));
        assert!(!looks_like_mpd("<html><body>merhaba</body></html>"));
    }

    #[test]
    fn parses_only_video_representations() {
        let m = parse(AKAMAI);
        // Ses temsili kalite listesine karismamali.
        assert_eq!(m.representations.len(), 3);
        assert!(m.representations.iter().all(|r| r.width.is_some()));
    }

    #[test]
    fn stream_indices_follow_document_order() {
        // Yanlis olursa kullanici 1080p secip 320x180 indirir.
        let m = parse(AKAMAI);
        assert_eq!(m.representations[0].video_stream_index, 0);
        assert_eq!(m.representations[0].resolution().as_deref(), Some("1024x576"));
        assert_eq!(m.representations[1].video_stream_index, 1);
        assert_eq!(m.representations[1].resolution().as_deref(), Some("1920x1080"));
        assert_eq!(m.representations[2].video_stream_index, 2);
        assert_eq!(m.representations[2].resolution().as_deref(), Some("320x180"));
    }

    #[test]
    fn reads_duration_and_bitrate() {
        let m = parse(AKAMAI);
        let duration = m.duration_seconds.expect("VOD suresi okunmali");
        assert!((duration - 634.566).abs() < 0.001, "sure: {duration}");
        assert_eq!(m.representations[1].bitrate_kbps(), Some(9914));
        assert!(!m.is_live);
    }

    #[test]
    fn live_manifests_report_no_duration() {
        // Canli yayinda "10 dakika" demek yaniltici olurdu.
        let live = AKAMAI.replace("type=\"static\"", "type=\"dynamic\"");
        let m = parse(&live);
        assert!(m.is_live);
        assert_eq!(m.duration_seconds, None);
        assert_eq!(m.representations.len(), 3, "kaliteler yine listelenmeli");
    }

    #[test]
    fn frame_rate_accepts_both_integer_and_fraction() {
        // NTSC manifestleri "30000/1001" yazar.
        assert_eq!(parse_frame_rate("30"), Some(30.0));
        let ntsc = parse_frame_rate("30000/1001").unwrap();
        assert!((ntsc - 29.97).abs() < 0.01, "ntsc: {ntsc}");
        assert_eq!(parse_frame_rate("30/0"), None);
        assert_eq!(parse_frame_rate("bozuk"), None);
    }

    #[test]
    fn size_estimate_from_bandwidth() {
        let m = parse(AKAMAI);
        // 9914554 bit/sn * 634.566 sn / 8 ~= 786 MB
        let est = m.representations[1].estimated_bytes(634.566).unwrap();
        assert!(
            (780_000_000..795_000_000).contains(&est),
            "tahmin araligin disinda: {est}"
        );
    }

    #[test]
    fn iso_duration_handles_fractional_seconds() {
        assert_eq!(parse_iso8601_duration("PT634.566S"), Some(634.566));
        assert_eq!(parse_iso8601_duration("PT1H2M30S"), Some(3750.0));
        assert_eq!(parse_iso8601_duration("bozuk"), None);
    }

    #[test]
    fn malformed_manifests_yield_nothing_instead_of_panicking() {
        for bad in [
            "",
            "<MPD>",
            "<MPD><Period><AdaptationSet",
            "<MPD><AdaptationSet contentType=\"video\"><Representation></MPD>",
        ] {
            let m = parse(bad);
            // Cokme yok; en fazla eksik veri.
            let _ = m.representations.len();
        }
    }

    #[test]
    fn representations_outside_a_video_set_are_ignored() {
        let subtitles_only = r#"<MPD type="static">
          <AdaptationSet contentType="text" mimeType="application/mp4">
            <Representation id="sub" bandwidth="1000"/>
          </AdaptationSet>
        </MPD>"#;
        assert!(parse(subtitles_only).representations.is_empty());
    }
}

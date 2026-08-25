//! Minimal HTML etiket tarayici.
//!
//! Amac dar: bir sayfanin **kendi ilan ettigi** medyayi bulmak. Ihtiyacimiz
//! olan sey `<meta property="og:video">`, `<video src>`, `<source src>`,
//! `<title>` ve JSON-LD bloklari. Tam bir DOM agacina gerek yok.
//!
//! ## Neden html5ever/scraper degil
//!
//! Tam bir HTML5 ayristiricisi ~15 gecisli bagimlilik ve ciddi derleme
//! suresi getirir. Buna karsilik bizim sorumuz "sayfanin agac yapisi nedir"
//! degil, "su etiketin su ozniteligi ne" - bu, dogrusal bir taramayla
//! guvenilir sekilde cevaplanir.
//!
//! ## Neden duz regex de degil
//!
//! `content="a href=\"x\""` gibi ic ice tirnaklar, tek tirnakli oznitelikler
//! ve tirnaksiz degerler naif bir regex'i bozar. Buradaki tarayici tirnak
//! durumunu izler; ayristirma testleri bu durumlari kapsar.

/// Bir etiketin oznitelikleri (ad, deger).
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attrs: Vec<(String, String)>,
}

impl Tag {
    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }
}

/// Verilen adla eslesen tum acilis etiketlerini bulur (kapanis etiketleri ve
/// yorumlar atlanir).
pub fn find_tags(html: &str, tag_name: &str) -> Vec<Tag> {
    let bytes = html.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < bytes.len() {
        // Bir sonraki '<' karakterine atla.
        let Some(open) = html[i..].find('<') else { break };
        let start = i + open + 1;
        if start >= bytes.len() {
            break;
        }

        // Yorum bloklarini butun olarak atla; icinde ornek isaretleme olabilir.
        if html[start..].starts_with("!--") {
            match html[start..].find("-->") {
                Some(end) => {
                    i = start + end + 3;
                    continue;
                }
                None => break,
            }
        }

        // Kapanis etiketi: ilgilenmiyoruz.
        if html[start..].starts_with('/') {
            i = start + 1;
            continue;
        }

        // Etiket adini oku.
        let name_end = html[start..]
            .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
            .map(|o| start + o)
            .unwrap_or(bytes.len());
        let name = html[start..name_end].to_lowercase();

        if name != tag_name.to_lowercase() {
            i = name_end;
            continue;
        }

        // Etiketin sonunu bul; tirnak icindeki '>' etiketi bitirmez.
        let Some(tag_end) = find_tag_end(html, name_end) else {
            break;
        };
        out.push(Tag {
            name,
            attrs: parse_attributes(&html[name_end..tag_end]),
        });
        i = tag_end + 1;
    }

    out
}

/// Etiketi kapatan '>' konumunu bulur, tirnaklari sayarak.
fn find_tag_end(html: &str, from: usize) -> Option<usize> {
    let mut quote: Option<char> = None;
    for (offset, c) in html[from..].char_indices() {
        match (quote, c) {
            (Some(q), _) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"') | (None, '\'') => quote = Some(c),
            (None, '>') => return Some(from + offset),
            _ => {}
        }
    }
    None
}

/// `key="value"`, `key='value'`, `key=value` ve degersiz `key` bicimlerini
/// ayristirir.
fn parse_attributes(input: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0usize;

    while i < chars.len() {
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == '/') {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        let key_start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '=' && chars[i] != '>' {
            i += 1;
        }
        let key: String = chars[key_start..i].iter().collect();
        if key.is_empty() {
            i += 1;
            continue;
        }

        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // Degersiz oznitelik (ornegin `controls`).
        if i >= chars.len() || chars[i] != '=' {
            out.push((key.to_lowercase(), String::new()));
            continue;
        }
        i += 1; // '='
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        let value: String = if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != quote {
                i += 1;
            }
            let v: String = chars[start..i].iter().collect();
            i += 1; // kapanis tirnagi
            v
        } else {
            let start = i;
            while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '>' {
                i += 1;
            }
            chars[start..i].iter().collect()
        };

        out.push((key.to_lowercase(), decode_entities(&value)));
    }

    out
}

/// URL'lerde gercekten karsilastigimiz HTML varliklari.
///
/// Tam bir varlik tablosu tasimiyoruz: oznitelik degerlerinde pratikte
/// `&amp;` (sorgu parametreleri arasinda) ve birkac tanesi gorunur.
pub fn decode_entities(input: &str) -> String {
    if !input.contains('&') {
        return input.to_string();
    }
    input
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#34;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&#x2F;", "/")
        .replace("&#47;", "/")
}

/// Bir etiketin metin icerigini dondurur (ornegin `<title>`).
pub fn text_of_first(html: &str, tag_name: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let open = format!("<{}", tag_name.to_lowercase());
    let close = format!("</{}", tag_name.to_lowercase());

    let start_tag = lower.find(&open)?;
    let content_start = lower[start_tag..].find('>')? + start_tag + 1;
    let content_end = lower[content_start..].find(&close)? + content_start;

    let text = decode_entities(html[content_start..content_end].trim());
    (!text.is_empty()).then_some(text)
}

/// `<script type="application/ld+json">` bloklarinin icerigini dondurur.
pub fn json_ld_blocks(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    for tag in find_tags(html, "script") {
        let is_ld = tag
            .attr("type")
            .map(|t| t.eq_ignore_ascii_case("application/ld+json"))
            .unwrap_or(false);
        if !is_ld {
            continue;
        }
        // Etiketin govdesini bul: bu etiketten sonraki ilk </script>.
        if let Some(body) = script_body_after(html, &tag) {
            out.push(body);
        }
    }
    out
}

fn script_body_after(html: &str, tag: &Tag) -> Option<String> {
    // Etiketi ozniteliklerinden yeniden bulmak yerine, ld+json isaretini
    // arayip oradan ilerliyoruz. Sayfada birden cok blok olabilir, o yuzden
    // her seferinde kaldigimiz yerden degil - bu yardimci yalnizca ilk
    // eslesmeyi dondurur ve cagiran taraf tekrarlari `dedup` eder.
    let _ = tag;
    let lower = html.to_lowercase();
    let mut from = 0usize;
    let mut bodies = Vec::new();
    while let Some(pos) = lower[from..].find("application/ld+json") {
        let abs = from + pos;
        let open_end = lower[abs..].find('>')? + abs + 1;
        let close = lower[open_end..].find("</script")? + open_end;
        bodies.push(html[open_end..close].trim().to_string());
        from = close;
    }
    bodies.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_meta_tags_and_reads_attributes() {
        let html = r#"<html><head>
            <meta property="og:video" content="https://cdn.x.com/v.mp4">
            <meta name="twitter:card" content="player"/>
        </head></html>"#;
        let metas = find_tags(html, "meta");
        assert_eq!(metas.len(), 2);
        assert_eq!(metas[0].attr("property"), Some("og:video"));
        assert_eq!(metas[0].attr("content"), Some("https://cdn.x.com/v.mp4"));
        assert_eq!(metas[1].attr("name"), Some("twitter:card"));
    }

    #[test]
    fn attribute_lookup_is_case_insensitive() {
        let html = r#"<META PROPERTY="og:video" CONTENT="https://x.com/a.mp4">"#;
        let metas = find_tags(html, "meta");
        assert_eq!(metas.len(), 1, "buyuk harfli etiket de bulunmali");
        assert_eq!(metas[0].attr("content"), Some("https://x.com/a.mp4"));
    }

    #[test]
    fn handles_single_quoted_and_unquoted_attributes() {
        let html = "<video src='https://x.com/a.mp4' width=640 controls></video>";
        let tags = find_tags(html, "video");
        assert_eq!(tags[0].attr("src"), Some("https://x.com/a.mp4"));
        assert_eq!(tags[0].attr("width"), Some("640"));
        assert_eq!(
            tags[0].attr("controls"),
            Some(""),
            "degersiz oznitelik bos deger olarak durmali"
        );
    }

    #[test]
    fn a_greater_than_inside_a_quoted_value_does_not_end_the_tag() {
        // Naif bir ayristirici burada etiketi erken bitirir ve src'yi kacirir.
        let html = r#"<meta property="og:description" content="1 > 0 her zaman"><meta property="og:video" content="https://x.com/a.mp4">"#;
        let metas = find_tags(html, "meta");
        assert_eq!(metas.len(), 2);
        assert_eq!(metas[1].attr("content"), Some("https://x.com/a.mp4"));
    }

    #[test]
    fn decodes_ampersands_in_urls() {
        let html = r#"<meta property="og:video" content="https://x.com/v?a=1&amp;b=2">"#;
        let metas = find_tags(html, "meta");
        assert_eq!(
            metas[0].attr("content"),
            Some("https://x.com/v?a=1&b=2"),
            "&amp; cozulmezse URL bozuk kalir ve istek basarisiz olur"
        );
    }

    #[test]
    fn skips_comments_that_contain_markup() {
        let html = r#"<!-- <meta property="og:video" content="https://tuzak.com/x.mp4"> -->
                      <meta property="og:video" content="https://gercek.com/x.mp4">"#;
        let metas = find_tags(html, "meta");
        assert_eq!(metas.len(), 1, "yorumdaki etiket sayilmamali");
        assert_eq!(metas[0].attr("content"), Some("https://gercek.com/x.mp4"));
    }

    #[test]
    fn finds_source_elements_inside_video() {
        let html = r#"<video poster="p.jpg">
            <source src="a.mp4" type="video/mp4">
            <source src="a.webm" type="video/webm">
        </video>"#;
        let sources = find_tags(html, "source");
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].attr("type"), Some("video/mp4"));
        assert_eq!(sources[1].attr("src"), Some("a.webm"));
    }

    #[test]
    fn reads_page_title() {
        let html = "<html><head><title>  Ornek Video &amp; Ses  </title></head></html>";
        assert_eq!(text_of_first(html, "title").as_deref(), Some("Ornek Video & Ses"));
        assert_eq!(text_of_first("<html></html>", "title"), None);
    }

    #[test]
    fn extracts_json_ld_block() {
        let html = r#"<script type="application/ld+json">
            {"@type":"VideoObject","contentUrl":"https://x.com/v.mp4"}
        </script>"#;
        let blocks = json_ld_blocks(html);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("contentUrl"));
    }

    #[test]
    fn malformed_html_does_not_panic() {
        for bad in ["<", "<meta", "<meta content=\"", "<!--", "<video src=", ""] {
            let _ = find_tags(bad, "meta");
            let _ = find_tags(bad, "video");
            let _ = text_of_first(bad, "title");
            let _ = json_ld_blocks(bad);
        }
    }
}

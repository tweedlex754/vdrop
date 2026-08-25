//! Genel HTTP saglayicisini gercek adreslere karsi dogrular.
//!
//! Birim testler HTML ayristirmayi sabit ornekle sinar; burada asil soru
//! su: **gercek dunyanin isaretlemesi** karsisinda ne oluyor? Wikimedia
//! Commons dosya sayfasi bunun icin iyi bir hedef, cunku tam da zor
//! kisimlari icerir:
//!
//!   - `<video poster>` + birden cok `<source src>`
//!   - URL'lerde `&amp;` (varlik cozulmezse adres bozulur)
//!   - `type="video/webm; codecs=&quot;vp9, opus&quot;"` (tirnak ici tirnak)
//!
//! Ayrica sabit bir kaynak: Wikimedia sayfalari yillardir ayni yapida.
//!
//!     cargo test -p vdrop-providers --test live_web -- --ignored --nocapture

use vdrop_providers::ProviderRegistry;

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn direct_media_url_reports_the_real_size_from_the_server() {
    // Uzantidan tahmin etmek yerine sunucuya soruyoruz; kazanc, kullanicinin
    // indirmeden once gercek boyutu gormesi.
    let url = "https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/360/Big_Buck_Bunny_360_10s_1MB.mp4";
    let reg = ProviderRegistry::with_defaults();
    let info = reg.resolve(url).await.expect("dogrudan medya cozumlenmeli");

    assert_eq!(info.streams.len(), 1);
    let s = &info.streams[0];
    assert_eq!(s.container.as_deref(), Some("mp4"));
    assert_eq!(
        s.estimated_size_bytes,
        Some(991_017),
        "boyut sunucunun Content-Length'inden gelmeli, tahminden degil"
    );
}

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn extracts_media_from_a_real_web_page() {
    let page = "https://commons.wikimedia.org/wiki/File:Big_Buck_Bunny_medium.ogv";
    let reg = ProviderRegistry::with_defaults();
    let info = reg.resolve(page).await.expect("sayfa cozumlenmeli");

    assert!(
        !info.streams.is_empty(),
        "sayfadaki <source> elemanlari aday olmaliydi"
    );

    // Baslik og:title'dan gelmeli, ham <title>'dan degil.
    let title = info.title.to_lowercase();
    assert!(
        title.contains("big buck bunny"),
        "baslik sayfadan alinmaliydi, bulunan: {}",
        info.title
    );

    // Kapak <video poster> ya da og:image'dan.
    assert!(
        info.thumbnail_url
            .as_deref()
            .map(|t| t.starts_with("https://"))
            .unwrap_or(false),
        "kapak gorseli bulunmaliydi"
    );

    // Her aday gercek bir medya adresi olmali - sayfa/HTML degil.
    for s in &info.streams {
        assert!(
            s.url.starts_with("https://"),
            "goreli adres cozulmemis: {}",
            s.url
        );
        assert!(
            !s.url.contains("&amp;"),
            "HTML varligi cozulmemis; bu adres istek atinca 404 verir: {}",
            s.url
        );
    }

    // Wikimedia birden cok transcode sunar; en az bir webm ya da ogv olmali.
    let has_known = info.streams.iter().any(|s| {
        matches!(
            s.container.as_deref(),
            Some("webm") | Some("ogv") | Some("mov") | Some("mp4")
        )
    });
    assert!(
        has_known,
        "tanidik bir kapsayici bekleniyordu, bulunan: {:?}",
        info.streams
            .iter()
            .map(|s| s.container.clone())
            .collect::<Vec<_>>()
    );

    // Ilk aday icin gercek boyut sorgulanir.
    assert!(
        info.streams[0].estimated_size_bytes.is_some(),
        "ilk aday icin boyut ogrenilmeliydi"
    );
}

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn a_page_without_media_fails_with_a_useful_message() {
    let reg = ProviderRegistry::with_defaults();
    let err = reg
        .resolve("https://example.com/")
        .await
        .expect_err("medyasiz sayfa hata dondurmeli");

    let message = err.to_string().to_lowercase();
    assert!(
        message.contains("bulunamad") || message.contains("medya"),
        "hata mesaji kullaniciya ne yapacagini soylemeli, bulunan: {err}"
    );
}

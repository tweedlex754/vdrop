//! HLS saglayicisini gercek bir master playlist'e karsi dogrular.
//!
//! Birim testler ayristiriciyi sabit metinle sinar; bu test asil zinciri
//! kanitlar: manifesti indir -> varyantlari cikar -> sure icin ikinci bir
//! istek at -> boyut tahmini uret.
//!
//!     cargo test -p vdrop-providers --test live_hls -- --ignored --nocapture

use vdrop_providers::{ProviderRegistry, StreamKind};

const MASTER: &str = "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8";

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn resolves_a_real_master_playlist_into_quality_options() {
    let reg = ProviderRegistry::with_defaults();
    let info = reg.resolve(MASTER).await.expect("master cozumlenebilmeli");

    assert!(
        info.streams.len() >= 4,
        "master playlist birden cok kalite icermeli, bulunan: {}",
        info.streams.len()
    );

    // En yuksek kalite basta olmali: varsayilan secim ilk satirdir.
    let bitrates: Vec<u32> = info
        .streams
        .iter()
        .map(|s| s.bitrate_kbps.unwrap_or(0))
        .collect();
    let mut sorted = bitrates.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(bitrates, sorted, "kaliteler azalan sirada olmali");

    let best = &info.streams[0];
    assert_eq!(best.resolution.as_deref(), Some("1920x1080"));
    assert!(matches!(best.kind, StreamKind::Muxed));

    // Her secenek master URL'i tasimali (varyantin kendi playlist'ini degil),
    // ve kendi program indeksini bilmeli.
    for stream in &info.streams {
        assert_eq!(
            stream.url, MASTER,
            "secenekler master URL'i tasimali; varyant playlist'i sessiz \
             video riski yaratir"
        );
        assert!(
            stream.variant_index.is_some(),
            "her varyant program indeksini bilmeli"
        );
    }

    // Indeksler benzersiz olmali; ayni indeks iki kez cikarsa kullanici
    // sectiginden baskasini indirir.
    let mut indices: Vec<u32> = info
        .streams
        .iter()
        .filter_map(|s| s.variant_index)
        .collect();
    let before = indices.len();
    indices.sort_unstable();
    indices.dedup();
    assert_eq!(indices.len(), before, "program indeksleri benzersiz olmali");

    // Sure ikinci bir istekle ogrenilir; oradan boyut tahmini gelir.
    let duration = info.duration_seconds.expect("VOD suresi okunabilmeli");
    assert!(duration > 500.0, "beklenen ~10 dakika, okunan: {duration}");

    let size = best
        .estimated_size_bytes
        .expect("sure bilindiginde boyut tahmini uretilmeli");
    assert!(
        size > 100_000_000,
        "1080p / ~10 dk icin tahmin cok kucuk: {size}"
    );
}

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn falls_back_gracefully_for_a_media_playlist() {
    // Dogrudan bir varyant playlist'i: master degil, tek kalite.
    let url = "https://test-streams.mux.dev/x36xhzz/url_2/193039199_mp4_h264_aac_ld_7.m3u8";
    let reg = ProviderRegistry::with_defaults();
    let info = reg.resolve(url).await.expect("medya playlist cozumlenebilmeli");

    assert_eq!(info.streams.len(), 1, "tek rendition, tek secenek");
    assert!(
        info.streams[0].variant_index.is_none(),
        "master olmadan program indeksi anlamsiz"
    );
    assert!(
        info.duration_seconds.unwrap_or(0.0) > 500.0,
        "VOD suresi EXTINF toplamindan gelmeli"
    );
}

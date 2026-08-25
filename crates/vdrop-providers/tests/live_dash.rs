//! DASH saglayicisini gercek bir manifeste karsi dogrular.
//!
//!     cargo test -p vdrop-providers --test live_dash -- --ignored --nocapture

use vdrop_providers::ProviderRegistry;

const MPD: &str = "https://dash.akamaized.net/akamai/bbb_30fps/bbb_30fps.mpd";

#[tokio::test]
#[ignore = "aga erisim gerektirir"]
async fn resolves_a_real_mpd_into_quality_options() {
    let reg = ProviderRegistry::with_defaults();
    let info = reg.resolve(MPD).await.expect("manifest cozumlenmeli");

    assert!(
        info.streams.len() >= 5,
        "bu manifest cok sayida temsil icerir, bulunan: {}",
        info.streams.len()
    );

    // Ses temsilleri kalite listesine karismamali: hepsinin cozunurlugu olmali.
    for s in &info.streams {
        assert!(
            s.resolution.is_some(),
            "cozunurluksuz secenek var; ses AdaptationSet'i sizmis olabilir: {s:?}"
        );
        assert_eq!(s.url, MPD, "secenekler manifest adresini tasimali");
        assert!(s.variant_index.is_some(), "her temsil akis indeksini bilmeli");
    }

    // En yuksek kalite basta.
    let bitrates: Vec<u32> = info
        .streams
        .iter()
        .map(|s| s.bitrate_kbps.unwrap_or(0))
        .collect();
    let mut sorted = bitrates.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(bitrates, sorted, "kaliteler azalan sirada olmali");

    // Indeksler benzersiz: ayni indeks iki kez cikarsa kullanici sectiginden
    // baskasini indirir.
    let mut indices: Vec<u32> = info.streams.iter().filter_map(|s| s.variant_index).collect();
    let before = indices.len();
    indices.sort_unstable();
    indices.dedup();
    assert_eq!(indices.len(), before, "akis indeksleri benzersiz olmali");

    // Sure manifestten okunur; oradan boyut tahmini cikar.
    let duration = info.duration_seconds.expect("VOD suresi okunmali");
    assert!(duration > 600.0, "beklenen ~10 dakika, okunan: {duration}");
    assert!(
        info.streams[0].estimated_size_bytes.is_some(),
        "sure bilindiginde boyut tahmini uretilmeli"
    );

    // En iyi temsil 4K olmali (manifestte 3840x2160 var).
    assert_eq!(info.streams[0].resolution.as_deref(), Some("3840x2160"));
}

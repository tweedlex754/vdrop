//! Araci saglayicinin **HTTP yolunu** sahte bir sunucuyla kosturur.
//!
//! Neden gerekli: `kick-video.download` bu saglayici yazilirken hicbir Kick
//! VOD'unu cozemiyordu, yani gercek basari yolu hic denenemedi. Birim
//! testleri yalnizca saf fonksiyonlari (adres ayiklama, JSON gezme)
//! olcuyordu; adres kurma, kanal kimligi cozme, durum kodu isleme ve
//! yanittan `MediaInfo` uretme adimlari kosulmamis kaliyordu.
//!
//! Buradaki sunucu ikisini birden sahteliyor: Kick'in kanal API'sini ve
//! aracinin kendisini. Boylece servis duzeldiginde geriye yalnizca "gercek
//! sema beklendigi gibi mi" sorusu kaliyor.

use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use vdrop_providers::{KickDownloadProvider, Provider, ProviderError, StreamKind};

/// Gorulen istek yollari; testler bunlara bakarak cagrinin dogru
/// kuruldugunu dogrular.
type Seen = Arc<Mutex<Vec<String>>>;

async fn spawn_server(vod_status: u16, vod_body: &'static str) -> (String, Seen) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let seen: Seen = Arc::new(Mutex::new(Vec::new()));
    let recorder = seen.clone();

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let recorder = recorder.clone();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n > 0 => n,
                    _ => return,
                };
                let request = String::from_utf8_lossy(&buf[..n]).to_string();
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or_default()
                    .to_string();
                recorder.lock().unwrap().push(path.clone());

                let (status, body) = if path.contains("/api/v2/channels/") {
                    (200u16, r#"{"id":668,"slug":"xqc"}"#.to_string())
                } else {
                    (vod_status, vod_body.to_string())
                };

                let head = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(head.as_bytes()).await;
                let _ = socket.write_all(body.as_bytes()).await;
                let _ = socket.shutdown().await;
            });
        }
    });

    (format!("http://{addr}"), seen)
}

const VOD_URL: &str = "https://kick.com/xqc/videos/b6fd5deb-9ac1-4091-87fb-758fdacfa003";

#[tokio::test]
async fn builds_the_request_the_service_expects_and_reads_the_answer() {
    // Sema bilinmedigi icin bilerek "tuhaf" bir govde: adres derine gomulu
    // ve alan adlari tahmin edilemez. Gezici ayristirici yine de bulmali.
    let (base, seen) = spawn_server(
        200,
        r#"{"output":{"session_title":"Yayin kaydi","assets":[{"blob":"https://cdn.test/vod/1080.mp4"}]}}"#,
    )
    .await;

    let provider = KickDownloadProvider::with_bases(
        vdrop_providers::default_client(),
        base.clone(),
        base.clone(),
    );

    let info = provider.resolve(VOD_URL).await.expect("cozumleme basarili olmali");

    assert_eq!(info.title, "Yayin kaydi", "baslik yanittan alinmali");
    assert_eq!(info.uploader.as_deref(), Some("xqc"));
    assert_eq!(info.streams.len(), 1);
    assert_eq!(info.streams[0].url, "https://cdn.test/vod/1080.mp4");
    assert!(matches!(info.streams[0].kind, StreamKind::Muxed));

    let paths = seen.lock().unwrap().clone();
    assert!(
        paths.iter().any(|p| p.contains("/api/v2/channels/xqc/info")),
        "kanal kimligi Kick'in API'sinden cozulmeli: {paths:?}"
    );
    let call = paths
        .iter()
        .find(|p| p.contains("/api/get-kick-video2"))
        .expect("araci cagrilmali");
    assert!(
        call.contains("channelId=668"),
        "cozulen kanal kimligi cagriya gecmeli: {call}"
    );
    assert!(
        call.contains("url=https%3A%2F%2Fkick.com%2Fxqc%2Fvideos%2F"),
        "adres yuzde-kodlanmis gitmeli: {call}"
    );
}

#[tokio::test]
async fn a_failing_service_is_a_capability_gap_not_a_crash() {
    // Servisin GERCEKTEN dondurdugu govde ve durum kodu (olculdu).
    let (base, _seen) = spawn_server(
        404,
        r#"{"error":"Failed to resolve Kick video","output":null}"#,
    )
    .await;

    let provider = KickDownloadProvider::with_bases(
        vdrop_providers::default_client(),
        base.clone(),
        base,
    );

    // `Unsupported` olmali: zincirin "bu saglayicinin elinden gelmiyor"
    // demesinin yolu bu. `Network` deseydik hata kullaniciya yayilir ve
    // aslinda yt-dlp ile cozulen bir adres icin bile ekrana hata basardi.
    match provider.resolve(VOD_URL).await {
        Err(ProviderError::Unsupported) => {}
        other => panic!("Unsupported bekleniyordu, gelen: {other:?}"),
    }
}

#[tokio::test]
async fn a_success_with_no_media_in_it_is_also_a_capability_gap() {
    let (base, _seen) = spawn_server(200, r#"{"output":{"note":"nothing here"}}"#).await;
    let provider = KickDownloadProvider::with_bases(
        vdrop_providers::default_client(),
        base.clone(),
        base,
    );
    match provider.resolve(VOD_URL).await {
        Err(ProviderError::Unsupported) => {}
        other => panic!("Unsupported bekleniyordu, gelen: {other:?}"),
    }
}

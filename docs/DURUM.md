# VDrop — Durum ve Devam Notları

> Bu dosya, çalışmayı bırakıp sonra kaldığı yerden devam edebilmek için
> yazıldı. Ne çalışıyor, **nasıl doğrulandı**, neyin doğrulanmadığı ve
> sıradaki iş — hepsi burada.
>
> Son güncelleme: 24 Ağustos 2026 · `7886b1b` + commit edilmemiş çalışma
> (doğrulama turu: dört ⚠️ maddesi kapatıldı, üç hata düzeltildi)

---

## 1. Tek cümlede

VDrop kurulabilir, çalışan bir Tauri 2 masaüstü uygulaması. Doğrudan dosya
bağlantılarını, HLS/DASH akışlarını (kalite seçimiyle) ve normal web
sayfalarından çıkarılan medyayı indiriyor; yt-dlp kuruluysa kapsam yüzlerce
siteye açılıyor.

Başlangıçtaki arşiv **derlenmiyordu**; şu an `.exe`/`.msi` üretiyor.

---

## 2. Sayılarla

| | |
|---|---|
| Birim testi | **217** (122 Rust workspace + 9 kabuk + 86 frontend) |
| Canlı ağ testi | **12** (`#[ignore]`, gerçek sunuculara bağlanır) |
| Clippy | Her iki ağaçta **0 uyarı** |
| Kaynak | ~13.700 satır (Rust + TS + CSS) |
| Commit | 12 |

Crate bazında: `vdrop-download` 27, `vdrop-providers` 67, `vdrop-storage` 11,
`vdrop-ytdlp` 12, `vdrop-media` 4, `src-tauri` 9, `frontend` 86.

---

## 3. Çalışan özellikler

### Çözümleme (URL → kalite listesi)

| Girdi | Sağlayıcı | Sonuç |
|---|---|---|
| `.m3u8` master playlist | `hls` | Varyant başına çözünürlük, bit hızı, kodek, boyut tahmini |
| `.mpd` | `dash` | Temsil başına aynı bilgiler |
| Web sayfası | `web` | `og:video`, JSON-LD `VideoObject`, `<video>`, `<source>` |
| Doğrudan dosya | `web` | `Content-Type` + gerçek `Content-Length` |
| Herhangi bir sayfa | `yt-dlp` | Site-özel çıkarım (yt-dlp kuruluysa) |

Zincir sırası: **yt-dlp → hls → dash → web**. Bir sağlayıcı `Unsupported`
dediğinde sıradaki denenir. **Hiçbir hata zinciri kesmez** — `Network` dahil;
hepsi başarısız olursa en somut hata raporlanır.

### İndirme

- Range istekleriyle devam ettirilebilir HTTP
- HLS/DASH: FFmpeg ile yeniden kodlamadan birleştirme, kalite seçimi
- **Altyazı indirme** (yalnızca HLS): manifestteki izler dil adıyla
  listelenir, seçilen iz SRT olarak iner. DASH için neden olmadığı §8'de
- yt-dlp formatları: yt-dlp'ye devredilir (`--continue` ile devam)
- **Duraklat/devam üç türde de aynı**: duraklatma süreci sonlandırır, `.part`
  diskte kalır, devam yeniden başlatır
- Uygulama kapanıp açılsa bile devam eder
- Eş zamanlılık limiti (1–16), çalışma sırasında değiştirilebilir
- **Bant genişliği limiti** (KB/sn, 0 = sınırsız), çalışma sırasında
  değiştirilebilir — kapsamı için §6'daki karara bakın

### Arayüz

Lumina Desktop tasarım sistemi. Kenar çubuğu (vibrancy) + içerik + alt durum
çubuğu. Beş ekran: Ana sayfa, Kuyruk, Kütüphane, Geçmiş, Ayarlar.
Açık/koyu/sistem teması, **20 dil** (Arapça dahil — sağdan sola düzen
çalışıyor, gözle doğrulandı).

Kütüphane ve Geçmiş'te **serbest metin araması** (başlık + adres/yol). Ortak
karar noktası `frontend/src/lib/search.ts`; iki ekran aynı sorguya aynı
cevabı verir.

Arama **Türkçe'ye göre katlanır**: "igdir" yazan kullanıcı "Iğdır" kaydını
bulur. Bunun bir bedeli var ve bilinçli seçildi — bkz. §6.

### Diğer

- Pano izleme (yakalanan bağlantıya **istek atmaz**, sadece teklif eder)
- Sistem bildirimleri (pencere küçükken de)
- Dosya adı sanitizasyonu (path traversal, aygıt adları, bidi, uzunluk)

---

## 4. Neyin nasıl doğrulandığı

Bu ayrım önemli: bazıları gerçek uygulamada ölçüldü, bazıları sadece test.

### ✅ Gerçek release uygulamasında ölçüldü

WebView2 uzaktan hata ayıklama (CDP) üzerinden:

- **CSP gerçekten uygulanıyor** ve IPC yine çalışıyor (dışa `fetch` engellendi)
- Dosya adı sanitizasyonu diskte: `CDP testi: bayrak?.mp4` → `CDP testi_ bayrak_.mp4`
- Gerçek indirme: 991017/991017 bayt, `ffprobe` ile geçerli 10 sn MP4
- HLS: arayüzden 184p seçildi → inen dosya **320x184**, sesli, tam 10:34
- DASH: 10 kalite, 4K'ya kadar, süre ve boyut tahminleriyle
- Sayfa çıkarımı: Wikimedia Commons → 4 ayırt edilebilir seçenek
- Pano: medya linki yakalandı → şerit → "Çözümle" → Ana sayfaya devir
- Bildirim `show()` **Ok** döndü
- Yeni arayüz: Inter yüklü, `blur(20px) saturate(1.2)`, durum çubuğu, token'lar

### ✅ Canlı ağ testleriyle (tekrarlanabilir)

```bash
npm run test:live
cargo test -p vdrop-providers --test live_web -- --ignored
cargo test -p vdrop-providers --test live_dash -- --ignored
cargo test -p vdrop-media --test live_dash -- --ignored
```

En kritik ikisi: **seçilen kalitenin gerçekten indiği**, HLS ve DASH için
ayrı ayrı, inen dosyanın çözünürlüğü `ffprobe` ile ölçülerek. Ayrıştırıcı
doğru olsa bile bu yanlış olsaydı kullanıcı 1080p seçip 240p indirirdi.

### ✅ Bu turda kapatılan maddeler

Hepsi **gerçek release `.exe`** üzerinde, WebView2 uzaktan hata ayıklama (CDP)
ile arayüz sürülerek; sonuç hem veritabanından hem diskten doğrulandı.

1. **Yeni arayüzde uçtan uca gerçek indirme — üç yolun da.**
   - HTTP: 991017/991017 bayt, `ffprobe` ile 640x360 h264, tam 10 sn
   - HLS: 1080p seçildi → 488204182 bayt, `variant_index=4`, `kind=stream`
   - yt-dlp: 300p OGV seçildi → 46935223 bayt (yt-dlp'nin bildirdiği boyutun
     aynısı), `provider_id=yt-dlp`, `format_id=0`
   Üç durumda da zincirin tamamı çalıştı: İndir → kuyruk kaydı → tamamlandı →
   kütüphane. Veritabanı doldu.
2. **Görsel doğrulama.** Arayüz gözle görüldü (ekran görüntüleri alındı).
   Bu bakış iki kusur ortaya çıkardı — ikisi de düzeltildi (bkz. §7).
3. **yt-dlp yolu.** yt-dlp 2026.08.19 kuruldu. Gerçek bir yt-dlp indirmesi
   yapıldı; Kick VOD'u da **aracısız** çözümlendi (5 kalite, 1080p60'a kadar).
4. **Kurulum paketinin kurulması.** NSIS kurucusu sessiz modda çalıştı,
   `%LOCALAPPDATA%\VDrop` altına kullanıcı bazında kuruldu, SmartScreen
   engeli çıkmadı, Başlat menüsü kısayolu oluştu, kurulan uygulama açıldı.
5. **Duraklat/devam arayüzden.** Kuyruk kartındaki düğmelerden sürüldü:
   14469568 baytta duraklatıldı, `.part` diskte tam o boyutta, 3 saniye
   sonra bayt sayısı **değişmedi** (duraklatma transferi gerçekten
   sonlandırıyor — §6'daki karar), devam edilince 30704510/30704510
   tamamlandı ve inen dosya referansla **bayt bayt özdeş** çıktı.

### ⚠️ Hâlâ doğrulanmadı


- **Konsol penceresi düzeltmesi dolaylı ölçüldü.** Çözümleme sırasında
  `conhost` sayısı 43→44 (o +1 muhtemelen ölçüm döngüsünün kendisi). Asıl
  garanti kod tarafında (`CREATE_NO_WINDOW`), gözle bakılmadı.
- **Kod imzalama** yok; indirilen kurucuda SmartScreen uyarısı beklenir.

## 5. Mimari haritası

```
frontend/                React + TypeScript (arayüz)
    │  Tauri IPC — 19 komut, tek olay kanalı (`download:event`)
    ▼
src-tauri/               Masaüstü kabuğu (ince tel bağlantısı)
    ├── main.rs          IPC komutları, transfer yönetimi
    ├── concurrency.rs   Eş zamanlılık kapısı (semafor)
    └── clipboard.rs     Pano izleyici
    │
    ├── vdrop-download   Resumable HTTP + dosya adı güvenliği
    ├── vdrop-media      HLS/DASH, FFmpeg boru hattı
    ├── vdrop-providers  URL → MediaInfo (hls, dash, web + HTML/JSON-LD)
    ├── vdrop-ytdlp      Opsiyonel: yt-dlp çıkarım + indirme
    └── vdrop-storage    SQLite, forward-only migration
```

**Kural:** iş mantığı `src-tauri` içinde değil, crate'lerde. Çekirdek mantık
Tauri olmadan saniyeler içinde test edilebiliyor.

### Veritabanı migration'ları

| Sürüm | İçerik |
|---|---|
| `0001_init` | Tüm tablolar |
| `0002_stream_kind_and_error` | `kind`, `error_message`, `thumbnail_url`, indeksler |
| `0003_variant_index` | HLS/DASH kalite seçimi (anlamı `provider_id`'ye bağlı) |
| `0004_format_id` | yt-dlp format kimliği (metin) |

Forward-only: yeni sürüm **eklenir**, mevcutlar asla değiştirilmez.

---

## 6. Karar günlüğü

Bunlar tartışıldı ve bilinçli olarak böyle bırakıldı.

**Duraklatma bağlantıyı kapatır.** Yerinde bekleyen bir indirme eş zamanlılık
yuvasını işgal ederdi: limit 3 iken 3 indirmeyi duraklatan kullanıcı
sıradakileri başlatamazdı — özelliğin var olma sebebi çalışmazdı.

**HLS'te varyantın kendi URL'i indirilmez.** Bazı yayınlarda ses ayrı
rendition'dadır; varyant playlist'i tek başına sessiz video verir. Master URL
+ program indeksi (`-map 0:p:N`) kullanılır.

**DASH farklı mekanizma.** Orada temsiller tek programın akışlarıdır:
`-map 0:v:N -map 0:a:0`. Aynı tamsayı iki formatta iki farklı şey demek — bu
yüzden `StreamSelector` enum'uyla taşınıyor.

**yt-dlp indirmeyi de üstlenir.** En yüksek kalite genelde ayrı video+ses ve
birleştirme ister; ayrıca format URL'leri kısa ömürlüdür, bir saat sonra
403 verir.

**Pano izleme hiçbir şey indirmez.** Pano iç ağ adreslerini, imzalı S3
bağlantılarını, parola sıfırlama linklerini taşır.

**Aracı sitenin yanıt şeması artık biliniyor.** İlk yazıldığında servis
hiçbir VOD'u çözemiyordu; sonradan çalışan bir örnek yakalandı:
`{ video: { title, channel, thumbnail, ... }, source: "…/master.m3u8" }`.
Gezici ayrıştırıcı bunu olduğu gibi karşılıyor ve `source` bir master
playlist olduğu için kalite listesi HLS sağlayıcısına devrediliyor —
kullanıcı tek seçenek yerine gerçek kaliteleri görüyor. Gerçek gövde teste
sabitlendi.

**Aracı site zincirin en sonunda.** Önceki karar "aracı siteler hiç
desteklenmiyor"du; kullanıcı isteğiyle değişti. Ama konumu bir sıralama
tercihi değil, gizlilik kuralı: `KickDownloadProvider` yalnızca
`kick.com/{kanal}/videos/{uuid}` biçimine eşleşir **ve** zincirin en sonunda
durur, yani ondan önceki her sağlayıcı (yt-dlp, hls, dash, web) başarısız
olmadıkça çözümlenen adres dışarı çıkmaz. Kick'i yt-dlp zaten aracısız
çözdüğü için pratikte hiç çağrılmaz. Kural teste yazıldı
(`registry_order_puts_hls_before_the_general_provider`): sağlayıcı yukarı
taşınırsa test kırılır ve nedenini söyler.

**Kimlik gizlenmiyor.** `VDrop/0.1.0` diye tanıtılır. Wikimedia gibi siteler
açıklayıcı User-Agent ister; dürüst olmak aynı zamanda çalışan yol.

**Bant genişliği limiti paylaşılan tek kovadır — ama her motor için değil.**
Kullanıcı "toplam şu kadarı geçmesin" der, "her indirme ayrı ayrı" demez; bu
yüzden HTTP indirmelerinin hepsi tek jeton kovasını paylaşır. yt-dlp ayrı bir
süreç olduğu için kendi `--limit-rate` bayrağıyla sınırlanır: aynı anda hem
yt-dlp hem düz HTTP indirmesi koşuyorsa toplam, sınırı aşabilir. **FFmpeg
akışları (HLS/DASH) hiç sınırlanmıyor** — birleştirmeyi FFmpeg yapıyor ve
okuma hızına müdahale edemiyoruz. Yarım bir çözüm, çünkü alternatifi
(indirmeyi kendimiz yapıp FFmpeg'e boru ile vermek) yeniden kodlamadan
birleştirme kazanımını riske atardı.

**Diller şablondan üretiliyor, elle yazılmıyor.** 157 anahtarı 18 dosyada
elle yazmak, birini atlamayı ya da yanlış gruba koymayı kaçınılmaz kılardı.
Şablon `en.ts`'ten çıkarıldı ve önce kendi üzerinde sınandı: en.ts kendi
değerlerinden yeniden üretilince **bayt bayt aynı** dosya çıkıyor. Üstüne iki
ağ daha var — `Dictionary` tipi eksik anahtarda derlemeyi durduruyor, i18n
testi 20 dilin tamamında anahtar eşliğini denetliyor. Dil seçici de
`Segmented`'dan açılır listeye çevrildi: yan yana düğmeler iki dille
çalışıyordu, yirmiyle `.track-toggle`'daki hizalama hatasının aynısını
verirdi. Her dil **kendi adıyla** yazılıyor; yanlışlıkla anlamadığı bir dile
düşen kullanıcının geri dönebilmesi için listede tanıdık bir şey görmesi
gerekir.

**Hata metnini arka uç değil arayüz kurar.** Arka uç `{ code, detail }`
döndürür; cümleyi arayüz kendi dilinde yazar. Sebep basit: dili yalnızca
arayüz biliyor. `detail` bilerek çevrilmez — "server returned 500" gibi
teknik izlerin hata raporlarında aranabilir olması, çevrilmiş olmasından
değerli. Tanınmayan bir kod gelirse arayüz çökmez, detayı gösterir; sessizce
"bir şeyler ters gitti" demek hatayı bildirmek isteyenin elini boşaltırdı.

**Teslim edilemeyecek seçenek listelenmiyor.** DASH altyazı izleri
ayrıştırılabiliyor ama FFmpeg onları dosyaya çeviremiyor (§8'deki ölçüm).
Listelemek, kullanıcıya her tıkladığında hata veren bir satır sunmak olurdu;
hiç göstermemek daha dürüst. Ayrıştırıcı kodu da geri alındı — çalışmayan bir
yolun yarısını ölü kod olarak taşımak §8'deki temizlik borcunu büyütürdü.

**Altyazı ayrı bir `kind`, ayrı bir sütun değil.** Altyazı indirmesi aynı
FFmpeg boru hattını kullanır ama çıktı argümanları tamamen farklıdır
(`-c:s srt`; video tarafındaki `-c copy`, `-bsf:a aac_adtstoasc` ve
`+faststart` altyazıda anlamsız). Durum veritabanına yeni bir sütun yerine
mevcut `kind` alanına yazıldı: `kind` zaten saklanıyor ve yeniden açılışta
okunuyor, dolayısıyla devam ettirme migration olmadan çalışıyor. Ayrıca
altyazı izinin kendi playlist adresi manifestte olduğu için master + `-map
0:s:N` yoluna hiç girilmedi.

**Arama diakritikleri katlar, ayırt etmez.** `Iğdır` ile `igdir`, `açık` ile
`acık` aynı sayılır. Kullanıcı klavyede diakritiksiz yazar; ayrımı korumak
onu "kayıt kayıp" sanmaya iterdi. Katlama `toLowerCase()`'ten **önce**
uygulanır: JavaScript `"İ"` için `i` + birleşik nokta üretir, sonradan
temizlemek yerine harf baştan sadeleştirilir.

**Hız ve kalan süre veritabanına yazılmaz.** Yeniden açılışta "3.2 MB/sn"
göstermek yalan olurdu.

---

## 7. Yol boyunca bulunan gerçek hatalar

Not olarak duruyor, çünkü hepsi aynı dersi veriyor: **çalıştırmadan bilinmez.**

| Nasıl bulundu | Neydi |
|---|---|
| Canlı test | 400 ms'den kısa indirmeler tek ilerleme olayı bile yaymıyordu — kullanıcı %0'dan "tamamlandı"ya atlıyordu |
| Kendi kodumu okuma | Duraklatma eş zamanlılık yuvasını bırakmıyordu |
| Uygulamayı çalıştırma | `tokio::spawn`, Tauri'nin `setup()` bloğunda panic ediyor (runtime dışı) |
| Frontend testi | `in` operatörü prototip zincirini dolaşıyor → `"toString"` geçerli dil kodu sayılıyordu → bozuk ayarla uygulama ilk render'da çökerdi |
| Frontend testi | Rust'ta eklenen alan TS tipinde yoktu (elle senkron maliyeti) |
| Kendi kodumu denetleme | İki indirme aynı dosya adını alabiliyordu (kuyrukta bekleyenin `.part`'ı henüz yok) |
| Gerçek veriye bakma | MIME parametreleri kapsayıcı sanılıyordu: `OGG; CODECS="THEORA, VORBIS"` |
| Kendi kodumu okuma | `describeStream` manifest kontrolünü çözünürlükten önce yapıyordu → tüm HLS satırları "Akış" derdi |
| Hata yolunu bağlarken | `download()` hatayı yakalayıp saklıyordu ama not yalnızca `phase === "error"` iken çiziliyordu; indirme hatasında faz "ready" kaldığı için **hiç görünmüyordu** — yt-dlp eksikken İndir'e basan kullanıcı hiçbir şey görmüyordu |
| Tüm takımı koşturma | `pause_semantics` tek başına geçip workspace koşusunda kırılıyordu: sabit `sleep(120ms)` bir *tahmindi*, yük altında ilk bayt o pencereye yetişmiyor ve indirme `downloaded = 0` ile duraklıyordu. Artık diske düşen ilk bayta senkronlanıyor |
| Gerçek bir siteyle deneme | yt-dlp sağlayıcısı, kodek alanı **boş** gelen formatların hepsini eliyordu (`None` → "görüntü yok" sayılıyordu). archive.org gibi çıkarıcılarda tüm formatlar düşüyor, zincir sessizce `web`'e iniyordu: kullanıcı 3 kalite yerine 1 tane görüyordu. Yalnızca açıkça `"none"` diyen alan yokluk demektir |
| Ekran görüntüsüne bakma | Arayüzdeki Türkçe metinlerde hiç diakritik yoktu ("Cozumle", "Kutuphane"). Rozetteki "TAMAMLANDİ" aynı karede iki şeyi ele verdi: sözlük ASCII'ydi **ve** CSS Türkçe büyütme kuralı uyguluyordu |
| Aynı ekran görüntüsü | `index.html`'de `lang="tr"` sabitti, hiç güncellenmiyordu. `text-transform: uppercase` dile duyarlı olduğu için İngilizce arayüzde "Downloading" rozeti "DOWNLOADİNG" olurdu |
| Kullanıcının bildirmesi | Alt süreçlerin hiçbiri `CREATE_NO_WINDOW` ile başlatılmıyordu: uygulama GUI olarak derlense de her ffmpeg/yt-dlp çağrısında konsol penceresi parlıyordu |
| Kullanıcının ekran görüntüsü | Zincir `Network` hatasında **kesiliyordu**: yt-dlp bir Kick VOD'u için geçici 404 alınca `web` ve son çare sağlayıcı hiç çalışmıyor, başka yoldan inebilecek video inmiyordu. Gerekçe iki şeyi karıştırıyormuş — hangi hatayı *raporladığımız* ile denemeyi *bırakıp bırakmadığımız* |
| Ekran görüntüsüne bakma | `.track-toggle` stili `grid-template-columns: 1fr 1fr` ile "tam iki iz vardır" varsayımını kodluyordu; üçüncü düğme alt satıra düşüp grubun hizasını bozdu. Testler bunu göremezdi |

---

## 8. Sıradaki iş — öncelik sırasıyla

### Önce doğrulama

Dört maddenin dördü de kapandı. Kalanlar §4'ün sonundaki kısa listede:
arayüzden duraklat/devam, konsol düzeltmesine gözle bakma, kod imzalama.

### Sonra özellikler

| İş | Neden | Büyüklük |
|---|---|---|
| **DASH altyazısı** | İzler manifestte var ama FFmpeg dosyaya çeviremiyor: TTML (`stpp.ttml.im1t`) akış olarak görünüyor ama **decoder yok**, `-c:s copy` de parçalı XML'i tek köke birleştiremiyor ("Attempting to write multiple TTML documents"); WebVTT (`wvtt`, `text/vtt`) izlerini FFmpeg'in DASH demuxer'ı **hiç göstermiyor** (4 metin AdaptationSet'i olan manifestte `ffprobe` sıfır altyazı akışı bildirdi). Çözüm segmentleri kendimiz indirip birleştirmek — ayrı bir altyazı demuxer'ı demek | Büyük |
| **Kuyruk sıralama/öncelik** | Orijinal spec'te var | Orta |
| **Bileşen güncelleyici** | Ayarlar'daki "Güncelle" düğmeleri şu an sadece durum gösteriyor | Orta — **çalıştırılabilir indirmek ayrı bir güvenlik kararı, kullanıcıya sorulmalı** |
| **Üst menü çubuğu** | Mockup'ta var (File/Edit/View/Window/Help) | Küçük ama Tauri'de native menü ayrı katman |
| **CI (GitHub Actions)** | Testler her değişiklikte koşsun | Küçük — ama git remote yok |
| **Kod imzalama** | SmartScreen uyarısını kaldırır | Sertifika gerekir |

### Ölü kod (temizlenebilir)

- `favorites` tablosu — oluşturuluyor, hiç kullanılmıyor
- `component_versions` tablosu — aynı
- `MediaInfo.is_playlist` — hiç okunmuyor
- `DownloadOptions.headers` — dolduruluyor ama hep boş
- `DirectMediaProvider` — artık kayıtlı değil (ağsız yedek olarak duruyor)

---

## 9. Komutlar

```bash
npm install            # npm workspaces: kök + frontend
npm run tauri:dev      # gerçek pencere (canlı yeniden yükleme)
npm run dev            # sadece arayüz, tarayıcıda (sahte IPC ile)
npm run tauri:build    # .exe / .msi / .dmg
```

```bash
npm test               # cargo test --workspace
npm run test:front     # vitest
npm run lint           # cargo clippy -D warnings
npm run test:live      # ağ gerektiren testler
```

Kurulum çıktıları: `src-tauri/target/release/bundle/{nsis,msi}/`

### Uçtan uca doğrulama (CDP)

Uygulama gerçek `.exe` olarak açılıp arayüzü WebView2'nin uzaktan hata
ayıklama protokolünden sürülebiliyor:

```bash
WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9333
VDROP_DATA_DIR=<gecici klasor>
```

`VDROP_DATA_DIR` **doluysa** veritabanı oradan açılır. Bu değişken bunun için
eklendi: Windows'ta `app_data_dir()` `SHGetKnownFolderPath`'e dayanır, yani
`%APPDATA%`'yı değiştirmek işe yaramaz — izolasyon olmadan test koşuları
kullanıcının gerçek veritabanına yazıyordu. Değişken boşken davranış eskisi
gibidir.

### Opsiyonel bileşenler

```bash
pip install -U yt-dlp        # site-özel çıkarım
winget install ffmpeg        # HLS/DASH birleştirme
```

İkisi de PATH'te ya da uygulamanın `bin/` klasöründe aranıyor; durum
Ayarlar → Bileşenler'de görünüyor.

---

## 10. Dosya haritası — nereye bakmalı

| Ne arıyorsan | Nerede |
|---|---|
| İndirme durum makinesi | `crates/vdrop-download/src/lib.rs` |
| Dosya adı güvenliği | `crates/vdrop-download/src/paths.rs` |
| HLS ayrıştırıcı | `crates/vdrop-providers/src/hls.rs` |
| DASH ayrıştırıcı | `crates/vdrop-providers/src/dash.rs` |
| Sayfadan medya çıkarma | `crates/vdrop-providers/src/extract.rs` + `html.rs` |
| FFmpeg komutu | `crates/vdrop-media/src/lib.rs` |
| yt-dlp komutu | `crates/vdrop-ytdlp/src/lib.rs` |
| IPC komutları | `src-tauri/src/main.rs` |
| Tasarım jetonları | `frontend/src/styles/tokens.css` |
| Arayüz durum makinesi | `frontend/src/stores/downloadsReducer.ts` |
| Çeviriler | `frontend/src/i18n/tr.ts` (kaynak dil), 19 dil dosyası yanında |
| Dil kaydı ve yönü | `frontend/src/i18n/index.tsx` (`LANGUAGES`) |
| Arama eşleştirmesi | `frontend/src/lib/search.ts` |
| Bant genişliği kovası | `crates/vdrop-download/src/rate.rs` |
| Aracı site sağlayıcısı | `crates/vdrop-providers/src/kickdl.rs` |
| Altyazı ayrıştırma | `crates/vdrop-providers/src/hls.rs` (`parse_subtitles`) |
| Hata kodu → metin | `frontend/src/lib/errors.ts` + `i18n/tr.ts` `errors` |

Mimarinin tamamı: `docs/ARCHITECTURE.md`
Kullanım ve kurulum: `README.md`

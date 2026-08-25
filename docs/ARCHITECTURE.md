# VDrop — Mimari ve Yol Haritası

Bu doküman uzun ürün spesifikasyonunu, gerçekten inşa edilebilir bir plana
indirger. Her bölümde soru şu: **ne çalışır hâle geliyor?**

---

## A. Özet

VDrop; Rust çekirdekli, Tauri 2 + React arayüzlü, Python/yt-dlp'siz bir
masaüstü medya indiricisi. Şu an doğrudan HTTP linklerini ve HLS/DASH
akışlarını indirebiliyor. Site-özel extractor'lar ("Provider Pack") ayrı bir
katman olarak planlı ama yazılmadı — bu, spec'in kendisinin de kabul ettiği
gibi ayrı ve büyük bir mühendislik yatırımı.

## B. Rakip analizi

| Ürün | Güçlü yönü | Zayıf yönü | VDrop fırsatı |
|---|---|---|---|
| yt-dlp (CLI) | En geniş site desteği | GUI yok, Python bağımlı | Native GUI + Rust çekirdek |
| 4K Video Downloader+ | Cilalı UI, playlist | Kapalı kaynak, ücretli sınırlar | Açık mimari, sınırsız kuyruk |
| Open Video Downloader | Basit, açık kaynak | Eski UI, sınırlı format kontrolü | Modern UI + gelişmiş format seçici |
| IDM (Windows) | Hızlı segmentli indirme | Yalnızca Windows, medya sitesi desteği zayıf | Cross-platform + medya-özel format seçimi |

## C. Kullanıcı akışı

```
URL yapıştır → Çözümle → Medya önizleme + format seçici
   → İndir → (Sırada → İniyor ⇄ Duraklatıldı → Tamamlandı)
   → Kuyrukta ilerleme/duraklat/devam/iptal
   → Tamamlananlar otomatik olarak Kütüphane'ye ve Geçmiş'e düşer
```

## D. Ekran haritası

| Ekran | Durum |
|---|---|
| Ana sayfa — URL, çözümleme, format seçimi, hedef klasör | ✅ |
| Kuyruk — ilerleme, duraklat/devam/iptal, dosyayı aç/göster | ✅ |
| Kütüphane — inen dosyalar, diskte var mı kontrolü | ✅ |
| Geçmiş — tamamlanan/iptal/başarısız kayıtlar | ✅ |
| Ayarlar — tema, dil, klasör, eş zamanlılık, bileşenler | ✅ |

## E. Ayarlar

Hepsi `settings` tablosunda `TEXT` olarak saklanır; çözümleme ve doğrulama
`frontend/src/stores/settingsStore.tsx` içinde yapılır. Bozuk bir değer
uygulamayı çökertmez, varsayılana düşer.

| Anahtar | Kontrol | Varsayılan |
|---|---|---|
| `theme` | system / light / dark | `system` |
| `language` | tr / en | `tr` |
| `download_folder` | klasör seçici | OS İndirilenler |
| `max_concurrent` | kaydırıcı 1–16 | `3` |
| `auto_open_folder` | anahtar | `off` |
| `clipboard_watch` | anahtar | `off` |
| `notifications` | anahtar | `on` |

> `max_concurrent` ve `clipboard_watch` **çalışma sırasında** uygulanır;
> yeniden başlatma gerekmez. Ayrıntı: bölüm I ve bölüm N2.

---

## F. Katmanlar

```
Sunum (React)
    │  Tauri IPC  (19 komut, tek olay kanalı)
    ▼
Alan (Rust)
    ├── vdrop-download   resumable HTTP + dosya adı güvenliği   ✅
    ├── vdrop-media      HLS/DASH, FFmpeg boru hattı            ✅
    ├── vdrop-providers  URL → MediaInfo                        ✅ (yalnızca generic)
    └── vdrop-storage    SQLite + forward-only migration        ✅
```

Kural: iş mantığı `src-tauri` içinde değil, crate'lerde yaşar. `main.rs` ince
bir tel bağlantısıdır. Bunun pratik faydası: çekirdek mantık Tauri olmadan,
saniyeler içinde test edilebilir.

## G. İndirme motoru

Durum makinesi:

```
Sırada → İniyor ⇄ Duraklatıldı → (Yeniden deneniyor) → Tamamlandı | Başarısız | İptal
```

- Range başlığıyla devam ettirme; veri `<dosya>.part` içine yazılır, ancak
  tamamlanınca hedef ada taşınır. Yarım dosya asla nihai adı almaz.
- Geçici ağ hatalarında üstel geri çekilme: 1s, 2s, 4s, 8s, 16s.
- Duraklat/iptal `watch` kanalıyla, parçalar arasında iş birlikçi (cooperative)
  olarak kontrol edilir.
- Döngü bitiminde **her zaman** bir son ilerleme olayı yayılır. Bu olmadan
  400 ms'den kısa süren indirmeler tek bir ilerleme olayı bile üretmiyordu:
  kullanıcı %0 görüp birden "tamamlandı"ya atlıyordu ve veritabanına hiç bayt
  yazılmıyordu. (Canlı testle yakalanan gerçek bir kusurdu.)

**Uygulama yeniden başlatıldığında devam:** açılışta `reconcile_interrupted()`
"iniyor" kalmış kayıtları "duraklatıldı"ya çeker. Kullanıcı Devam et'e
bastığında transfer yeniden başlar ve motor diskteki `.part` dosyasını bulup
Range ile kaldığı yerden devam eder. Bayt kaybı olmaz. Ayrıntı: bölüm H2.

## H. HLS / DASH

`vdrop-media`, `ffmpeg -c copy` ile segmentleri **yeniden kodlamadan**
birleştirir. Neden elle segment indirici yazılmadı: HLS'te AES-128 anahtar
rotasyonu, discontinuity işaretleri, varyant seçimi ve PTS yeniden zamanlama
var; bunu doğru yapmak başlı başına bir proje.

İlerleme `-progress pipe:1` ile makine okunur şekilde okunur. Yüzde için süre
gerekir (FFmpeg "kaç saniye işledim" der, "yüzde kaç" demez), o yüzden önce
`ffprobe` ile süre öğrenilir; toplam boyut işlenen süre oranından tahmin
edilir.

**Duraklatma yok.** FFmpeg bir alt süreçtir; Windows'ta POSIX `SIGSTOP`
karşılığı güvenli bir duraklatma yoktur. Arayüz bunu bilir ve akış
satırlarında Duraklat düğmesini hiç göstermez. İptal edilen akışın yarım
çıktısı silinir.

### H1. HLS kalite seçimi

Bir `.m3u8` bağlantısı genelde bir **master playlist**'tir: içinde video
yoktur, farklı kalitelerdeki varyantların listesi vardır. `vdrop-providers::hls`
bunu ayrıştırır ve her varyantı çözünürlük, bit hızı, kodek ve boyut
tahminiyle bir format seçeneğine çevirir.

Süre için ikinci bir istek atılır (en düşük bant genişlikli varyantın medya
playlist'i, `#EXTINF` toplamı). Süre tüm varyantlarda aynı olduğu için bir
istek yeter; süreden de her varyantın boyut tahmini çıkar.

İki incelik:

1. **Tırnak içindeki virgül.** `CODECS="mp4a.40.2,avc1.64001f"` virgül taşır;
   naif bir `split(',')` özniteliği ikiye böler ve sessizce yanlış bant
   genişliği/çözünürlük üretir. Ayrıştırıcı tırnak durumunu izler.
2. **Varyantın kendi URL'i indirilmez.** Bazı yayınlarda ses ayrı bir
   rendition'dadır (`#EXT-X-MEDIA:TYPE=AUDIO`) ve varyant playlist'i yalnızca
   görüntü taşır — sonuç sessiz bir video olurdu. Bunun yerine master URL'i
   FFmpeg'e verilip varyant **program indeksiyle** seçilir (`-map 0:p:N`).
   FFmpeg master playlist'teki her varyantı manifest sırasıyla bir program
   olarak açar; `-map 0:p:N` o programın tüm akışlarını (ses dahil) alır.

Seçim `downloads.variant_index` sütununda saklanır (migration 0003): aksi
halde duraklatılan ya da uygulama kapandıktan sonra devam ettirilen bir akış,
kullanıcının seçtiği 1080p yerine FFmpeg'in varsayılanına düşerdi.

### H2. DASH kalite seçimi

Aynı amaç, **farklı mekanizma** — ve bu ayrım kritik. FFmpeg iki manifest
türünü farklı açar:

| Format | FFmpeg'in gördüğü | Seçim |
|---|---|---|
| HLS master | her varyant ayrı bir **program** | `-map 0:p:N` |
| DASH | tüm temsiller tek programın ayrı **akışları** | `-map 0:v:N -map 0:a:0` |

DASH'te ses ayrı bir `AdaptationSet`'tedir, o yüzden `-map 0:a:0` şart:
yalnızca video akışını seçmek sessiz bir dosya üretir.

Aynı tamsayı iki formatta iki farklı şey ifade ettiği için seçim, tip güvenli
bir `StreamSelector` enum'uyla taşınır (`vdrop-media`). Hangi biçimin
kullanılacağı `downloads.provider_id` sütunundan türetilir — böylece
duraklatılıp devam ettirilen bir indirme de doğru komutu kurar.

MPD ayrıştırması `html::find_tags` üzerine kurulu: XML'de de çalışan bir
etiket tarayıcı, tam bir XML ayrıştırıcısı bu iş için fazla ağır olurdu. Tek
incelik, tarayıcının düz çalışması — belge önce `<AdaptationSet>` bloklarına
bölünür, yoksa ses ve altyazı temsilleri de kalite sanılırdı.

## H2. Duraklatma neden transferi sonlandırır

HTTP indirmelerinde duraklatma, bağlantıyı açık tutup yerinde beklemez;
görevi **sonlandırır** ve `.part` dosyasını diskte bırakır. İki sebep:

1. **Yuva işgali.** Bekleyen görev eş zamanlılık iznini elinde tutardı. Limit
   3 iken 3 indirmeyi duraklatan kullanıcı sıradakileri başlatamazdı — yani
   özelliğin var olma sebebi çalışmazdı.
2. **Boşta bağlantı.** Uzun duraklamalarda sunucu boşta duran bağlantıyı
   zaten düşürür; bu da yeniden deneme sayacını boşa yakar.

Dolayısıyla "devam et" her zaman **yeniden başlatmadır**. Aynı kod yolu
uygulama kapanıp açıldığında da işler — orada da elimizde yalnızca `.part`
dosyası vardır. Tek yol, iki senaryo.

## I. Eş zamanlılık kapısı

`src-tauri/src/concurrency.rs`. Her transfer başlamadan önce bir semafor izni
alır; fazlası kuyrukta bekler (iptal edilmez).

Limit çalışma sırasında değişebilir:

- **Artarsa** — `add_permits`, bekleyen bir indirme hemen başlar.
- **Azalırsa** — çalışan indirmeler **kesilmez** (kullanıcı veri kaybetmemeli).
  Fazla izinler arka planda "yutulur"; yeni limit, mevcut indirmeler bittikçe
  kademeli olarak yürürlüğe girer.

## J. Veritabanı

`downloads`, `history`, `settings`, `favorites`, `library_items`,
`component_versions`, `schema_migrations`.

Migration'lar forward-only ve her biri tek bir işlem içinde koşar: yarıda
kalırsa hiçbiri uygulanmamış sayılır. Yeni sürüm gerektiğinde `MIGRATIONS`
dizisine **yeni girdi eklenir**, mevcut girdiler asla değiştirilmez.

Uygulanmış sürümler:

| Sürüm | İçerik |
|---|---|
| `0001_init` | Tüm tablolar |
| `0002_stream_kind_and_error` | `downloads.kind` (http/stream), `error_message`, `thumbnail_url`; `history.destination_path`, `total_bytes`; indeksler |
| `0003_variant_index` | `downloads.variant_index` — seçilen kalite devam ettirmede korunsun diye. Anlamı `provider_id`'ye bağlıdır: HLS'te program indeksi, DASH'te video akış indeksi |

## K. Provider mimarisi

Kayıtlı sağlayıcılar, özelden genele:

| Sağlayıcı | Kapsam |
|---|---|
| `YtDlpProvider` | Opsiyonel; kuruluysa zincirin **başında**. Site-özel çıkarım |
| `HlsProvider` | `.m3u8` / `.m3u` → master playlist ayrıştırma, kalite listesi |
| `DashProvider` | `.mpd` → MPD ayrıştırma, kalite listesi |
| `WebProvider` | Her http(s) adresi. Adrese **sorar** (`Content-Type`): medya ise doğrudan indirilebilir olarak döner (gerçek `Content-Length` ile), HTML ise sayfadan medya çıkarır |

`DirectMediaProvider` artık kayıtlı değil: yaptığı iş (uzantıdan tahmin)
`WebProvider`ın kapsamında ve o, tahmin yerine sunucunun beyanına bakıyor.
Tip dışarı açık kalmaya devam ediyor — ağsız bir bağlamda hâlâ kullanılabilir.

### K1. Sayfadan medya çıkarma

Site-özel kod yazmadan geniş bir kapsam açan gözlem: **çoğu sayfa videosunu
zaten kendisi ilan eder.** Sosyal medya önizlemesi (Open Graph) ve arama
motoru zenginleştirmesi (JSON-LD `VideoObject`) için bunu koymak zorundalar.

Okunan kaynaklar, güven sırasıyla: `og:video:secure_url` → `og:video` →
JSON-LD `contentUrl` → `<video src>` → `<video><source src>` →
`twitter:player:stream`.

Üç incelik:

1. **Oynatıcı sayfaları ayıklanır.** `og:video` bazen gerçek dosyaya değil
   bir `/embed/xyz` sayfasına işaret eder; `og:video:type` `text/html` ise
   veya adres oynatılabilir bir uzantı taşımıyorsa aday sayılmaz. Yoksa
   indirme sonucunda diske HTML yazılırdı.
2. **HTML varlıkları çözülür.** URL'lerde `&amp;` çok yaygındır; çözülmezse
   istek 404 döner. `&quot;` ise `type="video/webm; codecs=&quot;vp9&quot;"`
   gibi iç içe tırnaklarda çıkar.
3. **MIME parametreleri kapsayıcı değildir.** `video/ogg; codecs="theora"`
   içinden `ogg` alınır; parametreler atılmazsa ekrana bu çöp basılır.

Kalite etiketi yayıncının kendi dosya adından okunur (`...480p.vp9.webm`).
Uydurma değil, sayfanın kendi adlandırması — ve onsuz aynı sayfadaki iki
`.webm` satırı ayırt edilemez.

**Kapsam dışı:** JavaScript ile çalışma zamanında kurulan oynatıcılar.

### K2. Site-özel sağlayıcılar — yt-dlp'ye devredildi

Özgün plan, site extractor'larını sandbox'lı bir JS çalışma zamanıyla kendi
içimizde yazmaktı. Bu, yt-dlp'nin yıllardır yaptığı işi baştan yapmak demek:
yüzlerce site, her biri kendi imza/cipher mantığıyla, ve hepsi sürekli
değişiyor. Dürüst cevap devretmek.

`vdrop-ytdlp` **opsiyonel** bir bileşen olarak bağlandı — tıpkı FFmpeg gibi.
Yoksa uygulama eskisi gibi çalışır; varsa zincirin başına geçer ve tanımadığı
adresler için `Unsupported` diyerek sırayı genel sayfa çıkarımına bırakır.

Sağlayıcı zinciri artık **yetenek eksikliğinde geri düşer**: `Unsupported` ve
`Parse` hataları bir sonrakini dener. `Network` hataları yayılır — 404'ü
"bu sayfada medya yok" diye örtmek kullanıcıya yanlış hikâye anlatmak olurdu.

Sandbox'lı JS çalışma zamanını yine de kendimiz yazmak isteseydik, önerilen
yol şuydu:

1. **Sandbox'lı JS runtime seçimi.** Adaylar: `boa` (saf Rust, az bağımlılık,
   yavaş) ve `QuickJS` bağlamaları (hızlı, C bağımlılığı). Karar öncesi ikisi
   de küçük bir spike ile denenmeli — kriter: bir HTML sayfasını ayrıştırıp
   JSON çıkarma süresi ve bellek tavanı.
2. **API yüzeyi:** `fetch(url, headers)`, `parseHtml`, `parseJson`,
   `returnMediaInfo`. Dosya sistemine ve shell'e erişim **yok**.
3. **İlk hedef:** RSS/JSON tabanlı podcast ve medya akışları gibi "kolay"
   siteler. Cipher korumalı siteler (YouTube) en son.

Risk: "her siteyle çalışsın" beklentisi, bu katman olmadan karşılanamaz. Bu,
projenin en büyük açık mühendislik kalemi.

## L. Güvenlik modeli

**Dosya adı sanitizasyonu — uygulandı.** `vdrop_download::paths`. Tehdit
modeli: `title` uzaktan gelen güvenilmeyen veridir. Engellenenler:

| Saldırı | Örnek | Sonuç |
|---|---|---|
| Path traversal | `../../etc/passwd` | `passwd` |
| Mutlak yola kaçış | `C:\Windows\evil.exe` | `evil.exe` |
| Windows aygıt adı | `CON`, `NUL`, `LPT1` | `_CON` |
| Sondaki nokta/boşluk | `report.txt.` | `report.txt` |
| Uzantı gizleme (RTL) | `evil\u202Egnp.exe` | override silinir, `.exe` görünür |

İki katmanlı: ad temizlenir **ve** `safe_join` üretilen yolun gerçekten hedef
klasörün doğrudan çocuğu olduğunu doğrular. Ayrıca `unique_destination` aynı
adlı bir dosyayı (ve yarım kalmış `.part` kardeşini) sessizce ezmez.

**Tauri capabilities.** `src-tauri/capabilities/default.json` en az ayrıcalık
ilkesiyle yazıldı: sadece klasör seçici, dosya/klasör açma ve pencere
kontrolü. CSP `null` değil — gerçek bir politika tanımlı.

**Kimlik bilgileri.** Çerez/token'lar düz SQLite'a **yazılmaz**. OS güvenli
kimlik deposu entegrasyonu yapılmadı; provider katmanıyla birlikte gelmeli.

## M. IPC

Tek olay kanalı: `download:event` → `{ id, event }`. Yedi ayrı olay adına
yedi ayrı dinleyici bağlamak yerine arayüz tek yerden abone olur.

Komutlar: `analyze_url`, `create_download`, `pause/resume/cancel_download`,
`list_downloads`, `remove_download`, `clear_finished`, `list_history`,
`clear_history`, `list_library`, `remove_library_item`, `paths_exist`,
`get_settings`, `set_setting`, `select_download_folder`, `app_info`,
`open_path`, `reveal_path`.

TypeScript karşılıkları `frontend/src/types/ipc.ts` içinde elle senkron
tutuluyor. Alan adları serde'nin `snake_case` çıktısıyla birebir aynı
tutuldu; `ts-rs` ile otomatik üretime geçiş mekanik olsun diye.

## N. Yerelleştirme

`frontend/src/i18n/`. Anahtar tabanlı, Türkçe kaynak dil, İngilizce ikinci.

`en.ts` dosyası `Dictionary` tipini sağlamak zorunda; `tr.ts`'e yeni bir
anahtar eklenip karşılığı yazılmazsa proje **derlenmez**. Eksik çeviri sessizce
ürüne sızamaz. `Widen<T>` yardımcısı yapıyı korurken değerleri `string`e
genişletir — yoksa `as const` yüzünden İngilizce metin yazmak tip hatası
verirdi.

RTL diller için altyapı hazır (`dir` alanı, `inset-inline-*` kullanımı) ama
henüz RTL dil eklenmedi.

## N2. Pano izleme ve bildirimler

`src-tauri/src/clipboard.rs`. Ayar açıkken 1,2 saniyede bir pano yoklanır;
ayar kapalıyken görev bir `watch` kanalında uyur (sürekli görev oluşturup yok
etmek yerine tek görevi kapıda bekletmek daha basit ve sızıntı riski yok).

**Yakalanan bağlantıya hiçbir ağ isteği atılmaz.** Arayüz bir şerit gösterir;
istek ancak kullanıcı "Çözümle"ye basınca gider. Gerekçe gizlilik: pano iş
yerinin iç ağ adreslerini, imzalı S3 bağlantılarını, parola sıfırlama
linklerini taşır.

Filtre bilinçli olarak dar: yalnızca bilinen medya uzantıları ve akış
manifestleri. Her `http` bağlantısında şerit çıkarmak, özelliği bir dakikada
kapatılır hale getirirdi. Yanlış pozitif yerine yanlış negatifi tercih
ediyoruz — kaçırılan bir link elle yapıştırılabilir.

Bildirimler Rust tarafından, indirme sonlandığında gönderilir; böylece pencere
küçültülmüşken de çalışır. Duraklatma gibi kullanıcının kendi yaptığı şeyler
bildirilmez — kendi tıkladığı şeyi ona haber vermek gürültüdür. Başarısızlık
sessizce yutulmaz, günlüğe yazılır: Windows'ta toast bildirimleri uygulamanın
Başlat menüsünde kayıtlı olmasını ister, yani taşınabilir çalıştırmalarda
başarısız olur ve sebebi görünür olmalıdır.

## O. Erişilebilirlik

- Tüm etkileşimli öğeler klavyeyle erişilebilir; `:focus-visible` ile görünür
  odak halkası (fare tıklamasında halka çıkmaz).
- İkon-only düğmelerde `aria-label` + `title` zorunlu.
- İlerleme çubukları `role="progressbar"`; toplam boyut bilinmiyorsa
  `aria-valuenow` verilmez (ekran okuyucu "yüzde 0" demek yerine belirsiz
  ilerleme okur).
- Anahtarlar `role="switch"` + `aria-checked`.
- `prefers-reduced-motion` tüm animasyonları durdurur.
- Renk asla tek sinyal değil: durum hem renk hem metin rozetiyle bildirilir.
  "Canlı" rengi olarak saf yeşil yerine teal seçildi; kırmızı-yeşil renk
  körlüğünde hata kırmızısından ayırt edilebilsin diye.

## P. Test

| Katman | Kapsam |
|---|---|
| `vdrop-download` | 17 test — sanitizasyon (path traversal, aygıt adları, bidi, UTF-8 kesme), sahte HTTP sunucusuna karşı tam indirme, Range devam ettirme, duraklatma semantiği |
| `vdrop-providers` | 51 test — HLS ayrıştırma (tırnaklı virgül, göreli URI, VOD/canlı ayrımı, boyut tahmini), HTML tarayıcı (tırnak içi `>`, yorum blokları, varlık çözme), sayfa çıkarımı (oynatıcı sayfası eleme, JSON-LD kaçışları, kalite etiketi) |
| `vdrop-storage` | 10 test — migration idempotency, ilerleme/durum kalıcılığı, çökme sonrası uzlaştırma, geçmiş/kütüphane |
| `src-tauri` | 9 test — eş zamanlılık kapısı (limit, artırma, çalışanı kesmeden azaltma) + pano sınıflandırıcı |
| `vdrop-ytdlp` | 11 test — ilerleme satırı ayrıştırma (eksik/NA alanlar), format sınıflandırma (video/ses/muxed), bit hızı geri düşüşü, hata satırı seçimi |
| `vdrop-media` | 4 test — manifest tespiti, hata ayıklama, FFmpeg bulma, seçici → FFmpeg argümanı |
| `frontend` | 65 test — durum makinesi (olay uygulama, tazeleme birleştirme, bayat hız bekçisi, telemetri), biçimleme, format satırı adlandırma, çeviri bütünlüğü |
| **Canlı** (`#[ignore]`) | 12 test — gerçek CDN'den indirme + bayt bayt Range devam ettirme; gerçek HLS probe/segment/iptal/temizlik; master playlist → kalite listesi; **seçilen varyantın gerçekten indiği** (çıktının çözünürlüğü ölçülerek); gerçek bir web sayfasından medya çıkarma (Wikimedia Commons); gerçek MPD → kalite listesi ve **seçilen DASH temsilinin gerçekten indiği** |

Frontend'de saf mantık React'ten ayrılmış (`downloadsReducer.ts`,
`streamLabel.ts`) — durum makinesini bir bileşen render ederek sınamak hem
yavaş hem dolaylı olurdu.

Eksik: bileşen render testleri, E2E, GitHub Actions.

## Q. Paketleme

`npm run tauri:build` → Windows (nsis/msi), macOS (dmg).

Kod imzalama / notarization yapılmadı. İmzasız bir Windows kurulumu
SmartScreen uyarısı gösterir; bu beklenen davranıştır ve ancak bir kod imzalama
sertifikasıyla giderilir.

## R. Fazlar

| Faz | Kapsam | Durum |
|---|---|---|
| 1 — Temel | Tauri kabuğu, Rust çekirdek, SQLite, temel UI | ✅ |
| 2 — İndirme motoru | Resumable HTTP, duraklat/devam/iptal, ilerleme | ✅ |
| 3 — Medya | HLS/DASH segment indirme + kalite seçimi, FFmpeg birleştirme | ✅ |
| 4 — UX | Kütüphane, Geçmiş, Ayarlar, temalar, i18n | ✅ |
| 5a — Sayfa çıkarımı | Open Graph / JSON-LD / `<video>` üzerinden medya | ✅ |
| 5b — Site provider'ları | yt-dlp entegrasyonu (opsiyonel bileşen) | ✅ |
| 6 — İleri | Zamanlayıcı, tarayıcı eklentisi, eklenti pazarı | ⬜ |
| 7 — Üretim | Kod imzalama, notarization, CI/CD, güncelleyici | ⬜ |

## S. Riskler

1. **Site kapsamı beklentisi.** "Her siteyle çalışsın" hedefi, provider
   sandbox'ı olmadan karşılanamaz. En büyük mühendislik riski.
2. **FFmpeg dağıtımı.** Şu an sistemde kurulu olması bekleniyor. Uygulamayla
   birlikte dağıtmak isteniyorsa LGPL/GPL yapılandırması netleştirilmeli ve
   bileşen versiyonlama (`component_versions` tablosu hazır) devreye alınmalı.
3. **İmzasız dağıtım.** Kod imzalama sertifikası olmadan Windows ve macOS
   kurulumları güvenlik uyarısı gösterir.

## T. Definition of Done — Faz 1-4

- [x] `cargo test --workspace` yeşil (36 test), `cargo clippy` uyarısız
- [x] Frontend `npm run build` hatasız (strict TS)
- [x] Dosya adı sanitizasyonu eklendi ve test edildi
- [x] `tauri dev` gerçek makinede pencere açıyor
- [x] Gerçek dosya indirme senaryosu otomatik testle doğrulandı
- [x] Gerçek HLS akışı otomatik testle doğrulandı
- [x] Kurulum paketi üretiliyor

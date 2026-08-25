// Turkce sozluk. Bu dosya anahtar kumesinin **kaynagidir**: diger diller
// `typeof tr` tipini saglamak zorundadir, boylece eksik ceviri derleme
// hatasina doner, calisma zamaninda bos metne degil.
//
// Yazim ilkesi: dugme ne yapiyorsa onu soyler ("İndir", "Gonder" degil).
// Hata mesajlari ozur dilemez, ne oldugunu ve ne yapilacagini soyler.

export const tr = {
  nav: {
    home: "Ana sayfa",
    queue: "Kuyruk",
    library: "Kütüphane",
    history: "Geçmiş",
    settings: "Ayarlar",
    sections: "Bölümler",
    engineReady: "Motor hazır",
  },

  status_bar: {
    throughput: "Toplam hız",
    active: "Aktif",
    pauseAll: "Hepsini duraklat",
    resumeAll: "Hepsini devam ettir",
    clearFinished: "Bitenleri temizle",
  },

  home: {
    title: "Video bağlantısını çözümle",
    videoAndAudio: "Video + Ses",
    audioOnly: "Yalnızca ses",
    subtitlesOnly: "Altyazı",
    noSubtitleTrack: "Bu kaynakta altyazı yok",
    addToQueue: "Kuyruğa ekle",
    noAudioTrack: "Bu kaynakta ayrı bir ses seçeneği yok",
    legacyTitle: "Bağlantı yapıştırın",
    subtitle:
      "VDrop bağlantıyı çözümler, kaliteyi siz seçersiniz. İndirme ancak siz onaylayınca başlar.",
    placeholder: "https://... veya doğrudan medya bağlantısı",
    analyze: "Çözümle",
    paste: "Panodan yapıştır",
    analyzing: "Çözümleniyor",
    download: "İndir",
    changeFolder: "Klasör seç",
    savingTo: "Kayıt yeri",
    streamNotice: "Bölümlü akış",
    streamNoticeBody:
      "Bu bir HLS/DASH akışı. VDrop bölümleri FFmpeg ile yeniden kodlamadan birleştirir. Bu tür indirmeler duraklatılamaz, yalnızca iptal edilebilir.",
    ffmpegMissing: "FFmpeg bulunamadı",
    ffmpegMissingBody:
      "Bölümlü akışları (.m3u8 / .mpd) indirmek için FFmpeg gerekir. Düz dosya bağlantıları FFmpeg olmadan da çalışır.",
  },

  queue: {
    title: "Aktif kuyruk",
    completed: "Tamamlananlar",
    itemsDownloading: "indirme sürüyor",
    analyzing: "Akış çözümleniyor...",
    connecting: "Bağlanıyor...",
    subtitle: "Devam eden ve tamamlanan indirmeler",
    empty: "Kuyruk boş",
    emptyBody: "Ana sayfada bir bağlantı çözümleyin, indirmeler burada görünsün.",
    clearFinished: "Bitenleri temizle",
    pause: "Duraklat",
    resume: "Devam et",
    cancel: "İptal",
    remove: "Listeden kaldır",
    openFolder: "Klasörde göster",
    openFile: "Dosyayı aç",
    retry: "Yeniden dene",
  },

  library: {
    title: "Kütüphane",
    subtitle: "İndirilen dosyalar",
    empty: "Kütüphane boş",
    emptyBody: "Tamamlanan indirmeler otomatik olarak buraya eklenir.",
    missing: "Dosya diskte yok",
    removeEntry: "Kayıttan kaldır",
    deleteFile: "Dosyayı sil",
  },

  history: {
    title: "Geçmiş",
    subtitle: "Tamamlanan, iptal edilen ve başarısız indirmeler",
    empty: "Geçmiş boş",
    emptyBody: "Biten her indirme burada kayıt bırakır.",
    clear: "Geçmişi temizle",
  },

  settings: {
    title: "Ayarlar",
    subtitle: "Tercihler bu bilgisayarda saklanır",

    groupGeneral: "Genel",
    theme: "Tema",
    themeHint: "Sistem temasını izleyebilir ya da sabitleyebilirsiniz",
    themeSystem: "Sistem",
    themeLight: "Açık",
    themeDark: "Koyu",
    language: "Dil",
    languageHint: "Arayüz dili",

    groupDownloads: "İndirme",
    folder: "İndirme klasörü",
    folderHint: "Yeni indirmeler buraya kaydedilir",
    choose: "Değiştir",
    concurrency: "Eş zamanlı indirme",
    concurrencyHint: "Aynı anda kaç indirme çalışsın; fazlası sırada bekler",
    bandwidth: "Hız sınırı",
    bandwidthHint: "Tüm indirmelerin toplam hızı; 0 sınırsız demektir",
    bandwidthUnit: "KB/sn",
    bandwidthUnlimited: "Sınırsız",
    autoOpen: "Bitince klasörü aç",
    autoOpenHint: "İndirme tamamlanınca dosyanın klasörünü aç",
    clipboard: "Panoyu izle",
    clipboardHint: "Kopyaladığınız medya bağlantılarını otomatik yakala",
    notifications: "Bildirimler",
    notificationsHint: "İndirme bitince sistem bildirimi göster",

    navGeneral: "Genel",
    navDownloads: "İndirmeler",
    navComponents: "Bileşenler",
    navAbout: "Hakkında",
    allComponentsOk: "Tüm bileşenler çalışıyor",
    someComponentsMissing: "Eksik bileşen var",
    installed: "Kurulu",
    notInstalled: "Kurulu değil",

    groupComponents: "Bileşenler",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Bölümlü akışları (.m3u8 / .mpd) birleştirmek için kullanılır",
    ffmpegFound: "Kurulu",
    ffmpegNotFound: "Bulunamadı",
    version: "Sürüm",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Site-özel çıkarım. Kuruluysa yüzlerce site desteklenir; kurulu değilse doğrudan bağlantılar ve akışlar yine çalışır.",
    ytdlpInstallHint: "Kurmak için: ",
    ffmpegInstallHint: "Kurmak için: ",

    groupAbout: "Hakkında",
    appVersion: "VDrop sürümü",
    engine: "Çekirdek",
    engineHint: "Rust + Tauri 2. Python veya yt-dlp bağımlılığı yok.",
  },

  status: {
    queued: "Sırada",
    downloading: "İniyor",
    paused: "Duraklatıldı",
    retrying: "Yeniden deneniyor",
    completed: "Tamamlandı",
    failed: "Başarısız",
    cancelled: "İptal edildi",
  },

  units: {
    perSecond: "/sn",
    remaining: "kaldı",
    of: "/",
  },

  clipboard: {
    caught: "Panoda bir medya bağlantısı var",
    resolve: "Çözümle",
    dismiss: "Yoksay",
  },

  /**
   * Arka uc hata kodlarinin karsiliklari.
   *
   * Anahtarlar `src-tauri/src/main.rs` icindeki `AppError::code` degerleridir.
   * Yeni bir kod eklenirse buraya da eklenmeli; eklenmezse arayuz cokmez,
   * teknik detayi gosterir (bkz. `lib/errors.ts`).
   */
  errors: {
    unknown: {
      title: "Beklenmeyen bir hata",
      body: "Ayrıntı aşağıda.",
    },
    empty_url: {
      title: "Önce bir bağlantı yapıştırın",
      body: "Bir video sayfasının adresini ya da doğrudan medya bağlantısını girin.",
    },
    unsupported: {
      title: "Bu adres desteklenmiyor",
      body: "VDrop bu bağlantıyı tanıyamadı. yt-dlp kuruluysa çok daha fazla site çözümlenir.",
    },
    network: {
      title: "Bağlantı kurulamadı",
      body: "Adres doğru mu, internet bağlantınız çalışıyor mu?",
    },
    drm: {
      title: "İçerik DRM korumalı",
      body: "VDrop DRM korumalı yayınları indiremez.",
    },
    parse: {
      title: "Medya bilgisi okunamadı",
      body: "Sunucu beklenmeyen bir yanıt verdi.",
    },
    no_media: {
      title: "Sayfada indirilebilir medya yok",
      body: "Video JavaScript ile yükleniyorsa VDrop henüz göremiyor; doğrudan medya bağlantısını yapıştırmayı deneyin.",
    },
    ytdlp_missing: {
      title: "yt-dlp kurulu değil",
      body: "Bu indirme yt-dlp gerektiriyor. Ayarlar > Bileşenler bölümüne bakın.",
    },
    ffmpeg_missing: {
      title: "FFmpeg kurulu değil",
      body: "HLS/DASH akışlarını birleştirmek için FFmpeg gerekir. Ayarlar > Bileşenler bölümüne bakın.",
    },
    record_missing: {
      title: "İndirme kaydı oluşturulamadı",
      body: "Kayıt veritabanına yazılamadı.",
    },
    internal: {
      title: "Beklenmeyen bir hata",
      body: "Ayrıntı aşağıda.",
    },
  },

  common: {
    search: "Ara",
    searchPlaceholder: "Başlıkta ve adreste ara",
    clearSearch: "Aramayı temizle",
    noResults: "Eşleşen kayıt yok",
    noResultsBody: "Farklı bir arama deneyin.",
    cancel: "Vazgeç",
    confirm: "Onayla",
    close: "Kapat",
    unknown: "Bilinmiyor",
    audio: "Ses",
    video: "Görüntü",
    stream: "Akış",
    subtitle: "Altyazı",
    file: "Dosya",
  },
} as const;

/**
 * `as const` her degeri kendi literal tipine kilitler ("Sırada" tipi
 * `"Sırada"` olur). Bu, anahtar yapisini korumak icin iyi ama ceviriler
 * icin felaket: en.ts'te "Queued" yazmak tip hatasi verirdi.
 *
 * `Widen` yapiyi aynen korur ama yaprak degerleri `string`e genisletir.
 * Sonuc: **eksik anahtar hala derleme hatasi**, farkli metin degil.
 */
type Widen<T> = {
  [K in keyof T]: T[K] extends string ? string : Widen<T[K]>;
};

export type Dictionary = Widen<typeof tr>;

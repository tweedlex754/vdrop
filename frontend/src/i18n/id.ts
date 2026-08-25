import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const id: Dictionary = {
  nav: {
    home: "Beranda",
    queue: "Antrean",
    library: "Pustaka",
    history: "Riwayat",
    settings: "Pengaturan",
    sections: "Bagian",
    engineReady: "Mesin siap",
  },

  status_bar: {
    throughput: "Kecepatan total",
    active: "Aktif",
    pauseAll: "Jeda semua",
    resumeAll: "Lanjutkan semua",
    clearFinished: "Bersihkan yang selesai",
  },

  home: {
    title: "Uraikan tautan video",
    videoAndAudio: "Video + audio",
    audioOnly: "Audio saja",
    subtitlesOnly: "Subtitel",
    noSubtitleTrack: "Sumber ini tidak punya subtitel",
    addToQueue: "Tambah ke antrean",
    noAudioTrack: "Sumber ini tidak punya jalur audio terpisah",
    legacyTitle: "Tempel tautan",
    subtitle:
      "VDrop menguraikan tautan dan kualitas Anda yang pilih. Tidak ada yang diunduh sampai Anda menyuruhnya.",
    placeholder: "https://... atau tautan media langsung",
    analyze: "Uraikan",
    paste: "Tempel dari papan klip",
    analyzing: "Menguraikan",
    download: "Unduh",
    changeFolder: "Pilih folder",
    savingTo: "Simpan ke",
    streamNotice: "Aliran tersegmentasi",
    streamNoticeBody:
      "Ini aliran HLS/DASH. VDrop menyambung segmen dengan FFmpeg tanpa menyandi ulang. Unduhan seperti ini bisa dibatalkan, tapi tidak bisa dijeda.",
    ffmpegMissing: "FFmpeg tidak ditemukan",
    ffmpegMissingBody:
      "Aliran tersegmentasi (.m3u8 / .mpd) memerlukan FFmpeg. Tautan berkas langsung tetap jalan tanpanya.",
  },

  queue: {
    title: "Antrean aktif",
    completed: "Selesai",
    itemsDownloading: "sedang diunduh",
    analyzing: "Menganalisis aliran...",
    connecting: "Menghubungkan...",
    subtitle: "Unduhan berjalan dan selesai",
    empty: "Antrean kosong",
    emptyBody: "Uraikan sebuah tautan di Beranda, unduhan akan muncul di sini.",
    clearFinished: "Bersihkan yang selesai",
    pause: "Jeda",
    resume: "Lanjutkan",
    cancel: "Batalkan",
    remove: "Hapus dari daftar",
    openFolder: "Tampilkan di folder",
    openFile: "Buka berkas",
    retry: "Coba lagi",
  },

  library: {
    title: "Pustaka",
    subtitle: "Berkas yang diunduh",
    empty: "Pustaka kosong",
    emptyBody: "Unduhan yang selesai otomatis masuk ke sini.",
    missing: "Berkas sudah hilang dari diska",
    removeEntry: "Hapus entri",
    deleteFile: "Hapus berkas",
  },

  history: {
    title: "Riwayat",
    subtitle: "Unduhan selesai, dibatalkan, dan gagal",
    empty: "Riwayat kosong",
    emptyBody: "Setiap unduhan yang selesai meninggalkan catatan di sini.",
    clear: "Bersihkan riwayat",
  },

  settings: {
    title: "Pengaturan",
    subtitle: "Preferensi disimpan di komputer ini",

    groupGeneral: "Umum",
    theme: "Tema",
    themeHint: "Ikuti tema sistem atau kunci satu",
    themeSystem: "Sistem",
    themeLight: "Terang",
    themeDark: "Gelap",
    language: "Bahasa",
    languageHint: "Bahasa antarmuka",

    groupDownloads: "Unduhan",
    folder: "Folder unduhan",
    folderHint: "Unduhan baru disimpan di sini",
    choose: "Ubah",
    concurrency: "Unduhan serentak",
    concurrencyHint: "Berapa yang jalan bersamaan; sisanya mengantre",
    bandwidth: "Batas kecepatan",
    bandwidthHint: "Kecepatan total semua unduhan; 0 berarti tanpa batas",
    bandwidthUnit: "KB/dtk",
    bandwidthUnlimited: "Tanpa batas",
    autoOpen: "Buka folder saat selesai",
    autoOpenHint: "Tampilkan berkas begitu unduhan selesai",
    clipboard: "Pantau papan klip",
    clipboardHint: "Tangkap tautan media saat Anda menyalinnya",
    notifications: "Notifikasi",
    notificationsHint: "Tampilkan notifikasi sistem saat unduhan selesai",

    navGeneral: "Umum",
    navDownloads: "Unduhan",
    navComponents: "Komponen",
    navAbout: "Tentang",
    allComponentsOk: "Semua komponen berjalan",
    someComponentsMissing: "Ada komponen yang hilang",
    installed: "Terpasang",
    notInstalled: "Belum terpasang",

    groupComponents: "Komponen",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Dipakai untuk menyambung aliran tersegmentasi (.m3u8 / .mpd)",
    ffmpegFound: "Terpasang",
    ffmpegNotFound: "Tidak ditemukan",
    version: "Versi",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Ekstraksi khusus per situs. Dengannya ratusan situs jalan; tanpanya tautan langsung dan aliran tetap jalan.",
    ytdlpInstallHint: "Untuk memasang: ",
    ffmpegInstallHint: "Untuk memasang: ",

    groupAbout: "Tentang",
    appVersion: "Versi VDrop",
    engine: "Inti",
    engineHint: "Rust + Tauri 2. Tanpa ketergantungan pada Python atau yt-dlp.",
  },

  status: {
    queued: "Mengantre",
    downloading: "Mengunduh",
    paused: "Dijeda",
    retrying: "Mencoba lagi",
    completed: "Selesai",
    failed: "Gagal",
    cancelled: "Dibatalkan",
  },

  units: {
    perSecond: "/dtk",
    remaining: "tersisa",
    of: "/",
  },

  clipboard: {
    caught: "Ada tautan media di papan klip Anda",
    resolve: "Uraikan",
    dismiss: "Abaikan",
  },

  errors: {
    unknown: {
      title: "Terjadi sesuatu yang tak terduga",
      body: "Rincian di bawah.",
    },
    empty_url: {
      title: "Tempel tautan dulu",
      body: "Masukkan alamat halaman video, atau tautan media langsung.",
    },
    unsupported: {
      title: "Alamat ini tidak didukung",
      body: "VDrop tidak mengenali tautannya. Dengan yt-dlp, jauh lebih banyak situs yang jalan.",
    },
    network: {
      title: "Tidak bisa menghubungi server",
      body: "Apakah alamatnya benar dan koneksi Anda jalan?",
    },
    drm: {
      title: "Konten dilindungi DRM",
      body: "VDrop tidak bisa mengunduh aliran yang dilindungi DRM.",
    },
    parse: {
      title: "Tidak bisa membaca informasi media",
      body: "Server menjawab dengan sesuatu yang tak terduga.",
    },
    no_media: {
      title: "Tidak ada media yang bisa diunduh di halaman ini",
      body: "Kalau videonya dimuat lewat JavaScript, VDrop belum bisa melihatnya; coba tautan media langsung.",
    },
    ytdlp_missing: {
      title: "yt-dlp belum terpasang",
      body: "Unduhan ini memerlukan yt-dlp. Lihat Pengaturan > Komponen.",
    },
    ffmpeg_missing: {
      title: "FFmpeg belum terpasang",
      body: "Menyambung aliran HLS/DASH memerlukan FFmpeg. Lihat Pengaturan > Komponen.",
    },
    record_missing: {
      title: "Tidak bisa membuat catatan unduhan",
      body: "Catatan gagal ditulis ke basis data.",
    },
    internal: {
      title: "Terjadi sesuatu yang tak terduga",
      body: "Rincian di bawah.",
    },
  },

  common: {
    search: "Cari",
    searchPlaceholder: "Cari di judul dan alamat",
    clearSearch: "Bersihkan pencarian",
    noResults: "Tidak ada yang cocok",
    noResultsBody: "Coba pencarian lain.",
    cancel: "Batal",
    confirm: "Konfirmasi",
    close: "Tutup",
    unknown: "Tidak diketahui",
    audio: "Audio",
    video: "Video",
    stream: "Aliran",
    subtitle: "Subtitel",
    file: "Berkas",
  },
};

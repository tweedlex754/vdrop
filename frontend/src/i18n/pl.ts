import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const pl: Dictionary = {
  nav: {
    home: "Start",
    queue: "Kolejka",
    library: "Biblioteka",
    history: "Historia",
    settings: "Ustawienia",
    sections: "Sekcje",
    engineReady: "Silnik gotowy",
  },

  status_bar: {
    throughput: "Łączna prędkość",
    active: "Aktywne",
    pauseAll: "Wstrzymaj wszystko",
    resumeAll: "Wznów wszystko",
    clearFinished: "Wyczyść ukończone",
  },

  home: {
    title: "Rozwiąż link do wideo",
    videoAndAudio: "Wideo + dźwięk",
    audioOnly: "Tylko dźwięk",
    subtitlesOnly: "Napisy",
    noSubtitleTrack: "To źródło nie ma napisów",
    addToQueue: "Dodaj do kolejki",
    noAudioTrack: "To źródło nie ma osobnej ścieżki dźwiękowej",
    legacyTitle: "Wklej link",
    subtitle:
      "VDrop rozwiązuje link, a jakość wybierasz Ty. Nic się nie pobiera, dopóki nie powiesz.",
    placeholder: "https://... albo bezpośredni link do mediów",
    analyze: "Rozwiąż",
    paste: "Wklej ze schowka",
    analyzing: "Rozwiązywanie",
    download: "Pobierz",
    changeFolder: "Wybierz folder",
    savingTo: "Zapis w",
    streamNotice: "Strumień segmentowany",
    streamNoticeBody:
      "To strumień HLS/DASH. VDrop łączy segmenty przez FFmpeg bez ponownego kodowania. Strumienie można anulować, ale nie wstrzymać.",
    ffmpegMissing: "Nie znaleziono FFmpeg",
    ffmpegMissingBody:
      "Strumienie segmentowane (.m3u8 / .mpd) wymagają FFmpeg. Bezpośrednie linki działają bez niego.",
  },

  queue: {
    title: "Aktywna kolejka",
    completed: "Ukończone",
    itemsDownloading: "w toku",
    analyzing: "Analiza strumienia...",
    connecting: "Łączenie...",
    subtitle: "Trwające i ukończone pobierania",
    empty: "Kolejka jest pusta",
    emptyBody: "Rozwiąż link na stronie startowej, a pobierania pojawią się tutaj.",
    clearFinished: "Wyczyść ukończone",
    pause: "Wstrzymaj",
    resume: "Wznów",
    cancel: "Anuluj",
    remove: "Usuń z listy",
    openFolder: "Pokaż w folderze",
    openFile: "Otwórz plik",
    retry: "Spróbuj ponownie",
  },

  library: {
    title: "Biblioteka",
    subtitle: "Pobrane pliki",
    empty: "Biblioteka jest pusta",
    emptyBody: "Ukończone pobierania trafiają tu automatycznie.",
    missing: "Plik zniknął z dysku",
    removeEntry: "Usuń wpis",
    deleteFile: "Usuń plik",
  },

  history: {
    title: "Historia",
    subtitle: "Ukończone, anulowane i nieudane pobierania",
    empty: "Historia jest pusta",
    emptyBody: "Każde ukończone pobieranie zostawia tu ślad.",
    clear: "Wyczyść historię",
  },

  settings: {
    title: "Ustawienia",
    subtitle: "Preferencje zostają na tym komputerze",

    groupGeneral: "Ogólne",
    theme: "Motyw",
    themeHint: "Podążaj za motywem systemu albo ustaw jeden",
    themeSystem: "System",
    themeLight: "Jasny",
    themeDark: "Ciemny",
    language: "Język",
    languageHint: "Język interfejsu",

    groupDownloads: "Pobierania",
    folder: "Folder pobierania",
    folderHint: "Nowe pobierania trafiają tutaj",
    choose: "Zmień",
    concurrency: "Równoczesne pobierania",
    concurrencyHint: "Ile działa naraz; reszta czeka",
    bandwidth: "Limit prędkości",
    bandwidthHint: "Łączna prędkość wszystkich pobierań; 0 oznacza bez limitu",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Bez limitu",
    autoOpen: "Otwórz folder po zakończeniu",
    autoOpenHint: "Pokaż plik, gdy pobieranie się skończy",
    clipboard: "Obserwuj schowek",
    clipboardHint: "Wyłapuj linki do mediów przy kopiowaniu",
    notifications: "Powiadomienia",
    notificationsHint: "Pokaż powiadomienie systemowe po zakończeniu",

    navGeneral: "Ogólne",
    navDownloads: "Pobierania",
    navComponents: "Składniki",
    navAbout: "O programie",
    allComponentsOk: "Wszystkie składniki działają",
    someComponentsMissing: "Brakuje składnika",
    installed: "Zainstalowany",
    notInstalled: "Niezainstalowany",

    groupComponents: "Składniki",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Służy do łączenia strumieni segmentowanych (.m3u8 / .mpd)",
    ffmpegFound: "Zainstalowany",
    ffmpegNotFound: "Nie znaleziono",
    version: "Wersja",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Ekstrakcja zależna od strony. Z nim działają setki serwisów; bez niego nadal działają bezpośrednie linki i strumienie.",
    ytdlpInstallHint: "Instalacja: ",
    ffmpegInstallHint: "Instalacja: ",

    groupAbout: "O programie",
    appVersion: "Wersja VDrop",
    engine: "Rdzeń",
    engineHint: "Rust + Tauri 2. Bez zależności od Pythona i yt-dlp.",
  },

  status: {
    queued: "W kolejce",
    downloading: "Pobieranie",
    paused: "Wstrzymane",
    retrying: "Ponawianie",
    completed: "Gotowe",
    failed: "Nieudane",
    cancelled: "Anulowane",
  },

  units: {
    perSecond: "/s",
    remaining: "zostało",
    of: "/",
  },

  clipboard: {
    caught: "W schowku jest link do mediów",
    resolve: "Rozwiąż",
    dismiss: "Zignoruj",
  },

  errors: {
    unknown: {
      title: "Stało się coś nieoczekiwanego",
      body: "Szczegóły poniżej.",
    },
    empty_url: {
      title: "Najpierw wklej link",
      body: "Podaj adres strony z wideo albo bezpośredni link do mediów.",
    },
    unsupported: {
      title: "Ten adres nie jest obsługiwany",
      body: "VDrop nie rozpoznał linku. Z yt-dlp działa znacznie więcej serwisów.",
    },
    network: {
      title: "Nie udało się połączyć z serwerem",
      body: "Czy adres jest poprawny i czy połączenie działa?",
    },
    drm: {
      title: "Treść jest chroniona DRM",
      body: "VDrop nie pobiera strumieni chronionych DRM.",
    },
    parse: {
      title: "Nie udało się odczytać informacji o mediach",
      body: "Serwer odpowiedział czymś nieoczekiwanym.",
    },
    no_media: {
      title: "Brak mediów do pobrania na stronie",
      body: "Jeśli wideo ładuje się przez JavaScript, VDrop go jeszcze nie widzi; spróbuj bezpośredniego linku.",
    },
    ytdlp_missing: {
      title: "yt-dlp nie jest zainstalowany",
      body: "To pobieranie wymaga yt-dlp. Zobacz Ustawienia > Składniki.",
    },
    ffmpeg_missing: {
      title: "FFmpeg nie jest zainstalowany",
      body: "Łączenie strumieni HLS/DASH wymaga FFmpeg. Zobacz Ustawienia > Składniki.",
    },
    record_missing: {
      title: "Nie udało się utworzyć wpisu pobierania",
      body: "Wpisu nie udało się zapisać w bazie danych.",
    },
    internal: {
      title: "Stało się coś nieoczekiwanego",
      body: "Szczegóły poniżej.",
    },
  },

  common: {
    search: "Szukaj",
    searchPlaceholder: "Szukaj w tytule i adresie",
    clearSearch: "Wyczyść wyszukiwanie",
    noResults: "Brak pasujących wpisów",
    noResultsBody: "Spróbuj innego wyszukiwania.",
    cancel: "Anuluj",
    confirm: "Potwierdź",
    close: "Zamknij",
    unknown: "Nieznane",
    audio: "Dźwięk",
    video: "Wideo",
    stream: "Strumień",
    subtitle: "Napisy",
    file: "Plik",
  },
};

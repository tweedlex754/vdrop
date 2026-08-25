import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const de: Dictionary = {
  nav: {
    home: "Startseite",
    queue: "Warteschlange",
    library: "Bibliothek",
    history: "Verlauf",
    settings: "Einstellungen",
    sections: "Bereiche",
    engineReady: "Engine bereit",
  },

  status_bar: {
    throughput: "Gesamttempo",
    active: "Aktiv",
    pauseAll: "Alle pausieren",
    resumeAll: "Alle fortsetzen",
    clearFinished: "Fertige entfernen",
  },

  home: {
    title: "Videolink auflösen",
    videoAndAudio: "Video + Audio",
    audioOnly: "Nur Audio",
    subtitlesOnly: "Untertitel",
    noSubtitleTrack: "Diese Quelle hat keine Untertitel",
    addToQueue: "Zur Warteschlange",
    noAudioTrack: "Diese Quelle hat keine separate Audiospur",
    legacyTitle: "Link einfügen",
    subtitle:
      "VDrop löst den Link auf, die Qualität wählen Sie. Der Download startet erst auf Ihr Wort.",
    placeholder: "https://... oder ein direkter Medienlink",
    analyze: "Auflösen",
    paste: "Aus Zwischenablage einfügen",
    analyzing: "Wird aufgelöst",
    download: "Herunterladen",
    changeFolder: "Ordner wählen",
    savingTo: "Speicherort",
    streamNotice: "Segmentierter Stream",
    streamNoticeBody:
      "Das ist ein HLS/DASH-Stream. VDrop fügt die Segmente mit FFmpeg ohne Neukodierung zusammen. Streams lassen sich abbrechen, aber nicht pausieren.",
    ffmpegMissing: "FFmpeg nicht gefunden",
    ffmpegMissingBody:
      "Segmentierte Streams (.m3u8 / .mpd) brauchen FFmpeg. Direkte Dateilinks funktionieren auch ohne.",
  },

  queue: {
    title: "Aktive Warteschlange",
    completed: "Abgeschlossen",
    itemsDownloading: "werden geladen",
    analyzing: "Stream wird analysiert...",
    connecting: "Verbinden...",
    subtitle: "Laufende und abgeschlossene Downloads",
    empty: "Warteschlange ist leer",
    emptyBody: "Lösen Sie auf der Startseite einen Link auf, dann erscheinen Downloads hier.",
    clearFinished: "Fertige entfernen",
    pause: "Pause",
    resume: "Fortsetzen",
    cancel: "Abbrechen",
    remove: "Aus Liste entfernen",
    openFolder: "Im Ordner zeigen",
    openFile: "Datei öffnen",
    retry: "Erneut versuchen",
  },

  library: {
    title: "Bibliothek",
    subtitle: "Heruntergeladene Dateien",
    empty: "Bibliothek ist leer",
    emptyBody: "Abgeschlossene Downloads landen automatisch hier.",
    missing: "Datei ist von der Festplatte verschwunden",
    removeEntry: "Eintrag entfernen",
    deleteFile: "Datei löschen",
  },

  history: {
    title: "Verlauf",
    subtitle: "Abgeschlossene, abgebrochene und fehlgeschlagene Downloads",
    empty: "Verlauf ist leer",
    emptyBody: "Jeder abgeschlossene Download hinterlässt hier einen Eintrag.",
    clear: "Verlauf leeren",
  },

  settings: {
    title: "Einstellungen",
    subtitle: "Einstellungen bleiben auf diesem Rechner",

    groupGeneral: "Allgemein",
    theme: "Design",
    themeHint: "Dem Systemdesign folgen oder eines festlegen",
    themeSystem: "System",
    themeLight: "Hell",
    themeDark: "Dunkel",
    language: "Sprache",
    languageHint: "Oberflächensprache",

    groupDownloads: "Downloads",
    folder: "Download-Ordner",
    folderHint: "Neue Downloads landen hier",
    choose: "Ändern",
    concurrency: "Gleichzeitige Downloads",
    concurrencyHint: "Wie viele gleichzeitig laufen; der Rest wartet",
    bandwidth: "Tempolimit",
    bandwidthHint: "Gesamttempo aller Downloads; 0 bedeutet unbegrenzt",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Unbegrenzt",
    autoOpen: "Ordner nach Abschluss öffnen",
    autoOpenHint: "Die Datei zeigen, sobald ein Download fertig ist",
    clipboard: "Zwischenablage beobachten",
    clipboardHint: "Medienlinks beim Kopieren auffangen",
    notifications: "Benachrichtigungen",
    notificationsHint: "Eine Systemmeldung zeigen, wenn ein Download fertig ist",

    navGeneral: "Allgemein",
    navDownloads: "Downloads",
    navComponents: "Komponenten",
    navAbout: "Über",
    allComponentsOk: "Alle Komponenten laufen",
    someComponentsMissing: "Eine Komponente fehlt",
    installed: "Installiert",
    notInstalled: "Nicht installiert",

    groupComponents: "Komponenten",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Fügt segmentierte Streams (.m3u8 / .mpd) zusammen",
    ffmpegFound: "Installiert",
    ffmpegNotFound: "Nicht gefunden",
    version: "Version",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Seitenspezifische Extraktion. Damit funktionieren Hunderte Seiten; ohne sie weiterhin direkte Links und Streams.",
    ytdlpInstallHint: "Installation: ",
    ffmpegInstallHint: "Installation: ",

    groupAbout: "Über",
    appVersion: "VDrop-Version",
    engine: "Kern",
    engineHint: "Rust + Tauri 2. Keine Abhängigkeit von Python oder yt-dlp.",
  },

  status: {
    queued: "In Warteschlange",
    downloading: "Lädt",
    paused: "Pausiert",
    retrying: "Neuer Versuch",
    completed: "Fertig",
    failed: "Fehlgeschlagen",
    cancelled: "Abgebrochen",
  },

  units: {
    perSecond: "/s",
    remaining: "übrig",
    of: "/",
  },

  clipboard: {
    caught: "Ein Medienlink liegt in der Zwischenablage",
    resolve: "Auflösen",
    dismiss: "Ignorieren",
  },

  errors: {
    unknown: {
      title: "Etwas Unerwartetes ist passiert",
      body: "Details unten.",
    },
    empty_url: {
      title: "Zuerst einen Link einfügen",
      body: "Geben Sie die Adresse einer Videoseite oder einen direkten Medienlink ein.",
    },
    unsupported: {
      title: "Diese Adresse wird nicht unterstützt",
      body: "VDrop hat den Link nicht erkannt. Mit yt-dlp funktionieren deutlich mehr Seiten.",
    },
    network: {
      title: "Server nicht erreichbar",
      body: "Stimmt die Adresse, und funktioniert Ihre Verbindung?",
    },
    drm: {
      title: "Der Inhalt ist DRM-geschützt",
      body: "VDrop kann DRM-geschützte Streams nicht herunterladen.",
    },
    parse: {
      title: "Medieninformationen nicht lesbar",
      body: "Der Server hat unerwartet geantwortet.",
    },
    no_media: {
      title: "Keine herunterladbaren Medien auf der Seite",
      body: "Wenn das Video per JavaScript lädt, sieht VDrop es noch nicht; versuchen Sie den direkten Medienlink.",
    },
    ytdlp_missing: {
      title: "yt-dlp ist nicht installiert",
      body: "Dieser Download braucht yt-dlp. Siehe Einstellungen > Komponenten.",
    },
    ffmpeg_missing: {
      title: "FFmpeg ist nicht installiert",
      body: "HLS/DASH-Streams zusammenzufügen braucht FFmpeg. Siehe Einstellungen > Komponenten.",
    },
    record_missing: {
      title: "Download-Eintrag konnte nicht angelegt werden",
      body: "Der Eintrag konnte nicht in die Datenbank geschrieben werden.",
    },
    internal: {
      title: "Etwas Unerwartetes ist passiert",
      body: "Details unten.",
    },
  },

  common: {
    search: "Suchen",
    searchPlaceholder: "Titel und Adresse durchsuchen",
    clearSearch: "Suche löschen",
    noResults: "Keine passenden Einträge",
    noResultsBody: "Versuchen Sie eine andere Suche.",
    cancel: "Abbrechen",
    confirm: "Bestätigen",
    close: "Schließen",
    unknown: "Unbekannt",
    audio: "Audio",
    video: "Video",
    stream: "Stream",
    subtitle: "Untertitel",
    file: "Datei",
  },
};

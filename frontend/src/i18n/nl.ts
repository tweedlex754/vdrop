import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const nl: Dictionary = {
  nav: {
    home: "Start",
    queue: "Wachtrij",
    library: "Bibliotheek",
    history: "Geschiedenis",
    settings: "Instellingen",
    sections: "Secties",
    engineReady: "Motor gereed",
  },

  status_bar: {
    throughput: "Totale snelheid",
    active: "Actief",
    pauseAll: "Alles pauzeren",
    resumeAll: "Alles hervatten",
    clearFinished: "Voltooide wissen",
  },

  home: {
    title: "Een videolink oplossen",
    videoAndAudio: "Video + audio",
    audioOnly: "Alleen audio",
    subtitlesOnly: "Ondertitels",
    noSubtitleTrack: "Deze bron heeft geen ondertitels",
    addToQueue: "Aan wachtrij toevoegen",
    noAudioTrack: "Deze bron heeft geen aparte audiotrack",
    legacyTitle: "Plak een link",
    subtitle:
      "VDrop lost de link op, jij kiest de kwaliteit. Er wordt niets gedownload tot jij het zegt.",
    placeholder: "https://... of een directe medialink",
    analyze: "Oplossen",
    paste: "Plakken vanaf klembord",
    analyzing: "Bezig met oplossen",
    download: "Downloaden",
    changeFolder: "Map kiezen",
    savingTo: "Opslaan in",
    streamNotice: "Gesegmenteerde stream",
    streamNoticeBody:
      "Dit is een HLS/DASH-stream. VDrop voegt de segmenten samen met FFmpeg zonder te hercoderen. Streams kun je annuleren, maar niet pauzeren.",
    ffmpegMissing: "FFmpeg niet gevonden",
    ffmpegMissingBody:
      "Gesegmenteerde streams (.m3u8 / .mpd) hebben FFmpeg nodig. Directe bestandslinks werken zonder.",
  },

  queue: {
    title: "Actieve wachtrij",
    completed: "Voltooid",
    itemsDownloading: "bezig",
    analyzing: "Stream analyseren...",
    connecting: "Verbinden...",
    subtitle: "Lopende en voltooide downloads",
    empty: "De wachtrij is leeg",
    emptyBody: "Los een link op via Start, dan verschijnen downloads hier.",
    clearFinished: "Voltooide wissen",
    pause: "Pauzeren",
    resume: "Hervatten",
    cancel: "Annuleren",
    remove: "Uit lijst verwijderen",
    openFolder: "In map tonen",
    openFile: "Bestand openen",
    retry: "Opnieuw proberen",
  },

  library: {
    title: "Bibliotheek",
    subtitle: "Gedownloade bestanden",
    empty: "De bibliotheek is leeg",
    emptyBody: "Voltooide downloads komen hier automatisch terecht.",
    missing: "Bestand staat niet meer op de schijf",
    removeEntry: "Vermelding verwijderen",
    deleteFile: "Bestand verwijderen",
  },

  history: {
    title: "Geschiedenis",
    subtitle: "Voltooide, geannuleerde en mislukte downloads",
    empty: "De geschiedenis is leeg",
    emptyBody: "Elke voltooide download laat hier een spoor achter.",
    clear: "Geschiedenis wissen",
  },

  settings: {
    title: "Instellingen",
    subtitle: "Voorkeuren blijven op deze computer",

    groupGeneral: "Algemeen",
    theme: "Thema",
    themeHint: "Het systeemthema volgen of er een vastzetten",
    themeSystem: "Systeem",
    themeLight: "Licht",
    themeDark: "Donker",
    language: "Taal",
    languageHint: "Taal van de interface",

    groupDownloads: "Downloads",
    folder: "Downloadmap",
    folderHint: "Nieuwe downloads worden hier opgeslagen",
    choose: "Wijzigen",
    concurrency: "Gelijktijdige downloads",
    concurrencyHint: "Hoeveel er tegelijk lopen; de rest wacht",
    bandwidth: "Snelheidslimiet",
    bandwidthHint: "Totale snelheid over alle downloads; 0 betekent onbeperkt",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Onbeperkt",
    autoOpen: "Map openen als klaar",
    autoOpenHint: "Het bestand tonen zodra een download klaar is",
    clipboard: "Klembord in de gaten houden",
    clipboardHint: "Medialinks opvangen terwijl je ze kopieert",
    notifications: "Meldingen",
    notificationsHint: "Een systeemmelding tonen als een download klaar is",

    navGeneral: "Algemeen",
    navDownloads: "Downloads",
    navComponents: "Onderdelen",
    navAbout: "Over",
    allComponentsOk: "Alle onderdelen werken",
    someComponentsMissing: "Er ontbreekt een onderdeel",
    installed: "Geïnstalleerd",
    notInstalled: "Niet geïnstalleerd",

    groupComponents: "Onderdelen",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Wordt gebruikt om gesegmenteerde streams (.m3u8 / .mpd) samen te voegen",
    ffmpegFound: "Geïnstalleerd",
    ffmpegNotFound: "Niet gevonden",
    version: "Versie",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Site-specifieke extractie. Daarmee werken honderden sites; zonder blijven directe links en streams werken.",
    ytdlpInstallHint: "Installeren: ",
    ffmpegInstallHint: "Installeren: ",

    groupAbout: "Over",
    appVersion: "VDrop-versie",
    engine: "Kern",
    engineHint: "Rust + Tauri 2. Geen afhankelijkheid van Python of yt-dlp.",
  },

  status: {
    queued: "In wachtrij",
    downloading: "Bezig",
    paused: "Gepauzeerd",
    retrying: "Opnieuw proberen",
    completed: "Klaar",
    failed: "Mislukt",
    cancelled: "Geannuleerd",
  },

  units: {
    perSecond: "/s",
    remaining: "resterend",
    of: "/",
  },

  clipboard: {
    caught: "Er staat een medialink op je klembord",
    resolve: "Oplossen",
    dismiss: "Negeren",
  },

  errors: {
    unknown: {
      title: "Er ging iets onverwachts mis",
      body: "Details hieronder.",
    },
    empty_url: {
      title: "Plak eerst een link",
      body: "Voer het adres van een videopagina of een directe medialink in.",
    },
    unsupported: {
      title: "Dit adres wordt niet ondersteund",
      body: "VDrop herkende de link niet. Met yt-dlp werken veel meer sites.",
    },
    network: {
      title: "Kon de server niet bereiken",
      body: "Klopt het adres, en werkt je verbinding?",
    },
    drm: {
      title: "De inhoud is DRM-beveiligd",
      body: "VDrop kan DRM-beveiligde streams niet downloaden.",
    },
    parse: {
      title: "Kon de media-informatie niet lezen",
      body: "De server antwoordde iets onverwachts.",
    },
    no_media: {
      title: "Geen downloadbare media op de pagina",
      body: "Als de video via JavaScript laadt, ziet VDrop hem nog niet; probeer de directe medialink.",
    },
    ytdlp_missing: {
      title: "yt-dlp is niet geïnstalleerd",
      body: "Deze download heeft yt-dlp nodig. Zie Instellingen > Onderdelen.",
    },
    ffmpeg_missing: {
      title: "FFmpeg is niet geïnstalleerd",
      body: "HLS/DASH-streams samenvoegen heeft FFmpeg nodig. Zie Instellingen > Onderdelen.",
    },
    record_missing: {
      title: "Kon het downloadrecord niet aanmaken",
      body: "Het record kon niet naar de database worden geschreven.",
    },
    internal: {
      title: "Er ging iets onverwachts mis",
      body: "Details hieronder.",
    },
  },

  common: {
    search: "Zoeken",
    searchPlaceholder: "Zoeken in titel en adres",
    clearSearch: "Zoekopdracht wissen",
    noResults: "Geen resultaten",
    noResultsBody: "Probeer een andere zoekopdracht.",
    cancel: "Annuleren",
    confirm: "Bevestigen",
    close: "Sluiten",
    unknown: "Onbekend",
    audio: "Audio",
    video: "Video",
    stream: "Stream",
    subtitle: "Ondertitel",
    file: "Bestand",
  },
};

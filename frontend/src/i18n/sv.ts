import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const sv: Dictionary = {
  nav: {
    home: "Start",
    queue: "Kö",
    library: "Bibliotek",
    history: "Historik",
    settings: "Inställningar",
    sections: "Avsnitt",
    engineReady: "Motorn redo",
  },

  status_bar: {
    throughput: "Total hastighet",
    active: "Aktiva",
    pauseAll: "Pausa alla",
    resumeAll: "Återuppta alla",
    clearFinished: "Rensa färdiga",
  },

  home: {
    title: "Lös upp en videolänk",
    videoAndAudio: "Video + ljud",
    audioOnly: "Endast ljud",
    subtitlesOnly: "Undertexter",
    noSubtitleTrack: "Den här källan har inga undertexter",
    addToQueue: "Lägg i kön",
    noAudioTrack: "Den här källan har inget separat ljudspår",
    legacyTitle: "Klistra in en länk",
    subtitle:
      "VDrop löser upp länken och du väljer kvaliteten. Inget laddas ner förrän du säger till.",
    placeholder: "https://... eller en direkt medielänk",
    analyze: "Lös upp",
    paste: "Klistra in från urklipp",
    analyzing: "Löser upp",
    download: "Ladda ner",
    changeFolder: "Välj mapp",
    savingTo: "Spara i",
    streamNotice: "Segmenterad ström",
    streamNoticeBody:
      "Detta är en HLS/DASH-ström. VDrop fogar ihop segmenten med FFmpeg utan omkodning. Sådana nedladdningar går att avbryta men inte pausa.",
    ffmpegMissing: "FFmpeg hittades inte",
    ffmpegMissingBody:
      "Segmenterade strömmar (.m3u8 / .mpd) kräver FFmpeg. Direkta fillänkar fungerar ändå.",
  },

  queue: {
    title: "Aktiv kö",
    completed: "Färdiga",
    itemsDownloading: "laddas ner",
    analyzing: "Analyserar strömmen...",
    connecting: "Ansluter...",
    subtitle: "Pågående och färdiga nedladdningar",
    empty: "Kön är tom",
    emptyBody: "Lös upp en länk på startsidan så dyker nedladdningarna upp här.",
    clearFinished: "Rensa färdiga",
    pause: "Pausa",
    resume: "Återuppta",
    cancel: "Avbryt",
    remove: "Ta bort från listan",
    openFolder: "Visa i mappen",
    openFile: "Öppna filen",
    retry: "Försök igen",
  },

  library: {
    title: "Bibliotek",
    subtitle: "Nedladdade filer",
    empty: "Biblioteket är tomt",
    emptyBody: "Färdiga nedladdningar hamnar här automatiskt.",
    missing: "Filen finns inte kvar på disken",
    removeEntry: "Ta bort posten",
    deleteFile: "Radera filen",
  },

  history: {
    title: "Historik",
    subtitle: "Färdiga, avbrutna och misslyckade nedladdningar",
    empty: "Historiken är tom",
    emptyBody: "Varje färdig nedladdning lämnar ett spår här.",
    clear: "Rensa historiken",
  },

  settings: {
    title: "Inställningar",
    subtitle: "Inställningarna stannar på den här datorn",

    groupGeneral: "Allmänt",
    theme: "Tema",
    themeHint: "Följ systemets tema eller lås ett",
    themeSystem: "System",
    themeLight: "Ljust",
    themeDark: "Mörkt",
    language: "Språk",
    languageHint: "Gränssnittets språk",

    groupDownloads: "Nedladdningar",
    folder: "Nedladdningsmapp",
    folderHint: "Nya nedladdningar sparas här",
    choose: "Ändra",
    concurrency: "Samtidiga nedladdningar",
    concurrencyHint: "Hur många som körs samtidigt; resten väntar",
    bandwidth: "Hastighetsgräns",
    bandwidthHint: "Total hastighet för alla nedladdningar; 0 betyder obegränsat",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Obegränsat",
    autoOpen: "Öppna mappen när det är klart",
    autoOpenHint: "Visa filen när en nedladdning är klar",
    clipboard: "Bevaka urklipp",
    clipboardHint: "Fånga medielänkar när du kopierar dem",
    notifications: "Aviseringar",
    notificationsHint: "Visa en systemavisering när en nedladdning är klar",

    navGeneral: "Allmänt",
    navDownloads: "Nedladdningar",
    navComponents: "Komponenter",
    navAbout: "Om",
    allComponentsOk: "Alla komponenter fungerar",
    someComponentsMissing: "En komponent saknas",
    installed: "Installerad",
    notInstalled: "Inte installerad",

    groupComponents: "Komponenter",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Används för att foga ihop segmenterade strömmar (.m3u8 / .mpd)",
    ffmpegFound: "Installerad",
    ffmpegNotFound: "Hittades inte",
    version: "Version",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Platsspecifik extrahering. Med den fungerar hundratals sajter; utan den fungerar direktlänkar och strömmar ändå.",
    ytdlpInstallHint: "Installera: ",
    ffmpegInstallHint: "Installera: ",

    groupAbout: "Om",
    appVersion: "VDrop-version",
    engine: "Kärna",
    engineHint: "Rust + Tauri 2. Inget beroende av Python eller yt-dlp.",
  },

  status: {
    queued: "I kö",
    downloading: "Laddas ner",
    paused: "Pausad",
    retrying: "Försöker igen",
    completed: "Klar",
    failed: "Misslyckades",
    cancelled: "Avbruten",
  },

  units: {
    perSecond: "/s",
    remaining: "kvar",
    of: "/",
  },

  clipboard: {
    caught: "Det finns en medielänk i urklipp",
    resolve: "Lös upp",
    dismiss: "Ignorera",
  },

  errors: {
    unknown: {
      title: "Något oväntat hände",
      body: "Detaljer nedan.",
    },
    empty_url: {
      title: "Klistra in en länk först",
      body: "Ange adressen till en videosida, eller en direkt medielänk.",
    },
    unsupported: {
      title: "Adressen stöds inte",
      body: "VDrop kände inte igen länken. Med yt-dlp fungerar betydligt fler sajter.",
    },
    network: {
      title: "Kunde inte nå servern",
      body: "Stämmer adressen, och fungerar din uppkoppling?",
    },
    drm: {
      title: "Innehållet är DRM-skyddat",
      body: "VDrop kan inte ladda ner DRM-skyddade strömmar.",
    },
    parse: {
      title: "Kunde inte läsa medieinformationen",
      body: "Servern svarade med något oväntat.",
    },
    no_media: {
      title: "Ingen nedladdningsbar media på sidan",
      body: "Om videon laddas via JavaScript ser VDrop den inte än; prova den direkta medielänken.",
    },
    ytdlp_missing: {
      title: "yt-dlp är inte installerat",
      body: "Den här nedladdningen kräver yt-dlp. Se Inställningar > Komponenter.",
    },
    ffmpeg_missing: {
      title: "FFmpeg är inte installerat",
      body: "Att foga ihop HLS/DASH-strömmar kräver FFmpeg. Se Inställningar > Komponenter.",
    },
    record_missing: {
      title: "Kunde inte skapa nedladdningsposten",
      body: "Posten kunde inte skrivas till databasen.",
    },
    internal: {
      title: "Något oväntat hände",
      body: "Detaljer nedan.",
    },
  },

  common: {
    search: "Sök",
    searchPlaceholder: "Sök i titel och adress",
    clearSearch: "Rensa sökningen",
    noResults: "Inga träffar",
    noResultsBody: "Prova en annan sökning.",
    cancel: "Avbryt",
    confirm: "Bekräfta",
    close: "Stäng",
    unknown: "Okänt",
    audio: "Ljud",
    video: "Video",
    stream: "Ström",
    subtitle: "Undertext",
    file: "Fil",
  },
};

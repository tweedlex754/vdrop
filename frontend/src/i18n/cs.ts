import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const cs: Dictionary = {
  nav: {
    home: "Domů",
    queue: "Fronta",
    library: "Knihovna",
    history: "Historie",
    settings: "Nastavení",
    sections: "Sekce",
    engineReady: "Jádro připraveno",
  },

  status_bar: {
    throughput: "Celková rychlost",
    active: "Aktivní",
    pauseAll: "Pozastavit vše",
    resumeAll: "Obnovit vše",
    clearFinished: "Vymazat dokončené",
  },

  home: {
    title: "Zpracovat odkaz na video",
    videoAndAudio: "Video + zvuk",
    audioOnly: "Jen zvuk",
    subtitlesOnly: "Titulky",
    noSubtitleTrack: "Tento zdroj nemá titulky",
    addToQueue: "Přidat do fronty",
    noAudioTrack: "Tento zdroj nemá samostatnou zvukovou stopu",
    legacyTitle: "Vložte odkaz",
    subtitle:
      "VDrop zpracuje odkaz a kvalitu si vyberete vy. Nic se nestahuje, dokud neřeknete.",
    placeholder: "https://... nebo přímý odkaz na média",
    analyze: "Zpracovat",
    paste: "Vložit ze schránky",
    analyzing: "Zpracovávám",
    download: "Stáhnout",
    changeFolder: "Vybrat složku",
    savingTo: "Uložit do",
    streamNotice: "Segmentovaný stream",
    streamNoticeBody:
      "Toto je stream HLS/DASH. VDrop spojí segmenty přes FFmpeg bez překódování. Takové stahování lze zrušit, ale ne pozastavit.",
    ffmpegMissing: "FFmpeg nenalezen",
    ffmpegMissingBody:
      "Segmentované streamy (.m3u8 / .mpd) potřebují FFmpeg. Přímé odkazy fungují i bez něj.",
  },

  queue: {
    title: "Aktivní fronta",
    completed: "Dokončené",
    itemsDownloading: "stahuje se",
    analyzing: "Analyzuji stream...",
    connecting: "Připojuji...",
    subtitle: "Probíhající a dokončená stahování",
    empty: "Fronta je prázdná",
    emptyBody: "Zpracujte odkaz na domovské stránce a stahování se objeví tady.",
    clearFinished: "Vymazat dokončené",
    pause: "Pozastavit",
    resume: "Obnovit",
    cancel: "Zrušit",
    remove: "Odebrat ze seznamu",
    openFolder: "Zobrazit ve složce",
    openFile: "Otevřít soubor",
    retry: "Zkusit znovu",
  },

  library: {
    title: "Knihovna",
    subtitle: "Stažené soubory",
    empty: "Knihovna je prázdná",
    emptyBody: "Dokončená stahování sem přibývají automaticky.",
    missing: "Soubor už na disku není",
    removeEntry: "Odebrat záznam",
    deleteFile: "Smazat soubor",
  },

  history: {
    title: "Historie",
    subtitle: "Dokončená, zrušená a neúspěšná stahování",
    empty: "Historie je prázdná",
    emptyBody: "Každé dokončené stahování tu nechá stopu.",
    clear: "Vymazat historii",
  },

  settings: {
    title: "Nastavení",
    subtitle: "Předvolby zůstávají v tomto počítači",

    groupGeneral: "Obecné",
    theme: "Motiv",
    themeHint: "Řídit se motivem systému nebo jeden zvolit",
    themeSystem: "Systém",
    themeLight: "Světlý",
    themeDark: "Tmavý",
    language: "Jazyk",
    languageHint: "Jazyk rozhraní",

    groupDownloads: "Stahování",
    folder: "Složka stahování",
    folderHint: "Nová stahování se ukládají sem",
    choose: "Změnit",
    concurrency: "Souběžná stahování",
    concurrencyHint: "Kolik jich běží najednou; ostatní čekají",
    bandwidth: "Omezení rychlosti",
    bandwidthHint: "Celková rychlost všech stahování; 0 znamená bez omezení",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Bez omezení",
    autoOpen: "Po dokončení otevřít složku",
    autoOpenHint: "Ukázat soubor, jakmile stahování skončí",
    clipboard: "Sledovat schránku",
    clipboardHint: "Zachytávat odkazy na média při kopírování",
    notifications: "Oznámení",
    notificationsHint: "Po dokončení zobrazit systémové oznámení",

    navGeneral: "Obecné",
    navDownloads: "Stahování",
    navComponents: "Součásti",
    navAbout: "O aplikaci",
    allComponentsOk: "Všechny součásti fungují",
    someComponentsMissing: "Chybí součást",
    installed: "Nainstalováno",
    notInstalled: "Nenainstalováno",

    groupComponents: "Součásti",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Slouží ke spojování segmentovaných streamů (.m3u8 / .mpd)",
    ffmpegFound: "Nainstalováno",
    ffmpegNotFound: "Nenalezeno",
    version: "Verze",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Extrakce podle konkrétních webů. S ním funguje stovky webů; bez něj zůstávají přímé odkazy a streamy.",
    ytdlpInstallHint: "Instalace: ",
    ffmpegInstallHint: "Instalace: ",

    groupAbout: "O aplikaci",
    appVersion: "Verze VDrop",
    engine: "Jádro",
    engineHint: "Rust + Tauri 2. Bez závislosti na Pythonu či yt-dlp.",
  },

  status: {
    queued: "Ve frontě",
    downloading: "Stahuje se",
    paused: "Pozastaveno",
    retrying: "Zkouším znovu",
    completed: "Hotovo",
    failed: "Selhalo",
    cancelled: "Zrušeno",
  },

  units: {
    perSecond: "/s",
    remaining: "zbývá",
    of: "/",
  },

  clipboard: {
    caught: "Ve schránce je odkaz na média",
    resolve: "Zpracovat",
    dismiss: "Ignorovat",
  },

  errors: {
    unknown: {
      title: "Stalo se něco nečekaného",
      body: "Podrobnosti níže.",
    },
    empty_url: {
      title: "Nejdřív vložte odkaz",
      body: "Zadejte adresu stránky s videem nebo přímý odkaz na média.",
    },
    unsupported: {
      title: "Tato adresa není podporována",
      body: "VDrop odkaz nerozpoznal. S yt-dlp funguje mnohem víc webů.",
    },
    network: {
      title: "Server se nepodařilo kontaktovat",
      body: "Je adresa správná a funguje vaše připojení?",
    },
    drm: {
      title: "Obsah je chráněn DRM",
      body: "VDrop neumí stahovat streamy chráněné DRM.",
    },
    parse: {
      title: "Informace o médiu se nepodařilo přečíst",
      body: "Server odpověděl něčím nečekaným.",
    },
    no_media: {
      title: "Na stránce není nic ke stažení",
      body: "Pokud se video načítá přes JavaScript, VDrop ho zatím nevidí; zkuste přímý odkaz.",
    },
    ytdlp_missing: {
      title: "yt-dlp není nainstalován",
      body: "Toto stahování potřebuje yt-dlp. Viz Nastavení > Součásti.",
    },
    ffmpeg_missing: {
      title: "FFmpeg není nainstalován",
      body: "Spojení streamů HLS/DASH potřebuje FFmpeg. Viz Nastavení > Součásti.",
    },
    record_missing: {
      title: "Záznam o stahování se nepodařilo vytvořit",
      body: "Záznam se nepodařilo zapsat do databáze.",
    },
    internal: {
      title: "Stalo se něco nečekaného",
      body: "Podrobnosti níže.",
    },
  },

  common: {
    search: "Hledat",
    searchPlaceholder: "Hledat v názvu a adrese",
    clearSearch: "Vymazat hledání",
    noResults: "Žádné odpovídající položky",
    noResultsBody: "Zkuste jiné hledání.",
    cancel: "Zrušit",
    confirm: "Potvrdit",
    close: "Zavřít",
    unknown: "Neznámé",
    audio: "Zvuk",
    video: "Video",
    stream: "Stream",
    subtitle: "Titulky",
    file: "Soubor",
  },
};

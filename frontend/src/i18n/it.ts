import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const it: Dictionary = {
  nav: {
    home: "Home",
    queue: "Coda",
    library: "Libreria",
    history: "Cronologia",
    settings: "Impostazioni",
    sections: "Sezioni",
    engineReady: "Motore pronto",
  },

  status_bar: {
    throughput: "Velocità totale",
    active: "Attivi",
    pauseAll: "Metti tutto in pausa",
    resumeAll: "Riprendi tutto",
    clearFinished: "Rimuovi completati",
  },

  home: {
    title: "Risolvi un link video",
    videoAndAudio: "Video + audio",
    audioOnly: "Solo audio",
    subtitlesOnly: "Sottotitoli",
    noSubtitleTrack: "Questa fonte non ha sottotitoli",
    addToQueue: "Aggiungi alla coda",
    noAudioTrack: "Questa fonte non ha una traccia audio separata",
    legacyTitle: "Incolla un link",
    subtitle:
      "VDrop risolve il link e la qualità la scegli tu. Niente viene scaricato finché non lo dici.",
    placeholder: "https://... o un link multimediale diretto",
    analyze: "Risolvi",
    paste: "Incolla dagli appunti",
    analyzing: "Risoluzione in corso",
    download: "Scarica",
    changeFolder: "Scegli cartella",
    savingTo: "Salva in",
    streamNotice: "Flusso segmentato",
    streamNoticeBody:
      "Questo è un flusso HLS/DASH. VDrop unisce i segmenti con FFmpeg senza ricodificare. I flussi si possono annullare, ma non mettere in pausa.",
    ffmpegMissing: "FFmpeg non trovato",
    ffmpegMissingBody:
      "I flussi segmentati (.m3u8 / .mpd) richiedono FFmpeg. I link diretti funzionano anche senza.",
  },

  queue: {
    title: "Coda attiva",
    completed: "Completati",
    itemsDownloading: "in corso",
    analyzing: "Analisi del flusso...",
    connecting: "Connessione...",
    subtitle: "Download in corso e completati",
    empty: "La coda è vuota",
    emptyBody: "Risolvi un link nella Home e i download compariranno qui.",
    clearFinished: "Rimuovi completati",
    pause: "Pausa",
    resume: "Riprendi",
    cancel: "Annulla",
    remove: "Rimuovi dalla lista",
    openFolder: "Mostra nella cartella",
    openFile: "Apri file",
    retry: "Riprova",
  },

  library: {
    title: "Libreria",
    subtitle: "File scaricati",
    empty: "La libreria è vuota",
    emptyBody: "I download completati finiscono qui automaticamente.",
    missing: "Il file non è più sul disco",
    removeEntry: "Rimuovi voce",
    deleteFile: "Elimina file",
  },

  history: {
    title: "Cronologia",
    subtitle: "Download completati, annullati e falliti",
    empty: "La cronologia è vuota",
    emptyBody: "Ogni download completato lascia una traccia qui.",
    clear: "Cancella cronologia",
  },

  settings: {
    title: "Impostazioni",
    subtitle: "Le preferenze restano su questo computer",

    groupGeneral: "Generale",
    theme: "Tema",
    themeHint: "Segui il tema di sistema o fissane uno",
    themeSystem: "Sistema",
    themeLight: "Chiaro",
    themeDark: "Scuro",
    language: "Lingua",
    languageHint: "Lingua dell interfaccia",

    groupDownloads: "Download",
    folder: "Cartella dei download",
    folderHint: "I nuovi download vengono salvati qui",
    choose: "Cambia",
    concurrency: "Download simultanei",
    concurrencyHint: "Quanti girano insieme; gli altri aspettano",
    bandwidth: "Limite di velocità",
    bandwidthHint: "Velocità totale di tutti i download; 0 significa senza limite",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Senza limite",
    autoOpen: "Apri la cartella al termine",
    autoOpenHint: "Mostra il file quando il download finisce",
    clipboard: "Controlla gli appunti",
    clipboardHint: "Cattura i link multimediali mentre li copi",
    notifications: "Notifiche",
    notificationsHint: "Mostra una notifica di sistema al termine",

    navGeneral: "Generale",
    navDownloads: "Download",
    navComponents: "Componenti",
    navAbout: "Informazioni",
    allComponentsOk: "Tutti i componenti funzionano",
    someComponentsMissing: "Manca un componente",
    installed: "Installato",
    notInstalled: "Non installato",

    groupComponents: "Componenti",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Serve a unire i flussi segmentati (.m3u8 / .mpd)",
    ffmpegFound: "Installato",
    ffmpegNotFound: "Non trovato",
    version: "Versione",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Estrazione specifica per sito. Con esso funzionano centinaia di siti; senza, restano link diretti e flussi.",
    ytdlpInstallHint: "Per installare: ",
    ffmpegInstallHint: "Per installare: ",

    groupAbout: "Informazioni",
    appVersion: "Versione di VDrop",
    engine: "Nucleo",
    engineHint: "Rust + Tauri 2. Nessuna dipendenza da Python o yt-dlp.",
  },

  status: {
    queued: "In coda",
    downloading: "In download",
    paused: "In pausa",
    retrying: "Nuovo tentativo",
    completed: "Fatto",
    failed: "Fallito",
    cancelled: "Annullato",
  },

  units: {
    perSecond: "/s",
    remaining: "rimanenti",
    of: "/",
  },

  clipboard: {
    caught: "Negli appunti c è un link multimediale",
    resolve: "Risolvi",
    dismiss: "Ignora",
  },

  errors: {
    unknown: {
      title: "È successo qualcosa di inatteso",
      body: "Dettagli sotto.",
    },
    empty_url: {
      title: "Prima incolla un link",
      body: "Inserisci l indirizzo di una pagina video o un link multimediale diretto.",
    },
    unsupported: {
      title: "Questo indirizzo non è supportato",
      body: "VDrop non ha riconosciuto il link. Con yt-dlp funzionano molti più siti.",
    },
    network: {
      title: "Impossibile raggiungere il server",
      body: "L indirizzo è giusto e la connessione funziona?",
    },
    drm: {
      title: "Il contenuto è protetto da DRM",
      body: "VDrop non può scaricare flussi protetti da DRM.",
    },
    parse: {
      title: "Impossibile leggere le informazioni del media",
      body: "Il server ha risposto in modo inatteso.",
    },
    no_media: {
      title: "Nessun media scaricabile nella pagina",
      body: "Se il video si carica via JavaScript, VDrop non lo vede ancora; prova con il link diretto.",
    },
    ytdlp_missing: {
      title: "yt-dlp non è installato",
      body: "Questo download richiede yt-dlp. Vedi Impostazioni > Componenti.",
    },
    ffmpeg_missing: {
      title: "FFmpeg non è installato",
      body: "Unire flussi HLS/DASH richiede FFmpeg. Vedi Impostazioni > Componenti.",
    },
    record_missing: {
      title: "Impossibile creare il record del download",
      body: "Il record non è stato scritto nel database.",
    },
    internal: {
      title: "È successo qualcosa di inatteso",
      body: "Dettagli sotto.",
    },
  },

  common: {
    search: "Cerca",
    searchPlaceholder: "Cerca nel titolo e nell indirizzo",
    clearSearch: "Cancella ricerca",
    noResults: "Nessun risultato",
    noResultsBody: "Prova un altra ricerca.",
    cancel: "Annulla",
    confirm: "Conferma",
    close: "Chiudi",
    unknown: "Sconosciuto",
    audio: "Audio",
    video: "Video",
    stream: "Flusso",
    subtitle: "Sottotitolo",
    file: "File",
  },
};

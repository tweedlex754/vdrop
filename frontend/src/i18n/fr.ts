import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const fr: Dictionary = {
  nav: {
    home: "Accueil",
    queue: "File d attente",
    library: "Bibliothèque",
    history: "Historique",
    settings: "Paramètres",
    sections: "Sections",
    engineReady: "Moteur prêt",
  },

  status_bar: {
    throughput: "Débit total",
    active: "Actifs",
    pauseAll: "Tout mettre en pause",
    resumeAll: "Tout reprendre",
    clearFinished: "Effacer les terminés",
  },

  home: {
    title: "Résoudre un lien vidéo",
    videoAndAudio: "Vidéo + audio",
    audioOnly: "Audio seul",
    subtitlesOnly: "Sous-titres",
    noSubtitleTrack: "Cette source n a pas de sous-titres",
    addToQueue: "Ajouter à la file",
    noAudioTrack: "Cette source n a pas de piste audio séparée",
    legacyTitle: "Collez un lien",
    subtitle:
      "VDrop résout le lien et vous choisissez la qualité. Rien ne se télécharge avant votre accord.",
    placeholder: "https://... ou un lien média direct",
    analyze: "Résoudre",
    paste: "Coller depuis le presse-papiers",
    analyzing: "Résolution en cours",
    download: "Télécharger",
    changeFolder: "Choisir un dossier",
    savingTo: "Enregistrer dans",
    streamNotice: "Flux segmenté",
    streamNoticeBody:
      "Ceci est un flux HLS/DASH. VDrop assemble les segments avec FFmpeg sans réencoder. Les flux peuvent être annulés, mais pas mis en pause.",
    ffmpegMissing: "FFmpeg introuvable",
    ffmpegMissingBody:
      "Les flux segmentés (.m3u8 / .mpd) nécessitent FFmpeg. Les liens directs fonctionnent sans lui.",
  },

  queue: {
    title: "File active",
    completed: "Terminés",
    itemsDownloading: "en cours",
    analyzing: "Analyse du flux...",
    connecting: "Connexion...",
    subtitle: "Téléchargements en cours et terminés",
    empty: "La file est vide",
    emptyBody: "Résolvez un lien sur l accueil et les téléchargements apparaîtront ici.",
    clearFinished: "Effacer les terminés",
    pause: "Pause",
    resume: "Reprendre",
    cancel: "Annuler",
    remove: "Retirer de la liste",
    openFolder: "Afficher dans le dossier",
    openFile: "Ouvrir le fichier",
    retry: "Réessayer",
  },

  library: {
    title: "Bibliothèque",
    subtitle: "Fichiers téléchargés",
    empty: "La bibliothèque est vide",
    emptyBody: "Les téléchargements terminés sont ajoutés ici automatiquement.",
    missing: "Le fichier a disparu du disque",
    removeEntry: "Retirer l entrée",
    deleteFile: "Supprimer le fichier",
  },

  history: {
    title: "Historique",
    subtitle: "Téléchargements terminés, annulés et échoués",
    empty: "L historique est vide",
    emptyBody: "Chaque téléchargement terminé laisse une trace ici.",
    clear: "Effacer l historique",
  },

  settings: {
    title: "Paramètres",
    subtitle: "Les préférences restent sur cet ordinateur",

    groupGeneral: "Général",
    theme: "Thème",
    themeHint: "Suivre le thème du système ou en fixer un",
    themeSystem: "Système",
    themeLight: "Clair",
    themeDark: "Sombre",
    language: "Langue",
    languageHint: "Langue de l interface",

    groupDownloads: "Téléchargements",
    folder: "Dossier de téléchargement",
    folderHint: "Les nouveaux téléchargements sont enregistrés ici",
    choose: "Changer",
    concurrency: "Téléchargements simultanés",
    concurrencyHint: "Combien tournent en même temps; les autres attendent",
    bandwidth: "Limite de débit",
    bandwidthHint: "Débit total de tous les téléchargements; 0 signifie illimité",
    bandwidthUnit: "Ko/s",
    bandwidthUnlimited: "Illimité",
    autoOpen: "Ouvrir le dossier à la fin",
    autoOpenHint: "Afficher le fichier une fois le téléchargement terminé",
    clipboard: "Surveiller le presse-papiers",
    clipboardHint: "Détecter les liens média lors de la copie",
    notifications: "Notifications",
    notificationsHint: "Afficher une notification système à la fin",

    navGeneral: "Général",
    navDownloads: "Téléchargements",
    navComponents: "Composants",
    navAbout: "À propos",
    allComponentsOk: "Tous les composants fonctionnent",
    someComponentsMissing: "Un composant manque",
    installed: "Installé",
    notInstalled: "Non installé",

    groupComponents: "Composants",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Sert à assembler les flux segmentés (.m3u8 / .mpd)",
    ffmpegFound: "Installé",
    ffmpegNotFound: "Introuvable",
    version: "Version",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Extraction spécifique aux sites. Avec lui, des centaines de sites fonctionnent; sans lui, les liens directs et les flux marchent toujours.",
    ytdlpInstallHint: "Pour installer: ",
    ffmpegInstallHint: "Pour installer: ",

    groupAbout: "À propos",
    appVersion: "Version de VDrop",
    engine: "Cœur",
    engineHint: "Rust + Tauri 2. Aucune dépendance à Python ou yt-dlp.",
  },

  status: {
    queued: "En file",
    downloading: "Téléchargement",
    paused: "En pause",
    retrying: "Nouvelle tentative",
    completed: "Terminé",
    failed: "Échec",
    cancelled: "Annulé",
  },

  units: {
    perSecond: "/s",
    remaining: "restant",
    of: "/",
  },

  clipboard: {
    caught: "Un lien média est dans le presse-papiers",
    resolve: "Résoudre",
    dismiss: "Ignorer",
  },

  errors: {
    unknown: {
      title: "Quelque chose d inattendu est arrivé",
      body: "Détails ci-dessous.",
    },
    empty_url: {
      title: "Collez d abord un lien",
      body: "Saisissez l adresse d une page vidéo ou un lien média direct.",
    },
    unsupported: {
      title: "Cette adresse n est pas prise en charge",
      body: "VDrop n a pas reconnu le lien. Avec yt-dlp, bien plus de sites fonctionnent.",
    },
    network: {
      title: "Impossible de joindre le serveur",
      body: "L adresse est-elle correcte et votre connexion fonctionne-t-elle?",
    },
    drm: {
      title: "Le contenu est protégé par DRM",
      body: "VDrop ne peut pas télécharger les flux protégés par DRM.",
    },
    parse: {
      title: "Impossible de lire les informations du média",
      body: "Le serveur a répondu quelque chose d inattendu.",
    },
    no_media: {
      title: "Aucun média téléchargeable sur la page",
      body: "Si la vidéo se charge en JavaScript, VDrop ne la voit pas encore; essayez le lien média direct.",
    },
    ytdlp_missing: {
      title: "yt-dlp n est pas installé",
      body: "Ce téléchargement nécessite yt-dlp. Voir Paramètres > Composants.",
    },
    ffmpeg_missing: {
      title: "FFmpeg n est pas installé",
      body: "Assembler des flux HLS/DASH nécessite FFmpeg. Voir Paramètres > Composants.",
    },
    record_missing: {
      title: "Impossible de créer l enregistrement du téléchargement",
      body: "L enregistrement n a pas pu être écrit dans la base de données.",
    },
    internal: {
      title: "Quelque chose d inattendu est arrivé",
      body: "Détails ci-dessous.",
    },
  },

  common: {
    search: "Rechercher",
    searchPlaceholder: "Rechercher dans le titre et l adresse",
    clearSearch: "Effacer la recherche",
    noResults: "Aucun résultat",
    noResultsBody: "Essayez une autre recherche.",
    cancel: "Annuler",
    confirm: "Confirmer",
    close: "Fermer",
    unknown: "Inconnu",
    audio: "Audio",
    video: "Vidéo",
    stream: "Flux",
    subtitle: "Sous-titre",
    file: "Fichier",
  },
};

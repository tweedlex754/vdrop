import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const es: Dictionary = {
  nav: {
    home: "Inicio",
    queue: "Cola",
    library: "Biblioteca",
    history: "Historial",
    settings: "Ajustes",
    sections: "Secciones",
    engineReady: "Motor listo",
  },

  status_bar: {
    throughput: "Velocidad total",
    active: "Activas",
    pauseAll: "Pausar todas",
    resumeAll: "Reanudar todas",
    clearFinished: "Limpiar terminadas",
  },

  home: {
    title: "Resolver un enlace de vídeo",
    videoAndAudio: "Vídeo + audio",
    audioOnly: "Solo audio",
    subtitlesOnly: "Subtítulos",
    noSubtitleTrack: "Esta fuente no tiene subtítulos",
    addToQueue: "Añadir a la cola",
    noAudioTrack: "Esta fuente no tiene pista de audio separada",
    legacyTitle: "Pega un enlace",
    subtitle:
      "VDrop resuelve el enlace y tú eliges la calidad. Nada se descarga hasta que lo digas.",
    placeholder: "https://... o un enlace directo",
    analyze: "Resolver",
    paste: "Pegar del portapapeles",
    analyzing: "Resolviendo",
    download: "Descargar",
    changeFolder: "Elegir carpeta",
    savingTo: "Guardar en",
    streamNotice: "Flujo segmentado",
    streamNoticeBody:
      "Esto es un flujo HLS/DASH. VDrop une los segmentos con FFmpeg sin recodificar. Los flujos se pueden cancelar, pero no pausar.",
    ffmpegMissing: "FFmpeg no encontrado",
    ffmpegMissingBody:
      "Los flujos segmentados (.m3u8 / .mpd) necesitan FFmpeg. Los enlaces directos funcionan sin él.",
  },

  queue: {
    title: "Cola activa",
    completed: "Completadas",
    itemsDownloading: "descargando",
    analyzing: "Analizando el flujo...",
    connecting: "Conectando...",
    subtitle: "Descargas en curso y terminadas",
    empty: "La cola está vacía",
    emptyBody: "Resuelve un enlace en Inicio y las descargas aparecerán aquí.",
    clearFinished: "Limpiar terminadas",
    pause: "Pausar",
    resume: "Reanudar",
    cancel: "Cancelar",
    remove: "Quitar de la lista",
    openFolder: "Mostrar en la carpeta",
    openFile: "Abrir archivo",
    retry: "Reintentar",
  },

  library: {
    title: "Biblioteca",
    subtitle: "Archivos descargados",
    empty: "La biblioteca está vacía",
    emptyBody: "Las descargas terminadas se añaden aquí automáticamente.",
    missing: "El archivo ya no está en el disco",
    removeEntry: "Quitar entrada",
    deleteFile: "Eliminar archivo",
  },

  history: {
    title: "Historial",
    subtitle: "Descargas terminadas, canceladas y fallidas",
    empty: "El historial está vacío",
    emptyBody: "Cada descarga terminada deja un registro aquí.",
    clear: "Limpiar historial",
  },

  settings: {
    title: "Ajustes",
    subtitle: "Las preferencias se guardan en este equipo",

    groupGeneral: "General",
    theme: "Tema",
    themeHint: "Seguir el tema del sistema o fijar uno",
    themeSystem: "Sistema",
    themeLight: "Claro",
    themeDark: "Oscuro",
    language: "Idioma",
    languageHint: "Idioma de la interfaz",

    groupDownloads: "Descargas",
    folder: "Carpeta de descargas",
    folderHint: "Las nuevas descargas se guardan aquí",
    choose: "Cambiar",
    concurrency: "Descargas simultáneas",
    concurrencyHint: "Cuántas se ejecutan a la vez; el resto espera",
    bandwidth: "Límite de velocidad",
    bandwidthHint: "Velocidad total de todas las descargas; 0 significa sin límite",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Sin límite",
    autoOpen: "Abrir la carpeta al terminar",
    autoOpenHint: "Mostrar el archivo cuando termine la descarga",
    clipboard: "Vigilar el portapapeles",
    clipboardHint: "Detectar enlaces de medios al copiarlos",
    notifications: "Notificaciones",
    notificationsHint: "Mostrar una notificación del sistema al terminar",

    navGeneral: "General",
    navDownloads: "Descargas",
    navComponents: "Componentes",
    navAbout: "Acerca de",
    allComponentsOk: "Todos los componentes funcionan",
    someComponentsMissing: "Falta un componente",
    installed: "Instalado",
    notInstalled: "No instalado",

    groupComponents: "Componentes",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Se usa para unir flujos segmentados (.m3u8 / .mpd)",
    ffmpegFound: "Instalado",
    ffmpegNotFound: "No encontrado",
    version: "Versión",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Extracción específica por sitio. Con ella funcionan cientos de sitios; sin ella siguen funcionando los enlaces directos y los flujos.",
    ytdlpInstallHint: "Para instalar: ",
    ffmpegInstallHint: "Para instalar: ",

    groupAbout: "Acerca de",
    appVersion: "Versión de VDrop",
    engine: "Núcleo",
    engineHint: "Rust + Tauri 2. Sin dependencia de Python ni yt-dlp.",
  },

  status: {
    queued: "En cola",
    downloading: "Descargando",
    paused: "En pausa",
    retrying: "Reintentando",
    completed: "Listo",
    failed: "Fallida",
    cancelled: "Cancelada",
  },

  units: {
    perSecond: "/s",
    remaining: "restante",
    of: "/",
  },

  clipboard: {
    caught: "Hay un enlace de medios en el portapapeles",
    resolve: "Resolver",
    dismiss: "Ignorar",
  },

  errors: {
    unknown: {
      title: "Ha ocurrido algo inesperado",
      body: "Detalles abajo.",
    },
    empty_url: {
      title: "Pega primero un enlace",
      body: "Introduce la dirección de una página de vídeo o un enlace directo.",
    },
    unsupported: {
      title: "Esta dirección no es compatible",
      body: "VDrop no reconoció el enlace. Con yt-dlp funcionan muchos más sitios.",
    },
    network: {
      title: "No se pudo contactar con el servidor",
      body: "¿La dirección es correcta y tu conexión funciona?",
    },
    drm: {
      title: "El contenido está protegido con DRM",
      body: "VDrop no puede descargar flujos protegidos con DRM.",
    },
    parse: {
      title: "No se pudo leer la información del medio",
      body: "El servidor respondió algo inesperado.",
    },
    no_media: {
      title: "No hay medios descargables en la página",
      body: "Si el vídeo se carga con JavaScript, VDrop aún no puede verlo; prueba con el enlace directo.",
    },
    ytdlp_missing: {
      title: "yt-dlp no está instalado",
      body: "Esta descarga necesita yt-dlp. Consulta Ajustes > Componentes.",
    },
    ffmpeg_missing: {
      title: "FFmpeg no está instalado",
      body: "Unir flujos HLS/DASH necesita FFmpeg. Consulta Ajustes > Componentes.",
    },
    record_missing: {
      title: "No se pudo crear el registro de la descarga",
      body: "No se pudo escribir el registro en la base de datos.",
    },
    internal: {
      title: "Ha ocurrido algo inesperado",
      body: "Detalles abajo.",
    },
  },

  common: {
    search: "Buscar",
    searchPlaceholder: "Buscar en título y dirección",
    clearSearch: "Limpiar búsqueda",
    noResults: "Sin resultados",
    noResultsBody: "Prueba otra búsqueda.",
    cancel: "Cancelar",
    confirm: "Confirmar",
    close: "Cerrar",
    unknown: "Desconocido",
    audio: "Audio",
    video: "Vídeo",
    stream: "Flujo",
    subtitle: "Subtítulo",
    file: "Archivo",
  },
};

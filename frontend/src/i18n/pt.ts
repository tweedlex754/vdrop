import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const pt: Dictionary = {
  nav: {
    home: "Início",
    queue: "Fila",
    library: "Biblioteca",
    history: "Histórico",
    settings: "Configurações",
    sections: "Seções",
    engineReady: "Motor pronto",
  },

  status_bar: {
    throughput: "Velocidade total",
    active: "Ativos",
    pauseAll: "Pausar tudo",
    resumeAll: "Retomar tudo",
    clearFinished: "Limpar concluídos",
  },

  home: {
    title: "Resolver um link de vídeo",
    videoAndAudio: "Vídeo + áudio",
    audioOnly: "Somente áudio",
    subtitlesOnly: "Legendas",
    noSubtitleTrack: "Esta fonte não tem legendas",
    addToQueue: "Adicionar à fila",
    noAudioTrack: "Esta fonte não tem faixa de áudio separada",
    legacyTitle: "Cole um link",
    subtitle:
      "O VDrop resolve o link e você escolhe a qualidade. Nada baixa até você mandar.",
    placeholder: "https://... ou um link de mídia direto",
    analyze: "Resolver",
    paste: "Colar da área de transferência",
    analyzing: "Resolvendo",
    download: "Baixar",
    changeFolder: "Escolher pasta",
    savingTo: "Salvar em",
    streamNotice: "Fluxo segmentado",
    streamNoticeBody:
      "Isto é um fluxo HLS/DASH. O VDrop junta os segmentos com FFmpeg sem recodificar. Fluxos podem ser cancelados, mas não pausados.",
    ffmpegMissing: "FFmpeg não encontrado",
    ffmpegMissingBody:
      "Fluxos segmentados (.m3u8 / .mpd) precisam do FFmpeg. Links diretos funcionam sem ele.",
  },

  queue: {
    title: "Fila ativa",
    completed: "Concluídos",
    itemsDownloading: "baixando",
    analyzing: "Analisando o fluxo...",
    connecting: "Conectando...",
    subtitle: "Downloads em andamento e concluídos",
    empty: "A fila está vazia",
    emptyBody: "Resolva um link no Início e os downloads aparecem aqui.",
    clearFinished: "Limpar concluídos",
    pause: "Pausar",
    resume: "Retomar",
    cancel: "Cancelar",
    remove: "Remover da lista",
    openFolder: "Mostrar na pasta",
    openFile: "Abrir arquivo",
    retry: "Tentar de novo",
  },

  library: {
    title: "Biblioteca",
    subtitle: "Arquivos baixados",
    empty: "A biblioteca está vazia",
    emptyBody: "Downloads concluídos entram aqui automaticamente.",
    missing: "O arquivo sumiu do disco",
    removeEntry: "Remover entrada",
    deleteFile: "Excluir arquivo",
  },

  history: {
    title: "Histórico",
    subtitle: "Downloads concluídos, cancelados e com falha",
    empty: "O histórico está vazio",
    emptyBody: "Todo download concluído deixa um registro aqui.",
    clear: "Limpar histórico",
  },

  settings: {
    title: "Configurações",
    subtitle: "As preferências ficam neste computador",

    groupGeneral: "Geral",
    theme: "Tema",
    themeHint: "Seguir o tema do sistema ou fixar um",
    themeSystem: "Sistema",
    themeLight: "Claro",
    themeDark: "Escuro",
    language: "Idioma",
    languageHint: "Idioma da interface",

    groupDownloads: "Downloads",
    folder: "Pasta de downloads",
    folderHint: "Novos downloads são salvos aqui",
    choose: "Alterar",
    concurrency: "Downloads simultâneos",
    concurrencyHint: "Quantos rodam ao mesmo tempo; o resto espera",
    bandwidth: "Limite de velocidade",
    bandwidthHint: "Velocidade total de todos os downloads; 0 significa sem limite",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Sem limite",
    autoOpen: "Abrir a pasta ao terminar",
    autoOpenHint: "Mostrar o arquivo quando o download terminar",
    clipboard: "Vigiar a área de transferência",
    clipboardHint: "Capturar links de mídia ao copiá-los",
    notifications: "Notificações",
    notificationsHint: "Mostrar uma notificação do sistema ao terminar",

    navGeneral: "Geral",
    navDownloads: "Downloads",
    navComponents: "Componentes",
    navAbout: "Sobre",
    allComponentsOk: "Todos os componentes funcionando",
    someComponentsMissing: "Falta um componente",
    installed: "Instalado",
    notInstalled: "Não instalado",

    groupComponents: "Componentes",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Usado para juntar fluxos segmentados (.m3u8 / .mpd)",
    ffmpegFound: "Instalado",
    ffmpegNotFound: "Não encontrado",
    version: "Versão",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Extração específica por site. Com ele, centenas de sites funcionam; sem ele, links diretos e fluxos continuam funcionando.",
    ytdlpInstallHint: "Para instalar: ",
    ffmpegInstallHint: "Para instalar: ",

    groupAbout: "Sobre",
    appVersion: "Versão do VDrop",
    engine: "Núcleo",
    engineHint: "Rust + Tauri 2. Sem dependência de Python ou yt-dlp.",
  },

  status: {
    queued: "Na fila",
    downloading: "Baixando",
    paused: "Pausado",
    retrying: "Tentando de novo",
    completed: "Pronto",
    failed: "Falhou",
    cancelled: "Cancelado",
  },

  units: {
    perSecond: "/s",
    remaining: "restante",
    of: "/",
  },

  clipboard: {
    caught: "Há um link de mídia na área de transferência",
    resolve: "Resolver",
    dismiss: "Ignorar",
  },

  errors: {
    unknown: {
      title: "Algo inesperado aconteceu",
      body: "Detalhes abaixo.",
    },
    empty_url: {
      title: "Cole um link primeiro",
      body: "Digite o endereço de uma página de vídeo ou um link de mídia direto.",
    },
    unsupported: {
      title: "Este endereço não é suportado",
      body: "O VDrop não reconheceu o link. Com yt-dlp, muitos mais sites funcionam.",
    },
    network: {
      title: "Não foi possível alcançar o servidor",
      body: "O endereço está certo e sua conexão funciona?",
    },
    drm: {
      title: "O conteúdo é protegido por DRM",
      body: "O VDrop não pode baixar fluxos protegidos por DRM.",
    },
    parse: {
      title: "Não foi possível ler as informações da mídia",
      body: "O servidor respondeu algo inesperado.",
    },
    no_media: {
      title: "Nenhuma mídia baixável na página",
      body: "Se o vídeo carrega via JavaScript, o VDrop ainda não o vê; tente o link de mídia direto.",
    },
    ytdlp_missing: {
      title: "yt-dlp não está instalado",
      body: "Este download precisa do yt-dlp. Veja Configurações > Componentes.",
    },
    ffmpeg_missing: {
      title: "FFmpeg não está instalado",
      body: "Juntar fluxos HLS/DASH precisa do FFmpeg. Veja Configurações > Componentes.",
    },
    record_missing: {
      title: "Não foi possível criar o registro do download",
      body: "O registro não pôde ser gravado no banco de dados.",
    },
    internal: {
      title: "Algo inesperado aconteceu",
      body: "Detalhes abaixo.",
    },
  },

  common: {
    search: "Buscar",
    searchPlaceholder: "Buscar no título e no endereço",
    clearSearch: "Limpar busca",
    noResults: "Nenhum resultado",
    noResultsBody: "Tente outra busca.",
    cancel: "Cancelar",
    confirm: "Confirmar",
    close: "Fechar",
    unknown: "Desconhecido",
    audio: "Áudio",
    video: "Vídeo",
    stream: "Fluxo",
    subtitle: "Legenda",
    file: "Arquivo",
  },
};

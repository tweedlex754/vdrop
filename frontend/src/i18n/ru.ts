import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const ru: Dictionary = {
  nav: {
    home: "Главная",
    queue: "Очередь",
    library: "Библиотека",
    history: "История",
    settings: "Настройки",
    sections: "Разделы",
    engineReady: "Движок готов",
  },

  status_bar: {
    throughput: "Общая скорость",
    active: "Активные",
    pauseAll: "Приостановить все",
    resumeAll: "Возобновить все",
    clearFinished: "Очистить завершенные",
  },

  home: {
    title: "Разобрать ссылку на видео",
    videoAndAudio: "Видео + аудио",
    audioOnly: "Только аудио",
    subtitlesOnly: "Субтитры",
    noSubtitleTrack: "У этого источника нет субтитров",
    addToQueue: "Добавить в очередь",
    noAudioTrack: "У этого источника нет отдельной аудиодорожки",
    legacyTitle: "Вставьте ссылку",
    subtitle:
      "VDrop разбирает ссылку, качество выбираете вы. Загрузка начнется только по вашей команде.",
    placeholder: "https://... или прямая ссылка на медиа",
    analyze: "Разобрать",
    paste: "Вставить из буфера обмена",
    analyzing: "Разбираем",
    download: "Скачать",
    changeFolder: "Выбрать папку",
    savingTo: "Сохранить в",
    streamNotice: "Сегментированный поток",
    streamNoticeBody:
      "Это поток HLS/DASH. VDrop соединяет сегменты через FFmpeg без перекодирования. Потоки можно отменить, но нельзя приостановить.",
    ffmpegMissing: "FFmpeg не найден",
    ffmpegMissingBody:
      "Сегментированные потоки (.m3u8 / .mpd) требуют FFmpeg. Прямые ссылки работают и без него.",
  },

  queue: {
    title: "Активная очередь",
    completed: "Завершенные",
    itemsDownloading: "идет загрузка",
    analyzing: "Анализ потока...",
    connecting: "Соединение...",
    subtitle: "Текущие и завершенные загрузки",
    empty: "Очередь пуста",
    emptyBody: "Разберите ссылку на главной, и загрузки появятся здесь.",
    clearFinished: "Очистить завершенные",
    pause: "Пауза",
    resume: "Продолжить",
    cancel: "Отменить",
    remove: "Убрать из списка",
    openFolder: "Показать в папке",
    openFile: "Открыть файл",
    retry: "Повторить",
  },

  library: {
    title: "Библиотека",
    subtitle: "Скачанные файлы",
    empty: "Библиотека пуста",
    emptyBody: "Завершенные загрузки попадают сюда автоматически.",
    missing: "Файла больше нет на диске",
    removeEntry: "Убрать запись",
    deleteFile: "Удалить файл",
  },

  history: {
    title: "История",
    subtitle: "Завершенные, отмененные и неудачные загрузки",
    empty: "История пуста",
    emptyBody: "Каждая завершенная загрузка оставляет здесь след.",
    clear: "Очистить историю",
  },

  settings: {
    title: "Настройки",
    subtitle: "Настройки хранятся на этом компьютере",

    groupGeneral: "Общие",
    theme: "Тема",
    themeHint: "Следовать теме системы или закрепить одну",
    themeSystem: "Система",
    themeLight: "Светлая",
    themeDark: "Темная",
    language: "Язык",
    languageHint: "Язык интерфейса",

    groupDownloads: "Загрузки",
    folder: "Папка загрузок",
    folderHint: "Новые загрузки сохраняются сюда",
    choose: "Изменить",
    concurrency: "Одновременные загрузки",
    concurrencyHint: "Сколько идет одновременно; остальные ждут",
    bandwidth: "Ограничение скорости",
    bandwidthHint: "Общая скорость всех загрузок; 0 значит без ограничения",
    bandwidthUnit: "КБ/с",
    bandwidthUnlimited: "Без ограничения",
    autoOpen: "Открыть папку по завершении",
    autoOpenHint: "Показать файл, когда загрузка закончится",
    clipboard: "Следить за буфером обмена",
    clipboardHint: "Ловить ссылки на медиа при копировании",
    notifications: "Уведомления",
    notificationsHint: "Показывать системное уведомление по завершении",

    navGeneral: "Общие",
    navDownloads: "Загрузки",
    navComponents: "Компоненты",
    navAbout: "О программе",
    allComponentsOk: "Все компоненты работают",
    someComponentsMissing: "Не хватает компонента",
    installed: "Установлен",
    notInstalled: "Не установлен",

    groupComponents: "Компоненты",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Соединяет сегментированные потоки (.m3u8 / .mpd)",
    ffmpegFound: "Установлен",
    ffmpegNotFound: "Не найден",
    version: "Версия",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Извлечение под конкретные сайты. С ним работают сотни сайтов; без него остаются прямые ссылки и потоки.",
    ytdlpInstallHint: "Установка: ",
    ffmpegInstallHint: "Установка: ",

    groupAbout: "О программе",
    appVersion: "Версия VDrop",
    engine: "Ядро",
    engineHint: "Rust + Tauri 2. Без зависимости от Python и yt-dlp.",
  },

  status: {
    queued: "В очереди",
    downloading: "Загрузка",
    paused: "Приостановлено",
    retrying: "Повтор",
    completed: "Готово",
    failed: "Ошибка",
    cancelled: "Отменено",
  },

  units: {
    perSecond: "/с",
    remaining: "осталось",
    of: "/",
  },

  clipboard: {
    caught: "В буфере обмена есть ссылка на медиа",
    resolve: "Разобрать",
    dismiss: "Пропустить",
  },

  errors: {
    unknown: {
      title: "Произошло что-то неожиданное",
      body: "Подробности ниже.",
    },
    empty_url: {
      title: "Сначала вставьте ссылку",
      body: "Введите адрес страницы с видео или прямую ссылку на медиа.",
    },
    unsupported: {
      title: "Этот адрес не поддерживается",
      body: "VDrop не распознал ссылку. С yt-dlp работает намного больше сайтов.",
    },
    network: {
      title: "Не удалось связаться с сервером",
      body: "Адрес верный, и соединение работает?",
    },
    drm: {
      title: "Содержимое защищено DRM",
      body: "VDrop не может скачивать потоки с защитой DRM.",
    },
    parse: {
      title: "Не удалось прочитать сведения о медиа",
      body: "Сервер ответил чем-то неожиданным.",
    },
    no_media: {
      title: "На странице нет медиа для скачивания",
      body: "Если видео загружается через JavaScript, VDrop его пока не видит; попробуйте прямую ссылку.",
    },
    ytdlp_missing: {
      title: "yt-dlp не установлен",
      body: "Для этой загрузки нужен yt-dlp. См. Настройки > Компоненты.",
    },
    ffmpeg_missing: {
      title: "FFmpeg не установлен",
      body: "Для склейки потоков HLS/DASH нужен FFmpeg. См. Настройки > Компоненты.",
    },
    record_missing: {
      title: "Не удалось создать запись загрузки",
      body: "Запись не удалось сохранить в базе данных.",
    },
    internal: {
      title: "Произошло что-то неожиданное",
      body: "Подробности ниже.",
    },
  },

  common: {
    search: "Поиск",
    searchPlaceholder: "Искать в названии и адресе",
    clearSearch: "Очистить поиск",
    noResults: "Совпадений нет",
    noResultsBody: "Попробуйте другой запрос.",
    cancel: "Отмена",
    confirm: "Подтвердить",
    close: "Закрыть",
    unknown: "Неизвестно",
    audio: "Аудио",
    video: "Видео",
    stream: "Поток",
    subtitle: "Субтитры",
    file: "Файл",
  },
};

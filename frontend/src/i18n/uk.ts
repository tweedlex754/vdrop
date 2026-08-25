import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const uk: Dictionary = {
  nav: {
    home: "Головна",
    queue: "Черга",
    library: "Бібліотека",
    history: "Історія",
    settings: "Налаштування",
    sections: "Розділи",
    engineReady: "Рушій готовий",
  },

  status_bar: {
    throughput: "Загальна швидкість",
    active: "Активні",
    pauseAll: "Призупинити все",
    resumeAll: "Відновити все",
    clearFinished: "Очистити завершені",
  },

  home: {
    title: "Розібрати посилання на відео",
    videoAndAudio: "Відео + аудіо",
    audioOnly: "Лише аудіо",
    subtitlesOnly: "Субтитри",
    noSubtitleTrack: "У цього джерела немає субтитрів",
    addToQueue: "Додати до черги",
    noAudioTrack: "У цього джерела немає окремої аудіодоріжки",
    legacyTitle: "Вставте посилання",
    subtitle:
      "VDrop розбирає посилання, а якість обираєте ви. Нічого не завантажується, доки ви не скажете.",
    placeholder: "https://... або пряме посилання на медіа",
    analyze: "Розібрати",
    paste: "Вставити з буфера обміну",
    analyzing: "Розбираємо",
    download: "Завантажити",
    changeFolder: "Обрати теку",
    savingTo: "Зберегти в",
    streamNotice: "Сегментований потік",
    streamNoticeBody:
      "Це потік HLS/DASH. VDrop зʼєднує сегменти через FFmpeg без перекодування. Потоки можна скасувати, але не призупинити.",
    ffmpegMissing: "FFmpeg не знайдено",
    ffmpegMissingBody:
      "Сегментовані потоки (.m3u8 / .mpd) потребують FFmpeg. Прямі посилання працюють і без нього.",
  },

  queue: {
    title: "Активна черга",
    completed: "Завершені",
    itemsDownloading: "триває",
    analyzing: "Аналіз потоку...",
    connecting: "Зʼєднання...",
    subtitle: "Поточні та завершені завантаження",
    empty: "Черга порожня",
    emptyBody: "Розберіть посилання на головній, і завантаження зʼявляться тут.",
    clearFinished: "Очистити завершені",
    pause: "Пауза",
    resume: "Продовжити",
    cancel: "Скасувати",
    remove: "Прибрати зі списку",
    openFolder: "Показати в теці",
    openFile: "Відкрити файл",
    retry: "Спробувати ще",
  },

  library: {
    title: "Бібліотека",
    subtitle: "Завантажені файли",
    empty: "Бібліотека порожня",
    emptyBody: "Завершені завантаження потрапляють сюди автоматично.",
    missing: "Файла більше немає на диску",
    removeEntry: "Прибрати запис",
    deleteFile: "Видалити файл",
  },

  history: {
    title: "Історія",
    subtitle: "Завершені, скасовані та невдалі завантаження",
    empty: "Історія порожня",
    emptyBody: "Кожне завершене завантаження лишає тут слід.",
    clear: "Очистити історію",
  },

  settings: {
    title: "Налаштування",
    subtitle: "Налаштування зберігаються на цьому компʼютері",

    groupGeneral: "Загальні",
    theme: "Тема",
    themeHint: "Слідувати темі системи або закріпити одну",
    themeSystem: "Система",
    themeLight: "Світла",
    themeDark: "Темна",
    language: "Мова",
    languageHint: "Мова інтерфейсу",

    groupDownloads: "Завантаження",
    folder: "Тека завантажень",
    folderHint: "Нові завантаження зберігаються тут",
    choose: "Змінити",
    concurrency: "Одночасні завантаження",
    concurrencyHint: "Скільки йде водночас; решта чекає",
    bandwidth: "Обмеження швидкості",
    bandwidthHint: "Загальна швидкість усіх завантажень; 0 означає без обмеження",
    bandwidthUnit: "КБ/с",
    bandwidthUnlimited: "Без обмеження",
    autoOpen: "Відкрити теку після завершення",
    autoOpenHint: "Показати файл, коли завантаження скінчиться",
    clipboard: "Стежити за буфером обміну",
    clipboardHint: "Ловити посилання на медіа під час копіювання",
    notifications: "Сповіщення",
    notificationsHint: "Показувати системне сповіщення після завершення",

    navGeneral: "Загальні",
    navDownloads: "Завантаження",
    navComponents: "Компоненти",
    navAbout: "Про програму",
    allComponentsOk: "Усі компоненти працюють",
    someComponentsMissing: "Бракує компонента",
    installed: "Встановлено",
    notInstalled: "Не встановлено",

    groupComponents: "Компоненти",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Зʼєднує сегментовані потоки (.m3u8 / .mpd)",
    ffmpegFound: "Встановлено",
    ffmpegNotFound: "Не знайдено",
    version: "Версія",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Витягання під конкретні сайти. З ним працюють сотні сайтів; без нього лишаються прямі посилання та потоки.",
    ytdlpInstallHint: "Встановлення: ",
    ffmpegInstallHint: "Встановлення: ",

    groupAbout: "Про програму",
    appVersion: "Версія VDrop",
    engine: "Ядро",
    engineHint: "Rust + Tauri 2. Без залежності від Python чи yt-dlp.",
  },

  status: {
    queued: "У черзі",
    downloading: "Завантаження",
    paused: "Призупинено",
    retrying: "Повтор",
    completed: "Готово",
    failed: "Помилка",
    cancelled: "Скасовано",
  },

  units: {
    perSecond: "/с",
    remaining: "лишилось",
    of: "/",
  },

  clipboard: {
    caught: "У буфері обміну є посилання на медіа",
    resolve: "Розібрати",
    dismiss: "Пропустити",
  },

  errors: {
    unknown: {
      title: "Сталося щось неочікуване",
      body: "Подробиці нижче.",
    },
    empty_url: {
      title: "Спершу вставте посилання",
      body: "Введіть адресу сторінки з відео або пряме посилання на медіа.",
    },
    unsupported: {
      title: "Ця адреса не підтримується",
      body: "VDrop не розпізнав посилання. З yt-dlp працює значно більше сайтів.",
    },
    network: {
      title: "Не вдалося звʼязатися з сервером",
      body: "Адреса правильна, і зʼєднання працює?",
    },
    drm: {
      title: "Вміст захищено DRM",
      body: "VDrop не завантажує потоки із захистом DRM.",
    },
    parse: {
      title: "Не вдалося прочитати відомості про медіа",
      body: "Сервер відповів чимось неочікуваним.",
    },
    no_media: {
      title: "На сторінці немає медіа для завантаження",
      body: "Якщо відео вантажиться через JavaScript, VDrop його ще не бачить; спробуйте пряме посилання.",
    },
    ytdlp_missing: {
      title: "yt-dlp не встановлено",
      body: "Це завантаження потребує yt-dlp. Див. Налаштування > Компоненти.",
    },
    ffmpeg_missing: {
      title: "FFmpeg не встановлено",
      body: "Склейка потоків HLS/DASH потребує FFmpeg. Див. Налаштування > Компоненти.",
    },
    record_missing: {
      title: "Не вдалося створити запис завантаження",
      body: "Запис не вдалося зберегти в базі даних.",
    },
    internal: {
      title: "Сталося щось неочікуване",
      body: "Подробиці нижче.",
    },
  },

  common: {
    search: "Пошук",
    searchPlaceholder: "Шукати в назві та адресі",
    clearSearch: "Очистити пошук",
    noResults: "Збігів немає",
    noResultsBody: "Спробуйте інший запит.",
    cancel: "Скасувати",
    confirm: "Підтвердити",
    close: "Закрити",
    unknown: "Невідомо",
    audio: "Аудіо",
    video: "Відео",
    stream: "Потік",
    subtitle: "Субтитри",
    file: "Файл",
  },
};

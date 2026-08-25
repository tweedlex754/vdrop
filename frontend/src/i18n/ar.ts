import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const ar: Dictionary = {
  nav: {
    home: "الرئيسية",
    queue: "قائمة الانتظار",
    library: "المكتبة",
    history: "السجل",
    settings: "الإعدادات",
    sections: "الأقسام",
    engineReady: "المحرك جاهز",
  },

  status_bar: {
    throughput: "السرعة الإجمالية",
    active: "نشط",
    pauseAll: "إيقاف الكل مؤقتا",
    resumeAll: "استئناف الكل",
    clearFinished: "مسح المكتملة",
  },

  home: {
    title: "تحليل رابط فيديو",
    videoAndAudio: "فيديو + صوت",
    audioOnly: "الصوت فقط",
    subtitlesOnly: "الترجمات",
    noSubtitleTrack: "لا توجد ترجمات في هذا المصدر",
    addToQueue: "إضافة إلى القائمة",
    noAudioTrack: "لا يوجد مسار صوتي منفصل في هذا المصدر",
    legacyTitle: "الصق رابطا",
    subtitle:
      "يحلل VDrop الرابط وأنت تختار الجودة. لا يبدأ أي تنزيل قبل موافقتك.",
    placeholder: "https://... أو رابط وسائط مباشر",
    analyze: "تحليل",
    paste: "لصق من الحافظة",
    analyzing: "جاري التحليل",
    download: "تنزيل",
    changeFolder: "اختيار مجلد",
    savingTo: "الحفظ في",
    streamNotice: "بث مجزأ",
    streamNoticeBody:
      "هذا بث HLS/DASH. يدمج VDrop المقاطع عبر FFmpeg دون إعادة ترميز. يمكن إلغاء هذه التنزيلات لكن لا يمكن إيقافها مؤقتا.",
    ffmpegMissing: "لم يتم العثور على FFmpeg",
    ffmpegMissingBody:
      "تحتاج البثوث المجزأة (.m3u8 / .mpd) إلى FFmpeg. الروابط المباشرة تعمل بدونه.",
  },

  queue: {
    title: "قائمة الانتظار النشطة",
    completed: "المكتملة",
    itemsDownloading: "قيد التنزيل",
    analyzing: "تحليل البث...",
    connecting: "جاري الاتصال...",
    subtitle: "التنزيلات الجارية والمكتملة",
    empty: "قائمة الانتظار فارغة",
    emptyBody: "حلل رابطا من الصفحة الرئيسية وستظهر التنزيلات هنا.",
    clearFinished: "مسح المكتملة",
    pause: "إيقاف مؤقت",
    resume: "استئناف",
    cancel: "إلغاء",
    remove: "إزالة من القائمة",
    openFolder: "إظهار في المجلد",
    openFile: "فتح الملف",
    retry: "إعادة المحاولة",
  },

  library: {
    title: "المكتبة",
    subtitle: "الملفات المنزلة",
    empty: "المكتبة فارغة",
    emptyBody: "تضاف التنزيلات المكتملة هنا تلقائيا.",
    missing: "الملف لم يعد موجودا على القرص",
    removeEntry: "إزالة السجل",
    deleteFile: "حذف الملف",
  },

  history: {
    title: "السجل",
    subtitle: "التنزيلات المكتملة والملغاة والفاشلة",
    empty: "السجل فارغ",
    emptyBody: "كل تنزيل مكتمل يترك أثرا هنا.",
    clear: "مسح السجل",
  },

  settings: {
    title: "الإعدادات",
    subtitle: "تحفظ التفضيلات على هذا الجهاز",

    groupGeneral: "عام",
    theme: "المظهر",
    themeHint: "اتباع مظهر النظام أو تثبيت واحد",
    themeSystem: "النظام",
    themeLight: "فاتح",
    themeDark: "داكن",
    language: "اللغة",
    languageHint: "لغة الواجهة",

    groupDownloads: "التنزيلات",
    folder: "مجلد التنزيل",
    folderHint: "تحفظ التنزيلات الجديدة هنا",
    choose: "تغيير",
    concurrency: "التنزيلات المتزامنة",
    concurrencyHint: "كم عددها في وقت واحد؛ والباقي ينتظر",
    bandwidth: "حد السرعة",
    bandwidthHint: "السرعة الإجمالية لكل التنزيلات؛ 0 يعني بلا حد",
    bandwidthUnit: "كب/ث",
    bandwidthUnlimited: "بلا حد",
    autoOpen: "فتح المجلد عند الانتهاء",
    autoOpenHint: "إظهار الملف عند انتهاء التنزيل",
    clipboard: "مراقبة الحافظة",
    clipboardHint: "التقاط روابط الوسائط عند نسخها",
    notifications: "الإشعارات",
    notificationsHint: "إظهار إشعار نظام عند انتهاء التنزيل",

    navGeneral: "عام",
    navDownloads: "التنزيلات",
    navComponents: "المكونات",
    navAbout: "حول",
    allComponentsOk: "كل المكونات تعمل",
    someComponentsMissing: "ينقص أحد المكونات",
    installed: "مثبت",
    notInstalled: "غير مثبت",

    groupComponents: "المكونات",
    ffmpeg: "FFmpeg",
    ffmpegHint: "يستخدم لدمج البثوث المجزأة (.m3u8 / .mpd)",
    ffmpegFound: "مثبت",
    ffmpegNotFound: "غير موجود",
    version: "الإصدار",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "استخراج خاص بكل موقع. معه تعمل مئات المواقع؛ وبدونه تبقى الروابط المباشرة والبثوث تعمل.",
    ytdlpInstallHint: "للتثبيت: ",
    ffmpegInstallHint: "للتثبيت: ",

    groupAbout: "حول",
    appVersion: "إصدار VDrop",
    engine: "النواة",
    engineHint: "Rust + Tauri 2. بلا اعتماد على Python أو yt-dlp.",
  },

  status: {
    queued: "في الانتظار",
    downloading: "قيد التنزيل",
    paused: "متوقف مؤقتا",
    retrying: "إعادة المحاولة",
    completed: "تم",
    failed: "فشل",
    cancelled: "ألغي",
  },

  units: {
    perSecond: "/ث",
    remaining: "متبق",
    of: "/",
  },

  clipboard: {
    caught: "يوجد رابط وسائط في الحافظة",
    resolve: "تحليل",
    dismiss: "تجاهل",
  },

  errors: {
    unknown: {
      title: "حدث شيء غير متوقع",
      body: "التفاصيل أدناه.",
    },
    empty_url: {
      title: "الصق رابطا أولا",
      body: "أدخل عنوان صفحة فيديو أو رابط وسائط مباشرا.",
    },
    unsupported: {
      title: "هذا العنوان غير مدعوم",
      body: "لم يتعرف VDrop على الرابط. مع yt-dlp تعمل مواقع أكثر بكثير.",
    },
    network: {
      title: "تعذر الوصول إلى الخادم",
      body: "هل العنوان صحيح وهل اتصالك يعمل؟",
    },
    drm: {
      title: "المحتوى محمي بنظام DRM",
      body: "لا يستطيع VDrop تنزيل البثوث المحمية بنظام DRM.",
    },
    parse: {
      title: "تعذرت قراءة معلومات الوسائط",
      body: "رد الخادم بشيء غير متوقع.",
    },
    no_media: {
      title: "لا توجد وسائط قابلة للتنزيل في الصفحة",
      body: "إذا كان الفيديو يحمل عبر JavaScript فلا يراه VDrop بعد؛ جرب الرابط المباشر.",
    },
    ytdlp_missing: {
      title: "yt-dlp غير مثبت",
      body: "يحتاج هذا التنزيل إلى yt-dlp. انظر الإعدادات > المكونات.",
    },
    ffmpeg_missing: {
      title: "FFmpeg غير مثبت",
      body: "يحتاج دمج بثوث HLS/DASH إلى FFmpeg. انظر الإعدادات > المكونات.",
    },
    record_missing: {
      title: "تعذر إنشاء سجل التنزيل",
      body: "تعذرت كتابة السجل في قاعدة البيانات.",
    },
    internal: {
      title: "حدث شيء غير متوقع",
      body: "التفاصيل أدناه.",
    },
  },

  common: {
    search: "بحث",
    searchPlaceholder: "ابحث في العنوان والرابط",
    clearSearch: "مسح البحث",
    noResults: "لا توجد نتائج مطابقة",
    noResultsBody: "جرب بحثا آخر.",
    cancel: "إلغاء",
    confirm: "تأكيد",
    close: "إغلاق",
    unknown: "غير معروف",
    audio: "صوت",
    video: "فيديو",
    stream: "بث",
    subtitle: "ترجمة",
    file: "ملف",
  },
};

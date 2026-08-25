import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const hi: Dictionary = {
  nav: {
    home: "होम",
    queue: "कतार",
    library: "लाइब्रेरी",
    history: "इतिहास",
    settings: "सेटिंग्स",
    sections: "अनुभाग",
    engineReady: "इंजन तैयार",
  },

  status_bar: {
    throughput: "कुल गति",
    active: "सक्रिय",
    pauseAll: "सभी रोकें",
    resumeAll: "सभी जारी रखें",
    clearFinished: "पूर्ण हटाएँ",
  },

  home: {
    title: "वीडियो लिंक हल करें",
    videoAndAudio: "वीडियो + ऑडियो",
    audioOnly: "केवल ऑडियो",
    subtitlesOnly: "उपशीर्षक",
    noSubtitleTrack: "इस स्रोत में उपशीर्षक नहीं हैं",
    addToQueue: "कतार में जोड़ें",
    noAudioTrack: "इस स्रोत में अलग ऑडियो ट्रैक नहीं है",
    legacyTitle: "लिंक चिपकाएँ",
    subtitle:
      "VDrop लिंक हल करता है और गुणवत्ता आप चुनते हैं। आपके कहे बिना कुछ भी डाउनलोड नहीं होता।",
    placeholder: "https://... या सीधा मीडिया लिंक",
    analyze: "हल करें",
    paste: "क्लिपबोर्ड से चिपकाएँ",
    analyzing: "हल कर रहे हैं",
    download: "डाउनलोड",
    changeFolder: "फ़ोल्डर चुनें",
    savingTo: "यहाँ सहेजें",
    streamNotice: "खंडित स्ट्रीम",
    streamNoticeBody:
      "यह HLS/DASH स्ट्रीम है। VDrop बिना दोबारा एन्कोड किए FFmpeg से खंड जोड़ता है। ऐसे डाउनलोड रद्द हो सकते हैं, रोके नहीं जा सकते।",
    ffmpegMissing: "FFmpeg नहीं मिला",
    ffmpegMissingBody:
      "खंडित स्ट्रीम (.m3u8 / .mpd) के लिए FFmpeg चाहिए। सीधे लिंक इसके बिना भी चलते हैं।",
  },

  queue: {
    title: "सक्रिय कतार",
    completed: "पूर्ण",
    itemsDownloading: "डाउनलोड हो रहे",
    analyzing: "स्ट्रीम का विश्लेषण...",
    connecting: "जुड़ रहे हैं...",
    subtitle: "चल रहे और पूर्ण डाउनलोड",
    empty: "कतार खाली है",
    emptyBody: "होम पर लिंक हल करें, डाउनलोड यहाँ दिखेंगे।",
    clearFinished: "पूर्ण हटाएँ",
    pause: "रोकें",
    resume: "जारी रखें",
    cancel: "रद्द करें",
    remove: "सूची से हटाएँ",
    openFolder: "फ़ोल्डर में दिखाएँ",
    openFile: "फ़ाइल खोलें",
    retry: "फिर कोशिश करें",
  },

  library: {
    title: "लाइब्रेरी",
    subtitle: "डाउनलोड की गई फ़ाइलें",
    empty: "लाइब्रेरी खाली है",
    emptyBody: "पूर्ण डाउनलोड अपने आप यहाँ जुड़ते हैं।",
    missing: "फ़ाइल डिस्क से गायब है",
    removeEntry: "प्रविष्टि हटाएँ",
    deleteFile: "फ़ाइल मिटाएँ",
  },

  history: {
    title: "इतिहास",
    subtitle: "पूर्ण, रद्द और असफल डाउनलोड",
    empty: "इतिहास खाली है",
    emptyBody: "हर पूर्ण डाउनलोड यहाँ रिकॉर्ड छोड़ता है।",
    clear: "इतिहास साफ़ करें",
  },

  settings: {
    title: "सेटिंग्स",
    subtitle: "प्राथमिकताएँ इसी कंप्यूटर पर रहती हैं",

    groupGeneral: "सामान्य",
    theme: "थीम",
    themeHint: "सिस्टम थीम अपनाएँ या एक तय करें",
    themeSystem: "सिस्टम",
    themeLight: "हल्का",
    themeDark: "गहरा",
    language: "भाषा",
    languageHint: "इंटरफ़ेस की भाषा",

    groupDownloads: "डाउनलोड",
    folder: "डाउनलोड फ़ोल्डर",
    folderHint: "नए डाउनलोड यहाँ सहेजे जाते हैं",
    choose: "बदलें",
    concurrency: "एक साथ डाउनलोड",
    concurrencyHint: "एक समय में कितने चलें; बाकी प्रतीक्षा करेंगे",
    bandwidth: "गति सीमा",
    bandwidthHint: "सभी डाउनलोड की कुल गति; 0 का अर्थ असीमित",
    bandwidthUnit: "KB/से",
    bandwidthUnlimited: "असीमित",
    autoOpen: "पूरा होने पर फ़ोल्डर खोलें",
    autoOpenHint: "डाउनलोड पूरा होते ही फ़ाइल दिखाएँ",
    clipboard: "क्लिपबोर्ड देखें",
    clipboardHint: "कॉपी करते ही मीडिया लिंक पकड़ें",
    notifications: "सूचनाएँ",
    notificationsHint: "डाउनलोड पूरा होने पर सिस्टम सूचना दिखाएँ",

    navGeneral: "सामान्य",
    navDownloads: "डाउनलोड",
    navComponents: "घटक",
    navAbout: "बारे में",
    allComponentsOk: "सभी घटक काम कर रहे हैं",
    someComponentsMissing: "एक घटक अनुपस्थित है",
    installed: "स्थापित",
    notInstalled: "स्थापित नहीं",

    groupComponents: "घटक",
    ffmpeg: "FFmpeg",
    ffmpegHint: "खंडित स्ट्रीम (.m3u8 / .mpd) जोड़ने में उपयोग",
    ffmpegFound: "स्थापित",
    ffmpegNotFound: "नहीं मिला",
    version: "संस्करण",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "साइट-विशेष निष्कर्षण। इसके साथ सैकड़ों साइटें चलती हैं; इसके बिना भी सीधे लिंक और स्ट्रीम चलते हैं।",
    ytdlpInstallHint: "स्थापित करें: ",
    ffmpegInstallHint: "स्थापित करें: ",

    groupAbout: "बारे में",
    appVersion: "VDrop संस्करण",
    engine: "कोर",
    engineHint: "Rust + Tauri 2। Python या yt-dlp पर निर्भरता नहीं।",
  },

  status: {
    queued: "कतार में",
    downloading: "डाउनलोड हो रहा",
    paused: "रुका हुआ",
    retrying: "फिर कोशिश",
    completed: "हो गया",
    failed: "असफल",
    cancelled: "रद्द",
  },

  units: {
    perSecond: "/से",
    remaining: "शेष",
    of: "/",
  },

  clipboard: {
    caught: "क्लिपबोर्ड में एक मीडिया लिंक है",
    resolve: "हल करें",
    dismiss: "अनदेखा करें",
  },

  errors: {
    unknown: {
      title: "कुछ अप्रत्याशित हुआ",
      body: "विवरण नीचे।",
    },
    empty_url: {
      title: "पहले एक लिंक चिपकाएँ",
      body: "किसी वीडियो पेज का पता या सीधा मीडिया लिंक डालें।",
    },
    unsupported: {
      title: "यह पता समर्थित नहीं है",
      body: "VDrop लिंक पहचान नहीं पाया। yt-dlp के साथ कहीं ज़्यादा साइटें चलती हैं।",
    },
    network: {
      title: "सर्वर तक नहीं पहुँच सके",
      body: "क्या पता सही है और आपका कनेक्शन चल रहा है?",
    },
    drm: {
      title: "सामग्री DRM से सुरक्षित है",
      body: "VDrop DRM से सुरक्षित स्ट्रीम डाउनलोड नहीं कर सकता।",
    },
    parse: {
      title: "मीडिया जानकारी पढ़ी नहीं जा सकी",
      body: "सर्वर ने अप्रत्याशित उत्तर दिया।",
    },
    no_media: {
      title: "पेज पर डाउनलोड योग्य मीडिया नहीं है",
      body: "अगर वीडियो JavaScript से लोड होता है तो VDrop उसे अभी नहीं देख पाता; सीधा मीडिया लिंक आज़माएँ।",
    },
    ytdlp_missing: {
      title: "yt-dlp स्थापित नहीं है",
      body: "इस डाउनलोड के लिए yt-dlp चाहिए। देखें सेटिंग्स > घटक।",
    },
    ffmpeg_missing: {
      title: "FFmpeg स्थापित नहीं है",
      body: "HLS/DASH स्ट्रीम जोड़ने के लिए FFmpeg चाहिए। देखें सेटिंग्स > घटक।",
    },
    record_missing: {
      title: "डाउनलोड रिकॉर्ड नहीं बना",
      body: "रिकॉर्ड डेटाबेस में लिखा नहीं जा सका।",
    },
    internal: {
      title: "कुछ अप्रत्याशित हुआ",
      body: "विवरण नीचे।",
    },
  },

  common: {
    search: "खोजें",
    searchPlaceholder: "शीर्षक और पते में खोजें",
    clearSearch: "खोज साफ़ करें",
    noResults: "कोई मेल नहीं मिला",
    noResultsBody: "कोई और खोज आज़माएँ।",
    cancel: "रद्द करें",
    confirm: "पुष्टि करें",
    close: "बंद करें",
    unknown: "अज्ञात",
    audio: "ऑडियो",
    video: "वीडियो",
    stream: "स्ट्रीम",
    subtitle: "उपशीर्षक",
    file: "फ़ाइल",
  },
};

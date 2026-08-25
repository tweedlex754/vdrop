import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const zh: Dictionary = {
  nav: {
    home: "首页",
    queue: "队列",
    library: "媒体库",
    history: "历史",
    settings: "设置",
    sections: "分区",
    engineReady: "引擎就绪",
  },

  status_bar: {
    throughput: "总速度",
    active: "进行中",
    pauseAll: "全部暂停",
    resumeAll: "全部继续",
    clearFinished: "清除已完成",
  },

  home: {
    title: "解析视频链接",
    videoAndAudio: "视频 + 音频",
    audioOnly: "仅音频",
    subtitlesOnly: "字幕",
    noSubtitleTrack: "该来源没有字幕",
    addToQueue: "加入队列",
    noAudioTrack: "该来源没有独立音轨",
    legacyTitle: "粘贴链接",
    subtitle:
      "VDrop 负责解析链接，画质由你决定。在你确认之前不会开始下载。",
    placeholder: "https://... 或直接媒体链接",
    analyze: "解析",
    paste: "从剪贴板粘贴",
    analyzing: "解析中",
    download: "下载",
    changeFolder: "选择文件夹",
    savingTo: "保存到",
    streamNotice: "分段流",
    streamNoticeBody:
      "这是 HLS/DASH 流。VDrop 用 FFmpeg 合并分段且不重新编码。此类下载可以取消，但无法暂停。",
    ffmpegMissing: "未找到 FFmpeg",
    ffmpegMissingBody:
      "分段流（.m3u8 / .mpd）需要 FFmpeg。直接文件链接没有它也能用。",
  },

  queue: {
    title: "活动队列",
    completed: "已完成",
    itemsDownloading: "下载中",
    analyzing: "正在分析流...",
    connecting: "连接中...",
    subtitle: "进行中与已完成的下载",
    empty: "队列为空",
    emptyBody: "在首页解析一个链接，下载就会出现在这里。",
    clearFinished: "清除已完成",
    pause: "暂停",
    resume: "继续",
    cancel: "取消",
    remove: "从列表移除",
    openFolder: "在文件夹中显示",
    openFile: "打开文件",
    retry: "重试",
  },

  library: {
    title: "媒体库",
    subtitle: "已下载的文件",
    empty: "媒体库为空",
    emptyBody: "完成的下载会自动加到这里。",
    missing: "文件已从磁盘上消失",
    removeEntry: "移除条目",
    deleteFile: "删除文件",
  },

  history: {
    title: "历史",
    subtitle: "已完成、已取消和失败的下载",
    empty: "历史为空",
    emptyBody: "每个完成的下载都会在这里留下记录。",
    clear: "清除历史",
  },

  settings: {
    title: "设置",
    subtitle: "偏好设置保存在这台电脑上",

    groupGeneral: "常规",
    theme: "主题",
    themeHint: "跟随系统主题或固定一个",
    themeSystem: "系统",
    themeLight: "浅色",
    themeDark: "深色",
    language: "语言",
    languageHint: "界面语言",

    groupDownloads: "下载",
    folder: "下载文件夹",
    folderHint: "新的下载保存到这里",
    choose: "更改",
    concurrency: "同时下载数",
    concurrencyHint: "同时进行多少个；其余排队等待",
    bandwidth: "速度限制",
    bandwidthHint: "所有下载的总速度；0 表示不限制",
    bandwidthUnit: "KB/秒",
    bandwidthUnlimited: "不限制",
    autoOpen: "完成后打开文件夹",
    autoOpenHint: "下载完成后显示该文件",
    clipboard: "监视剪贴板",
    clipboardHint: "复制媒体链接时自动捕获",
    notifications: "通知",
    notificationsHint: "下载完成时显示系统通知",

    navGeneral: "常规",
    navDownloads: "下载",
    navComponents: "组件",
    navAbout: "关于",
    allComponentsOk: "所有组件正常",
    someComponentsMissing: "缺少组件",
    installed: "已安装",
    notInstalled: "未安装",

    groupComponents: "组件",
    ffmpeg: "FFmpeg",
    ffmpegHint: "用于合并分段流（.m3u8 / .mpd）",
    ffmpegFound: "已安装",
    ffmpegNotFound: "未找到",
    version: "版本",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "站点专用解析。安装后可支持数百个站点；不装也仍支持直接链接和流。",
    ytdlpInstallHint: "安装方式：",
    ffmpegInstallHint: "安装方式：",

    groupAbout: "关于",
    appVersion: "VDrop 版本",
    engine: "内核",
    engineHint: "Rust + Tauri 2。不依赖 Python 或 yt-dlp。",
  },

  status: {
    queued: "排队中",
    downloading: "下载中",
    paused: "已暂停",
    retrying: "重试中",
    completed: "完成",
    failed: "失败",
    cancelled: "已取消",
  },

  units: {
    perSecond: "/秒",
    remaining: "剩余",
    of: "/",
  },

  clipboard: {
    caught: "剪贴板里有一个媒体链接",
    resolve: "解析",
    dismiss: "忽略",
  },

  errors: {
    unknown: {
      title: "发生了意外情况",
      body: "详情见下方。",
    },
    empty_url: {
      title: "请先粘贴链接",
      body: "请输入视频页面地址，或直接媒体链接。",
    },
    unsupported: {
      title: "不支持该地址",
      body: "VDrop 没有识别出这个链接。装上 yt-dlp 可支持更多站点。",
    },
    network: {
      title: "无法连接服务器",
      body: "地址正确吗，网络连接正常吗？",
    },
    drm: {
      title: "内容受 DRM 保护",
      body: "VDrop 无法下载受 DRM 保护的流。",
    },
    parse: {
      title: "无法读取媒体信息",
      body: "服务器返回了意外的内容。",
    },
    no_media: {
      title: "页面上没有可下载的媒体",
      body: "如果视频通过 JavaScript 加载，VDrop 暂时看不到；可以试试直接媒体链接。",
    },
    ytdlp_missing: {
      title: "未安装 yt-dlp",
      body: "此下载需要 yt-dlp。请见 设置 > 组件。",
    },
    ffmpeg_missing: {
      title: "未安装 FFmpeg",
      body: "合并 HLS/DASH 流需要 FFmpeg。请见 设置 > 组件。",
    },
    record_missing: {
      title: "无法创建下载记录",
      body: "记录未能写入数据库。",
    },
    internal: {
      title: "发生了意外情况",
      body: "详情见下方。",
    },
  },

  common: {
    search: "搜索",
    searchPlaceholder: "搜索标题和地址",
    clearSearch: "清除搜索",
    noResults: "没有匹配的条目",
    noResultsBody: "换个关键词试试。",
    cancel: "取消",
    confirm: "确认",
    close: "关闭",
    unknown: "未知",
    audio: "音频",
    video: "视频",
    stream: "流",
    subtitle: "字幕",
    file: "文件",
  },
};

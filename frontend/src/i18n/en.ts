import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const en: Dictionary = {
  nav: {
    home: "Home",
    queue: "Queue",
    library: "Library",
    history: "History",
    settings: "Settings",
    sections: "Sections",
    engineReady: "Engine ready",
  },

  status_bar: {
    throughput: "Total speed",
    active: "Active",
    pauseAll: "Pause all",
    resumeAll: "Resume all",
    clearFinished: "Clear finished",
  },

  home: {
    title: "Resolve a video link",
    videoAndAudio: "Video + audio",
    audioOnly: "Audio only",
    subtitlesOnly: "Subtitles",
    noSubtitleTrack: "This source has no subtitles",
    addToQueue: "Add to queue",
    noAudioTrack: "This source has no separate audio option",
    legacyTitle: "Paste a link",
    subtitle:
      "VDrop resolves the link and you pick the quality. Nothing downloads until you say so.",
    placeholder: "https://... or a direct media link",
    analyze: "Resolve",
    paste: "Paste from clipboard",
    analyzing: "Resolving",
    download: "Download",
    changeFolder: "Choose folder",
    savingTo: "Saving to",
    streamNotice: "Segmented stream",
    streamNoticeBody:
      "This is an HLS/DASH stream. VDrop joins the segments with FFmpeg without re-encoding. Streams can be cancelled but not paused.",
    ffmpegMissing: "FFmpeg not found",
    ffmpegMissingBody:
      "Segmented streams (.m3u8 / .mpd) need FFmpeg. Direct file links work without it.",
  },

  queue: {
    title: "Active queue",
    completed: "Completed",
    itemsDownloading: "downloading",
    analyzing: "Analyzing stream...",
    connecting: "Connecting...",
    subtitle: "Running and finished downloads",
    empty: "Queue is empty",
    emptyBody: "Resolve a link on Home and downloads show up here.",
    clearFinished: "Clear finished",
    pause: "Pause",
    resume: "Resume",
    cancel: "Cancel",
    remove: "Remove from list",
    openFolder: "Show in folder",
    openFile: "Open file",
    retry: "Try again",
  },

  library: {
    title: "Library",
    subtitle: "Downloaded files",
    empty: "Library is empty",
    emptyBody: "Finished downloads are added here automatically.",
    missing: "File is gone from disk",
    removeEntry: "Remove entry",
    deleteFile: "Delete file",
  },

  history: {
    title: "History",
    subtitle: "Finished, cancelled and failed downloads",
    empty: "History is empty",
    emptyBody: "Every finished download leaves a record here.",
    clear: "Clear history",
  },

  settings: {
    title: "Settings",
    subtitle: "Preferences are stored on this computer",

    groupGeneral: "General",
    theme: "Theme",
    themeHint: "Follow the system theme or pin one",
    themeSystem: "System",
    themeLight: "Light",
    themeDark: "Dark",
    language: "Language",
    languageHint: "Interface language",

    groupDownloads: "Downloads",
    folder: "Download folder",
    folderHint: "New downloads are saved here",
    choose: "Change",
    concurrency: "Simultaneous downloads",
    concurrencyHint: "How many run at once; the rest wait in line",
    bandwidth: "Speed limit",
    bandwidthHint: "Total speed across all downloads; 0 means unlimited",
    bandwidthUnit: "KB/s",
    bandwidthUnlimited: "Unlimited",
    autoOpen: "Open folder when done",
    autoOpenHint: "Reveal the file once a download finishes",
    clipboard: "Watch clipboard",
    clipboardHint: "Catch media links as you copy them",
    notifications: "Notifications",
    notificationsHint: "Show a system notification when a download finishes",

    navGeneral: "General",
    navDownloads: "Downloads",
    navComponents: "Components",
    navAbout: "About",
    allComponentsOk: "All components working",
    someComponentsMissing: "A component is missing",
    installed: "Installed",
    notInstalled: "Not installed",

    groupComponents: "Components",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Used to join segmented streams (.m3u8 / .mpd)",
    ffmpegFound: "Installed",
    ffmpegNotFound: "Not found",
    version: "Version",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Site-specific extraction. With it, hundreds of sites work; without it, direct links and streams still do.",
    ytdlpInstallHint: "To install: ",
    ffmpegInstallHint: "To install: ",

    groupAbout: "About",
    appVersion: "VDrop version",
    engine: "Engine",
    engineHint: "Rust + Tauri 2. No Python or yt-dlp dependency.",
  },

  status: {
    queued: "Queued",
    downloading: "Downloading",
    paused: "Paused",
    retrying: "Retrying",
    completed: "Done",
    failed: "Failed",
    cancelled: "Cancelled",
  },

  units: {
    perSecond: "/s",
    remaining: "left",
    of: "/",
  },

  clipboard: {
    caught: "A media link is on your clipboard",
    resolve: "Resolve",
    dismiss: "Ignore",
  },

  errors: {
    unknown: {
      title: "Something unexpected happened",
      body: "Details below.",
    },
    empty_url: {
      title: "Paste a link first",
      body: "Enter the address of a video page, or a direct media link.",
    },
    unsupported: {
      title: "This address is not supported",
      body: "VDrop did not recognise the link. Installing yt-dlp covers far more sites.",
    },
    network: {
      title: "Could not reach the server",
      body: "Is the address right, and is your connection working?",
    },
    drm: {
      title: "The content is DRM protected",
      body: "VDrop cannot download DRM protected streams.",
    },
    parse: {
      title: "Could not read the media information",
      body: "The server answered with something unexpected.",
    },
    no_media: {
      title: "No downloadable media on the page",
      body: "If the video loads through JavaScript, VDrop cannot see it yet; try pasting the direct media link.",
    },
    ytdlp_missing: {
      title: "yt-dlp is not installed",
      body: "This download needs yt-dlp. See Settings > Components.",
    },
    ffmpeg_missing: {
      title: "FFmpeg is not installed",
      body: "Joining HLS/DASH streams needs FFmpeg. See Settings > Components.",
    },
    record_missing: {
      title: "Could not create the download record",
      body: "The record could not be written to the database.",
    },
    internal: {
      title: "Something unexpected happened",
      body: "Details below.",
    },
  },

  common: {
    search: "Search",
    searchPlaceholder: "Search title and address",
    clearSearch: "Clear search",
    noResults: "No matching entries",
    noResultsBody: "Try a different search.",
    cancel: "Cancel",
    confirm: "Confirm",
    close: "Close",
    unknown: "Unknown",
    audio: "Audio",
    video: "Video",
    stream: "Stream",
    subtitle: "Subtitle",
    file: "File",
  },
};

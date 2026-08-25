import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const ja: Dictionary = {
  nav: {
    home: "ホーム",
    queue: "キュー",
    library: "ライブラリ",
    history: "履歴",
    settings: "設定",
    sections: "セクション",
    engineReady: "エンジン準備完了",
  },

  status_bar: {
    throughput: "合計速度",
    active: "実行中",
    pauseAll: "すべて一時停止",
    resumeAll: "すべて再開",
    clearFinished: "完了分を消去",
  },

  home: {
    title: "動画リンクを解決",
    videoAndAudio: "映像 + 音声",
    audioOnly: "音声のみ",
    subtitlesOnly: "字幕",
    noSubtitleTrack: "このソースに字幕はありません",
    addToQueue: "キューに追加",
    noAudioTrack: "このソースに独立した音声トラックはありません",
    legacyTitle: "リンクを貼り付け",
    subtitle:
      "VDrop がリンクを解決し、画質はあなたが選びます。指示するまでダウンロードは始まりません。",
    placeholder: "https://... または直接メディアリンク",
    analyze: "解決",
    paste: "クリップボードから貼り付け",
    analyzing: "解決中",
    download: "ダウンロード",
    changeFolder: "フォルダーを選択",
    savingTo: "保存先",
    streamNotice: "セグメント配信",
    streamNoticeBody:
      "これは HLS/DASH 配信です。VDrop は再エンコードせずに FFmpeg でセグメントを結合します。この種のダウンロードは中止できますが一時停止はできません。",
    ffmpegMissing: "FFmpeg が見つかりません",
    ffmpegMissingBody:
      "セグメント配信（.m3u8 / .mpd）には FFmpeg が必要です。直接リンクは無くても動きます。",
  },

  queue: {
    title: "実行中のキュー",
    completed: "完了",
    itemsDownloading: "ダウンロード中",
    analyzing: "配信を解析中...",
    connecting: "接続中...",
    subtitle: "実行中および完了したダウンロード",
    empty: "キューは空です",
    emptyBody: "ホームでリンクを解決すると、ここに表示されます。",
    clearFinished: "完了分を消去",
    pause: "一時停止",
    resume: "再開",
    cancel: "中止",
    remove: "一覧から削除",
    openFolder: "フォルダーで表示",
    openFile: "ファイルを開く",
    retry: "再試行",
  },

  library: {
    title: "ライブラリ",
    subtitle: "ダウンロード済みファイル",
    empty: "ライブラリは空です",
    emptyBody: "完了したダウンロードは自動でここに入ります。",
    missing: "ファイルがディスクにありません",
    removeEntry: "項目を削除",
    deleteFile: "ファイルを削除",
  },

  history: {
    title: "履歴",
    subtitle: "完了・中止・失敗したダウンロード",
    empty: "履歴は空です",
    emptyBody: "完了したダウンロードはここに記録が残ります。",
    clear: "履歴を消去",
  },

  settings: {
    title: "設定",
    subtitle: "設定はこのパソコンに保存されます",

    groupGeneral: "一般",
    theme: "テーマ",
    themeHint: "システムのテーマに従うか固定する",
    themeSystem: "システム",
    themeLight: "ライト",
    themeDark: "ダーク",
    language: "言語",
    languageHint: "表示言語",

    groupDownloads: "ダウンロード",
    folder: "保存フォルダー",
    folderHint: "新しいダウンロードはここに保存されます",
    choose: "変更",
    concurrency: "同時ダウンロード数",
    concurrencyHint: "同時に実行する数。残りは順番待ちです",
    bandwidth: "速度制限",
    bandwidthHint: "すべてのダウンロードの合計速度。0 は無制限",
    bandwidthUnit: "KB/秒",
    bandwidthUnlimited: "無制限",
    autoOpen: "完了時にフォルダーを開く",
    autoOpenHint: "ダウンロード完了後にファイルを表示",
    clipboard: "クリップボードを監視",
    clipboardHint: "メディアリンクをコピー時に取り込む",
    notifications: "通知",
    notificationsHint: "ダウンロード完了時にシステム通知を表示",

    navGeneral: "一般",
    navDownloads: "ダウンロード",
    navComponents: "コンポーネント",
    navAbout: "情報",
    allComponentsOk: "すべてのコンポーネントが動作中",
    someComponentsMissing: "不足しているコンポーネントがあります",
    installed: "インストール済み",
    notInstalled: "未インストール",

    groupComponents: "コンポーネント",
    ffmpeg: "FFmpeg",
    ffmpegHint: "セグメント配信（.m3u8 / .mpd）の結合に使用",
    ffmpegFound: "インストール済み",
    ffmpegNotFound: "見つかりません",
    version: "バージョン",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "サイト別の抽出。導入すると数百のサイトに対応します。無くても直接リンクと配信は動きます。",
    ytdlpInstallHint: "インストール: ",
    ffmpegInstallHint: "インストール: ",

    groupAbout: "情報",
    appVersion: "VDrop のバージョン",
    engine: "コア",
    engineHint: "Rust + Tauri 2。Python や yt-dlp への依存はありません。",
  },

  status: {
    queued: "待機中",
    downloading: "ダウンロード中",
    paused: "一時停止中",
    retrying: "再試行中",
    completed: "完了",
    failed: "失敗",
    cancelled: "中止",
  },

  units: {
    perSecond: "/秒",
    remaining: "残り",
    of: "/",
  },

  clipboard: {
    caught: "クリップボードにメディアリンクがあります",
    resolve: "解決",
    dismiss: "無視",
  },

  errors: {
    unknown: {
      title: "予期しない問題が起きました",
      body: "詳細は下記。",
    },
    empty_url: {
      title: "まずリンクを貼り付けてください",
      body: "動画ページのアドレス、または直接メディアリンクを入力してください。",
    },
    unsupported: {
      title: "このアドレスには対応していません",
      body: "VDrop はリンクを認識できませんでした。yt-dlp を入れると対応サイトが大幅に増えます。",
    },
    network: {
      title: "サーバーに接続できません",
      body: "アドレスは正しいですか、接続は生きていますか。",
    },
    drm: {
      title: "コンテンツは DRM で保護されています",
      body: "VDrop は DRM 保護された配信をダウンロードできません。",
    },
    parse: {
      title: "メディア情報を読み取れません",
      body: "サーバーが予期しない応答を返しました。",
    },
    no_media: {
      title: "ページにダウンロードできるメディアがありません",
      body: "動画が JavaScript で読み込まれる場合、VDrop はまだ検出できません。直接メディアリンクを試してください。",
    },
    ytdlp_missing: {
      title: "yt-dlp がインストールされていません",
      body: "このダウンロードには yt-dlp が必要です。設定 > コンポーネント をご覧ください。",
    },
    ffmpeg_missing: {
      title: "FFmpeg がインストールされていません",
      body: "HLS/DASH 配信の結合には FFmpeg が必要です。設定 > コンポーネント をご覧ください。",
    },
    record_missing: {
      title: "ダウンロード記録を作成できません",
      body: "記録をデータベースに書き込めませんでした。",
    },
    internal: {
      title: "予期しない問題が起きました",
      body: "詳細は下記。",
    },
  },

  common: {
    search: "検索",
    searchPlaceholder: "タイトルとアドレスを検索",
    clearSearch: "検索をクリア",
    noResults: "一致する項目がありません",
    noResultsBody: "別の語で試してください。",
    cancel: "中止",
    confirm: "確認",
    close: "閉じる",
    unknown: "不明",
    audio: "音声",
    video: "映像",
    stream: "配信",
    subtitle: "字幕",
    file: "ファイル",
  },
};

import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const ko: Dictionary = {
  nav: {
    home: "홈",
    queue: "대기열",
    library: "라이브러리",
    history: "기록",
    settings: "설정",
    sections: "섹션",
    engineReady: "엔진 준비됨",
  },

  status_bar: {
    throughput: "전체 속도",
    active: "진행 중",
    pauseAll: "모두 일시정지",
    resumeAll: "모두 재개",
    clearFinished: "완료 항목 지우기",
  },

  home: {
    title: "동영상 링크 해석",
    videoAndAudio: "영상 + 오디오",
    audioOnly: "오디오만",
    subtitlesOnly: "자막",
    noSubtitleTrack: "이 소스에는 자막이 없습니다",
    addToQueue: "대기열에 추가",
    noAudioTrack: "이 소스에는 별도 오디오 트랙이 없습니다",
    legacyTitle: "링크를 붙여넣으세요",
    subtitle:
      "VDrop이 링크를 해석하고 화질은 사용자가 고릅니다. 확인하기 전에는 내려받지 않습니다.",
    placeholder: "https://... 또는 직접 미디어 링크",
    analyze: "해석",
    paste: "클립보드에서 붙여넣기",
    analyzing: "해석 중",
    download: "다운로드",
    changeFolder: "폴더 선택",
    savingTo: "저장 위치",
    streamNotice: "분할 스트림",
    streamNoticeBody:
      "HLS/DASH 스트림입니다. VDrop은 재인코딩 없이 FFmpeg으로 조각을 잇습니다. 이런 다운로드는 취소할 수 있지만 일시정지할 수 없습니다.",
    ffmpegMissing: "FFmpeg을 찾을 수 없음",
    ffmpegMissingBody:
      "분할 스트림(.m3u8 / .mpd)에는 FFmpeg이 필요합니다. 직접 링크는 없이도 동작합니다.",
  },

  queue: {
    title: "활성 대기열",
    completed: "완료됨",
    itemsDownloading: "진행 중",
    analyzing: "스트림 분석 중...",
    connecting: "연결 중...",
    subtitle: "진행 중이거나 완료된 다운로드",
    empty: "대기열이 비어 있습니다",
    emptyBody: "홈에서 링크를 해석하면 여기에 나타납니다.",
    clearFinished: "완료 항목 지우기",
    pause: "일시정지",
    resume: "재개",
    cancel: "취소",
    remove: "목록에서 제거",
    openFolder: "폴더에서 보기",
    openFile: "파일 열기",
    retry: "다시 시도",
  },

  library: {
    title: "라이브러리",
    subtitle: "내려받은 파일",
    empty: "라이브러리가 비어 있습니다",
    emptyBody: "완료된 다운로드는 자동으로 여기에 추가됩니다.",
    missing: "파일이 디스크에서 사라졌습니다",
    removeEntry: "항목 제거",
    deleteFile: "파일 삭제",
  },

  history: {
    title: "기록",
    subtitle: "완료, 취소, 실패한 다운로드",
    empty: "기록이 비어 있습니다",
    emptyBody: "완료된 다운로드는 여기에 기록을 남깁니다.",
    clear: "기록 지우기",
  },

  settings: {
    title: "설정",
    subtitle: "환경설정은 이 컴퓨터에 저장됩니다",

    groupGeneral: "일반",
    theme: "테마",
    themeHint: "시스템 테마를 따르거나 하나로 고정",
    themeSystem: "시스템",
    themeLight: "밝게",
    themeDark: "어둡게",
    language: "언어",
    languageHint: "인터페이스 언어",

    groupDownloads: "다운로드",
    folder: "저장 폴더",
    folderHint: "새 다운로드는 여기에 저장됩니다",
    choose: "변경",
    concurrency: "동시 다운로드 수",
    concurrencyHint: "한 번에 몇 개를 실행할지. 나머지는 대기합니다",
    bandwidth: "속도 제한",
    bandwidthHint: "모든 다운로드의 합계 속도. 0은 제한 없음",
    bandwidthUnit: "KB/초",
    bandwidthUnlimited: "제한 없음",
    autoOpen: "완료 시 폴더 열기",
    autoOpenHint: "다운로드가 끝나면 파일 보이기",
    clipboard: "클립보드 감시",
    clipboardHint: "복사할 때 미디어 링크 잡아내기",
    notifications: "알림",
    notificationsHint: "다운로드가 끝나면 시스템 알림 표시",

    navGeneral: "일반",
    navDownloads: "다운로드",
    navComponents: "구성 요소",
    navAbout: "정보",
    allComponentsOk: "모든 구성 요소 정상",
    someComponentsMissing: "구성 요소가 없습니다",
    installed: "설치됨",
    notInstalled: "설치되지 않음",

    groupComponents: "구성 요소",
    ffmpeg: "FFmpeg",
    ffmpegHint: "분할 스트림(.m3u8 / .mpd)을 잇는 데 사용",
    ffmpegFound: "설치됨",
    ffmpegNotFound: "찾을 수 없음",
    version: "버전",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "사이트별 추출. 설치하면 수백 개 사이트를 지원합니다. 없어도 직접 링크와 스트림은 동작합니다.",
    ytdlpInstallHint: "설치: ",
    ffmpegInstallHint: "설치: ",

    groupAbout: "정보",
    appVersion: "VDrop 버전",
    engine: "코어",
    engineHint: "Rust + Tauri 2. Python이나 yt-dlp에 의존하지 않습니다.",
  },

  status: {
    queued: "대기 중",
    downloading: "내려받는 중",
    paused: "일시정지됨",
    retrying: "다시 시도 중",
    completed: "완료",
    failed: "실패",
    cancelled: "취소됨",
  },

  units: {
    perSecond: "/초",
    remaining: "남음",
    of: "/",
  },

  clipboard: {
    caught: "클립보드에 미디어 링크가 있습니다",
    resolve: "해석",
    dismiss: "무시",
  },

  errors: {
    unknown: {
      title: "예상하지 못한 문제가 발생했습니다",
      body: "자세한 내용은 아래에.",
    },
    empty_url: {
      title: "먼저 링크를 붙여넣으세요",
      body: "동영상 페이지 주소나 직접 미디어 링크를 입력하세요.",
    },
    unsupported: {
      title: "지원하지 않는 주소입니다",
      body: "VDrop이 링크를 알아보지 못했습니다. yt-dlp를 설치하면 훨씬 많은 사이트를 지원합니다.",
    },
    network: {
      title: "서버에 연결할 수 없습니다",
      body: "주소가 맞고 인터넷 연결이 정상인가요?",
    },
    drm: {
      title: "콘텐츠가 DRM으로 보호되어 있습니다",
      body: "VDrop은 DRM으로 보호된 스트림을 내려받을 수 없습니다.",
    },
    parse: {
      title: "미디어 정보를 읽을 수 없습니다",
      body: "서버가 예상 밖의 응답을 보냈습니다.",
    },
    no_media: {
      title: "페이지에 내려받을 미디어가 없습니다",
      body: "동영상이 JavaScript로 불러와진다면 VDrop은 아직 볼 수 없습니다. 직접 미디어 링크를 시도해 보세요.",
    },
    ytdlp_missing: {
      title: "yt-dlp가 설치되지 않았습니다",
      body: "이 다운로드에는 yt-dlp가 필요합니다. 설정 > 구성 요소를 보세요.",
    },
    ffmpeg_missing: {
      title: "FFmpeg이 설치되지 않았습니다",
      body: "HLS/DASH 스트림을 이으려면 FFmpeg이 필요합니다. 설정 > 구성 요소를 보세요.",
    },
    record_missing: {
      title: "다운로드 기록을 만들지 못했습니다",
      body: "기록을 데이터베이스에 쓰지 못했습니다.",
    },
    internal: {
      title: "예상하지 못한 문제가 발생했습니다",
      body: "자세한 내용은 아래에.",
    },
  },

  common: {
    search: "검색",
    searchPlaceholder: "제목과 주소에서 검색",
    clearSearch: "검색 지우기",
    noResults: "일치하는 항목이 없습니다",
    noResultsBody: "다른 검색어로 시도해 보세요.",
    cancel: "취소",
    confirm: "확인",
    close: "닫기",
    unknown: "알 수 없음",
    audio: "오디오",
    video: "영상",
    stream: "스트림",
    subtitle: "자막",
    file: "파일",
  },
};

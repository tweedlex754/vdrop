import type { Dictionary } from "./tr";

// `Dictionary` tipi burada zorunlu tutuldugu icin, tr.ts'e yeni bir anahtar
// eklendiginde ve burada karsiligi yazilmadiginda proje **derlenmez**.
// Ceviri eksigi sessizce urune sizmaz.

export const vi: Dictionary = {
  nav: {
    home: "Trang chủ",
    queue: "Hàng đợi",
    library: "Thư viện",
    history: "Lịch sử",
    settings: "Cài đặt",
    sections: "Mục",
    engineReady: "Bộ máy sẵn sàng",
  },

  status_bar: {
    throughput: "Tốc độ tổng",
    active: "Đang chạy",
    pauseAll: "Tạm dừng tất cả",
    resumeAll: "Tiếp tục tất cả",
    clearFinished: "Xóa mục đã xong",
  },

  home: {
    title: "Phân giải liên kết video",
    videoAndAudio: "Video + âm thanh",
    audioOnly: "Chỉ âm thanh",
    subtitlesOnly: "Phụ đề",
    noSubtitleTrack: "Nguồn này không có phụ đề",
    addToQueue: "Thêm vào hàng đợi",
    noAudioTrack: "Nguồn này không có luồng âm thanh riêng",
    legacyTitle: "Dán một liên kết",
    subtitle:
      "VDrop phân giải liên kết, còn chất lượng do bạn chọn. Không tải gì cho tới khi bạn đồng ý.",
    placeholder: "https://... hoặc liên kết media trực tiếp",
    analyze: "Phân giải",
    paste: "Dán từ khay nhớ tạm",
    analyzing: "Đang phân giải",
    download: "Tải xuống",
    changeFolder: "Chọn thư mục",
    savingTo: "Lưu vào",
    streamNotice: "Luồng phân đoạn",
    streamNoticeBody:
      "Đây là luồng HLS/DASH. VDrop nối các đoạn bằng FFmpeg mà không mã hóa lại. Loại này có thể hủy nhưng không thể tạm dừng.",
    ffmpegMissing: "Không tìm thấy FFmpeg",
    ffmpegMissingBody:
      "Luồng phân đoạn (.m3u8 / .mpd) cần FFmpeg. Liên kết tệp trực tiếp vẫn chạy mà không cần.",
  },

  queue: {
    title: "Hàng đợi đang chạy",
    completed: "Đã hoàn tất",
    itemsDownloading: "đang tải",
    analyzing: "Đang phân tích luồng...",
    connecting: "Đang kết nối...",
    subtitle: "Các lượt tải đang chạy và đã xong",
    empty: "Hàng đợi trống",
    emptyBody: "Phân giải một liên kết ở Trang chủ, các lượt tải sẽ hiện ở đây.",
    clearFinished: "Xóa mục đã xong",
    pause: "Tạm dừng",
    resume: "Tiếp tục",
    cancel: "Hủy",
    remove: "Bỏ khỏi danh sách",
    openFolder: "Hiện trong thư mục",
    openFile: "Mở tệp",
    retry: "Thử lại",
  },

  library: {
    title: "Thư viện",
    subtitle: "Tệp đã tải",
    empty: "Thư viện trống",
    emptyBody: "Các lượt tải xong được thêm vào đây tự động.",
    missing: "Tệp không còn trên đĩa",
    removeEntry: "Bỏ mục",
    deleteFile: "Xóa tệp",
  },

  history: {
    title: "Lịch sử",
    subtitle: "Các lượt tải đã xong, đã hủy và thất bại",
    empty: "Lịch sử trống",
    emptyBody: "Mỗi lượt tải xong đều để lại dấu vết ở đây.",
    clear: "Xóa lịch sử",
  },

  settings: {
    title: "Cài đặt",
    subtitle: "Tùy chọn được lưu trên máy này",

    groupGeneral: "Chung",
    theme: "Giao diện",
    themeHint: "Theo giao diện hệ thống hoặc cố định một kiểu",
    themeSystem: "Hệ thống",
    themeLight: "Sáng",
    themeDark: "Tối",
    language: "Ngôn ngữ",
    languageHint: "Ngôn ngữ giao diện",

    groupDownloads: "Tải xuống",
    folder: "Thư mục tải xuống",
    folderHint: "Các lượt tải mới được lưu vào đây",
    choose: "Đổi",
    concurrency: "Tải xuống đồng thời",
    concurrencyHint: "Bao nhiêu chạy cùng lúc; số còn lại xếp hàng",
    bandwidth: "Giới hạn tốc độ",
    bandwidthHint: "Tốc độ tổng của mọi lượt tải; 0 nghĩa là không giới hạn",
    bandwidthUnit: "KB/giây",
    bandwidthUnlimited: "Không giới hạn",
    autoOpen: "Mở thư mục khi xong",
    autoOpenHint: "Hiện tệp ngay khi tải xong",
    clipboard: "Theo dõi khay nhớ tạm",
    clipboardHint: "Bắt liên kết media khi bạn sao chép",
    notifications: "Thông báo",
    notificationsHint: "Hiện thông báo hệ thống khi tải xong",

    navGeneral: "Chung",
    navDownloads: "Tải xuống",
    navComponents: "Thành phần",
    navAbout: "Giới thiệu",
    allComponentsOk: "Mọi thành phần đều chạy",
    someComponentsMissing: "Thiếu một thành phần",
    installed: "Đã cài",
    notInstalled: "Chưa cài",

    groupComponents: "Thành phần",
    ffmpeg: "FFmpeg",
    ffmpegHint: "Dùng để nối các luồng phân đoạn (.m3u8 / .mpd)",
    ffmpegFound: "Đã cài",
    ffmpegNotFound: "Không tìm thấy",
    version: "Phiên bản",
    ytdlp: "yt-dlp",
    ytdlpHint:
      "Trích xuất riêng theo trang. Có nó thì hàng trăm trang chạy được; không có thì liên kết trực tiếp và luồng vẫn chạy.",
    ytdlpInstallHint: "Để cài: ",
    ffmpegInstallHint: "Để cài: ",

    groupAbout: "Giới thiệu",
    appVersion: "Phiên bản VDrop",
    engine: "Lõi",
    engineHint: "Rust + Tauri 2. Không phụ thuộc Python hay yt-dlp.",
  },

  status: {
    queued: "Trong hàng đợi",
    downloading: "Đang tải",
    paused: "Đã tạm dừng",
    retrying: "Đang thử lại",
    completed: "Xong",
    failed: "Thất bại",
    cancelled: "Đã hủy",
  },

  units: {
    perSecond: "/giây",
    remaining: "còn lại",
    of: "/",
  },

  clipboard: {
    caught: "Có một liên kết media trong khay nhớ tạm",
    resolve: "Phân giải",
    dismiss: "Bỏ qua",
  },

  errors: {
    unknown: {
      title: "Có chuyện ngoài dự tính",
      body: "Chi tiết bên dưới.",
    },
    empty_url: {
      title: "Hãy dán một liên kết trước",
      body: "Nhập địa chỉ trang video, hoặc liên kết media trực tiếp.",
    },
    unsupported: {
      title: "Địa chỉ này không được hỗ trợ",
      body: "VDrop không nhận ra liên kết. Cài yt-dlp thì nhiều trang hơn hẳn sẽ chạy.",
    },
    network: {
      title: "Không liên hệ được máy chủ",
      body: "Địa chỉ có đúng không, và kết nối của bạn có chạy không?",
    },
    drm: {
      title: "Nội dung được bảo vệ bằng DRM",
      body: "VDrop không tải được luồng có bảo vệ DRM.",
    },
    parse: {
      title: "Không đọc được thông tin media",
      body: "Máy chủ trả lời điều gì đó ngoài dự tính.",
    },
    no_media: {
      title: "Không có media nào tải được trên trang",
      body: "Nếu video nạp qua JavaScript thì VDrop chưa thấy được; hãy thử liên kết media trực tiếp.",
    },
    ytdlp_missing: {
      title: "Chưa cài yt-dlp",
      body: "Lượt tải này cần yt-dlp. Xem Cài đặt > Thành phần.",
    },
    ffmpeg_missing: {
      title: "Chưa cài FFmpeg",
      body: "Nối luồng HLS/DASH cần FFmpeg. Xem Cài đặt > Thành phần.",
    },
    record_missing: {
      title: "Không tạo được bản ghi tải xuống",
      body: "Không ghi được bản ghi vào cơ sở dữ liệu.",
    },
    internal: {
      title: "Có chuyện ngoài dự tính",
      body: "Chi tiết bên dưới.",
    },
  },

  common: {
    search: "Tìm kiếm",
    searchPlaceholder: "Tìm trong tiêu đề và địa chỉ",
    clearSearch: "Xóa tìm kiếm",
    noResults: "Không có mục nào khớp",
    noResultsBody: "Thử tìm cách khác.",
    cancel: "Hủy",
    confirm: "Xác nhận",
    close: "Đóng",
    unknown: "Không rõ",
    audio: "Âm thanh",
    video: "Video",
    stream: "Luồng",
    subtitle: "Phụ đề",
    file: "Tệp",
  },
};

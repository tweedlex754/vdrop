// Rust tarafindaki tiplerin birebir TypeScript karsiligi.
//
// Kaynaklar:
//   crates/vdrop-download/src/lib.rs   -> DownloadEvent
//   crates/vdrop-providers/src/lib.rs  -> MediaInfo / StreamOption
//   crates/vdrop-storage/src/lib.rs    -> DownloadRecord / HistoryRecord / LibraryItem
//   src-tauri/src/main.rs              -> AnalyzeResult / AppInfo / DownloadEventPayload
//
// Su an elle senkron tutuluyor. Bu dosyayi Rust'tan uretmek (ts-rs) mantikli
// bir sonraki adim; alan adlari serde'nin snake_case ciktisiyla ayni tutuldu
// ki o gecis mekanik olsun.

export type DownloadEvent =
  | { type: "started"; total_bytes: number | null }
  | {
      type: "progress";
      downloaded_bytes: number;
      total_bytes: number | null;
      speed_bps: number;
      eta_seconds: number | null;
    }
  | { type: "paused"; downloaded_bytes: number }
  | { type: "retrying"; attempt: number; delay_ms: number }
  | { type: "completed"; path: string; total_bytes: number }
  | { type: "failed"; message: string }
  | { type: "cancelled" };

export interface DownloadEventPayload {
  id: string;
  event: DownloadEvent;
}

export type StreamKind = "Video" | "Audio" | "Muxed" | "Subtitle";

export interface StreamOption {
  id: string;
  kind: StreamKind;
  url: string;
  container: string | null;
  codec: string | null;
  resolution: string | null;
  fps: number | null;
  bitrate_kbps: number | null;
  language: string | null;
  /**
   * Yayincinin bu secenege verdigi ad ("English (forced)", "1080"...).
   *
   * Iki altyazi izi de `en` olabilir; biri tam ceviri, digeri yalnizca
   * yabanci replikleri gosteren "forced" iz. Dil kodu tek basina ikisini
   * ayirt etmiyor.
   */
  label: string | null;
  estimated_size_bytes: number | null;
  /**
   * HLS master playlist'teki program indeksi.
   *
   * Bu alan doluyken `url` varyantin kendi playlist'i DEGIL, master
   * manifestin adresidir; secim FFmpeg'e `-map 0:p:N` olarak gecer.
   * Gerekcesi: bazi yayinlarda ses ayri bir renditiondadir ve varyant
   * playlist'i tek basina sessiz video verir.
   */
  variant_index: number | null;
}

export interface MediaInfo {
  title: string;
  uploader: string | null;
  thumbnail_url: string | null;
  duration_seconds: number | null;
  description: string | null;
  upload_date: string | null;
  streams: StreamOption[];
  is_playlist: boolean;
}

export interface AnalyzeResult {
  media: MediaInfo;
  /**
   * Cozumlemeyi yapan saglayici: "yt-dlp" | "hls" | "dash" | "web".
   * Indirme cagrisi buna gore kurulur - yt-dlp formatlari format kimligiyle
   * indirilir, digerleri dogrudan adresle.
   */
  provider_id: string;
  is_stream: boolean;
  ffmpeg_available: boolean;
}

/** Veritabanindaki kalici indirme kaydi. */
export interface DownloadRecord {
  id: string;
  url: string;
  title: string | null;
  destination_path: string;
  total_bytes: number | null;
  downloaded_bytes: number;
  status: DownloadStatus;
  provider_id: string | null;
  kind: DownloadKind;
  error_message: string | null;
  thumbnail_url: string | null;
  /**
   * Secilen HLS kalitesinin program indeksi; duz indirmelerde `null`.
   * Kalici: duraklatilip devam ettirilen bir akis, kullanicinin sectigi
   * kaliteye geri donmeli.
   */
  variant_index: number | null;
  /** yt-dlp format kimligi; diger indirme turlerinde `null`. */
  format_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface HistoryRecord {
  id: string;
  url: string;
  title: string | null;
  status: DownloadStatus;
  destination_path: string | null;
  total_bytes: number | null;
  completed_at: string;
}

export interface LibraryItem {
  id: string;
  title: string | null;
  file_path: string;
  duration_seconds: number | null;
  resolution: string | null;
  codec: string | null;
  file_size_bytes: number | null;
  downloaded_at: string;
}

export interface AppInfo {
  version: string;
  /** "windows" | "macos" | "linux" - kabuktan gelir, sniffing degil. */
  os: string;
  ffmpeg_version: string | null;
  ytdlp_version: string | null;
  default_download_dir: string;
  max_concurrent: number;
}

export type DownloadKind = "http" | "stream" | "ytdlp" | "subtitle";

export type DownloadStatus =
  | "queued"
  | "downloading"
  | "paused"
  | "retrying"
  | "completed"
  | "failed"
  | "cancelled";

/**
 * Kalici kayit (`DownloadRecord`) + yalnizca bellekte yasayan anlik olculer.
 * Hiz ve kalan sure bilincli olarak veritabanina yazilmaz: bunlar ancak
 * indirme calisirken anlamlidir, uygulama yeniden acildiginda "3.2 MB/sn"
 * gostermek yalan olurdu.
 */
export interface DownloadView extends DownloadRecord {
  speedBps: number;
  etaSeconds: number | null;
  /**
   * Son ilerleme olayinin zamani (ms). Bir indirme takilirsa olay akisi
   * susar ama son hiz degeri ekranda asili kalirdi; bekci (watchdog) bu
   * damgaya bakip bayat hizi sifirlar. Olcum aleti yalan soylememeli.
   */
  lastProgressAt: number;
}

export const ACTIVE_STATUSES: DownloadStatus[] = [
  "queued",
  "downloading",
  "retrying",
];

export const FINISHED_STATUSES: DownloadStatus[] = [
  "completed",
  "failed",
  "cancelled",
];

/**
 * Arka ucun dondurdugu yapisal hata.
 *
 * `code` makine icindir ve cevrilir; `detail` teknik izdir (HTTP durumu,
 * ayristirici mesaji) ve tek dilde kalir - hata raporlarinda aranabilir
 * olmasi cevrilmis olmasindan daha degerli.
 */
export interface AppError {
  code: string;
  detail?: string | null;
}

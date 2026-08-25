// Tarayici onizleme koprusu — SADECE gelistirme icin.
//
// VDrop bir Tauri uygulamasidir; `invoke` yalnizca native pencerede calisir.
// Ama arayuzde kucuk bir hizalama duzeltmek icin her seferinde Rust'i
// yeniden derlemek (dakikalar) mantiksiz. Bu modul `npm run dev` ile acilan
// tarayicida IPC'yi bellek ici sahte bir arka ucla taklit eder: HMR ile
// aninda gorursunuz.
//
// Uretim paketine GIRMEZ: cagrilan yer `import.meta.env.DEV` ile korunur,
// Rollup bu dali agactan tamamen budar.

import type {
  AnalyzeResult,
  AppInfo,
  DownloadEventPayload,
  DownloadRecord,
  HistoryRecord,
  LibraryItem,
} from "../types/ipc";

type Listener = (payload: DownloadEventPayload) => void;

const listeners = new Set<Listener>();
const downloads = new Map<string, DownloadRecord>();
const history: HistoryRecord[] = [];
const library: LibraryItem[] = [];
const timers = new Map<string, number>();

const settings: Record<string, string> = {
  theme: "system",
  language: "tr",
  download_folder: "C:\\Users\\Ornek\\Downloads",
  max_concurrent: "3",
  notifications: "on",
  clipboard_watch: "off",
  auto_open_folder: "off",
};

let counter = 0;
const nextId = () => `dev-${++counter}`;
const now = () => new Date().toISOString().replace("T", " ").slice(0, 19);

function emit(payload: DownloadEventPayload) {
  listeners.forEach((fn) => fn(payload));
}

/** Gercekci bir indirme simulasyonu: dalgali hiz, ara sira duraklama. */
function simulate(record: DownloadRecord) {
  const total = record.total_bytes ?? 80 * 1024 * 1024;
  let done = record.downloaded_bytes;

  emit({ id: record.id, event: { type: "started", total_bytes: total } });

  const timer = window.setInterval(() => {
    const current = downloads.get(record.id);
    if (!current || current.status === "paused") return;

    // 1.5-6 MB/sn arasi dalgalanan bir hiz: sabit bir sayi ilerleme
    // cubugunun gercekci gorunmesini engellerdi.
    const speed = (1.5 + Math.random() * 4.5) * 1024 * 1024;
    done = Math.min(total, done + speed * 0.5);
    current.downloaded_bytes = done;

    if (done >= total) {
      window.clearInterval(timer);
      timers.delete(record.id);
      current.status = "completed";
      history.unshift({
        id: nextId(),
        url: current.url,
        title: current.title,
        status: "completed",
        destination_path: current.destination_path,
        total_bytes: total,
        completed_at: now(),
      });
      library.unshift({
        id: current.id,
        title: current.title,
        file_path: current.destination_path,
        duration_seconds: 214,
        resolution: "1920x1080",
        codec: "h264",
        file_size_bytes: total,
        downloaded_at: now(),
      });
      emit({
        id: record.id,
        event: {
          type: "completed",
          path: current.destination_path,
          total_bytes: total,
        },
      });
      return;
    }

    emit({
      id: record.id,
      event: {
        type: "progress",
        downloaded_bytes: Math.round(done),
        total_bytes: total,
        speed_bps: speed,
        eta_seconds: Math.round((total - done) / speed),
      },
    });
  }, 500);

  timers.set(record.id, timer);
}

function stop(id: string) {
  const timer = timers.get(id);
  if (timer) {
    window.clearInterval(timer);
    timers.delete(id);
  }
}

export function devInvoke<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  const result = handle(cmd, args);
  // Gercek IPC gibi asenkron davransin: senkron cozulen bir sahte, gercek
  // dunyada olusacak yarislari gizlerdi.
  return new Promise((resolve) => setTimeout(() => resolve(result as T), 90));
}

function handle(cmd: string, a: Record<string, unknown>): unknown {
  switch (cmd) {
    case "analyze_url": {
      const url = String(a.url ?? "");
      const isStream = /\.(m3u8|mpd)(\?|$)/i.test(url);
      const result: AnalyzeResult = {
        provider_id: isStream ? "hls" : "web",
        is_stream: isStream,
        ffmpeg_available: true,
        media: {
          title: isStream ? "x36xhzz.m3u8" : "Buyuk Tavsan Filmi - 1080p",
          uploader: "Blender Foundation",
          thumbnail_url: null,
          duration_seconds: 634,
          description: null,
          upload_date: null,
          is_playlist: false,
          streams: isStream
            ? // Gercek bir master playlist gibi: birden cok kalite, en
              // yuksek basta, her biri program indeksiyle.
              [
                {
                  id: "hls-4",
                  kind: "Muxed",
                  url,
                  container: "m3u8",
                  codec: "mp4a.40.2,avc1.640028",
                  resolution: "1920x1080",
                  fps: null,
                  bitrate_kbps: 6221,
                  language: null,
                  label: null,
                  estimated_size_bytes: 493 * 1024 * 1024,
                  variant_index: 4,
                },
                {
                  id: "hls-0",
                  kind: "Muxed",
                  url,
                  container: "m3u8",
                  codec: "mp4a.40.2,avc1.64001f",
                  resolution: "1280x720",
                  fps: null,
                  bitrate_kbps: 2149,
                  language: null,
                  label: null,
                  estimated_size_bytes: 170 * 1024 * 1024,
                  variant_index: 0,
                },
                {
                  id: "hls-3",
                  kind: "Muxed",
                  url,
                  container: "m3u8",
                  codec: "mp4a.40.2,avc1.64001f",
                  resolution: "848x480",
                  fps: null,
                  bitrate_kbps: 836,
                  language: null,
                  label: null,
                  estimated_size_bytes: 66 * 1024 * 1024,
                  variant_index: 3,
                },
                {
                  id: "hls-1",
                  kind: "Muxed",
                  url,
                  container: "m3u8",
                  codec: "mp4a.40.5,avc1.42000d",
                  resolution: "320x184",
                  fps: null,
                  bitrate_kbps: 246,
                  language: null,
                  label: null,
                  estimated_size_bytes: 19 * 1024 * 1024,
                  variant_index: 1,
                },
              ]
            : [
                {
                  id: "1080",
                  kind: "Muxed",
                  url,
                  container: "mp4",
                  codec: "h264",
                  resolution: "1920x1080",
                  fps: 30,
                  bitrate_kbps: 4200,
                  language: null,
                  label: null,
                  estimated_size_bytes: 148 * 1024 * 1024,
                  variant_index: null,
                },
                {
                  id: "720",
                  kind: "Muxed",
                  url,
                  container: "mp4",
                  codec: "h264",
                  resolution: "1280x720",
                  fps: 30,
                  bitrate_kbps: 2100,
                  language: null,
                  label: null,
                  estimated_size_bytes: 74 * 1024 * 1024,
                  variant_index: null,
                },
                {
                  id: "audio",
                  kind: "Audio",
                  url,
                  container: "m4a",
                  codec: "aac",
                  resolution: null,
                  fps: null,
                  bitrate_kbps: 128,
                  language: null,
                  label: null,
                  estimated_size_bytes: 9 * 1024 * 1024,
                  variant_index: null,
                },
              ],
        },
      };
      return result;
    }

    case "create_download": {
      const record: DownloadRecord = {
        id: nextId(),
        url: String(a.url ?? ""),
        title: (a.title as string) ?? null,
        destination_path: `${settings.download_folder}\\${a.suggestedName}`,
        total_bytes: 148 * 1024 * 1024,
        downloaded_bytes: 0,
        status: "queued",
        provider_id: "direct-http",
        kind: /\.(m3u8|mpd)(\?|$)/i.test(String(a.url)) ? "stream" : "http",
        variant_index: (a.variantIndex as number) ?? null,
        format_id: (a.formatId as string) ?? null,
        error_message: null,
        thumbnail_url: (a.thumbnailUrl as string) ?? null,
        created_at: now(),
        updated_at: now(),
      };
      downloads.set(record.id, record);
      setTimeout(() => simulate(record), 250);
      return record;
    }

    case "pause_download": {
      const rec = downloads.get(String(a.id));
      if (rec) {
        rec.status = "paused";
        emit({
          id: rec.id,
          event: { type: "paused", downloaded_bytes: rec.downloaded_bytes },
        });
      }
      return null;
    }

    case "resume_download": {
      const rec = downloads.get(String(a.id));
      if (rec) {
        rec.status = "downloading";
        if (!timers.has(rec.id)) simulate(rec);
      }
      return null;
    }

    case "cancel_download": {
      const rec = downloads.get(String(a.id));
      if (rec) {
        stop(rec.id);
        rec.status = "cancelled";
        emit({ id: rec.id, event: { type: "cancelled" } });
      }
      return null;
    }

    case "list_downloads":
      return [...downloads.values()].reverse();

    case "remove_download":
      stop(String(a.id));
      downloads.delete(String(a.id));
      return null;

    case "clear_finished": {
      let n = 0;
      for (const [id, rec] of downloads) {
        if (["completed", "failed", "cancelled"].includes(rec.status)) {
          downloads.delete(id);
          n++;
        }
      }
      return n;
    }

    case "list_history":
      return history;

    case "clear_history": {
      const n = history.length;
      history.length = 0;
      return n;
    }

    case "list_library":
      return library;

    case "remove_library_item": {
      const i = library.findIndex((x) => x.id === a.id);
      if (i >= 0) library.splice(i, 1);
      return null;
    }

    case "paths_exist":
      return Object.fromEntries(
        (a.paths as string[]).map((p) => [p, true])
      );

    case "get_settings":
      return { ...settings };

    case "set_setting":
      settings[String(a.key)] = String(a.value);
      return null;

    case "select_download_folder":
      return "C:\\Users\\Ornek\\Videos";

    case "app_info":
      return {
        version: "0.1.0-dev",
        os: "windows",
        ffmpeg_version: "ffmpeg version N-125875 (tarayici onizlemesi)",
        ytdlp_version: "2026.08.01 (tarayici onizlemesi)",
        default_download_dir: settings.download_folder,
        max_concurrent: Number(settings.max_concurrent),
      } satisfies AppInfo;

    case "open_path":
    case "reveal_path":
      console.info(`[onizleme] ${cmd}:`, a.path);
      return null;

    default:
      throw new Error(`[onizleme] bilinmeyen komut: ${cmd}`);
  }
}

export function devListen(handler: Listener): () => void {
  listeners.add(handler);
  return () => listeners.delete(handler);
}

import type { MediaInfo, StreamOption } from "../types/ipc";

/**
 * Bayt bicimleme. Ikili katlar (1024) kullanilir cunku dosya boyutlari
 * isletim sisteminde de boyle gorunur; kullanici Explorer'daki sayiyla
 * VDrop'taki sayiyi karsilastirdiginda ayni olsun.
 *
 * Sabit ondalik basamak: sayi 9.9 -> 10.0 gecerken satir genisligi
 * degismesin diye 1 basamakta sabitlenir.
 */
export function formatBytes(bytes: number | null | undefined): string {
  if (bytes == null || !Number.isFinite(bytes) || bytes < 0) return "—";
  if (bytes < 1024) return `${Math.round(bytes)} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let i = 0;
  while (value >= 1024 && i < units.length - 1) {
    value /= 1024;
    i += 1;
  }
  return `${value.toFixed(1)} ${units[i]}`;
}

export function formatSpeed(bps: number, perSecondSuffix: string): string {
  if (!Number.isFinite(bps) || bps <= 0) return "—";
  return `${formatBytes(bps)}${perSecondSuffix}`;
}

/** Kalan sure: 1 saatin altinda m:ss, ustunde s:mm:ss. */
export function formatEta(seconds: number | null | undefined): string {
  if (seconds == null || !Number.isFinite(seconds) || seconds < 0) return "—";
  const s = Math.floor(seconds % 60);
  const m = Math.floor((seconds / 60) % 60);
  const h = Math.floor(seconds / 3600);
  const pad = (n: number) => n.toString().padStart(2, "0");
  return h > 0 ? `${h}:${pad(m)}:${pad(s)}` : `${m}:${pad(s)}`;
}

export function formatDuration(seconds: number | null | undefined): string | null {
  if (seconds == null || !Number.isFinite(seconds) || seconds <= 0) return null;
  return formatEta(seconds);
}

/** SQLite `datetime('now')` UTC yazar; yerel saate cevirip kisa gosteriyoruz. */
export function formatTimestamp(value: string, locale: string): string {
  const iso = value.includes("T") ? value : value.replace(" ", "T") + "Z";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleString(locale, {
    day: "2-digit",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function percent(done: number, total: number | null | undefined): number {
  if (!total || total <= 0) return 0;
  return Math.min(100, Math.max(0, (done / total) * 100));
}

/**
 * Kullaniciya onerilecek dosya adi.
 *
 * Buradaki temizlik yalnizca **gorsel** bir kolayliktir; gercek guvenlik
 * kontrolu Rust tarafinda `safe_join` ile yapilir. Arayuz katmanina
 * guvenlik yuku bindirmiyoruz: kotu niyetli bir provider bu fonksiyonu
 * atlayip dogrudan IPC cagirabilir.
 */
export function suggestFilename(
  media: MediaInfo | null,
  stream: StreamOption
): string {
  const raw = (media?.title || "vdrop-indirme").trim();
  // Uzantiyi ayri ekleyecegiz; baslikta zaten varsa tekrar etmesin.
  const base = raw.replace(/\.(mp4|mkv|webm|mp3|m4a|m3u8|mpd)$/i, "");
  const ext = pickExtension(stream);
  return `${base}.${ext}`;
}

function pickExtension(stream: StreamOption): string {
  const c = stream.container?.toLowerCase();
  // Manifest uzantilari dosya uzantisi degildir: HLS/DASH birlestirilince
  // sonuc bir mp4 olur.
  if (!c || c === "m3u8" || c === "mpd" || c === "hls-or-dash") {
    return stream.kind === "Audio" ? "m4a" : "mp4";
  }
  return c;
}

/** Yol icinden dosya adini ayirir (Windows ve POSIX ayraclari). */
export function basename(path: string): string {
  const parts = path.split(/[\\/]/);
  return parts[parts.length - 1] || path;
}

/**
 * RFC 6381 kodek dizesini okunabilir ada cevirir.
 *
 * HLS manifestleri kodekleri `avc1.640028,mp4a.40.2` gibi bildirir. Bu
 * kullaniciya hicbir sey soylemez; "H.264 · AAC" ise kalite karsilastirmasi
 * yaparken gercekten ise yarar (ornegin AV1 ayni bit hizinda daha iyidir).
 *
 * Taninmayan kodlar oldugu gibi birakilir: uydurmaktansa ham veriyi
 * gostermek daha durust.
 */
export function prettyCodecs(codecs: string | null): string | null {
  if (!codecs) return null;
  const names = codecs
    .split(",")
    .map((c) => prettyCodec(c.trim()))
    .filter((c): c is string => Boolean(c));
  // Ayni aile iki kez gecebilir (ornegin iki video akisi); tekrar etmeyelim.
  return [...new Set(names)].join(" · ") || null;
}

function prettyCodec(code: string): string | null {
  if (!code) return null;
  const family = code.split(".")[0].toLowerCase();
  switch (family) {
    case "avc1":
    case "avc3":
      return "H.264";
    case "hev1":
    case "hvc1":
      return "H.265";
    case "av01":
      return "AV1";
    case "vp09":
    case "vp9":
      return "VP9";
    case "vp08":
    case "vp8":
      return "VP8";
    case "mp4a":
      // mp4a.40.2 = AAC-LC, mp4a.40.5 = HE-AAC. Ayrimi gostermiyoruz:
      // kullanicinin karari icin "AAC" yeterli, alt profil gurultu.
      return "AAC";
    case "ec-3":
      return "E-AC-3";
    case "ac-3":
      return "AC-3";
    case "opus":
      return "Opus";
    case "flac":
      return "FLAC";
    case "mp3":
      return "MP3";
    default:
      return code;
  }
}

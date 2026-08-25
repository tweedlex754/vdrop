import type { StreamOption } from "../types/ipc";
import { formatBytes, prettyCodecs } from "./format";

export interface StreamLabels {
  audio: string;
  stream: string;
  file: string;
  subtitle: string;
}

export interface StreamLabel {
  name: string;
  specs: string[];
}

/**
 * Bir format satirinin adi ve teknik sutunu.
 *
 * Iki kural:
 *
 * 1. **Ad, satirlari birbirinden ayiran eksendir.** Kullanicinin burada
 *    verdigi karar kalite kararidir - videoda cozunurluk, seste bit hizi.
 *    Geri kalan her sey (kapsayici, kodek, boyut) sagdaki teknik sutuna
 *    duser.
 * 2. **Ad ile sutun ayni seyi soylemez.** Ada tasinan olcu sutundan
 *    cikarilir; yoksa "Ses 128 kbps ... 128 kbps" gibi bir tekrar olur ve
 *    goz hangisinin onemli oldugunu ayirt edemez.
 *
 * Bilinmeyen degerler uydurulmaz: cozunurlugu olmayan duz bir CDN linkine
 * "1080p" demek kullaniciyi yanlis yonlendirir.
 */
export function describeStream(
  stream: StreamOption,
  labels: StreamLabels
): StreamLabel {
  // Altyazi dali EN BASTA: altyazi izinin cozunurlugu ve bit hizi yok,
  // asagidaki kontroller onu "Dosya - SRT" diye adlandirirdi. Kullanicinin
  // burada aradigi sey dil, bicim degil.
  if (stream.kind === "Subtitle") {
    const name = stream.label ?? stream.language ?? labels.subtitle;
    const specs = stream.language && stream.label ? [stream.language] : [];
    return { name, specs };
  }

  const container = stream.container?.toUpperCase();
  const isManifest =
    container === "M3U8" || container === "MPD" || container === "HLS-OR-DASH";

  const specs: string[] = [];
  if (container && !isManifest) specs.push(container);
  const codec = prettyCodecs(stream.codec);
  if (codec) specs.push(codec);

  const size =
    stream.estimated_size_bytes != null
      ? formatBytes(stream.estimated_size_bytes)
      : null;
  const bitrate =
    stream.bitrate_kbps != null ? `${stream.bitrate_kbps} kbps` : null;

  // DIKKAT: manifest kontrolu cozunurluk kontrolunden SONRA gelmeli.
  // HLS varyantlarinin hepsinin kapsayicisi "m3u8"dir; once manifeste
  // bakarsak 1080p ve 240p satirlarinin ikisi de "Akis" der ve kalite
  // listesi ise yaramaz hale gelir.
  const quality = shorthandResolution(stream.resolution);
  if (quality) {
    // 60 fps ayirt edici bir kalite farkidir, adin parcasi olmali.
    const name =
      stream.fps && stream.fps >= 50
        ? `${quality}${Math.round(stream.fps)}`
        : quality;
    if (bitrate) specs.push(bitrate);
    if (size) specs.push(size);
    return { name, specs };
  }

  // Cozunurlugu olmayan bir manifest: tek renditionlu akis ya da DASH.
  // Burada "Akis" dogru ad - uydurulacak bir kalite bilgisi yok.
  if (isManifest) {
    if (bitrate) specs.push(bitrate);
    if (size) specs.push(size);
    return { name: labels.stream, specs };
  }

  if (stream.kind === "Audio") {
    // Ses akislarinda ayirt edici eksen bit hizidir: ada o cikar, sutunda
    // tekrar edilmez.
    if (size) specs.push(size);
    return { name: bitrate ?? labels.audio, specs };
  }

  if (bitrate) specs.push(bitrate);
  if (size) specs.push(size);
  return { name: labels.file, specs };
}

/**
 * "1920x1080" -> "1080p". Dikey videolarda kisa kenar alinir.
 *
 * Zaten "480p" gibi bir etiket geldiyse (sayfa cikariminda yayincinin kendi
 * dosya adindan okunur) oldugu gibi birakilir.
 */
export function shorthandResolution(resolution: string | null): string | null {
  if (!resolution) return null;
  const match = resolution.match(/(\d+)\s*[x×]\s*(\d+)/i);
  if (!match) return resolution;
  const shorter = Math.min(Number(match[1]), Number(match[2]));
  return `${shorter}p`;
}

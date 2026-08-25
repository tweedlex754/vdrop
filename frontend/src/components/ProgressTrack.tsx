import type { DownloadStatus } from "../types/ipc";

/**
 * IMZA BILESEN: segmentli transfer izi.
 *
 * Duz bir ilerleme cubugu yerine segmentli bir iz kullaniyoruz cunku transfer
 * gercekten de parcalar halinde gelir: HTTP chunk'lari, HLS segmentleri.
 * Gorsel form isin gercek mekanigini yansitir, sadece suslemek icin degildir.
 *
 * Toplam boyut bilinmiyorsa (Content-Length yok, ya da FFmpeg akisinin
 * basi) belirsiz (indeterminate) moda gecer: yalanci bir yuzde uydurmak
 * yerine "calisiyor ama ne kadar kaldigini bilmiyorum" der.
 */
export function ProgressTrack({
  percent,
  status,
  indeterminate,
}: {
  percent: number;
  status: DownloadStatus;
  indeterminate?: boolean;
}) {
  const tone =
    status === "downloading"
      ? "is-live"
      : status === "completed"
        ? "is-done"
        : status === "failed"
          ? "is-error"
          : "";

  const showIndeterminate =
    indeterminate && (status === "downloading" || status === "retrying");

  return (
    <div
      className={`track ${tone} ${showIndeterminate ? "indeterminate" : ""}`}
      role="progressbar"
      aria-valuemin={0}
      aria-valuemax={100}
      // Belirsiz modda aria-valuenow verilmez: ekran okuyucu "yuzde 0" diye
      // okumak yerine "belirsiz ilerleme" der.
      aria-valuenow={showIndeterminate ? undefined : Math.round(percent)}
    >
      <div
        className="track-fill"
        style={{ width: `${showIndeterminate ? 35 : percent}%` }}
      />
    </div>
  );
}

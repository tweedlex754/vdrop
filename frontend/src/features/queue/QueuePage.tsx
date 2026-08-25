import { useT } from "../../i18n";
import * as api from "../../services/vdrop";
import { useDownloads } from "../../stores/downloadsStore";
import type { DownloadView } from "../../types/ipc";
import {
  basename,
  formatBytes,
  formatEta,
  formatSpeed,
  percent,
} from "../../lib/format";
import { ProgressTrack } from "../../components/ProgressTrack";
import { Icon } from "../../components/Icon";
import { EmptyState, IconButton, StatusPill } from "../../components/ui";

export function QueuePage() {
  const t = useT();
  const { items, activeCount, totalSpeedBps } = useDownloads();

  const active = items.filter(
    (i) => !["completed", "cancelled", "failed"].includes(i.status)
  );
  const finished = items.filter((i) =>
    ["completed", "cancelled", "failed"].includes(i.status)
  );

  if (items.length === 0) {
    return (
      <div className="content-inner">
        <EmptyState icon="inbox" title={t.queue.empty} body={t.queue.emptyBody} />
      </div>
    );
  }

  return (
    <div className="content-inner">
      {active.length > 0 && (
        <section className="queue-section">
          <header className="page-head">
            <div className="page-head-main">
              <span className="page-head-icon">
                <Icon name="download" size={26} />
              </span>
              <div>
                <h1 className="title">{t.queue.title}</h1>
                <p className="page-sub num">
                  {activeCount} {t.queue.itemsDownloading}
                  {totalSpeedBps > 0 && (
                    <>
                      {" · "}
                      {formatBytes(totalSpeedBps)}
                      {t.units.perSecond}
                    </>
                  )}
                </p>
              </div>
            </div>
          </header>

          <div className="rows">
            {active.map((item) => (
              <DownloadRow key={item.id} item={item} />
            ))}
          </div>
        </section>
      )}

      {finished.length > 0 && (
        <section className="queue-section">
          <div className="queue-section-head">
            <Icon name="check" size={18} style={{ color: "var(--success)" }} />
            <h2 className="section-title">{t.queue.completed}</h2>
          </div>
          <div className="rows">
            {finished.map((item) => (
              <DownloadRow key={item.id} item={item} compact />
            ))}
          </div>
        </section>
      )}
    </div>
  );
}

function DownloadRow({
  item,
  compact,
}: {
  item: DownloadView;
  compact?: boolean;
}) {
  const t = useT();
  const { pause, resume, cancel, remove } = useDownloads();

  const pct = percent(item.downloaded_bytes, item.total_bytes);
  const running = item.status === "downloading" || item.status === "retrying";
  const finished = ["completed", "cancelled", "failed"].includes(item.status);
  const starting = item.status === "queued" || item.total_bytes == null;

  // FFmpeg ve yt-dlp alt surecleri guvenli sekilde duraklatilamaz... ama
  // ikisinde de "duraklat" transferi sonlandirip `.part` dosyasini
  // biraktigi icin devam ettirme calisir. Yani HTTP ile ayni: her ucunde
  // Duraklat gosteriliyor.
  const canPause = running || item.status === "queued";
  const canResume = item.status === "paused" || item.status === "failed";

  return (
    <article className="row">
      <div className="row-thumb">
        {item.thumbnail_url ? (
          <img
            src={item.thumbnail_url}
            alt=""
            onError={(e) => {
              e.currentTarget.style.display = "none";
            }}
          />
        ) : (
          <Icon name="image" size={22} />
        )}
      </div>

      <div className="row-main">
        <div className="row-top">
          <span className="row-title" title={item.title ?? item.url}>
            {item.title || basename(item.destination_path)}
          </span>
          <StatusPill status={item.status} />

          <div className="row-actions">
            {canPause && (
              <IconButton
                icon="pause"
                label={t.queue.pause}
                onClick={() => void pause(item.id)}
              />
            )}
            {canResume && (
              <IconButton
                icon={item.status === "failed" ? "refresh" : "play"}
                label={item.status === "failed" ? t.queue.retry : t.queue.resume}
                onClick={() => void resume(item.id)}
              />
            )}
            {item.status === "completed" && (
              <>
                <IconButton
                  icon="file"
                  label={t.queue.openFile}
                  onClick={() => void api.openPath(item.destination_path)}
                />
                <IconButton
                  icon="folder"
                  label={t.queue.openFolder}
                  onClick={() => void api.revealPath(item.destination_path)}
                />
              </>
            )}
            {!finished && (
              <IconButton
                icon="close"
                label={t.queue.cancel}
                onClick={() => void cancel(item.id)}
              />
            )}
            <IconButton
              icon="trash"
              label={t.queue.remove}
              tone="danger"
              onClick={() => void remove(item.id, false)}
            />
          </div>
        </div>

        <div className="row-meta">
          <span>{sourceLabel(item)}</span>
          {item.total_bytes != null && (
            <>
              <span aria-hidden="true">·</span>
              <span className="num">
                {formatBytes(item.downloaded_bytes)} / {formatBytes(item.total_bytes)}
              </span>
            </>
          )}
        </div>

        {!compact && (
          <div className="row-progress">
            <ProgressTrack
              percent={pct}
              status={item.status}
              indeterminate={item.total_bytes == null}
            />
            {item.total_bytes != null && (
              <span className="caption num">{Math.round(pct)}%</span>
            )}
          </div>
        )}

        <div className="row-stats">
          {starting && running && <span>{t.queue.connecting}</span>}
          {running && item.speedBps > 0 && (
            <span className="live">
              {formatSpeed(item.speedBps, t.units.perSecond)}
            </span>
          )}
          {running && item.etaSeconds != null && (
            <span>
              {formatEta(item.etaSeconds)} {t.units.remaining}
            </span>
          )}
          {item.status === "failed" && item.error_message && (
            <span className="row-error" title={item.error_message}>
              {item.error_message}
            </span>
          )}
        </div>
      </div>
    </article>
  );
}

/**
 * Satirin ikinci satirindaki kaynak ozeti: "youtube.com · MP4 · 1080p".
 *
 * Alan adini adresten cikariyoruz - kullanicinin bu indirmeyi hatirlamasi
 * icin en hizli ipucu, hangi siteden geldigi.
 */
function sourceLabel(item: DownloadView): string {
  const parts: string[] = [];
  try {
    parts.push(new URL(item.url).hostname.replace(/^www\./, ""));
  } catch {
    // Adres cozumlenemiyorsa alan adi da yok; kalan bilgiyle devam.
  }
  const ext = item.destination_path.split(".").pop();
  if (ext && ext.length <= 5) parts.push(ext.toUpperCase());
  return parts.join(" · ") || item.kind;
}

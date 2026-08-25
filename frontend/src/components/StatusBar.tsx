import { useT } from "../i18n";
import { useDownloads } from "../stores/downloadsStore";
import { useSettings } from "../stores/settingsStore";
import { formatBytes } from "../lib/format";

/**
 * Uygulamanin altinda, her ekranda gorunen durum cubugu.
 *
 * Iki isi var:
 *
 * 1. **Toplam okuma.** Kullanici Ayarlar'da olsa bile "bir sey iniyor mu, ne
 *    hizda, kac tane" sorusunun cevabi hep ayni yerde. Bir indirme
 *    yoneticisini, sirasi gelince bakilan bir listeden ayiran sey arka planda
 *    olan biteni surekli gorunur tutmasidir.
 *
 * 2. **Toplu eylemler.** Hepsini duraklat / devam ettir / bitenleri temizle.
 *    Bunlar kuyruk ekranina ozgu degil: kullanici bir toplantiya girerken
 *    hangi ekranda olursa olsun hepsini duraklatabilmeli.
 */
export function StatusBar() {
  const t = useT();
  const { items, activeCount, totalSpeedBps, pause, resume, clearFinished } =
    useDownloads();
  const { settings } = useSettings();

  const live = totalSpeedBps > 0;
  const pausable = items.filter(
    (i) => i.status === "downloading" || i.status === "queued"
  );
  const resumable = items.filter((i) => i.status === "paused");
  const finished = items.filter((i) =>
    ["completed", "cancelled", "failed"].includes(i.status)
  );

  return (
    <footer className="statusbar">
      <div className={`statusbar-metrics ${live ? "live" : ""}`}>
        {live && <span className="live-dot" aria-hidden="true" />}
        <span>
          {t.status_bar.throughput}: {live ? `${formatBytes(totalSpeedBps)}${t.units.perSecond}` : "0 KB/s"}
        </span>
        <span aria-hidden="true">·</span>
        <span>
          {t.status_bar.active}: {activeCount}
          <span className="muted">/{settings.max_concurrent}</span>
        </span>
      </div>

      <div className="statusbar-actions">
        <button
          onClick={() => pausable.forEach((i) => void pause(i.id))}
          disabled={pausable.length === 0}
        >
          {t.status_bar.pauseAll}
        </button>
        <button
          onClick={() => resumable.forEach((i) => void resume(i.id))}
          disabled={resumable.length === 0}
        >
          {t.status_bar.resumeAll}
        </button>
        <button
          onClick={() => void clearFinished()}
          disabled={finished.length === 0}
        >
          {t.status_bar.clearFinished}
        </button>
      </div>
    </footer>
  );
}

import { useEffect } from "react";
import { useT } from "../i18n";
import type { ClipboardLink } from "../services/vdrop";
import { Icon } from "./Icon";

/**
 * Panoda yakalanan baglanti icin bildirim seridi.
 *
 * Ilkeler:
 *  - **Engellemez.** Modal degil; kullanici gormezden gelip isine devam
 *    edebilir. Yakalanan bir link, kullanicinin isini bolecek kadar onemli
 *    degildir.
 *  - **Kendiliginden kaybolur.** Kapatilmayi bekleyen bir serit, bir sure
 *    sonra arayuzun kalici bir parcasi gibi gorunmeye baslar.
 *  - **Adres degil ad gosterir.** Imzali bir CDN URL'i 400 karakter olabilir;
 *    kullaniciya lazim olan dosya adidir.
 */
const AUTO_DISMISS_MS = 15_000;

export function ClipboardToast({
  link,
  onResolve,
  onDismiss,
}: {
  link: ClipboardLink;
  onResolve: (url: string) => void;
  onDismiss: () => void;
}) {
  const t = useT();

  // Her yeni baglanti sayaci sifirlar (link.url bagimliliginda).
  useEffect(() => {
    const timer = window.setTimeout(onDismiss, AUTO_DISMISS_MS);
    return () => window.clearTimeout(timer);
  }, [link.url, onDismiss]);

  return (
    <div className="toast" role="status" aria-live="polite">
      <span className="toast-icon">
        <Icon name="inbox" size={16} />
      </span>

      <div className="toast-text">
        <span className="toast-title">{t.clipboard.caught}</span>
        <span className="toast-label" title={link.url}>
          {link.label}
        </span>
      </div>

      <button className="btn btn-primary" onClick={() => onResolve(link.url)}>
        {t.clipboard.resolve}
      </button>
      <button
        className="btn btn-ghost"
        onClick={onDismiss}
        aria-label={t.clipboard.dismiss}
        title={t.clipboard.dismiss}
      >
        <Icon name="close" size={15} />
      </button>
    </div>
  );
}

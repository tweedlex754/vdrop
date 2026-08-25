import type { ReactNode } from "react";
import { useT } from "../i18n";
import type { DownloadStatus } from "../types/ipc";
import { Icon, type IconName } from "./Icon";

export function StatusPill({ status }: { status: DownloadStatus }) {
  const t = useT();
  return <span className={`pill pill-${status}`}>{t.status[status]}</span>;
}

export function EmptyState({
  icon,
  title,
  body,
}: {
  icon: IconName;
  title: string;
  body: string;
}) {
  return (
    <div className="empty">
      <span className="empty-icon">
        <Icon name={icon} size={28} />
      </span>
      <h2 className="section-title">{title}</h2>
      <p>{body}</p>
    </div>
  );
}

export function PageHead({
  title,
  subtitle,
  icon,
  actions,
}: {
  title: string;
  subtitle?: string;
  icon?: IconName;
  actions?: ReactNode;
}) {
  return (
    <header className="page-head">
      <div className="page-head-main">
        {icon && (
          <span className="page-head-icon">
            <Icon name={icon} size={26} />
          </span>
        )}
        <div>
          <h1 className="title">{title}</h1>
          {subtitle && <p className="page-sub">{subtitle}</p>}
        </div>
      </div>
      {actions && <div style={{ display: "flex", gap: 8 }}>{actions}</div>}
    </header>
  );
}

export function Note({
  tone,
  title,
  body,
}: {
  tone: "info" | "warn" | "danger";
  title: string;
  body: string;
}) {
  const icon: IconName = tone === "info" ? "inbox" : "alert";
  return (
    <div
      className={`note note-${tone}`}
      role={tone === "danger" ? "alert" : undefined}
    >
      <Icon name={icon} size={18} />
      <div>
        <strong className="note-title">{title}</strong>
        <span className="note-body">{body}</span>
      </div>
    </div>
  );
}

/**
 * Sadece ikondan olusan dugme.
 *
 * Gorsel olarak etiketsiz oldugu icin `aria-label` ve `title` zorunlu
 * tutuluyor: hem ekran okuyucu hem fareyle bekleyen kullanici dugmenin ne
 * yaptigini ogrenebilsin. Satirlarin ustune gelince belirdikleri icin
 * (bkz. `.row-actions`) etiketsiz birakmak iki kez cezalandirici olurdu.
 */
export function IconButton({
  icon,
  label,
  onClick,
  tone,
  disabled,
}: {
  icon: IconName;
  label: string;
  onClick: () => void;
  tone?: "danger";
  disabled?: boolean;
}) {
  return (
    <button
      className={`btn btn-ghost ${tone === "danger" ? "btn-danger" : ""}`}
      onClick={onClick}
      disabled={disabled}
      aria-label={label}
      title={label}
    >
      <Icon name={icon} size={16} />
    </button>
  );
}

export function Segmented<T extends string>({
  value,
  options,
  onChange,
  ariaLabel,
}: {
  value: T;
  options: { value: T; label: string }[];
  onChange: (value: T) => void;
  ariaLabel: string;
}) {
  return (
    <div className="segmented" role="group" aria-label={ariaLabel}>
      {options.map((opt) => (
        <button
          key={opt.value}
          aria-pressed={value === opt.value}
          onClick={() => onChange(opt.value)}
        >
          {opt.label}
        </button>
      ))}
    </div>
  );
}

export function Switch({
  checked,
  onChange,
  ariaLabel,
}: {
  checked: boolean;
  onChange: (next: boolean) => void;
  ariaLabel: string;
}) {
  return (
    <button
      role="switch"
      aria-checked={checked}
      aria-label={ariaLabel}
      className="switch"
      onClick={() => onChange(!checked)}
    />
  );
}

export function SettingRow({
  name,
  hint,
  children,
}: {
  name: string;
  hint?: string;
  children: ReactNode;
}) {
  return (
    <div className="setting">
      <div className="setting-text">
        <div className="setting-name">{name}</div>
        {hint && <div className="setting-hint">{hint}</div>}
      </div>
      <div className="setting-control">{children}</div>
    </div>
  );
}

/**
 * Liste ekranlarinin arama kutusu.
 *
 * `PageHead`'in `actions` yuvasina konuyor: arama bir eylem degil bir
 * filtre, ama basligin yaninda durunca hangi listeyi daralttigi gorunur
 * kaliyor. Temizleme dugmesi yalnizca yazi varken cikiyor - bos kutunun
 * yaninda duran bir carpi ne yapacagini soylemiyor.
 */
export function SearchBox({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) {
  const t = useT();
  return (
    <div className="searchbox">
      <Icon name="search" size={15} className="muted" />
      <input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={t.common.searchPlaceholder}
        aria-label={t.common.search}
        spellCheck={false}
        autoComplete="off"
      />
      {value !== "" && (
        <button
          className="searchbox-clear"
          onClick={() => onChange("")}
          aria-label={t.common.clearSearch}
          title={t.common.clearSearch}
        >
          <Icon name="close" size={13} />
        </button>
      )}
    </div>
  );
}

import { useT } from "../i18n";
import { useDownloads } from "../stores/downloadsStore";
import { useSettings } from "../stores/settingsStore";
import { BrandMark, Icon, type IconName } from "./Icon";

export type Section = "home" | "queue" | "library" | "history" | "settings";

const PRIMARY: { id: Section; icon: IconName }[] = [
  { id: "home", icon: "home" },
  { id: "queue", icon: "queue" },
  { id: "library", icon: "library" },
  { id: "history", icon: "history" },
];

export function Sidebar({
  section,
  onNavigate,
}: {
  section: Section;
  onNavigate: (s: Section) => void;
}) {
  const t = useT();
  const { activeCount } = useDownloads();
  const { appInfo } = useSettings();

  return (
    <aside className="sidebar">
      <div className="brand">
        <div className="brand-mark">
          <BrandMark />
        </div>
        <div className="brand-text">
          <div className="brand-name">VDrop</div>
          <div className="caption path">{t.nav.engineReady}</div>
        </div>
      </div>

      <nav className="nav" aria-label={t.nav.sections}>
        {PRIMARY.map((item) => (
          <NavItem
            key={item.id}
            item={item}
            active={section === item.id}
            label={t.nav[item.id]}
            count={item.id === "queue" ? activeCount : 0}
            onClick={() => onNavigate(item.id)}
          />
        ))}
      </nav>

      <div className="sidebar-foot">
        <NavItem
          item={{ id: "settings", icon: "settings" }}
          active={section === "settings"}
          label={t.nav.settings}
          count={0}
          onClick={() => onNavigate("settings")}
        />

        {/* Bilesen durumu. Alt durum cubugu anlik hizi gosterir; burasi
            "sistem hazir mi" sorusunun cevabi - eksik bir bilesen ancak
            indirmeye kalkinca degil, en bastan gorunmeli. */}
        <div style={{ marginTop: 8 }}>
          <ComponentRow
            icon="terminal"
            label="yt-dlp"
            present={Boolean(appInfo?.ytdlp_version)}
          />
          <ComponentRow
            icon="film"
            label="FFmpeg"
            present={Boolean(appInfo?.ffmpeg_version)}
          />
        </div>
      </div>
    </aside>
  );
}

function NavItem({
  item,
  active,
  label,
  count,
  onClick,
}: {
  item: { id: Section; icon: IconName };
  active: boolean;
  label: string;
  count: number;
  onClick: () => void;
}) {
  return (
    <button
      className={`nav-item ${active ? "active" : ""}`}
      aria-current={active ? "page" : undefined}
      onClick={onClick}
      title={label}
    >
      <Icon name={item.icon} size={20} />
      <span>{label}</span>
      {count > 0 && <span className="nav-count">{count}</span>}
    </button>
  );
}

function ComponentRow({
  icon,
  label,
  present,
}: {
  icon: IconName;
  label: string;
  present: boolean;
}) {
  return (
    <div className={`engine-row ${present ? "ok" : "missing"}`}>
      <Icon name={present ? "check" : "alert"} size={14} />
      <span>{label}</span>
      <Icon name={icon} size={13} className="sr-only" />
    </div>
  );
}

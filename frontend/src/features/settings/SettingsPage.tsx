import { useState } from "react";
import {
  LANGUAGE_CODES,
  LANGUAGES,
  useT,
  type LanguageCode,
} from "../../i18n";
import * as api from "../../services/vdrop";
import { useSettings, type ThemePreference } from "../../stores/settingsStore";
import { Icon, type IconName } from "../../components/Icon";
import { PageHead, SettingRow, Segmented, Switch } from "../../components/ui";
import {
  ffmpegInstallCommand,
  ytdlpInstallCommand,
} from "../../lib/installCommands";

type Panel = "general" | "downloads" | "components" | "about";

export function SettingsPage() {
  const t = useT();
  const { settings, appInfo, update } = useSettings();
  const [panel, setPanel] = useState<Panel>("general");

  const sections: { id: Panel; label: string }[] = [
    { id: "general", label: t.settings.navGeneral },
    { id: "downloads", label: t.settings.navDownloads },
    { id: "components", label: t.settings.navComponents },
    { id: "about", label: t.settings.navAbout },
  ];

  return (
    <div className="content-inner">
      <PageHead title={t.settings.title} subtitle={t.settings.subtitle} />

      <div className="settings">
        {/* Alt navigasyon: ayarlar tek bir uzun sayfa oldugunda kullanici
            aradigini kaydirarak arar. Bolumlere ayirmak, her bolumu tek
            ekranda gorunur tutuyor. */}
        <nav className="settings-nav" aria-label={t.settings.title}>
          {sections.map((s) => (
            <button
              key={s.id}
              aria-current={panel === s.id ? "true" : undefined}
              onClick={() => setPanel(s.id)}
            >
              {s.label}
            </button>
          ))}
        </nav>

        <div>
          {panel === "general" && (
            <div className="panel">
              <SettingRow name={t.settings.theme} hint={t.settings.themeHint}>
                <Segmented<ThemePreference>
                  ariaLabel={t.settings.theme}
                  value={settings.theme}
                  onChange={(v) => void update("theme", v)}
                  options={[
                    { value: "system", label: t.settings.themeSystem },
                    { value: "light", label: t.settings.themeLight },
                    { value: "dark", label: t.settings.themeDark },
                  ]}
                />
              </SettingRow>

              <SettingRow
                name={t.settings.language}
                hint={t.settings.languageHint}
              >
                {/* Segmented degil acilir liste: yan yana dugmeler iki dille
                    calisiyordu, yirmiyle satira sigmaz ve grubun hizasini
                    bozardi (ayni hata `.track-toggle`de ucuncu dugme
                    eklenince goruldu). Her dil KENDI adiyla yaziliyor:
                    yanlis dile dusen kullanicinin geri donebilmesi icin
                    listede tanidik bir sey gormesi gerekir. */}
                <select
                  className="format-select setting-select"
                  value={settings.language}
                  aria-label={t.settings.language}
                  onChange={(e) => void update("language", e.target.value as LanguageCode)}
                >
                  {LANGUAGE_CODES.map((code) => (
                    <option key={code} value={code}>
                      {LANGUAGES[code].label}
                    </option>
                  ))}
                </select>
              </SettingRow>

              <SettingRow
                name={t.settings.notifications}
                hint={t.settings.notificationsHint}
              >
                <Switch
                  ariaLabel={t.settings.notifications}
                  checked={settings.notifications}
                  onChange={(v) => void update("notifications", v)}
                />
              </SettingRow>
            </div>
          )}

          {panel === "downloads" && (
            <div className="panel">
              <SettingRow name={t.settings.folder} hint={t.settings.folderHint}>
                <span
                  className="path caption"
                  style={{ maxWidth: 240 }}
                  title={settings.download_folder}
                >
                  {settings.download_folder || "—"}
                </span>
                <button
                  className="btn"
                  onClick={async () => {
                    const folder = await api.selectDownloadFolder();
                    if (folder) await update("download_folder", folder);
                  }}
                >
                  <Icon name="folder" size={15} />
                  {t.settings.choose}
                </button>
              </SettingRow>

              <SettingRow
                name={t.settings.concurrency}
                hint={t.settings.concurrencyHint}
              >
                <input
                  className="slider"
                  type="range"
                  min={1}
                  max={16}
                  step={1}
                  value={settings.max_concurrent}
                  aria-label={t.settings.concurrency}
                  onChange={(e) =>
                    void update("max_concurrent", Number(e.target.value))
                  }
                />
                <span className="setting-value">{settings.max_concurrent}</span>
              </SettingRow>

              <SettingRow
                name={t.settings.bandwidth}
                hint={t.settings.bandwidthHint}
              >
                {/* Kaydirac degil sayi girdisi: aralik 0'dan on binlere
                    uzaniyor ve kullanicinin aklinda genelde belirli bir
                    rakam var ("500 KB/sn"), bir oran degil. */}
                <input
                  className="setting-number num"
                  type="number"
                  min={0}
                  step={50}
                  value={settings.bandwidth_limit_kbps}
                  aria-label={t.settings.bandwidth}
                  onChange={(e) =>
                    void update(
                      "bandwidth_limit_kbps",
                      Math.max(0, Number(e.target.value) || 0)
                    )
                  }
                />
                <span className="setting-value">
                  {settings.bandwidth_limit_kbps === 0
                    ? t.settings.bandwidthUnlimited
                    : t.settings.bandwidthUnit}
                </span>
              </SettingRow>

              <SettingRow
                name={t.settings.autoOpen}
                hint={t.settings.autoOpenHint}
              >
                <Switch
                  ariaLabel={t.settings.autoOpen}
                  checked={settings.auto_open_folder}
                  onChange={(v) => void update("auto_open_folder", v)}
                />
              </SettingRow>

              <SettingRow
                name={t.settings.clipboard}
                hint={t.settings.clipboardHint}
              >
                <Switch
                  ariaLabel={t.settings.clipboard}
                  checked={settings.clipboard_watch}
                  onChange={(v) => void update("clipboard_watch", v)}
                />
              </SettingRow>
            </div>
          )}

          {panel === "components" && <Components />}

          {panel === "about" && (
            <div className="panel">
              <SettingRow name={t.settings.appVersion}>
                <span className="setting-value">{appInfo?.version ?? "—"}</span>
              </SettingRow>
              <SettingRow name={t.settings.engine} hint={t.settings.engineHint}>
                <span className="setting-value">Rust · Tauri 2</span>
              </SettingRow>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function Components() {
  const t = useT();
  const { appInfo } = useSettings();

  // Kurulum ipucunun komut kismi dile degil isletim sistemine bagli. Kabuk
  // bilgisi gelmeden once Windows'a dusmek yerine bos birakmiyoruz: ipucu
  // zaten yalnizca bilesen eksikken gosteriliyor ve o an appInfo hazir.
  const os = appInfo?.os ?? "";

  const items: {
    icon: IconName;
    name: string;
    hint: string;
    version: string | null;
    installHint: string;
  }[] = [
    {
      icon: "terminal",
      name: t.settings.ytdlp,
      hint: t.settings.ytdlpHint,
      version: appInfo?.ytdlp_version ?? null,
      installHint: `${t.settings.ytdlpInstallHint}${ytdlpInstallCommand(os)}`,
    },
    {
      icon: "film",
      name: t.settings.ffmpeg,
      hint: t.settings.ffmpegHint,
      version: appInfo?.ffmpeg_version ?? null,
      installHint: `${t.settings.ffmpegInstallHint}${ffmpegInstallCommand(os)}`,
    },
  ];

  const allPresent = items.every((i) => i.version);

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
      <div
        className={`note ${allPresent ? "note-info" : "note-warn"}`}
        style={{ width: "100%" }}
      >
        <Icon name={allPresent ? "check" : "alert"} size={17} />
        <span>
          {allPresent
            ? t.settings.allComponentsOk
            : t.settings.someComponentsMissing}
        </span>
      </div>

      {items.map((item) => (
        <div
          key={item.name}
          className={`component ${item.version ? "" : "needs-attention"}`}
        >
          <span className="component-icon">
            <Icon name={item.icon} size={22} />
          </span>

          <div className="component-text">
            <div className="component-name">
              {item.name}
              <span
                className={`pill ${item.version ? "pill-completed" : "pill-retrying"}`}
              >
                {item.version ? t.settings.installed : t.settings.notInstalled}
              </span>
            </div>
            {/* Surum dizesi secilebilir: bir sorun bildirirken kullanicinin
                kopyalayacagi ilk sey budur. */}
            <div className="component-status selectable">
              {item.version ?? `${item.hint} — ${item.installHint}`}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}

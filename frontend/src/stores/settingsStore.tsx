import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import * as api from "../services/vdrop";
import type { AppInfo } from "../types/ipc";
import { isLanguageCode, type LanguageCode } from "../i18n";

export type ThemePreference = "system" | "light" | "dark";

export interface Settings {
  theme: ThemePreference;
  language: LanguageCode;
  download_folder: string;
  max_concurrent: number;
  /** Toplam indirme hizi siniri, KB/sn. **0 = sinirsiz.** */
  bandwidth_limit_kbps: number;
  notifications: boolean;
  clipboard_watch: boolean;
  auto_open_folder: boolean;
}

const FALLBACK: Settings = {
  theme: "system",
  language: "tr",
  download_folder: "",
  max_concurrent: 3,
  bandwidth_limit_kbps: 0,
  notifications: true,
  clipboard_watch: false,
  auto_open_folder: false,
};

/**
 * SQLite ayarlari `TEXT` olarak saklar (sema basit ve ileri uyumlu kalsin
 * diye). Cozumleme burada yapilir; bozuk ya da eksik bir deger uygulamayi
 * cokertmek yerine sessizce varsayilana duser.
 */
function parseSettings(raw: Record<string, string>): Settings {
  const bool = (key: keyof Settings, fallback: boolean) => {
    const v = raw[key];
    if (v == null) return fallback;
    return v === "on" || v === "true" || v === "1";
  };

  const theme = raw.theme;
  const lang = raw.language;
  const concurrency = Number.parseInt(raw.max_concurrent ?? "", 10);
  const bandwidth = Number.parseInt(raw.bandwidth_limit_kbps ?? "", 10);

  return {
    theme:
      theme === "light" || theme === "dark" || theme === "system"
        ? theme
        : FALLBACK.theme,
    language: lang && isLanguageCode(lang) ? lang : FALLBACK.language,
    download_folder: raw.download_folder ?? FALLBACK.download_folder,
    max_concurrent: Number.isFinite(concurrency)
      ? Math.min(16, Math.max(1, concurrency))
      : FALLBACK.max_concurrent,
    // Negatif ya da bozuk bir deger sinirsiza duser. Ters yon - bozuk ayar
    // yuzunden indirmelerin 1 KB/sn'ye inmesi - cok daha kotu bir hata olurdu.
    bandwidth_limit_kbps:
      Number.isFinite(bandwidth) && bandwidth > 0 ? bandwidth : FALLBACK.bandwidth_limit_kbps,
    notifications: bool("notifications", FALLBACK.notifications),
    clipboard_watch: bool("clipboard_watch", FALLBACK.clipboard_watch),
    auto_open_folder: bool("auto_open_folder", FALLBACK.auto_open_folder),
  };
}

/** Ayarlar SQLite'ta TEXT tutulur; boolean'lar "on"/"off" olarak yazilir. */
function serialize(value: Settings[keyof Settings]): string {
  if (typeof value === "boolean") return value ? "on" : "off";
  return String(value);
}

interface Ctx {
  settings: Settings;
  appInfo: AppInfo | null;
  ready: boolean;
  update: <K extends keyof Settings>(key: K, value: Settings[K]) => Promise<void>;
}

const SettingsContext = createContext<Ctx | null>(null);

export function SettingsProvider({ children }: { children: ReactNode }) {
  const [settings, setSettings] = useState<Settings>(FALLBACK);
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      try {
        const [raw, info] = await Promise.all([
          api.getSettings(),
          api.getAppInfo(),
        ]);
        if (cancelled) return;
        setSettings(parseSettings(raw));
        setAppInfo(info);
      } catch (err) {
        // Ayarlar okunamazsa uygulama yine de acilmali: varsayilanlarla
        // devam eder, kullanici Ayarlar ekranindan duzeltebilir.
        console.error("Ayarlar okunamadi:", err);
      } finally {
        if (!cancelled) setReady(true);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const update = useCallback(
    async <K extends keyof Settings>(key: K, value: Settings[K]) => {
      // Iyimser guncelleme: arayuz beklemeden tepki verir. Yazma basarisiz
      // olursa eski degere geri doneriz.
      const previous = settings;
      setSettings((s) => ({ ...s, [key]: value }));
      try {
        await api.setSetting(key, serialize(value));
      } catch (err) {
        console.error(`Ayar yazilamadi (${key}):`, err);
        setSettings(previous);
      }
    },
    [settings]
  );

  const value = useMemo<Ctx>(
    () => ({ settings, appInfo, ready, update }),
    [settings, appInfo, ready, update]
  );

  return (
    <SettingsContext.Provider value={value}>{children}</SettingsContext.Provider>
  );
}

export function useSettings(): Ctx {
  const ctx = useContext(SettingsContext);
  if (!ctx)
    throw new Error("useSettings, SettingsProvider icinde kullanilmalidir");
  return ctx;
}

/**
 * Tema tercihini <html data-theme> uzerine yansitir. "system" secildiginde
 * ozniteligi tamamen kaldiririz; boylece tokens.css'teki
 * `prefers-color-scheme` medya sorgusu devreye girer ve kullanici isletim
 * sistemi temasini degistirdiginde uygulama aninda uyar.
 */
export function useAppliedTheme(theme: ThemePreference) {
  useEffect(() => {
    const root = document.documentElement;
    if (theme === "system") root.removeAttribute("data-theme");
    else root.setAttribute("data-theme", theme);

    // index.html'deki kucuk betik acilista bu degeri okur ve React
    // yuklenmeden temayi uygular. Aksi halde koyu tema kullanan biri her
    // aciliste bir kare beyaz ekran gorurdu.
    try {
      localStorage.setItem("vdrop.theme", theme);
    } catch {
      // Depolama kapaliysa onemli degil: tek kaybimiz o ilk kare.
    }
  }, [theme]);
}

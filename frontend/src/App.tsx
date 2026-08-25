import { useCallback, useEffect, useRef, useState } from "react";
import { I18nProvider } from "./i18n";
import {
  SettingsProvider,
  useAppliedTheme,
  useSettings,
} from "./stores/settingsStore";
import { DownloadsProvider } from "./stores/downloadsStore";
import { Sidebar, type Section } from "./components/Sidebar";
import { StatusBar } from "./components/StatusBar";
import { ClipboardToast } from "./components/ClipboardToast";
import { HomePage } from "./features/home/HomePage";
import { QueuePage } from "./features/queue/QueuePage";
import { LibraryPage } from "./features/library/LibraryPage";
import { HistoryPage } from "./features/history/HistoryPage";
import { SettingsPage } from "./features/settings/SettingsPage";
import * as api from "./services/vdrop";
import type { DownloadView } from "./types/ipc";
import type { ClipboardLink } from "./services/vdrop";

export default function App() {
  return (
    <SettingsProvider>
      <Localized />
    </SettingsProvider>
  );
}

/**
 * Ayarlar yuklendikten sonra dil ve temayi uygular.
 *
 * `SettingsProvider` disarida, `I18nProvider` icerde: dil bir ayardir, o
 * yuzden once ayarlarin okunmus olmasi gerekir. Ayarlar hazir olana kadar
 * hicbir sey cizmiyoruz - aksi halde uygulama once Turkce acilip sonra
 * Ingilizce'ye atlardi.
 */
function Localized() {
  const { settings, ready } = useSettings();
  useAppliedTheme(settings.theme);

  if (!ready) return null;

  return (
    <I18nProvider lang={settings.language}>
      <Shell />
    </I18nProvider>
  );
}

function Shell() {
  const [section, setSection] = useState<Section>("home");
  const [caught, setCaught] = useState<ClipboardLink | null>(null);
  // Ana ekrana "su baglantiyi al" demenin yolu. Sayac, ayni baglanti ikinci
  // kez gonderildiginde de effect'in tetiklenmesini saglar.
  const [handoff, setHandoff] = useState<{ url: string; nonce: number } | null>(
    null
  );
  const nonce = useRef(0);
  const { settings } = useSettings();

  const handleCompleted = useCallback(
    (item: DownloadView) => {
      if (settings.auto_open_folder) {
        void api.revealPath(item.destination_path);
      }
    },
    [settings.auto_open_folder]
  );

  // Pano izleyici arka ucta calisir; burada sadece sonucu gosteriyoruz.
  // Ayar kapaliyken arka uc zaten olay yaymaz, ekstra bir kontrol gerekmez.
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let disposed = false;
    void api
      .onClipboardLink((link) => setCaught(link))
      .then((fn) => (disposed ? fn() : (unlisten = fn)));
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  const resolveCaught = useCallback((url: string) => {
    setCaught(null);
    nonce.current += 1;
    setHandoff({ url, nonce: nonce.current });
    setSection("home");
  }, []);

  return (
    <DownloadsProvider onCompleted={handleCompleted}>
      <div className="shell">
        <Sidebar section={section} onNavigate={setSection} />

        <main className="content">
          {section === "home" && (
            <HomePage onStarted={() => setSection("queue")} handoff={handoff} />
          )}
          {section === "queue" && <QueuePage />}
          {section === "library" && <LibraryPage />}
          {section === "history" && <HistoryPage />}
          {section === "settings" && <SettingsPage />}
        </main>

        <StatusBar />
      </div>

      {caught && (
        <ClipboardToast
          link={caught}
          onResolve={resolveCaught}
          onDismiss={() => setCaught(null)}
        />
      )}
    </DownloadsProvider>
  );
}

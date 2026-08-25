import { useCallback, useEffect, useState } from "react";
import { useLocale, useT } from "../../i18n";
import * as api from "../../services/vdrop";
import type { LibraryItem } from "../../types/ipc";
import { basename, formatBytes, formatTimestamp } from "../../lib/format";
import {
  EmptyState,
  IconButton,
  PageHead,
  SearchBox,
} from "../../components/ui";
import { matchesQuery } from "../../lib/search";

export function LibraryPage() {
  const t = useT();
  const { lang } = useLocale();
  const [items, setItems] = useState<LibraryItem[]>([]);
  const [present, setPresent] = useState<Record<string, boolean>>({});
  const [loaded, setLoaded] = useState(false);
  const [query, setQuery] = useState("");

  const load = useCallback(async () => {
    const rows = await api.listLibrary();
    setItems(rows);
    setLoaded(true);
    // Kullanici dosyayi Explorer'dan silmis olabilir. Kutuphane bunu bilmeden
    // "ac" dugmesi gosterirse tiklama sessizce hicbir sey yapmaz; onun yerine
    // kaydi "diskte yok" diye isaretliyoruz.
    if (rows.length > 0) {
      setPresent(await api.pathsExist(rows.map((r) => r.file_path)));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  // Kullanici bu ekrandayken bir indirme biterse liste kendiliginden
  // guncellensin. Ekranlar gezinmede yeniden baglandigi icin diger tazelik
  // yollari zaten kapali; acik kalan tek durum buydu.
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let disposed = false;
    void api
      .onDownloadEvent(({ event }) => {
        if (event.type === "completed") void load();
      })
      .then((fn) => (disposed ? fn() : (unlisten = fn)));
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [load]);

  if (loaded && items.length === 0) {
    return (
      <div className="content-inner">
        <EmptyState
          icon="library"
          title={t.library.empty}
          body={t.library.emptyBody}
        />
      </div>
    );
  }

  // Baslik VE dosya yolu aranir: kullanici bazen "hangi videoydu" diye
  // baslikta, bazen "hangi klasordeydi" diye yolda arar.
  const visible = items.filter((item) =>
    matchesQuery(query, [item.title, item.file_path])
  );

  return (
    <div className="content-inner">
      <PageHead
        title={t.library.title}
        subtitle={t.library.subtitle}
        actions={<SearchBox value={query} onChange={setQuery} />}
      />

      {visible.length === 0 ? (
        <EmptyState
          icon="search"
          title={t.common.noResults}
          body={t.common.noResultsBody}
        />
      ) : (
      <div className="panel">
        {visible.map((item) => {
          const exists = present[item.file_path] !== false;
          return (
            <div className="list-item" key={item.id}>
              <div className="list-main">
                <div className={`list-title ${exists ? "" : "missing"}`}>
                  {item.title || basename(item.file_path)}
                </div>
                <div className="list-sub" title={item.file_path}>
                  {exists ? item.file_path : t.library.missing}
                </div>
              </div>

              <span className="num" style={{ fontSize: 11, opacity: 0.7 }}>
                {formatBytes(item.file_size_bytes)}
              </span>
              <span className="num" style={{ fontSize: 11, opacity: 0.7 }}>
                {formatTimestamp(item.downloaded_at, lang)}
              </span>

              <div className="row-actions">
                {exists && (
                  <>
                    <IconButton
                      icon="file"
                      label={t.queue.openFile}
                      onClick={() => void api.openPath(item.file_path)}
                    />
                    <IconButton
                      icon="folder"
                      label={t.queue.openFolder}
                      onClick={() => void api.revealPath(item.file_path)}
                    />
                  </>
                )}
                <IconButton
                  icon="close"
                  label={t.library.removeEntry}
                  onClick={async () => {
                    await api.removeLibraryItem(item.id, false);
                    await load();
                  }}
                />
                {exists && (
                  <IconButton
                    icon="trash"
                    label={t.library.deleteFile}
                    tone="danger"
                    onClick={async () => {
                      await api.removeLibraryItem(item.id, true);
                      await load();
                    }}
                  />
                )}
              </div>
            </div>
          );
        })}
      </div>
      )}
    </div>
  );
}

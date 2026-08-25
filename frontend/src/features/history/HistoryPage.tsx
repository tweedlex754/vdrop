import { useCallback, useEffect, useState } from "react";
import { useLocale, useT } from "../../i18n";
import * as api from "../../services/vdrop";
import type { HistoryRecord } from "../../types/ipc";
import { formatBytes, formatTimestamp } from "../../lib/format";
import {
  EmptyState,
  PageHead,
  SearchBox,
  StatusPill,
} from "../../components/ui";
import { matchesQuery } from "../../lib/search";

export function HistoryPage() {
  const t = useT();
  const { lang } = useLocale();
  const [rows, setRows] = useState<HistoryRecord[]>([]);
  const [loaded, setLoaded] = useState(false);
  const [query, setQuery] = useState("");

  const load = useCallback(async () => {
    setRows(await api.listHistory(200));
    setLoaded(true);
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  if (loaded && rows.length === 0) {
    return (
      <div className="content-inner">
        <EmptyState
          icon="history"
          title={t.history.empty}
          body={t.history.emptyBody}
        />
      </div>
    );
  }

  const visible = rows.filter((row) => matchesQuery(query, [row.title, row.url]));

  return (
    <div className="content-inner">
      <PageHead
        title={t.history.title}
        subtitle={t.history.subtitle}
        actions={
          <>
          <SearchBox value={query} onChange={setQuery} />
          <button
            className="btn btn-danger"
            onClick={async () => {
              await api.clearHistory();
              await load();
            }}
          >
            {t.history.clear}
          </button>
          </>
        }
      />

      {visible.length === 0 ? (
        <EmptyState
          icon="search"
          title={t.common.noResults}
          body={t.common.noResultsBody}
        />
      ) : (
      <div className="panel">
        {visible.map((row) => (
          <div className="list-item" key={row.id}>
            <div className="list-main">
              <div className="list-title">{row.title || row.url}</div>
              <div className="list-sub" title={row.url}>
                {row.url}
              </div>
            </div>

            <span className="num" style={{ fontSize: 11, opacity: 0.7 }}>
              {formatBytes(row.total_bytes)}
            </span>
            <span className="num" style={{ fontSize: 11, opacity: 0.7 }}>
              {formatTimestamp(row.completed_at, lang)}
            </span>
            <StatusPill status={row.status} />
          </div>
        ))}
      </div>
      )}
    </div>
  );
}

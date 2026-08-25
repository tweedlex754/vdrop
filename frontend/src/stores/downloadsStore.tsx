import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  type ReactNode,
} from "react";
import type { DownloadView } from "../types/ipc";
import * as api from "../services/vdrop";
import {
  EMPTY,
  aggregate,
  reducer,
  selectItems,
  toView,
} from "./downloadsReducer";

/**
 * Indirme durumunun tek kaynagi.
 *
 * Durum makinesinin kendisi `downloadsReducer.ts` icinde ve saf; burasi
 * yalnizca onu React'e ve IPC'ye bagliyor. Bolme sebebi test edilebilirlik:
 * olay -> durum donusumunu bir bilesen render ederek sinamak hem yavas hem
 * dolayli olurdu.
 */

interface Ctx {
  items: DownloadView[];
  loaded: boolean;
  activeCount: number;
  totalSpeedBps: number;
  refresh: () => Promise<void>;
  create: (args: api.CreateDownloadArgs) => Promise<DownloadView>;
  pause: (id: string) => Promise<void>;
  resume: (id: string) => Promise<void>;
  cancel: (id: string) => Promise<void>;
  remove: (id: string, deleteFile: boolean) => Promise<void>;
  clearFinished: () => Promise<void>;
}

const DownloadsContext = createContext<Ctx | null>(null);

export function DownloadsProvider({
  children,
  onCompleted,
}: {
  children: ReactNode;
  /** Tamamlanan indirme icin yan etki (ornegin klasoru acmak). */
  onCompleted?: (item: DownloadView) => void;
}) {
  const [state, dispatch] = useReducer(reducer, EMPTY);

  // Callback'i ref'te tutuyoruz: her render'da yeniden olusan bir prop
  // yuzunden olay aboneligi sokulup yeniden kurulmasin.
  const completedRef = useRef(onCompleted);
  completedRef.current = onCompleted;

  const stateRef = useRef(state);
  stateRef.current = state;

  const refresh = useCallback(async () => {
    const records = await api.listDownloads();
    dispatch({ kind: "hydrate", records });
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let disposed = false;

    void api
      .onDownloadEvent(({ id, event }) => {
        if (!(id in stateRef.current.byId)) {
          // Baska bir pencereden/oturumdan gelmis olabilir: listeyi tazele.
          void refresh();
          return;
        }
        dispatch({ kind: "event", id, event });

        if (event.type === "completed") {
          const item = stateRef.current.byId[id];
          if (item) {
            completedRef.current?.({
              ...item,
              status: "completed",
              destination_path: event.path,
            });
          }
        }
      })
      .then((fn) => {
        if (disposed) fn();
        else unlisten = fn;
      });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [refresh]);

  // Bekci: 5 saniyedir ilerleme olayi gelmeyen bir indirmenin hizi sifirlanir.
  useEffect(() => {
    const timer = window.setInterval(() => {
      dispatch({ kind: "expireStaleSpeeds", olderThan: Date.now() - 5000 });
    }, 2500);
    return () => window.clearInterval(timer);
  }, []);

  const create = useCallback(async (args: api.CreateDownloadArgs) => {
    const record = await api.createDownload(args);
    dispatch({ kind: "upsert", record });
    return toView(record);
  }, []);

  const pause = useCallback(async (id: string) => {
    await api.pauseDownload(id);
  }, []);

  const resume = useCallback(async (id: string) => {
    await api.resumeDownload(id);
  }, []);

  const cancel = useCallback(async (id: string) => {
    await api.cancelDownload(id);
  }, []);

  const remove = useCallback(async (id: string, deleteFile: boolean) => {
    await api.removeDownload(id, deleteFile);
    dispatch({ kind: "remove", id });
  }, []);

  const clearFinished = useCallback(async () => {
    await api.clearFinished();
    const records = await api.listDownloads();
    dispatch({ kind: "hydrate", records });
  }, []);

  const items = useMemo(() => selectItems(state), [state]);
  const { activeCount, totalSpeedBps } = useMemo(() => aggregate(items), [items]);

  const value: Ctx = {
    items,
    loaded: state.loaded,
    activeCount,
    totalSpeedBps,
    refresh,
    create,
    pause,
    resume,
    cancel,
    remove,
    clearFinished,
  };

  return (
    <DownloadsContext.Provider value={value}>
      {children}
    </DownloadsContext.Provider>
  );
}

export function useDownloads(): Ctx {
  const ctx = useContext(DownloadsContext);
  if (!ctx)
    throw new Error("useDownloads, DownloadsProvider icinde kullanilmalidir");
  return ctx;
}

import type { DownloadEvent, DownloadRecord, DownloadView } from "../types/ipc";

/**
 * Indirme listesinin durum makinesi — **saf**, React'ten bagimsiz.
 *
 * Ayri bir dosyada olmasinin sebebi test edilebilirlik: burada gercek is
 * mantigi var (olaylarin duruma cevrilmesi, bayat hizlarin sifirlanmasi,
 * veritabani ile bellek arasindaki birlestirme) ve bunu bir bileseni
 * render ederek test etmek hem yavas hem dolayli olurdu.
 *
 * Iki veri akisi birlestirilir:
 *   1. SQLite'taki kalici kayitlar (acilis ve islem sonrasi)
 *   2. Motorun canli olaylari (hiz, kalan sure, anlik bayt)
 *
 * Catisma olursa **veritabani kazanir**: arayuz hicbir zaman gercekte
 * olmayan bir durumu gostermemeli.
 */

export type State = {
  byId: Record<string, DownloadView>;
  order: string[];
  loaded: boolean;
};

export type Action =
  | { kind: "hydrate"; records: DownloadRecord[] }
  | { kind: "upsert"; record: DownloadRecord }
  | { kind: "event"; id: string; event: DownloadEvent }
  | { kind: "remove"; id: string }
  | { kind: "expireStaleSpeeds"; olderThan: number };

export const EMPTY: State = { byId: {}, order: [], loaded: false };

/**
 * Kalici kaydi goruntu nesnesine cevirir.
 *
 * Hiz ve kalan sure onceki goruntuden tasinir: bunlar veritabaninda yok
 * (olmamali da - yeniden acilista "3.2 MB/sn" gostermek yalan olurdu), ama
 * bir tazeleme sirasinda ekranda titremeleri de gerekmez.
 */
export function toView(
  record: DownloadRecord,
  previous?: DownloadView
): DownloadView {
  return {
    ...record,
    speedBps: previous?.speedBps ?? 0,
    etaSeconds: previous?.etaSeconds ?? null,
    lastProgressAt: previous?.lastProgressAt ?? 0,
  };
}

export function reducer(state: State, action: Action): State {
  switch (action.kind) {
    case "hydrate": {
      const byId: Record<string, DownloadView> = {};
      for (const rec of action.records) {
        byId[rec.id] = toView(rec, state.byId[rec.id]);
      }
      return { byId, order: action.records.map((r) => r.id), loaded: true };
    }

    case "upsert": {
      const rec = action.record;
      const exists = rec.id in state.byId;
      return {
        ...state,
        byId: { ...state.byId, [rec.id]: toView(rec, state.byId[rec.id]) },
        // Yeni kayitlar basa: kullanici az once ekledigi seyi aramasin.
        order: exists ? state.order : [rec.id, ...state.order],
      };
    }

    case "remove": {
      const { [action.id]: _dropped, ...rest } = state.byId;
      return {
        ...state,
        byId: rest,
        order: state.order.filter((id) => id !== action.id),
      };
    }

    case "expireStaleSpeeds": {
      let changed = false;
      const byId = { ...state.byId };
      for (const [id, item] of Object.entries(byId)) {
        const stalled =
          item.status === "downloading" &&
          item.speedBps > 0 &&
          item.lastProgressAt < action.olderThan;
        if (stalled) {
          byId[id] = { ...item, speedBps: 0, etaSeconds: null };
          changed = true;
        }
      }
      // Referansi degistirmemek onemli: gereksiz bir yeni nesne, listeyi
      // izleyen her bileseni yeniden cizdirirdi.
      return changed ? { ...state, byId } : state;
    }

    case "event": {
      const existing = state.byId[action.id];
      // Bilinmeyen kimlik: cagiran taraf tazeleme yapar. Buradan uydurma
      // bir kayit yaratmak, veritabaninda olmayan bir satir gosterirdi.
      if (!existing) return state;
      const next = applyEvent(existing, action.event);
      return next === existing
        ? state
        : { ...state, byId: { ...state.byId, [action.id]: next } };
    }
  }
}

export function applyEvent(
  item: DownloadView,
  event: DownloadEvent,
  now: number = Date.now()
): DownloadView {
  switch (event.type) {
    case "started":
      return {
        ...item,
        status: "downloading",
        total_bytes: event.total_bytes,
        error_message: null,
      };

    case "progress":
      return {
        ...item,
        status: "downloading",
        downloaded_bytes: event.downloaded_bytes,
        total_bytes: event.total_bytes,
        speedBps: event.speed_bps,
        etaSeconds: event.eta_seconds,
        lastProgressAt: now,
      };

    case "paused":
      return {
        ...item,
        status: "paused",
        downloaded_bytes: event.downloaded_bytes,
        speedBps: 0,
        etaSeconds: null,
      };

    case "retrying":
      return { ...item, status: "retrying", speedBps: 0, etaSeconds: null };

    case "completed":
      return {
        ...item,
        status: "completed",
        downloaded_bytes: event.total_bytes,
        total_bytes: event.total_bytes,
        destination_path: event.path,
        speedBps: 0,
        etaSeconds: null,
        error_message: null,
      };

    case "failed":
      return {
        ...item,
        status: "failed",
        error_message: event.message,
        speedBps: 0,
        etaSeconds: null,
      };

    case "cancelled":
      return { ...item, status: "cancelled", speedBps: 0, etaSeconds: null };
  }
}

/** Kenar cubugundaki telemetri seridinin okudugu toplamlar. */
export function aggregate(items: DownloadView[]): {
  activeCount: number;
  totalSpeedBps: number;
} {
  let activeCount = 0;
  let totalSpeedBps = 0;
  for (const item of items) {
    if (item.status === "downloading" || item.status === "retrying") {
      activeCount += 1;
    }
    // Hiz yalnizca gercekten akan indirmelerden toplanir: yeniden deneme
    // bekleyen bir isin hizi sifirdir ama "aktif" sayilir.
    if (item.status === "downloading") totalSpeedBps += item.speedBps;
  }
  return { activeCount, totalSpeedBps };
}

export function selectItems(state: State): DownloadView[] {
  return state.order.map((id) => state.byId[id]).filter(Boolean);
}

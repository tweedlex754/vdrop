import { describe, expect, it } from "vitest";
import type { DownloadEvent, DownloadRecord } from "../types/ipc";
import {
  EMPTY,
  aggregate,
  applyEvent,
  reducer,
  selectItems,
  toView,
  type State,
} from "./downloadsReducer";

function record(id: string, over: Partial<DownloadRecord> = {}): DownloadRecord {
  return {
    id,
    url: `https://x.com/${id}.mp4`,
    title: id,
    destination_path: `C:\\indirilenler\\${id}.mp4`,
    total_bytes: 1000,
    downloaded_bytes: 0,
    status: "queued",
    provider_id: "web",
    kind: "http",
    error_message: null,
    thumbnail_url: null,
    variant_index: null,
    format_id: null,
    created_at: "2026-08-24 12:00:00",
    updated_at: "2026-08-24 12:00:00",
    ...over,
  };
}

function withItems(...records: DownloadRecord[]): State {
  return reducer(EMPTY, { kind: "hydrate", records });
}

describe("hydrate", () => {
  it("veritabanindaki sirayi korur ve yuklendi olarak isaretler", () => {
    const state = withItems(record("a"), record("b"));
    expect(state.order).toEqual(["a", "b"]);
    expect(state.loaded).toBe(true);
    expect(selectItems(state).map((i) => i.id)).toEqual(["a", "b"]);
  });

  it("canli olculeri korur ama kalici alanlari veritabanindan alir", () => {
    // Senaryo: indirme akarken bir tazeleme geliyor. Hiz ekranda titremesin,
    // ama durum/bayt veritabaninin dedigi olsun.
    let state = withItems(record("a"));
    state = reducer(state, {
      kind: "event",
      id: "a",
      event: {
        type: "progress",
        downloaded_bytes: 500,
        total_bytes: 1000,
        speed_bps: 2048,
        eta_seconds: 3,
      },
    });
    expect(state.byId.a.speedBps).toBe(2048);

    state = reducer(state, {
      kind: "hydrate",
      records: [record("a", { downloaded_bytes: 600, status: "downloading" })],
    });
    expect(state.byId.a.speedBps).toBe(2048);
    expect(state.byId.a.downloaded_bytes).toBe(600);
  });

  it("veritabaninda olmayan kayitlari listeden atar", () => {
    // Kullanici baska bir yerden sildiyse arayuzde hayalet satir kalmamali.
    let state = withItems(record("a"), record("b"));
    state = reducer(state, { kind: "hydrate", records: [record("b")] });
    expect(state.order).toEqual(["b"]);
    expect(state.byId.a).toBeUndefined();
  });
});

describe("upsert", () => {
  it("yeni kaydi listenin basina koyar", () => {
    let state = withItems(record("eski"));
    state = reducer(state, { kind: "upsert", record: record("yeni") });
    expect(state.order).toEqual(["yeni", "eski"]);
  });

  it("var olan kaydi yerinde gunceller, siraya ikinci kez eklemez", () => {
    let state = withItems(record("a"), record("b"));
    state = reducer(state, {
      kind: "upsert",
      record: record("a", { status: "completed" }),
    });
    expect(state.order).toEqual(["a", "b"]);
    expect(state.byId.a.status).toBe("completed");
  });
});

describe("remove", () => {
  it("kaydi hem haritadan hem siradan siler", () => {
    let state = withItems(record("a"), record("b"));
    state = reducer(state, { kind: "remove", id: "a" });
    expect(state.order).toEqual(["b"]);
    expect(state.byId.a).toBeUndefined();
  });
});

describe("olaylar", () => {
  it("bilinmeyen kimlik icin durumu degistirmez", () => {
    // Uydurma bir satir yaratmak, veritabaninda olmayan bir indirmeyi
    // gostermek olurdu. Cagiran taraf bunun yerine tazeleme yapar.
    const state = withItems(record("a"));
    const next = reducer(state, {
      kind: "event",
      id: "yok-boyle-bir-sey",
      event: { type: "cancelled" },
    });
    expect(next).toBe(state);
  });

  it("ilerleme olayi bayt, hiz ve kalan sureyi tasir", () => {
    const item = toView(record("a"));
    const next = applyEvent(
      item,
      {
        type: "progress",
        downloaded_bytes: 750,
        total_bytes: 1000,
        speed_bps: 4096,
        eta_seconds: 12,
      },
      1_700_000_000_000
    );
    expect(next.status).toBe("downloading");
    expect(next.downloaded_bytes).toBe(750);
    expect(next.speedBps).toBe(4096);
    expect(next.etaSeconds).toBe(12);
    expect(next.lastProgressAt).toBe(1_700_000_000_000);
  });

  it("duraklatma hizi ve kalan sureyi sifirlar", () => {
    // Duraklatilmis bir indirmenin yaninda "3.2 MB/sn" yazmasi yalan olurdu.
    let item = toView(record("a"));
    item = applyEvent(item, {
      type: "progress",
      downloaded_bytes: 500,
      total_bytes: 1000,
      speed_bps: 3000,
      eta_seconds: 5,
    });
    const paused = applyEvent(item, { type: "paused", downloaded_bytes: 512 });
    expect(paused.status).toBe("paused");
    expect(paused.downloaded_bytes).toBe(512);
    expect(paused.speedBps).toBe(0);
    expect(paused.etaSeconds).toBeNull();
  });

  it("tamamlanma, ilerlemeyi tam boyuta esitler ve hedefi gunceller", () => {
    // Motor son bayti raporlamadan bitmis olabilir; %98'de kalan bir cubuk
    // gostermek yerine tamamlanmayi kesin yaziyoruz.
    const item = toView(record("a", { downloaded_bytes: 980 }));
    const done = applyEvent(item, {
      type: "completed",
      path: "C:\\indirilenler\\son-ad.mp4",
      total_bytes: 1000,
    });
    expect(done.downloaded_bytes).toBe(1000);
    expect(done.total_bytes).toBe(1000);
    expect(done.destination_path).toBe("C:\\indirilenler\\son-ad.mp4");
    expect(done.error_message).toBeNull();
  });

  it("basarisizlik mesaji tasir, yeniden basarili olunca temizlenir", () => {
    let item = toView(record("a"));
    item = applyEvent(item, { type: "failed", message: "ag hatasi" });
    expect(item.status).toBe("failed");
    expect(item.error_message).toBe("ag hatasi");

    // Yeniden denenip basladiginda eski hata ekranda asili kalmamali.
    item = applyEvent(item, { type: "started", total_bytes: 1000 });
    expect(item.status).toBe("downloading");
    expect(item.error_message).toBeNull();
  });

  it("yeniden deneme aktif sayilir ama hiz sifirdir", () => {
    const item = applyEvent(toView(record("a")), {
      type: "retrying",
      attempt: 2,
      delay_ms: 2000,
    });
    expect(item.status).toBe("retrying");
    expect(item.speedBps).toBe(0);
  });

  it("her olay turu bir sonuc uretir", () => {
    // Yeni bir olay turu eklenip burada unutulursa TypeScript yakalar;
    // bu test de calisma zamaninda cokme olmadigini garanti eder.
    const events: DownloadEvent[] = [
      { type: "started", total_bytes: null },
      { type: "progress", downloaded_bytes: 1, total_bytes: null, speed_bps: 1, eta_seconds: null },
      { type: "paused", downloaded_bytes: 1 },
      { type: "retrying", attempt: 1, delay_ms: 1 },
      { type: "completed", path: "x", total_bytes: 1 },
      { type: "failed", message: "x" },
      { type: "cancelled" },
    ];
    for (const event of events) {
      expect(applyEvent(toView(record("a")), event)).toBeDefined();
    }
  });
});

describe("bayat hiz bekcisi", () => {
  it("olay gelmeyi kesen bir indirmenin hizini sifirlar", () => {
    // Bir transfer takilirsa olay akisi susar ama son hiz ekranda asili
    // kalirdi. Olcum aleti yalan soylememeli.
    let state = withItems(record("a"));
    state = reducer(state, {
      kind: "event",
      id: "a",
      event: {
        type: "progress",
        downloaded_bytes: 100,
        total_bytes: 1000,
        speed_bps: 5000,
        eta_seconds: 10,
      },
    });
    const stampedAt = state.byId.a.lastProgressAt;

    state = reducer(state, {
      kind: "expireStaleSpeeds",
      olderThan: stampedAt + 1,
    });
    expect(state.byId.a.speedBps).toBe(0);
    expect(state.byId.a.etaSeconds).toBeNull();
  });

  it("taze bir indirmeye dokunmaz ve referansi degistirmez", () => {
    // Gereksiz yeni nesne, listeyi izleyen her bileseni yeniden cizdirir.
    let state = withItems(record("a"));
    state = reducer(state, {
      kind: "event",
      id: "a",
      event: {
        type: "progress",
        downloaded_bytes: 100,
        total_bytes: 1000,
        speed_bps: 5000,
        eta_seconds: 10,
      },
    });
    const before = state;
    const after = reducer(state, {
      kind: "expireStaleSpeeds",
      olderThan: state.byId.a.lastProgressAt - 1000,
    });
    expect(after).toBe(before);
  });

  it("duraklatilmis indirmeyi bekci ilgilendirmez", () => {
    let state = withItems(record("a", { status: "paused" }));
    const before = state;
    state = reducer(state, { kind: "expireStaleSpeeds", olderThan: Date.now() });
    expect(state).toBe(before);
  });
});

describe("telemetri toplamlari", () => {
  it("aktif sayisi indirme ve yeniden denemeyi kapsar", () => {
    const items = [
      toView(record("a", { status: "downloading" })),
      toView(record("b", { status: "retrying" })),
      toView(record("c", { status: "paused" })),
      toView(record("d", { status: "completed" })),
    ];
    expect(aggregate(items).activeCount).toBe(2);
  });

  it("hiz yalnizca gercekten akan indirmelerden toplanir", () => {
    // "retrying" aktiftir ama beklemededir; hizi toplama katmak,
    // kenar cubugunda olmayan bir akis gostermek olurdu.
    const items = [
      { ...toView(record("a", { status: "downloading" })), speedBps: 1000 },
      { ...toView(record("b", { status: "downloading" })), speedBps: 2000 },
      { ...toView(record("c", { status: "retrying" })), speedBps: 9999 },
      { ...toView(record("d", { status: "paused" })), speedBps: 9999 },
    ];
    expect(aggregate(items).totalSpeedBps).toBe(3000);
  });

  it("bos listede sifir doner", () => {
    expect(aggregate([])).toEqual({ activeCount: 0, totalSpeedBps: 0 });
  });
});

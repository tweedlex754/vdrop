import { describe, expect, it } from "vitest";
import {
  basename,
  formatBytes,
  formatDuration,
  formatEta,
  formatSpeed,
  formatTimestamp,
  percent,
  prettyCodecs,
  suggestFilename,
} from "./format";
import type { MediaInfo, StreamOption } from "../types/ipc";

function stream(over: Partial<StreamOption> = {}): StreamOption {
  return {
    id: "s",
    kind: "Muxed",
    url: "https://x.com/v.mp4",
    container: "mp4",
    codec: null,
    resolution: null,
    fps: null,
    bitrate_kbps: null,
    language: null,
    label: null,
    estimated_size_bytes: null,
    variant_index: null,
    ...over,
  };
}

describe("formatBytes", () => {
  it("ikili katlar kullanir, boylece Explorer'daki sayiyla ayni cikar", () => {
    expect(formatBytes(512)).toBe("512 B");
    expect(formatBytes(1024)).toBe("1.0 KB");
    expect(formatBytes(1024 * 1024)).toBe("1.0 MB");
    expect(formatBytes(1024 * 1024 * 1024)).toBe("1.0 GB");
  });

  it("basamak sayisini sabit tutar; satir genisligi guncellenirken oynamasin", () => {
    expect(formatBytes(9.9 * 1024 * 1024)).toMatch(/^\d+\.\d MB$/);
    expect(formatBytes(10.1 * 1024 * 1024)).toMatch(/^\d+\.\d MB$/);
  });

  it("bilinmeyen ve gecersiz degerler icin tire doner, sifir demez", () => {
    // "0 B" yazmak, boyutu bilmedigimiz bir dosya icin yanlis bir iddiadir.
    expect(formatBytes(null)).toBe("—");
    expect(formatBytes(undefined)).toBe("—");
    expect(formatBytes(-1)).toBe("—");
    expect(formatBytes(NaN)).toBe("—");
    expect(formatBytes(Infinity)).toBe("—");
  });

  it("sifir gecerli bir degerdir", () => {
    expect(formatBytes(0)).toBe("0 B");
  });
});

describe("formatSpeed", () => {
  it("birimi ekler", () => {
    expect(formatSpeed(2048, "/sn")).toBe("2.0 KB/sn");
  });

  it("akis yokken tire doner", () => {
    expect(formatSpeed(0, "/sn")).toBe("—");
    expect(formatSpeed(-5, "/sn")).toBe("—");
  });
});

describe("formatEta", () => {
  it("bir saatin altinda d:ss, ustunde s:dd:ss", () => {
    expect(formatEta(65)).toBe("1:05");
    expect(formatEta(3661)).toBe("1:01:01");
    expect(formatEta(0)).toBe("0:00");
  });

  it("saniyeler her zaman iki basamakli", () => {
    expect(formatEta(9)).toBe("0:09");
    expect(formatEta(3609)).toBe("1:00:09");
  });

  it("bilinmiyorsa tire", () => {
    expect(formatEta(null)).toBe("—");
    expect(formatEta(-1)).toBe("—");
  });
});

describe("formatDuration", () => {
  it("sifir ve negatif sure yoktur; null doner", () => {
    // Sure bilinmiyorsa onizlemede hic gosterilmemeli.
    expect(formatDuration(0)).toBeNull();
    expect(formatDuration(-3)).toBeNull();
    expect(formatDuration(null)).toBeNull();
    expect(formatDuration(634)).toBe("10:34");
  });
});

describe("percent", () => {
  it("orani hesaplar ve 0-100 arasina sikistirir", () => {
    expect(percent(50, 100)).toBe(50);
    expect(percent(150, 100)).toBe(100);
    expect(percent(-5, 100)).toBe(0);
  });

  it("toplam bilinmiyorsa sifir doner (belirsiz mod devreye girer)", () => {
    expect(percent(500, null)).toBe(0);
    expect(percent(500, 0)).toBe(0);
  });
});

describe("formatTimestamp", () => {
  it("SQLite'in UTC bicimini yerel saate cevirir", () => {
    // SQLite datetime('now') -> "2026-08-24 12:00:00" (UTC, T ve Z yok)
    const out = formatTimestamp("2026-08-24 12:00:00", "tr");
    expect(out).not.toBe("2026-08-24 12:00:00");
    expect(out.length).toBeGreaterThan(5);
  });

  it("cozulemeyen degeri oldugu gibi birakir", () => {
    expect(formatTimestamp("bozuk-tarih", "tr")).toBe("bozuk-tarih");
  });
});

describe("prettyCodecs", () => {
  it("RFC 6381 kodlarini okunabilir ada cevirir", () => {
    expect(prettyCodecs("avc1.640028")).toBe("H.264");
    expect(prettyCodecs("mp4a.40.2")).toBe("AAC");
    expect(prettyCodecs("hvc1.1.6.L93.B0")).toBe("H.265");
    expect(prettyCodecs("av01.0.05M.08")).toBe("AV1");
    expect(prettyCodecs("vp09.00.10.08")).toBe("VP9");
  });

  it("coklu kodek dizesini birlestirir", () => {
    expect(prettyCodecs("mp4a.40.2,avc1.640028")).toBe("AAC · H.264");
  });

  it("ayni aileyi iki kez yazmaz", () => {
    expect(prettyCodecs("avc1.640028,avc1.64001f")).toBe("H.264");
  });

  it("taninmayan kodu oldugu gibi birakir; uydurmaz", () => {
    expect(prettyCodecs("bilinmeyen1.2.3")).toBe("bilinmeyen1.2.3");
  });

  it("bos girdide null", () => {
    expect(prettyCodecs(null)).toBeNull();
    expect(prettyCodecs("")).toBeNull();
  });
});

describe("suggestFilename", () => {
  const media = (title: string): MediaInfo => ({
    title,
    uploader: null,
    thumbnail_url: null,
    duration_seconds: null,
    description: null,
    upload_date: null,
    streams: [],
    is_playlist: false,
  });

  it("baslik + kapsayici uzantisi", () => {
    expect(suggestFilename(media("Tatil videosu"), stream())).toBe(
      "Tatil videosu.mp4"
    );
  });

  it("basliktaki uzantiyi tekrarlamaz", () => {
    expect(suggestFilename(media("klip.mp4"), stream())).toBe("klip.mp4");
  });

  it("manifest uzantisi dosya uzantisi degildir", () => {
    // HLS birlestirilince sonuc bir mp4 olur; ".m3u8" ile kaydetmek
    // dosyayi oynatilamaz gosterirdi.
    expect(
      suggestFilename(media("Canli yayin"), stream({ container: "m3u8" }))
    ).toBe("Canli yayin.mp4");
    expect(
      suggestFilename(media("Canli"), stream({ container: "hls-or-dash" }))
    ).toBe("Canli.mp4");
  });

  it("ses akisinda m4a uretir", () => {
    expect(
      suggestFilename(
        media("Podcast"),
        stream({ container: "m3u8", kind: "Audio" })
      )
    ).toBe("Podcast.m4a");
  });

  it("basliksiz medyada makul bir yedek ad kullanir", () => {
    expect(suggestFilename(null, stream())).toBe("vdrop-indirme.mp4");
  });
});

describe("basename", () => {
  it("hem Windows hem POSIX ayraclarini anlar", () => {
    expect(basename("C:\\indirilenler\\film.mp4")).toBe("film.mp4");
    expect(basename("/home/x/film.mp4")).toBe("film.mp4");
    expect(basename("film.mp4")).toBe("film.mp4");
  });
});

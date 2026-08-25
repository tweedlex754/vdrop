import { describe, expect, it } from "vitest";
import { describeStream, shorthandResolution } from "./streamLabel";
import type { StreamOption } from "../types/ipc";

const LABELS = { audio: "Ses", stream: "Akis", file: "Dosya", subtitle: "Altyazi" };

describe("altyazi satirlari", () => {
  it("dili degil, yayincinin verdigi adi one cikarir", () => {
    // Iki iz de "en" olabilir; ayirt eden sey ad. Dil kodu ikincil bilgi
    // olarak yaninda durur.
    const label = describeStream(
      stream({
        kind: "Subtitle",
        container: "srt",
        language: "en",
        label: "English (forced)",
      }),
      LABELS
    );
    expect(label.name).toBe("English (forced)");
    expect(label.specs).toEqual(["en"]);
  });

  it("ad yoksa dile duser", () => {
    const label = describeStream(
      stream({ kind: "Subtitle", container: "srt", language: "tr" }),
      LABELS
    );
    expect(label.name).toBe("tr");
    expect(label.specs).toEqual([]);
  });

  it("ne ad ne dil varsa yine de bir sey soyler", () => {
    const label = describeStream(
      stream({ kind: "Subtitle", container: "srt" }),
      LABELS
    );
    expect(label.name).toBe(LABELS.subtitle);
  });

  it("altyaziyi 'Dosya - SRT' diye adlandirmaz", () => {
    // Altyazi dali eklenmeden once buraya dusuyordu: kapsayici manifest
    // degil, cozunurluk yok, kind Audio degil -> son dal. Kullaniciya
    // hicbir sey soylemeyen bir satir.
    const label = describeStream(
      stream({ kind: "Subtitle", container: "srt", language: "fr" }),
      LABELS
    );
    expect(label.name).not.toBe(LABELS.file);
    expect(label.specs).not.toContain("SRT");
  });
});

function stream(over: Partial<StreamOption> = {}): StreamOption {
  return {
    id: "s",
    kind: "Muxed",
    url: "https://x.com/v.mp4",
    container: null,
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

describe("shorthandResolution", () => {
  it("WxH bicimini kisa etikete cevirir", () => {
    expect(shorthandResolution("1920x1080")).toBe("1080p");
    expect(shorthandResolution("1280x720")).toBe("720p");
  });

  it("dikey videoda kisa kenari alir", () => {
    // 1080x1920 bir telefon videosudur; "1080p" dogru etikettir.
    expect(shorthandResolution("1080x1920")).toBe("1080p");
  });

  it("zaten kisa olan etiketi oldugu gibi birakir", () => {
    // Sayfa cikariminda yayincinin dosya adindan "480p" gelebilir.
    expect(shorthandResolution("480p")).toBe("480p");
  });

  it("bos degerde null", () => {
    expect(shorthandResolution(null)).toBeNull();
  });
});

describe("describeStream — ad ayirt edici eksendir", () => {
  it("videoda ad cozunurluktur, kapsayici teknik sutuna duser", () => {
    const label = describeStream(
      stream({
        container: "mp4",
        codec: "avc1.640028",
        resolution: "1920x1080",
        bitrate_kbps: 4200,
        estimated_size_bytes: 148 * 1024 * 1024,
      }),
      LABELS
    );
    expect(label.name).toBe("1080p");
    expect(label.specs).toEqual(["MP4", "H.264", "4200 kbps", "148.0 MB"]);
  });

  it("HLS varyantlarinin hepsi 'Akis' demez", () => {
    // Regresyon testi: manifest kontrolu cozunurluk kontrolunden once
    // yapilirsa her varyant "Akis" der ve kalite listesi ise yaramaz.
    const hd = describeStream(
      stream({ container: "m3u8", resolution: "1920x1080", variant_index: 4 }),
      LABELS
    );
    const sd = describeStream(
      stream({ container: "m3u8", resolution: "320x184", variant_index: 1 }),
      LABELS
    );
    expect(hd.name).toBe("1080p");
    expect(sd.name).toBe("184p");
    expect(hd.name).not.toBe(sd.name);
  });

  it("manifest kapsayicisi teknik sutunda tekrarlanmaz", () => {
    // "M3U8" satirlarin hepsinde ayni; bilgi tasimaz, gurultu yapar.
    const label = describeStream(
      stream({ container: "m3u8", resolution: "1280x720" }),
      LABELS
    );
    expect(label.specs).not.toContain("M3U8");
  });

  it("60 fps adin parcasidir; ayirt edici bir kalite farkidir", () => {
    const label = describeStream(
      stream({ resolution: "1920x1080", fps: 60 }),
      LABELS
    );
    expect(label.name).toBe("1080p60");
  });

  it("30 fps varsayilandir, ada eklenmez", () => {
    const label = describeStream(
      stream({ resolution: "1920x1080", fps: 30 }),
      LABELS
    );
    expect(label.name).toBe("1080p");
  });

  it("ses akisinda ad bit hizidir ve sutunda tekrarlanmaz", () => {
    const label = describeStream(
      stream({
        kind: "Audio",
        container: "m4a",
        bitrate_kbps: 128,
        estimated_size_bytes: 9 * 1024 * 1024,
      }),
      LABELS
    );
    expect(label.name).toBe("128 kbps");
    expect(label.specs).not.toContain("128 kbps");
    expect(label.specs).toContain("9.0 MB");
  });

  it("bit hizi bilinmeyen seste genel etiket kullanilir", () => {
    const label = describeStream(stream({ kind: "Audio" }), LABELS);
    expect(label.name).toBe("Ses");
  });

  it("cozunurluksuz manifest gercekten 'Akis'tir", () => {
    // Tek renditionlu HLS ya da DASH: uydurulacak kalite bilgisi yok.
    const label = describeStream(stream({ container: "mpd" }), LABELS);
    expect(label.name).toBe("Akis");
  });

  it("cozunurluksuz duz dosyada kapsayici ada girer", () => {
    const label = describeStream(stream({ container: "mkv" }), LABELS);
    expect(label.name).toBe("Dosya");
    expect(label.specs).toContain("MKV");
  });

  it("bilinmeyen degerler icin uydurma yapmaz", () => {
    const label = describeStream(stream(), LABELS);
    expect(label.name).toBe("Dosya");
    expect(label.specs).toEqual([]);
  });

  it("ayni sayfadan gelen iki webm ayirt edilebilir kalir", () => {
    // Sayfa cikariminda kalite etiketi yayincinin dosya adindan okunur.
    const a = describeStream(
      stream({ container: "webm", resolution: "480p" }),
      LABELS
    );
    const b = describeStream(
      stream({ container: "webm", resolution: "240p" }),
      LABELS
    );
    expect(a.name).toBe("480p");
    expect(b.name).toBe("240p");
  });
});

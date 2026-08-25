import { describe, expect, it } from "vitest";
import { LANGUAGES, LANGUAGE_CODES, isLanguageCode } from "./index";

/** Ic ice sozlukten "queue.pause" gibi duz anahtar listesi cikarir. */
function flatten(obj: unknown, prefix = ""): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [key, value] of Object.entries(obj as Record<string, unknown>)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "string") out[path] = value;
    else Object.assign(out, flatten(value, path));
  }
  return out;
}

describe("ceviri butunlugu", () => {
  it("her dil ayni anahtar kumesine sahiptir", () => {
    // TypeScript zaten eksik anahtari derleme hatasina cevirir; bu test
    // fazladan anahtari da yakalar (bir dilde silinip digerinde unutulan).
    const sets = LANGUAGE_CODES.map((code) =>
      Object.keys(flatten(LANGUAGES[code].dict)).sort()
    );
    for (let i = 1; i < sets.length; i++) {
      expect(sets[i]).toEqual(sets[0]);
    }
  });

  it("hicbir ceviri bos degildir", () => {
    // Bos bir dize derlenir ama arayuzde gorunmez bir dugme birakir.
    for (const code of LANGUAGE_CODES) {
      const flat = flatten(LANGUAGES[code].dict);
      const empty = Object.entries(flat)
        .filter(([, value]) => value.trim() === "")
        .map(([key]) => key);
      expect(empty, `${code} dilinde bos ceviriler`).toEqual([]);
    }
  });

  it("Turkce ve Ingilizce metinler gercekten farklidir", () => {
    // Bir dil eklenirken kopyala-yapistir yapilip cevrilmemis anahtarlar
    // kalabilir. Birebir ayni olan cok sayida deger bunun isaretidir.
    const tr = flatten(LANGUAGES.tr.dict);
    const en = flatten(LANGUAGES.en.dict);

    // Bazi degerler dogal olarak aynidir: kisaltmalar, semboller, marka
    // adlari. Bunlari haric tutuyoruz.
    const legitimatelyIdentical = new Set([
      "units.of",
      "settings.ffmpeg",
      "settings.ytdlp",
      "settings.appVersion",
    ]);

    const identical = Object.keys(tr).filter(
      (key) => tr[key] === en[key] && !legitimatelyIdentical.has(key)
    );
    expect(identical, "cevrilmemis olabilecek anahtarlar").toEqual([]);
  });

  it("her dil bir yon bildirir", () => {
    for (const code of LANGUAGE_CODES) {
      expect(["ltr", "rtl"]).toContain(LANGUAGES[code].dir);
      expect(LANGUAGES[code].label.length).toBeGreaterThan(0);
    }
  });
});

describe("isLanguageCode", () => {
  it("bilinen kodlari kabul, bilinmeyenleri reddeder", () => {
    expect(isLanguageCode("tr")).toBe(true);
    expect(isLanguageCode("en")).toBe(true);
    expect(isLanguageCode("klingon")).toBe(false);
    expect(isLanguageCode("")).toBe(false);
  });

  it("prototip zincirinden gelen adlari dil sanmaz", () => {
    // `"toString" in LANGUAGES` true doner; naif bir kontrol burada
    // patlar ve ayarlardan gelen bozuk bir deger uygulamayi cokertirdi.
    expect(isLanguageCode("toString")).toBe(false);
    expect(isLanguageCode("constructor")).toBe(false);
  });
});

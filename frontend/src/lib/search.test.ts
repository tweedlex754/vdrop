import { describe, expect, it } from "vitest";
import { matchesQuery } from "./search";

describe("liste aramasi", () => {
  it("bos sorgu her kaydi birakir", () => {
    // Arama kutusu bosken liste tam olmali; "hicbir sey aramadim" ile
    // "hicbir sey bulamadim" ayni sey degil.
    expect(matchesQuery("", ["herhangi bir sey"])).toBe(true);
    expect(matchesQuery("   ", [null])).toBe(true);
  });

  it("bos alanlari atlar, cokmez", () => {
    expect(matchesQuery("x", [null, undefined])).toBe(false);
    expect(matchesQuery("x", [null, "xyz"])).toBe(true);
  });

  it("alanlardan herhangi biri eslesirse yeter", () => {
    // Kutuphane baslikta VE yolda arar: kullanici bazen "hangi videoydu",
    // bazen "hangi klasordeydi" diye hatirlar.
    expect(matchesQuery("indirilenler", ["Tatil", "C:/indirilenler/a.mp4"])).toBe(true);
  });

  it("buyuk/kucuk harf ayrimi yapmaz", () => {
    expect(matchesQuery("BUNNY", ["Big_Buck_Bunny.mp4"])).toBe(true);
    expect(matchesQuery("bunny", ["BIG_BUCK_BUNNY.MP4"])).toBe(true);
  });

  it("Turkce noktali/noktasiz i sorununu cozer", () => {
    // JavaScript'in varsayilani burada kiriliyor:
    //   "İSTANBUL".toLowerCase() → "i̇stanbul"  (birlesik noktali i)
    // Duz `includes` ile bu "istanbul" ile ESLESMEZ.
    expect(matchesQuery("istanbul", ["İSTANBUL yayini.mp4"])).toBe(true);
    expect(matchesQuery("İSTANBUL", ["istanbul yayini.mp4"])).toBe(true);
    expect(matchesQuery("ISPARTA", ["Isparta gezisi"])).toBe(true);
  });

  it("diakritiksiz yazan kullaniciyi bulur", () => {
    // ASIL AMAC: kullanici klavyede "igdir" yazar, dosya adi "Iğdır"dir.
    // Katlama olmasa arama bos doner ve kullanici kaydi kayip sanir.
    expect(matchesQuery("igdir", ["Iğdır yolculugu.mp4"])).toBe(true);
    expect(matchesQuery("sarki", ["Şarkı listesi"])).toBe(true);
    expect(matchesQuery("gunes", ["Güneş doğarken"])).toBe(true);
    expect(matchesQuery("cocuk", ["Çocuk şarkıları"])).toBe(true);
  });

  it("ters yonde de calisir: diakritikli sorgu, sade metin", () => {
    expect(matchesQuery("Şarkı", ["sarki listesi"])).toBe(true);
  });

  it("Turkce disi diakritikleri de katlar", () => {
    expect(matchesQuery("cafe", ["Café Müzik"])).toBe(true);
    expect(matchesQuery("senor", ["El Señor"])).toBe(true);
  });

  it("KABUL EDILEN BEDEL: katlama harfleri ayirt etmeyi birakir", () => {
    // "acik" ile "açık" ayni sayilir. Bu bilincli: burasi bir indirme
    // yoneticisinin arama kutusu, hukuki metin aramasi degil. Kullanici
    // diakritiksiz yazar ve sonucu bulmayi bekler.
    expect(matchesQuery("acik", ["Açık Radyo"])).toBe(true);
  });

  it("eslesmeyeni elemeye devam eder", () => {
    expect(matchesQuery("zzz", ["Big Buck Bunny", "Jellyfish"])).toBe(false);
  });
});

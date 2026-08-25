/**
 * Liste ekranlarindaki (Kutuphane, Gecmis) serbest metin aramasi.
 *
 * Ayri bir dosyada duruyor cunku karar tek yerde verilmeli: ayni sorgu
 * Kutuphane'de ve Gecmis'te ayni sekilde eslesmeli, yoksa kullanici
 * "burada buldum, orada bulamadim" der.
 */

/**
 * `haystack` alanlarindan herhangi biri `query` ile eslesiyor mu?
 *
 * @param query     Kullanicinin yazdigi ham metin (bos olabilir).
 * @param haystack  Aranacak alanlar; `null`/`undefined` degerler atlanmali.
 * @returns         Kayit listede kalsin mi.
 *
 * Bos sorgu **her zaman** true doner: arama kutusu bosken liste tam olmali.
 */
export function matchesQuery(
  query: string,
  haystack: Array<string | null | undefined>
): boolean {
  const q = query.trim();
  if (!q) return true;

  const needle = normalize(q);
  return haystack.some((field) => field != null && normalize(field).includes(needle));
}

/**
 * Turkce harfleri ASCII karsiliklarina katlar.
 *
 * `toLowerCase()`'ten ONCE uygulanmali: JavaScript'in kucuk harfe cevirmesi
 * "İ" icin "i" + birlesik nokta ureten iki karakterlik bir dizi verir, o
 * yuzden sonradan temizlemek yerine harfi bastan sadelestiriyoruz.
 */
const KATLAMA: Record<string, string> = {
  ı: "i", İ: "i", I: "i", i: "i",
  ğ: "g", Ğ: "g",
  ş: "s", Ş: "s",
  ç: "c", Ç: "c",
  ö: "o", Ö: "o",
  ü: "u", Ü: "u",
};

/**
 * Karsilastirma icin metni sadelestirir.
 *
 * Uc adim, sirasi onemli:
 *   1. Turkce harfleri katla  - "Iğdır" → "Igdir"  (noktali i tuzagindan once)
 *   2. Kucuk harfe cevir      - "Igdir" → "igdir"
 *   3. Kalan diakritikleri at - "café"  → "cafe"   (NFD + birlesik isaretler)
 *
 * KABUL EDILEN BEDEL: bu katlama "açık" ile "acık"i ayni sayar. Bilincli bir
 * secim - burasi bir indirme yoneticisinin arama kutusu, hukuki metin
 * aramasi degil. Kullanici klavyede diakritiksiz yazar ve kaydi bulmayi
 * bekler; ayrimi korumak onu "kayit kayip" sanmaya iterdi.
 */
function normalize(value: string): string {
  return value
    .replace(/[ıİIiğĞşŞçÇöÖüÜ]/g, (ch) => KATLAMA[ch])
    .toLowerCase()
    .normalize("NFD")
    .replace(/\p{M}/gu, "");
}

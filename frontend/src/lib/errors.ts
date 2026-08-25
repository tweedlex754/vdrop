import type { Dictionary } from "../i18n/tr";
import type { AppError } from "../types/ipc";

/** Arka ucun yapisal hatasi mi, baska bir sey mi? */
function isAppError(value: unknown): value is AppError {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as AppError).code === "string"
  );
}

/**
 * Hatayi kullanicinin dilinde bir baslik + govdeye cevirir.
 *
 * Neden burada: dili yalnizca arayuz biliyor. Arka uc metin gonderirse o
 * metin her zaman tek dilde olur - kullanici Ingilizce arayuzde Turkce
 * cumle gorurdu (ya da tersi).
 *
 * **Hicbir bilgi kaybolmaz.** Taninmayan bir kod gelirse teknik detay (ya da
 * kodun kendisi) govdede gosterilir; sessizce "bir seyler ters gitti"ye
 * dusmek, hatayi bildirmek isteyen kullanicinin elini bosaltirdi.
 */
export function describeError(
  err: unknown,
  t: Dictionary
): { title: string; body: string } {
  // `unknown` her zaman vardir (tipten geliyor); dinamik arama yalnizca
  // kodun kendisi icin gerekiyor.
  const fallback = t.errors.unknown;
  const table = t.errors as unknown as Record<
    string,
    { title: string; body: string } | undefined
  >;

  if (!isAppError(err)) {
    // Tauri disinda (tarayici gelistirmesi) ya da beklenmedik bir firlatma.
    return { title: fallback.title, body: String(err) };
  }

  const known = table[err.code];
  const detail = err.detail?.trim();

  if (!known) {
    return { title: fallback.title, body: detail || err.code };
  }

  // Teknik iz varsa parantez icinde korunur: kullaniciya yol gosteren cumle
  // once gelir, ama "sunucu 500" bilgisi de kaybolmaz.
  return {
    title: known.title,
    body: detail ? `${known.body} (${detail})` : known.body,
  };
}

import {
  createContext,
  useContext,
  useEffect,
  useMemo,
  type ReactNode,
} from "react";
import { tr, type Dictionary } from "./tr";
import { en } from "./en";
import { ar } from "./ar";
import { cs } from "./cs";
import { de } from "./de";
import { es } from "./es";
import { fr } from "./fr";
import { hi } from "./hi";
import { id } from "./id";
import { it } from "./it";
import { ja } from "./ja";
import { ko } from "./ko";
import { nl } from "./nl";
import { pl } from "./pl";
import { pt } from "./pt";
import { ru } from "./ru";
import { sv } from "./sv";
import { uk } from "./uk";
import { vi } from "./vi";
import { zh } from "./zh";

export const LANGUAGES = {
  // Sira: once elle yazilan iki kaynak dil, sonra kod sirasina gore
  // digerleri. Acilir listede kullanicinin taradigi sira bu.
  tr: { dict: tr, label: "Türkçe", dir: "ltr" as const },
  en: { dict: en, label: "English", dir: "ltr" as const },
  ar: { dict: ar, label: "العربية", dir: "rtl" as const },
  cs: { dict: cs, label: "Čeština", dir: "ltr" as const },
  de: { dict: de, label: "Deutsch", dir: "ltr" as const },
  es: { dict: es, label: "Español", dir: "ltr" as const },
  fr: { dict: fr, label: "Français", dir: "ltr" as const },
  hi: { dict: hi, label: "हिन्दी", dir: "ltr" as const },
  id: { dict: id, label: "Bahasa Indonesia", dir: "ltr" as const },
  it: { dict: it, label: "Italiano", dir: "ltr" as const },
  ja: { dict: ja, label: "日本語", dir: "ltr" as const },
  ko: { dict: ko, label: "한국어", dir: "ltr" as const },
  nl: { dict: nl, label: "Nederlands", dir: "ltr" as const },
  pl: { dict: pl, label: "Polski", dir: "ltr" as const },
  pt: { dict: pt, label: "Português", dir: "ltr" as const },
  ru: { dict: ru, label: "Русский", dir: "ltr" as const },
  sv: { dict: sv, label: "Svenska", dir: "ltr" as const },
  uk: { dict: uk, label: "Українська", dir: "ltr" as const },
  vi: { dict: vi, label: "Tiếng Việt", dir: "ltr" as const },
  zh: { dict: zh, label: "中文", dir: "ltr" as const },
};

export type LanguageCode = keyof typeof LANGUAGES;

export const LANGUAGE_CODES = Object.keys(LANGUAGES) as LanguageCode[];

interface I18nValue {
  t: Dictionary;
  lang: LanguageCode;
  dir: "ltr" | "rtl";
}

const I18nContext = createContext<I18nValue | null>(null);

export function I18nProvider({
  lang,
  children,
}: {
  lang: LanguageCode;
  children: ReactNode;
}) {
  const value = useMemo<I18nValue>(() => {
    const entry = LANGUAGES[lang] ?? LANGUAGES.tr;
    return { t: entry.dict, lang, dir: entry.dir };
  }, [lang]);

  // Belgenin dilini de guncelle. `index.html` sabit `lang="tr"` ile geliyordu
  // ve hicbir yerde degistirilmiyordu; `text-transform: uppercase` bunu
  // dikkate aldigi icin Ingilizce arayuzde Turkce buyutme kurali uygulaniyor,
  // "Downloading" rozeti "DOWNLOADİNG" olarak ciziliyordu. Ekran okuyucular
  // da metni yanlis dilde seslendiriyordu.
  useEffect(() => {
    document.documentElement.lang = value.lang;
    document.documentElement.dir = value.dir;
  }, [value.lang, value.dir]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

/**
 * Sozlugu **nesne olarak** dondururuz (`t.queue.pause`), `t("queue.pause")`
 * gibi bir dize anahtariyla degil. Sebep: nokta notasyonu TypeScript'in
 * kontrolunden gecer - yanlis yazilan bir anahtar derleme hatasi olur,
 * arayuzde "queue.pasue" yazan bir dugme degil.
 */
export function useT(): Dictionary {
  const ctx = useContext(I18nContext);
  if (!ctx) throw new Error("useT, I18nProvider icinde kullanilmalidir");
  return ctx.t;
}

export function useLocale(): { lang: LanguageCode; dir: "ltr" | "rtl" } {
  const ctx = useContext(I18nContext);
  if (!ctx) throw new Error("useLocale, I18nProvider icinde kullanilmalidir");
  return { lang: ctx.lang, dir: ctx.dir };
}

export function isLanguageCode(value: string): value is LanguageCode {
  // `in` DEGIL: o operator prototip zincirini de dolasir, yani "toString"
  // ve "constructor" gecerli dil kodu sayilirdi. Veritabaninda boyle bozuk
  // bir deger olsaydi `LANGUAGES["toString"]` bir fonksiyon donerdi,
  // `.dict` undefined olurdu ve uygulama ilk cizimde cokerdi.
  return Object.hasOwn(LANGUAGES, value);
}

import { describe, expect, it } from "vitest";
import { describeError } from "./errors";
import { tr } from "../i18n/tr";
import { en } from "../i18n/en";

describe("hata cozumleme", () => {
  it("bilinen kodu kullanicinin dilinde anlatir", () => {
    const out = describeError({ code: "ytdlp_missing" }, tr);
    expect(out.title).toBe(tr.errors.ytdlp_missing.title);
    expect(out.body).toBe(tr.errors.ytdlp_missing.body);
  });

  it("ayni kod dile gore degisir", () => {
    const trOut = describeError({ code: "no_media" }, tr);
    const enOut = describeError({ code: "no_media" }, en);
    expect(trOut.title).not.toBe(enOut.title);
    expect(enOut.title).toBe(en.errors.no_media.title);
  });

  it("teknik detayi korur ama one cikarmaz", () => {
    // "sunucu 500" bilgisi hata bildiren kullanici icin degerli; yine de
    // once yol gosteren cumle gelmeli.
    const out = describeError({ code: "network", detail: "server returned 500" }, tr);
    expect(out.title).toBe(tr.errors.network.title);
    expect(out.body.startsWith(tr.errors.network.body)).toBe(true);
    expect(out.body).toContain("server returned 500");
  });

  it("TANIMADIGI kodda bile bilgi kaybetmez", () => {
    // Arka uca yeni bir kod eklenip ceviri unutulursa arayuz cokmemeli ve
    // "bir seyler ters gitti" deyip elimizi bosaltmamali.
    const out = describeError({ code: "brand_new_code", detail: "boom" }, tr);
    expect(out.title).toBe(tr.errors.unknown.title);
    expect(out.body).toBe("boom");
  });

  it("detay yoksa taninmayan kodun kendisini gosterir", () => {
    const out = describeError({ code: "brand_new_code" }, tr);
    expect(out.body).toBe("brand_new_code");
  });

  it("yapisal olmayan firlatmalari da karsilar", () => {
    // Tarayici gelistirmesinde ya da beklenmedik bir yerde duz Error gelebilir.
    const out = describeError(new Error("kablo koptu"), tr);
    expect(out.title).toBe(tr.errors.unknown.title);
    expect(out.body).toContain("kablo koptu");
  });

  it("bos detayi yok sayar", () => {
    const out = describeError({ code: "network", detail: "   " }, tr);
    expect(out.body).toBe(tr.errors.network.body);
  });
});

// Tauri IPC'nin tek sarmalayicisi. Bilesenler asla dogrudan `invoke`
// cagirmaz; her komut burada tiplenir. Boylece Rust tarafinda bir imza
// degistiginde tek bir dosya guncellenir ve TypeScript kalan her cagri
// yerini bize gosterir.

import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  AnalyzeResult,
  AppInfo,
  DownloadEventPayload,
  DownloadRecord,
  HistoryRecord,
  LibraryItem,
} from "../types/ipc";

/**
 * Tauri penceresinde miyiz? Native kabuk `__TAURI_INTERNALS__` global'ini
 * enjekte eder; duz bir tarayicida bu yoktur.
 */
const IN_TAURI =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

/**
 * Tum IPC bu kapidan gecer.
 *
 * Tauri disinda VE gelistirme modundaysak bellek ici sahte arka uca duseriz;
 * boylece arayuz tarayicida HMR ile gelistirilebilir (Rust'i her degisiklikte
 * yeniden derlemek dakikalar aliyor). `import.meta.env.DEV` sabit oldugu icin
 * Rollup bu dali uretim paketinden tamamen budar - sahte veri urune sizmaz.
 */
async function invoke<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  if (!IN_TAURI) {
    if (import.meta.env.DEV) {
      const { devInvoke } = await import("./devBridge");
      return devInvoke<T>(cmd, args);
    }
    throw new Error(
      "VDrop komutlari yalnizca uygulama penceresinde calisir."
    );
  }
  return tauriInvoke<T>(cmd, args);
}

// --- Cozumleme -------------------------------------------------------------

export const analyzeUrl = (url: string) =>
  invoke<AnalyzeResult>("analyze_url", { url });

// --- Indirme yasam dongusu -------------------------------------------------

export interface CreateDownloadArgs {
  url: string;
  suggestedName: string;
  title?: string | null;
  folder?: string | null;
  thumbnailUrl?: string | null;
  variantIndex?: number | null;
  container?: string | null;
  formatId?: string | null;
}

/**
 * Yeni indirme olusturur ve olusan kaydi dondurur.
 *
 * Kaydi geri dondurmesi bilincli: arayuz kendi kimlik uretip "sanirim su an
 * boyle" diye bir satir cizmez, dogrudan veritabaninin yazdigi gercegi
 * gosterir. Boylece kimlik/durum ikilemi olusmaz.
 */
export const createDownload = (args: CreateDownloadArgs) =>
  invoke<DownloadRecord>("create_download", {
    url: args.url,
    suggestedName: args.suggestedName,
    title: args.title ?? null,
    folder: args.folder ?? null,
    thumbnailUrl: args.thumbnailUrl ?? null,
    variantIndex: args.variantIndex ?? null,
    container: args.container ?? null,
    formatId: args.formatId ?? null,
  });

export const pauseDownload = (id: string) =>
  invoke<void>("pause_download", { id });

export const resumeDownload = (id: string) =>
  invoke<void>("resume_download", { id });

export const cancelDownload = (id: string) =>
  invoke<void>("cancel_download", { id });

export const listDownloads = () => invoke<DownloadRecord[]>("list_downloads");

export const removeDownload = (id: string, deleteFile: boolean) =>
  invoke<void>("remove_download", { id, deleteFile });

export const clearFinished = () => invoke<number>("clear_finished");

// --- Gecmis ve kutuphane ---------------------------------------------------

export const listHistory = (limit?: number) =>
  invoke<HistoryRecord[]>("list_history", { limit: limit ?? null });

export const clearHistory = () => invoke<number>("clear_history");

export const listLibrary = () => invoke<LibraryItem[]>("list_library");

export const removeLibraryItem = (id: string, deleteFile: boolean) =>
  invoke<void>("remove_library_item", { id, deleteFile });

export const pathsExist = (paths: string[]) =>
  invoke<Record<string, boolean>>("paths_exist", { paths });

// --- Ayarlar ve sistem -----------------------------------------------------

export const getSettings = () => invoke<Record<string, string>>("get_settings");

export const setSetting = (key: string, value: string) =>
  invoke<void>("set_setting", { key, value });

export const selectDownloadFolder = () =>
  invoke<string | null>("select_download_folder");

export const getAppInfo = () => invoke<AppInfo>("app_info");

export const openPath = (path: string) => invoke<void>("open_path", { path });

export const revealPath = (path: string) =>
  invoke<void>("reveal_path", { path });

// --- Olaylar ---------------------------------------------------------------

/**
 * Tum indirme olaylari tek bir kanaldan gelir (`download:event`). Yedi ayri
 * olay adi yerine tek abonelik: dinleyici sizintisi riski dusuk, sira
 * garantisi net.
 */
export interface ClipboardLink {
  url: string;
  label: string;
  is_stream: boolean;
}

/**
 * Pano izleyici bir medya baglantisi yakaladiginda tetiklenir.
 * Arka uc bu baglantiya **hicbir istek atmaz**; karar kullanicinindir.
 * Tarayici onizlemesinde pano erisimi yok, sessizce bos abonelik doner.
 */
export async function onClipboardLink(
  handler: (link: ClipboardLink) => void
): Promise<UnlistenFn> {
  if (!IN_TAURI) return () => {};
  return listen<ClipboardLink>("clipboard:link", (evt) => handler(evt.payload));
}

export async function onDownloadEvent(
  handler: (payload: DownloadEventPayload) => void
): Promise<UnlistenFn> {
  if (!IN_TAURI) {
    if (import.meta.env.DEV) {
      const { devListen } = await import("./devBridge");
      return devListen(handler);
    }
    return () => {};
  }
  return listen<DownloadEventPayload>("download:event", (evt) =>
    handler(evt.payload)
  );
}

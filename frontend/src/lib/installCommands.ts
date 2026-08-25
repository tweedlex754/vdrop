/**
 * Eksik bilesenler icin platforma uygun kurulum komutu.
 *
 * Ceviri dosyalari yalnizca "Kurmak icin:" gibi bir on ek tutuyor; komutun
 * kendisi burada uretiliyor. Sebep: komut dile degil isletim sistemine bagli.
 * Yirmi dile `winget install ffmpeg` gomuldugunde macOS ve Linux kullanicisi
 * makinesinde bulunmayan bir arac oneriliyordu - metin dogru cevrilmis olsa
 * bile tavsiye yanlisti.
 *
 * `os` degeri kabuktan (`std::env::consts::OS`) geliyor; tarayici sniffing'i
 * degil.
 */
export type OsName = string;

/**
 * FFmpeg icin kurulum komutu.
 *
 * Linux'ta paket yoneticisi dagitima gore degisiyor; apt en yayginidir ve
 * ipucunun amaci tam komutu vermek degil, dogru yone isaret etmek.
 */
export function ffmpegInstallCommand(os: OsName): string {
  switch (os) {
    case "macos":
      return "brew install ffmpeg";
    case "linux":
      return "sudo apt install ffmpeg";
    default:
      return "winget install ffmpeg";
  }
}

/**
 * yt-dlp icin kurulum komutu.
 *
 * macOS ve Linux'ta `pip` cogu zaman PATH'te degil ya da sistem Python'ina
 * yazmayi reddediyor (PEP 668), o yuzden oralarda paket yoneticisi/pipx
 * oneriliyor.
 */
export function ytdlpInstallCommand(os: OsName): string {
  switch (os) {
    case "macos":
      return "brew install yt-dlp";
    case "linux":
      return "pipx install yt-dlp";
    default:
      return "pip install -U yt-dlp";
  }
}

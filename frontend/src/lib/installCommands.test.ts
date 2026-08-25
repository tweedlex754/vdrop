import { describe, expect, it } from "vitest";
import { ffmpegInstallCommand, ytdlpInstallCommand } from "./installCommands";

describe("kurulum komutlari", () => {
  it("her platform icin o platformda var olan araci onerir", () => {
    // Asil hata buydu: macOS ve Linux kullanicisina `winget` oneriliyordu.
    expect(ffmpegInstallCommand("macos")).toBe("brew install ffmpeg");
    expect(ffmpegInstallCommand("linux")).toBe("sudo apt install ffmpeg");
    expect(ffmpegInstallCommand("windows")).toBe("winget install ffmpeg");

    expect(ytdlpInstallCommand("macos")).toBe("brew install yt-dlp");
    expect(ytdlpInstallCommand("linux")).toBe("pipx install yt-dlp");
    expect(ytdlpInstallCommand("windows")).toBe("pip install -U yt-dlp");
  });

  it("tanimadigi platformda Windows komutuna duser", () => {
    // Kabuk yeni bir hedef eklerse arayuz bos ipucu gostermesin.
    expect(ffmpegInstallCommand("freebsd")).toBe("winget install ffmpeg");
    expect(ytdlpInstallCommand("")).toBe("pip install -U yt-dlp");
  });
});

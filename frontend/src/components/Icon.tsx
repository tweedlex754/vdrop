// Satir ici SVG ikon seti.
//
// Neden harici ikon paketi yok: VDrop cevrimdisi calisan bir masaustu
// uygulamasi. Kullandigimiz 14 ikon icin 40+ KB'lik bir bagimlilik tasimak
// (ve onu guncel tutmak) mantiksiz. Hepsi tek bir stroke agirliginda (1.6)
// cizildi ki arayuzde tutarli bir cizgi kalinligi olsun.

type IconProps = {
  name: IconName;
  size?: number;
  className?: string;
  style?: React.CSSProperties;
};

export type IconName =
  | "home"
  | "queue"
  | "library"
  | "history"
  | "settings"
  | "pause"
  | "play"
  | "close"
  | "trash"
  | "folder"
  | "file"
  | "download"
  | "alert"
  | "check"
  | "refresh"
  | "inbox"
  | "image"
  | "terminal"
  | "film"
  | "package"
  | "clipboard"
  | "sliders"
  | "search";

const PATHS: Record<IconName, string> = {
  // Buyutec: cember + sap, diger ikonlarla ayni 1.6 kalinlikta.
  search: "M9 4.5a4.5 4.5 0 1 0 0 9 4.5 4.5 0 0 0 0-9M12.4 12.4 16.2 16.2",
  home: "M3 9.5 10 3.5l7 6M4.5 8.5V16a.5.5 0 0 0 .5.5h3.5v-4h3v4H15a.5.5 0 0 0 .5-.5V8.5",
  queue: "M3 5h9M3 10h9M3 15h6M15.5 9v6M13 12.5l2.5 2.5 2.5-2.5",
  library: "M3 6.5A1.5 1.5 0 0 1 4.5 5h3l1.5 2h6.5A1.5 1.5 0 0 1 17 8.5v6A1.5 1.5 0 0 1 15.5 16h-11A1.5 1.5 0 0 1 3 14.5z",
  history: "M10 5.5v5l3 1.5M3.6 8.2A6.75 6.75 0 1 1 3.3 11M3.2 5v3.3h3.3",
  settings: "M4 6h5M12 6h4M4 14h4M11 14h5M10.5 4.5v3M8.5 12.5v3",
  pause: "M7.5 5v10M12.5 5v10",
  play: "M6.5 4.5 15 10l-8.5 5.5z",
  close: "m5.5 5.5 9 9M14.5 5.5l-9 9",
  trash: "M4 6h12M8 6V4.5h4V6M6 6l.7 9.1a1 1 0 0 0 1 .9h4.6a1 1 0 0 0 1-.9L14 6M8.5 9v4M11.5 9v4",
  folder: "M3 6.5A1.5 1.5 0 0 1 4.5 5h3l1.5 2h6.5A1.5 1.5 0 0 1 17 8.5v6A1.5 1.5 0 0 1 15.5 16h-11A1.5 1.5 0 0 1 3 14.5z",
  file: "M5 3.5h5.5L15 8v8.5H5zM10.5 3.5V8H15",
  download: "M10 3.5v9M6.5 9.5 10 13l3.5-3.5M4 15.5h12",
  alert: "M10 4.2 3.2 16h13.6zM10 8.5v3.2M10 13.8v.05",
  check: "m4.5 10.5 3.5 3.5 7.5-8",
  refresh: "M16 10a6 6 0 1 1-1.8-4.3M16 4v3.2h-3.2",
  inbox: "M3.5 11h3.2l1.1 2h4.4l1.1-2h3.2M3.5 11 6 5h8l2.5 6v4a1 1 0 0 1-1 1h-11a1 1 0 0 1-1-1z",
  // Kucuk resim yer tutucusu: dag + gunes.
  image: "M3.5 5.5h13v9h-13zM3.5 12l3.2-3.2 3 3 2.4-2.4 4.4 4.1M12.6 8.1v.05",
  // yt-dlp: komut istemi.
  terminal: "M3.5 4.5h13v11h-13zM6 8.2l2.2 2.1L6 12.4M10.5 12.6h4",
  // FFmpeg: film seridi.
  film: "M3.5 5h13v10h-13zM7 5v10M13 5v10M3.5 8.3h3.5M3.5 11.7h3.5M13 8.3h3.5M13 11.7h3.5",
  // Uygulama guncellemesi: kutu + yukari ok.
  package: "M3.5 6.6 10 3.5l6.5 3.1v6.8L10 16.5 3.5 13.4zM3.5 6.6 10 9.8l6.5-3.2M10 9.8v6.7",
  clipboard: "M7.5 4.5h5v2h-5zM6 5.5H4.7A1.2 1.2 0 0 0 3.5 6.7v8.6A1.2 1.2 0 0 0 4.7 16.5h10.6a1.2 1.2 0 0 0 1.2-1.2V6.7a1.2 1.2 0 0 0-1.2-1.2H14",
  sliders: "M4 6.5h5M13 6.5h3M4 13.5h3M11 13.5h5M11 6.5a1.5 1.5 0 1 0 3 0 1.5 1.5 0 1 0-3 0M7 13.5a1.5 1.5 0 1 0 3 0 1.5 1.5 0 1 0-3 0",
};

/** Dolgu ile cizilmesi gereken ikonlar (ok basi gibi kapali sekiller). */
const FILLED: IconName[] = ["play"];

export function Icon({ name, size = 17, className, style }: IconProps) {
  const filled = FILLED.includes(name);
  return (
    <svg
      className={className}
      style={style}
      width={size}
      height={size}
      viewBox="0 0 20 20"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.6}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
      focusable="false"
    >
      <path d={PATHS[name]} fill={filled ? "currentColor" : "none"} />
    </svg>
  );
}

/**
 * Kenar cubugundaki marka isareti.
 *
 * Uygulama ikonunun sadelestirilmis hali: indirme oku + oynat ucgeni.
 * Ikonun kendisini kucuk boyutta gostermek yerine yeniden ciziyoruz -
 * 40px'lik bir dairede 256px'lik bir PNG bulanik durur.
 */
export function BrandMark({ size = 22 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden="true"
      focusable="false"
    >
      <defs>
        <linearGradient id="vdrop-mark" x1="0" y1="0" x2="1" y2="0">
          <stop offset="0%" stopColor="#2c379e" />
          <stop offset="100%" stopColor="#2f76e6" />
        </linearGradient>
      </defs>
      {/* indirme oku */}
      <path d="M6.6 3.6h3.2v6.2h2.4L8.2 16 3.9 9.8h2.7z" fill="url(#vdrop-mark)" />
      {/* oynat ucgeni */}
      <path d="M14 4.4 21 10l-7 5.6z" fill="url(#vdrop-mark)" />
    </svg>
  );
}

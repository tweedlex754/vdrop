// VDrop marka ikonu üreteci — bağımlılıksız PNG encoder (zlib built-in).
// 4x supersampling ile anti-aliasing yapar.
import zlib from "node:zlib";
import fs from "node:fs";

const S = 1024;      // çıktı boyutu
const SS = 4;        // supersampling faktörü
const N = S * SS;

// --- Geometri yardımcıları -------------------------------------------------
const clamp = (v, a, b) => (v < a ? a : v > b ? b : v);
const lerp = (a, b, t) => a + (b - a) * t;

function insideRoundedRect(x, y, w, h, r) {
  const dx = Math.max(r - x, 0, x - (w - r));
  const dy = Math.max(r - y, 0, y - (h - r));
  if (dx === 0 || dy === 0) return x >= 0 && y >= 0 && x <= w && y <= h;
  return dx * dx + dy * dy <= r * r;
}

// Damla: alttaki daire + tepe noktasından daireye teğet üçgen.
function makeTeardrop(cx, cy, r, tipY) {
  const d = cy - tipY;                    // tepe ile merkez arası
  const tanLen = Math.sqrt(Math.max(d * d - r * r, 0));
  const alpha = Math.asin(clamp(r / d, -1, 1));   // teğetin eksenden sapması
  // teğet değme noktaları
  const t1 = { x: cx - r * Math.cos(alpha), y: cy - r * Math.sin(alpha) };
  const t2 = { x: cx + r * Math.cos(alpha), y: cy - r * Math.sin(alpha) };
  const tip = { x: cx, y: tipY };
  const _ = tanLen;
  return (x, y) => {
    if ((x - cx) ** 2 + (y - cy) ** 2 <= r * r) return true;   // daire kısmı
    return pointInTriangle(x, y, tip, t1, t2);                  // sivri kısım
  };
}

function sign(px, py, ax, ay, bx, by) {
  return (px - bx) * (ay - by) - (ax - bx) * (py - by);
}
function pointInTriangle(px, py, a, b, c) {
  const d1 = sign(px, py, a.x, a.y, b.x, b.y);
  const d2 = sign(px, py, b.x, b.y, c.x, c.y);
  const d3 = sign(px, py, c.x, c.y, a.x, a.y);
  const neg = d1 < 0 || d2 < 0 || d3 < 0;
  const pos = d1 > 0 || d2 > 0 || d3 > 0;
  return !(neg && pos);
}

// Aşağı ok: dikey gövde (yuvarlak uçlu) + üçgen başlık
function makeDownArrow(cx, topY, botY, shaftW, headW, headH) {
  const shaftBot = botY - headH;
  const head = [
    { x: cx - headW / 2, y: shaftBot },
    { x: cx + headW / 2, y: shaftBot },
    { x: cx, y: botY },
  ];
  return (x, y) => {
    if (y >= topY && y <= shaftBot && Math.abs(x - cx) <= shaftW / 2) return true;
    if (Math.hypot(x - cx, y - topY) <= shaftW / 2) return true;   // yuvarlak üst uç
    return pointInTriangle(x, y, head[0], head[1], head[2]);
  };
}

// --- Palet ----------------------------------------------------------------
const BG_TOP = [99, 102, 241];      // indigo-500
const BG_BOT = [139, 92, 246];      // violet-500
const DROP = [255, 255, 255];

// --- Render ---------------------------------------------------------------
const teardrop = makeTeardrop(N * 0.5, N * 0.605, N * 0.215, N * 0.215);
const arrow = makeDownArrow(N * 0.5, N * 0.50, N * 0.715, N * 0.075, N * 0.22, N * 0.115);
const radius = N * 0.225;

// Downsample için akümülatör
const acc = new Float32Array(S * S * 4);

for (let sy = 0; sy < N; sy++) {
  const y = sy + 0.5;
  const oy = (sy / SS) | 0;
  for (let sx = 0; sx < N; sx++) {
    const x = sx + 0.5;
    let r = 0, g = 0, b = 0, a = 0;

    if (insideRoundedRect(x, y, N, N, radius)) {
      const t = y / N;
      r = lerp(BG_TOP[0], BG_BOT[0], t);
      g = lerp(BG_TOP[1], BG_BOT[1], t);
      b = lerp(BG_TOP[2], BG_BOT[2], t);
      a = 255;

      if (teardrop(x, y) && !arrow(x, y)) {
        [r, g, b] = DROP;
      }
    }

    const ox = (sx / SS) | 0;
    const i = (oy * S + ox) * 4;
    acc[i] += r; acc[i + 1] += g; acc[i + 2] += b; acc[i + 3] += a;
  }
}

const samples = SS * SS;
const raw = Buffer.alloc(S * (S * 4 + 1));
for (let y = 0; y < S; y++) {
  raw[y * (S * 4 + 1)] = 0;                       // filter type: None
  for (let x = 0; x < S; x++) {
    const src = (y * S + x) * 4;
    const dst = y * (S * 4 + 1) + 1 + x * 4;
    for (let c = 0; c < 4; c++) raw[dst + c] = Math.round(acc[src + c] / samples);
  }
}

// --- PNG çıktısı ----------------------------------------------------------
const CRC_TABLE = (() => {
  const t = new Int32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    t[n] = c;
  }
  return t;
})();
function crc32(buf) {
  let c = -1;
  for (let i = 0; i < buf.length; i++) c = CRC_TABLE[(c ^ buf[i]) & 0xff] ^ (c >>> 8);
  return (c ^ -1) >>> 0;
}
function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(body));
  return Buffer.concat([len, body, crc]);
}

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(S, 0);
ihdr.writeUInt32BE(S, 4);
ihdr[8] = 8;    // bit depth
ihdr[9] = 6;    // color type: RGBA
const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk("IHDR", ihdr),
  chunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
  chunk("IEND", Buffer.alloc(0)),
]);

fs.writeFileSync(new URL("./vdrop-logo.png", import.meta.url), png);
console.log(`vdrop-logo.png yazıldı — ${S}x${S}, ${(png.length / 1024).toFixed(1)} KB`);

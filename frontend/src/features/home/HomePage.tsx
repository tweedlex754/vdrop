import { useEffect, useMemo, useRef, useState } from "react";
import { useT } from "../../i18n";
import * as api from "../../services/vdrop";
import { describeError } from "../../lib/errors";
import { useDownloads } from "../../stores/downloadsStore";
import { useSettings } from "../../stores/settingsStore";
import type { AnalyzeResult, StreamOption } from "../../types/ipc";
import { formatDuration, suggestFilename } from "../../lib/format";
import { describeStream } from "../../lib/streamLabel";
import { Icon } from "../../components/Icon";
import { Note } from "../../components/ui";

type Phase = "idle" | "analyzing" | "ready" | "error";
type Track = "video" | "audio" | "subtitle";

export function HomePage({
  onStarted,
  handoff,
}: {
  onStarted: () => void;
  /** Pano seridinden devredilen baglanti. `nonce` ayni URL'in tekrar
      gonderilmesini de tetiklenebilir kilar. */
  handoff?: { url: string; nonce: number } | null;
}) {
  const t = useT();
  const { create } = useDownloads();
  const { settings, update } = useSettings();

  const [url, setUrl] = useState("");
  const [phase, setPhase] = useState<Phase>("idle");
  const [result, setResult] = useState<AnalyzeResult | null>(null);
  const [track, setTrack] = useState<Track>("video");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [error, setError] = useState<{ title: string; body: string } | null>(
    null
  );
  const [busy, setBusy] = useState(false);
  const analyzing = useRef(false);

  const media = result?.media;

  // Formatlari iki gruba ayiriyoruz: kullanicinin ilk karari "video mu, yalnizca
  // ses mi" - kalite ikinci karar. Bu ayrimi tek bir uzun listeye gomeyip
  // acikca sormak, yt-dlp'nin 20+ formatinda listeyi okunur tutuyor.
  const groups = useMemo(() => {
    const all = media?.streams ?? [];
    const audio = all.filter((s) => s.kind === "Audio");
    const subtitle = all.filter((s) => s.kind === "Subtitle");
    // DIKKAT: altyazilar "Audio degil" filtresine takilip kalite listesine
    // sizardi; acikca disarida birakiliyorlar.
    const video = all.filter(
      (s) => s.kind !== "Audio" && s.kind !== "Subtitle"
    );
    // Yalnizca ses iceren bir kaynakta video grubu bos kalmasin - ama
    // altyazilar yedege de karismasin.
    const fallback = all.filter((s) => s.kind !== "Subtitle");
    return { video: video.length ? video : fallback, audio, subtitle };
  }, [media]);

  const options =
    track === "audio"
      ? groups.audio
      : track === "subtitle"
        ? groups.subtitle
        : groups.video;
  const selected: StreamOption | null =
    options.find((s) => s.id === selectedId) ?? options[0] ?? null;

  // Devredilen baglantiyi kutuya yaz ve dogrudan cozumle. Kullanici zaten
  // seride "Cozumle" diyerek onay verdi; ikinci bir tiklama istemek
  // gereksiz bir adim olurdu.
  const analyzeRef = useRef<(value?: string) => void>();
  useEffect(() => {
    if (!handoff) return;
    setUrl(handoff.url);
    analyzeRef.current?.(handoff.url);
  }, [handoff]);

  async function analyze(override?: string) {
    const trimmed = (override ?? url).trim();
    // `phase` yerine ref: durum guncellemesi asenkron oldugu icin arka arkaya
    // gelen iki cagri ayni karede ikisi de "idle" gorup gecebilirdi.
    if (!trimmed || analyzing.current) return;
    analyzing.current = true;
    setPhase("analyzing");
    setError(null);
    try {
      const res = await api.analyzeUrl(trimmed);
      setResult(res);
      setTrack("video");
      setSelectedId(null);
      setPhase("ready");
    } catch (e) {
      setError(describeError(e, t));
      setResult(null);
      setPhase("error");
    } finally {
      analyzing.current = false;
    }
  }
  analyzeRef.current = (value) => void analyze(value);

  async function download(goToQueue: boolean) {
    if (!selected || !result || busy) return;
    setBusy(true);
    setError(null);
    try {
      await create({
        url: selected.url,
        suggestedName: suggestFilename(result.media, selected),
        title: result.media.title,
        folder: settings.download_folder || null,
        thumbnailUrl: result.media.thumbnail_url,
        variantIndex: selected.variant_index,
        container: selected.container,
        // yt-dlp formatlarinda secim, format kimligiyle yapilir; adres
        // sayfanin kendisidir ve indirmeyi yt-dlp ustlenir.
        formatId: result.provider_id === "yt-dlp" ? selected.id : null,
      });
      setUrl("");
      setResult(null);
      setSelectedId(null);
      setPhase("idle");
      if (goToQueue) onStarted();
    } catch (e) {
      setError(describeError(e, t));
    } finally {
      setBusy(false);
    }
  }

  async function pasteFromClipboard() {
    try {
      const text = await navigator.clipboard.readText();
      if (text.trim()) setUrl(text.trim());
    } catch {
      // Pano izni yoksa sessizce gec: kullanici elle yapistirabilir.
    }
  }

  const blockedByFfmpeg = result?.is_stream === true && !result.ffmpeg_available;
  const duration = formatDuration(media?.duration_seconds);

  return (
    <div className="home">
      <h1 className="display home-title">{t.home.title}</h1>

      <div className="intake">
        <Icon name="download" size={18} className="muted" />
        <input
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && void analyze()}
          placeholder={t.home.placeholder}
          aria-label={t.home.placeholder}
          spellCheck={false}
          autoComplete="off"
          autoFocus
        />
        <button
          className="btn btn-ghost"
          onClick={() => void pasteFromClipboard()}
          aria-label={t.home.paste}
          title={t.home.paste}
        >
          <Icon name="clipboard" size={17} />
        </button>
        <button
          className="btn btn-primary"
          onClick={() => void analyze()}
          disabled={phase === "analyzing" || !url.trim()}
        >
          {phase === "analyzing" ? t.home.analyzing : t.home.analyze}
        </button>
      </div>

      {phase === "idle" && (
        <p className="caption" style={{ maxWidth: "60ch", textAlign: "center" }}>
          {t.home.subtitle}
        </p>
      )}

      {/* `phase` kosulu YOK: indirme hatasinda faz "ready" kaliyordu, yani
          yt-dlp eksikken Indir'e basan kullanici hicbir sey gormuyordu -
          hata yakalaniyor, saklaniyor, hic cizilmiyordu. */}
      {error && <Note tone="danger" title={error.title} body={error.body} />}

      {phase === "ready" && media && result.is_stream && (
        <Note
          tone={blockedByFfmpeg ? "warn" : "info"}
          title={blockedByFfmpeg ? t.home.ffmpegMissing : t.home.streamNotice}
          body={
            blockedByFfmpeg ? t.home.ffmpegMissingBody : t.home.streamNoticeBody
          }
        />
      )}

      {phase === "ready" && media && (
        <section className="card preview">
          <div className="preview-media">
            {media.thumbnail_url ? (
              <img
                src={media.thumbnail_url}
                alt=""
                onError={(e) => {
                  e.currentTarget.style.display = "none";
                }}
              />
            ) : (
              <Icon name="image" size={40} />
            )}

            <div className="chips">
              {qualityChip(selected) && (
                <span className="chip">{qualityChip(selected)}</span>
              )}
              {selected?.container && (
                <span className="chip">{selected.container.toUpperCase()}</span>
              )}
            </div>

            {duration && <span className="duration-badge num">{duration}</span>}
          </div>

          <div className="preview-body">
            <div>
              <h2 className="preview-title">{media.title}</h2>
              <div className="preview-by">
                {media.uploader && <span>{media.uploader}</span>}
                {media.uploader && <span aria-hidden="true">·</span>}
                <span>{result.provider_id}</span>
              </div>
            </div>

            <div
              className="track-toggle"
              role="group"
              aria-label={t.home.videoAndAudio}
            >
              <button
                aria-pressed={track === "video"}
                onClick={() => {
                  setTrack("video");
                  setSelectedId(null);
                }}
              >
                {t.home.videoAndAudio}
              </button>
              <button
                aria-pressed={track === "audio"}
                disabled={groups.audio.length === 0}
                title={
                  groups.audio.length === 0 ? t.home.noAudioTrack : undefined
                }
                onClick={() => {
                  setTrack("audio");
                  setSelectedId(null);
                }}
              >
                {t.home.audioOnly}
              </button>
              <button
                aria-pressed={track === "subtitle"}
                disabled={groups.subtitle.length === 0}
                title={
                  groups.subtitle.length === 0
                    ? t.home.noSubtitleTrack
                    : undefined
                }
                onClick={() => {
                  setTrack("subtitle");
                  setSelectedId(null);
                }}
              >
                {t.home.subtitlesOnly}
              </button>
            </div>

            {/* Radyo listesi yerine acilir liste: yt-dlp bir videoda 20'den
                fazla format bildirebiliyor ve radyo listesi karti tasiyordu. */}
            <select
              className="format-select"
              value={selected?.id ?? ""}
              onChange={(e) => setSelectedId(e.target.value)}
              aria-label={t.home.download}
            >
              {options.map((stream) => {
                const label = describeStream(stream, {
                  audio: t.common.audio,
                  subtitle: t.common.subtitle,
                  stream: t.common.stream,
                  file: t.common.file,
                });
                const specs = label.specs.join(" · ");
                return (
                  <option key={stream.id} value={stream.id}>
                    {specs ? `${label.name} — ${specs}` : label.name}
                  </option>
                );
              })}
            </select>

            <div className="dest-line">
              <Icon name="folder" size={14} />
              <span className="path" title={settings.download_folder}>
                {settings.download_folder || "—"}
              </span>
              <button
                className="btn btn-ghost"
                onClick={async () => {
                  const folder = await api.selectDownloadFolder();
                  if (folder) await update("download_folder", folder);
                }}
              >
                {t.settings.choose}
              </button>
            </div>

            <div className="preview-actions">
              <button
                className="btn btn-primary btn-lg"
                onClick={() => void download(true)}
                disabled={!selected || busy || blockedByFfmpeg}
              >
                <Icon name="download" size={17} />
                {t.home.download}
              </button>
              <button
                className="btn btn-lg"
                onClick={() => void download(false)}
                disabled={!selected || busy || blockedByFfmpeg}
              >
                <Icon name="queue" size={17} />
                {t.home.addToQueue}
              </button>
            </div>
          </div>
        </section>
      )}
    </div>
  );
}

/**
 * Kucuk resmin uzerindeki kalite cipi.
 *
 * Yalnizca gercekten ayirt edici oldugunda gosterilir: "4K" bilgi tasir,
 * "Dosya" tasimaz. Cozunurluk bilinmiyorsa cip hic cizilmez - bos bir rozet
 * yer kaplar ama bir sey soylemez.
 */
function qualityChip(stream: StreamOption | null): string | null {
  if (!stream?.resolution) return null;
  const match = stream.resolution.match(/(\d+)\s*[x×]\s*(\d+)/i);
  const height = match
    ? Math.min(Number(match[1]), Number(match[2]))
    : Number(stream.resolution.replace(/\D/g, ""));
  if (!height) return null;
  if (height >= 2000) return "4K";
  if (height >= 1400) return "2K";
  return `${height}p`;
}

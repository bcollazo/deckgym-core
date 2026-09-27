// Optional "bring your own assets" card images. The repo itself ships zero card images (see the
// plan's "No card art" decision) and images are off by default; when the viewer is given a URL
// pattern, cards render real images fetched by card id instead of the procedural card face.
//
// Pattern placeholders (substituted in `buildCardImageUrl`):
//   {id}     raw card id, URL-encoded            "A1 001" -> "A1%20001"
//   {id_}    card id with spaces -> underscores   "A1 001" -> "A1_001"
//   {set}    the set portion of the id            "A1 001" -> "A1"
//   {number} the number portion of the id         "A1 001" -> "001"
//   {lang}   the configured language              default "en_US"

import * as PIXI from "pixi.js";

export interface CardImageConfig {
  /** `null` means images are off — always fall back to the procedural card face. */
  pattern: string | null;
  lang: string;
}

export const DEFAULT_LANG = "en_US";

const STORAGE_PATTERN_KEY = "deckgym-viewer:card-image-pattern";
const STORAGE_LANG_KEY = "deckgym-viewer:card-image-lang";

function readLocalStorage(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

function writeLocalStorage(key: string, value: string | null): void {
  try {
    if (value === null) window.localStorage.removeItem(key);
    else window.localStorage.setItem(key, value);
  } catch {
    // Ignore (private browsing, disabled storage, etc.) — settings just won't persist.
  }
}

/**
 * Resolves the active config, highest priority first: `?cards=`/`?lang=` URL params, then a
 * pattern saved earlier via the settings popover (localStorage), then the build-time
 * `VITE_CARD_IMAGE_URL` env var. No match at any level means images stay off.
 */
export function resolveCardImageConfig(): CardImageConfig {
  const params = new URLSearchParams(window.location.search);
  const urlPattern = params.get("cards");
  const urlLang = params.get("lang");

  const pattern = urlPattern || readLocalStorage(STORAGE_PATTERN_KEY) || import.meta.env.VITE_CARD_IMAGE_URL || null;
  const lang = urlLang || readLocalStorage(STORAGE_LANG_KEY) || DEFAULT_LANG;

  return { pattern: pattern || null, lang };
}

/** Persists a new config (localStorage) and reloads the page with it reflected in `?cards=`/
 * `?lang=`, so the change is both durable and shareable via URL. Pass `pattern: null` to turn
 * images back off. */
export function saveCardImageConfigAndReload(config: CardImageConfig): void {
  writeLocalStorage(STORAGE_PATTERN_KEY, config.pattern);
  writeLocalStorage(STORAGE_LANG_KEY, config.lang === DEFAULT_LANG ? null : config.lang);

  const url = new URL(window.location.href);
  if (config.pattern) {
    url.searchParams.set("cards", config.pattern);
    url.searchParams.set("lang", config.lang);
  } else {
    url.searchParams.delete("cards");
    url.searchParams.delete("lang");
  }
  window.location.href = url.toString();
}

function idParts(id: string): { set: string; number: string } {
  const spaceIdx = id.lastIndexOf(" ");
  if (spaceIdx === -1) return { set: id, number: "" };
  return { set: id.slice(0, spaceIdx), number: id.slice(spaceIdx + 1) };
}

export function buildCardImageUrl(pattern: string, id: string, lang: string): string {
  const { set, number } = idParts(id);
  return pattern
    .replaceAll("{id_}", id.replaceAll(" ", "_"))
    .replaceAll("{id}", encodeURIComponent(id))
    .replaceAll("{set}", set)
    .replaceAll("{number}", number)
    .replaceAll("{lang}", lang);
}

export interface CardImageStore {
  readonly enabled: boolean;
  /** Synchronous lookup: a texture if it's already loaded, `undefined` otherwise (not configured,
   * still loading, or failed) — callers should render the procedural fallback for `undefined`. */
  getTexture(id: string): PIXI.Texture | undefined;
  /** Starts loading `id`'s image if it hasn't been tried yet. Safe to call repeatedly. */
  ensureLoading(id: string): void;
  /** Calls `cb` once when `id`'s load finishes (success or failure) — check `getTexture` again
   * inside `cb`. Also starts loading `id` if needed. Returns an unsubscribe function. */
  onSettled(id: string, cb: () => void): () => void;
}

const DISABLED_STORE: CardImageStore = {
  enabled: false,
  getTexture: () => undefined,
  ensureLoading: () => {},
  onSettled: () => () => {},
};

export function createCardImageStore(config: CardImageConfig): CardImageStore {
  if (!config.pattern) return DISABLED_STORE;
  const pattern = config.pattern;
  const lang = config.lang;

  const textures = new Map<string, PIXI.Texture | null>(); // null = failed
  const inFlight = new Set<string>();
  const listeners = new Map<string, Set<() => void>>();

  function settle(id: string) {
    const cbs = listeners.get(id);
    listeners.delete(id);
    cbs?.forEach((cb) => cb());
  }

  function ensureLoading(id: string) {
    if (textures.has(id) || inFlight.has(id)) return;
    inFlight.add(id);
    const url = buildCardImageUrl(pattern, id, lang);
    PIXI.Assets.load<PIXI.Texture>({ src: url, data: { crossOrigin: "anonymous" } })
      .then((texture) => {
        textures.set(id, texture);
      })
      .catch(() => {
        // One warning per id, no more — a missing/broken image for one card shouldn't spam the
        // console for the rest of the game.
        console.warn(`[deckgym viewer] card image failed to load for "${id}": ${url}`);
        textures.set(id, null);
      })
      .finally(() => {
        inFlight.delete(id);
        settle(id);
      });
  }

  return {
    enabled: true,
    getTexture: (id) => textures.get(id) ?? undefined,
    ensureLoading,
    onSettled(id, cb) {
      if (!textures.has(id) && !inFlight.has(id)) ensureLoading(id);
      if (!listeners.has(id)) listeners.set(id, new Set());
      listeners.get(id)!.add(cb);
      return () => listeners.get(id)?.delete(cb);
    },
  };
}

/** Kicks off loading every id up front (called once when a replay loads), so most cards are ready
 * before the viewer ever needs to draw them. */
export function preloadCardImages(store: CardImageStore, ids: Iterable<string>): void {
  if (!store.enabled) return;
  for (const id of ids) store.ensureLoading(id);
}

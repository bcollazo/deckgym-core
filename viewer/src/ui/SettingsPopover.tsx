import { useState } from "react";
import { DEFAULT_LANG, resolveCardImageConfig, saveCardImageConfigAndReload } from "../board/cardImages";

const LANGUAGES = [
  { value: "en_US", label: "English (US)" },
  { value: "es_ES", label: "Spanish" },
  { value: "ja_JP", label: "Japanese" },
  { value: "ko_KR", label: "Korean" },
  { value: "zh_TW", label: "Chinese (Traditional)" },
];

/**
 * Header settings popover for the optional "bring your own" card images (see
 * `board/cardImages.ts` and `docs/replay-viewer-plan.md`'s "Round 2" section). The repo ships no
 * images and this is off by default — a pattern here is just an example placeholder, not a
 * built-in default.
 */
export function SettingsPopover() {
  const [open, setOpen] = useState(false);
  const initial = resolveCardImageConfig();
  const [pattern, setPattern] = useState(initial.pattern ?? "");
  const [lang, setLang] = useState(initial.lang || DEFAULT_LANG);

  return (
    <div className="settings-popover-wrap">
      <button type="button" onClick={() => setOpen((v) => !v)} title="Card image settings">
        ⚙ Images
      </button>
      {open && (
        <div className="settings-popover">
          <label htmlFor="card-image-pattern">Card image URL pattern</label>
          <input
            id="card-image-pattern"
            type="text"
            value={pattern}
            onChange={(e) => setPattern(e.target.value)}
            placeholder="e.g. https://cards.deckgym.com/cards/{lang}/{id_}.webp"
          />
          <p className="settings-hint">
            Placeholders: <code>{"{id}"}</code> (URL-encoded), <code>{"{id_}"}</code> (underscored),{" "}
            <code>{"{set}"}</code>, <code>{"{number}"}</code>, <code>{"{lang}"}</code>. Leave empty to use the
            built-in procedural cards.
          </p>
          <label htmlFor="card-image-lang">Language</label>
          <select id="card-image-lang" value={lang} onChange={(e) => setLang(e.target.value)}>
            {LANGUAGES.map((l) => (
              <option key={l.value} value={l.value}>
                {l.label}
              </option>
            ))}
          </select>
          <div className="settings-actions">
            <button type="button" onClick={() => saveCardImageConfigAndReload({ pattern: pattern.trim() || null, lang })}>
              Save &amp; reload
            </button>
            <button
              type="button"
              className="secondary"
              onClick={() => {
                setPattern("");
                saveCardImageConfigAndReload({ pattern: null, lang });
              }}
            >
              Turn off images
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

# deckgym replay viewer

A static web app that loads a deckgym replay (`.json`, written by the CLI's `--replay-dir` flag)
and steps through the game with animated transitions. See
`../docs/replay-viewer-plan.md` for the full design.

Stack: React 19 + TypeScript, the board is [PixiJS v8](https://pixijs.com/) hosted via
[`@pixi/react` v8](https://react.pixijs.io/), transitions are [GSAP](https://gsap.com/) timelines
(`PixiPlugin`, `MotionPathPlugin`, `CustomEase`), plus [`pixi-filters`](https://pixijs.io/filters/)
for the shockwave/bloom effects. Vite for the build.

## Generating a replay

From the repo root:

```bash
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt \
  -n 3 --players r,r --replay-dir replays/
```

This writes one `<game_id>.json` file per game into `replays/`.

## Running the viewer

```bash
npm install
npm run dev
```

Then either:
- drag a replay `.json` file onto the page, or use the file picker,
- pass `?url=/path/to/replay.json` in the URL to load one over HTTP, or
- do nothing — the app falls back to the bundled `public/sample-replay.json` demo replay.

## Controls

- `←` / `→`: step one action backward/forward (animated)
- `Shift+←` / `Shift+→`: jump to the previous/next turn boundary
- `Home` / `End`: jump to the start/end of the game
- `Space`: play/pause autoplay
- Click a row in the log to seek there; click/drag the scrubber to seek anywhere (scrubbing more
  than one step jumps straight to that snapshot, no animation)
- The speed dropdown controls `gsap.globalTimeline.timeScale()`, so it affects every animation

## Card images (optional, off by default)

The repo ships **no card images** — cards render procedurally by default (a type-colored frame,
name, HP bar, etc. — see `board/cardArt.ts`). If you have somewhere to fetch real card images by
id, you can point the viewer at it and it'll use them instead, falling back to the procedural card
for any id that 404s or errors.

Give it a URL pattern with placeholders:

| Placeholder | Meaning | `"A1 001"` becomes |
| --- | --- | --- |
| `{id}`     | raw id, URL-encoded    | `A1%20001` |
| `{id_}`    | id, spaces → underscores | `A1_001` |
| `{set}`    | the set portion of the id | `A1` |
| `{number}` | the number portion of the id | `001` |
| `{lang}`   | the configured language (default `en_US`) | `en_US` |

Configure it, highest priority first:
1. `?cards=<pattern>&lang=<lang>` URL params,
2. the ⚙ Images popover in the header (saved to `localStorage`, then reloads the page with it
   reflected in the URL),
3. `VITE_CARD_IMAGE_URL` at build time (e.g. in a `.env` file).

deckgym.com happens to run a public, CORS-enabled host that works with this pattern:
`https://cards.deckgym.com/cards/{lang}/{id_}.webp` — that's just an example to try, not a
built-in default; the pattern field starts empty and images stay off until you set one.

## Quality checks

```bash
npm run build       # tsc -b && vite build
npx tsc -b           # typecheck only
npx vitest run       # unit tests (anim/diff.ts)
```

## Regenerating the sample replay

```bash
cd ..
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt \
  -n 1 --seed <a seed that produces a decisive, non-tie game> --players aa,aa \
  --replay-dir /tmp/sample
cp /tmp/sample/<game_id>.json viewer/public/sample-replay.json
```

## Notable implementation choices

- `board/scene.ts` builds the entire Pixi scene graph **imperatively** inside `@pixi/react`'s
  `<Application onInit>`, rather than as nested `@pixi/react` JSX. The board is a persistent,
  GSAP-driven scene that outlives React re-renders, and the declarative reconciler is a poor fit
  for "mutate these objects' properties over time" — see that file's header comment.
- `store/playbackController.ts` is plain TypeScript (no React) so Pixi/GSAP objects never become
  React state; `App.tsx` subscribes to it with `useSyncExternalStore`.
- `anim/buildTimeline.ts` builds one GSAP timeline per transition, cached by `playbackController`,
  so stepping backward `.reverse()`s the same timeline rather than re-deriving one. See that file's
  header comment for how a timeline built lazily for a *backward*-first traversal is primed to have
  correct start values.
- No card art ships with the repo: cards are drawn procedurally by default in `board/cardArt.ts`
  (type-colored frame, name, HP bar, energy pips, status badges, a holo shimmer for `ex` cards), or
  render a real image (see "Card images" above) with the same overlays composited on top.
- The board's spatial layout (`board/layout.ts`) mirrors the official app: a portrait column with
  the opponent's half (mirrored) above a center divider and the viewer's own half below, each with
  a name bar, hand row, bench, and active spot, plus deck/discard/energy-zone icons at the outer
  corners.

## Known simplifications (see the plan doc's "Deviations" section for the full list)

- Deck and hand *counts* (not identities) drive most of the board — `ViewState`'s hand/deck are
  unordered id lists with no stable per-card instance identity across snapshots when there are
  duplicates, so cards flying out of/into a board slot (play, evolve, KO, attach a tool) are
  individually animated because the action that caused them names the exact card, while other
  hand/deck count changes (a draw, a search effect) just update the count. The discard pile is the
  one exception: since it's a stack, its actual top card (`discard.at(-1)`) is shown small.
- Decorative one-shot flourishes (particle bursts, screen shake, the shockwave filter, confetti)
  animate independently of the reversible per-step timeline, so they don't un-play when you step
  backward through them — only the state-bearing tweens (position, alpha, scale, HP bar, card
  content) do.

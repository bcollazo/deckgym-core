# deckgym replay viewer

A static web app that loads a deckgym replay (`.json`, written by the CLI's `--replay-dir` flag)
and steps through the game with animated transitions. See
`../docs/replay-viewer-plan.md` for the full design.

Stack: React 19 + TypeScript, the board is [PixiJS v8](https://pixijs.com/) hosted via
[`@pixi/react` v8](https://react.pixijs.io/), transitions are [GSAP](https://gsap.com/) timelines
(`PixiPlugin`, `MotionPathPlugin`, `CustomEase`), plus [`pixi-filters`](https://pixijs.io/filters/)
for the glow/bloom effects. Vite for the build.

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

### Playing one game and opening it straight from the terminal

For the "play a game, look at it" loop, `cargo run -- play` (root README's "Replay viewer" section)
plays exactly one game and prints a link on its last line:

```bash
cd viewer && npm run dev        # leave this running
```
```bash
# in another terminal, from the repo root
cargo run -- play example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt --players e2,r
```

Ctrl+Click (or Cmd+Click) the printed `http://localhost:5173/?url=/replays/<game_id>.json` link to
open that exact game. This works because `vite.config.ts` has a small dev-only plugin that serves
the repo-root `replays/` folder at `/replays/*` (JSON only, no path traversal) — it runs under
`npm run dev` and `npm run preview` both, but not the static `dist/` build `npm run build` produces
(there's no server there to serve `replays/` from). If `--viewer-url`/`DECKGYM_VIEWER_URL` points
somewhere other than the default `http://localhost:5173`, or `--replay-dir` isn't the default
`replays/`, see that flag's `--help` text for what has to line up for the printed link to resolve.

## Controls

- `←` / `→`: step one action backward/forward (animated)
- `Shift+←` / `Shift+→`: jump to the previous/next turn boundary
- `Home` / `End`: jump to the start/end of the game
- `Space`: play/pause autoplay
- Click a row in the log to seek there; click/drag the scrubber to seek anywhere (scrubbing more
  than one step jumps straight to that snapshot, no animation)
- The speed dropdown controls `gsap.globalTimeline.timeScale()`, so it affects every animation

## Card images (optional, off by default)

The repo ships **no card images** — cards render procedurally by default (a name+HP header, stage/
`ex` badges, a type-colored art band, its attacks with their energy cost, its ability, a weakness/
retreat-cost footer — see `board/cardArt.ts` and the plan doc's "Round 3" notes). If you have
somewhere to fetch real card images by id, you can point the viewer at it and it'll use them
instead, sized with `object-fit: contain` semantics (scaled uniformly, centered, never distorted,
clipped to the card's rounded corners), falling back to the procedural card for any id that 404s or
errors. A real image draws with no frame/border of its own (just the image, rounded corners, and the
live HP/energy/status/tool overlays every card gets) — the frame/holo-shimmer treatment is only for
procedural cards, which need *something* to read as a card edge around a flat color fill.

Images must be the same aspect ratio as deckgym's own (367x512, `board/layout.ts`'s `CARD_ASPECT`)
to fill the card without letterboxing; a different ratio still renders correctly, just letterboxed.

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
npx vitest run       # unit tests (anim/diff.ts, board/layout.ts, board/imageFit.ts, anim/formatAction.ts)
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
  with real card info (see "Card images" above for what's shown, tiered by the card's own rendered
  size — a small hand card just gets a name+HP header), or render a real image with HP
  bar/energy-pip/status/tool overlays composited on top either way.
- The board's spatial layout (`board/layout.ts`) mirrors the official app, matched against a
  reference screenshot: a portrait column, tightly packed (actives overlapping the center divider
  slightly, bench snug against the active, a large fanned/rotated player hand), with the opponent's
  half (mirrored) above the divider and the viewer's own half below, plus deck/discard/energy-zone
  icons at the outer corners.
- The canvas's aspect ratio is enforced by a wrapper `<div>` carrying the real `aspect-ratio`
  (computed from `BOARD_WIDTH`/`BOARD_HEIGHT`, `board/Board.tsx`), with the canvas filling `100%`
  of it via a `!important` rule (`index.css`) — `<Application autoDensity>` sets the canvas's own
  CSS size as an inline style, which otherwise silently defeats a plain responsive stylesheet rule
  and can stretch everything on the canvas non-uniformly. See the plan doc's "Round 3" notes if
  this needs touching again.
- Every in-play slot (active or bench) reserves a small "HP strip" (a bar + number, side by side)
  directly *above* its card, not floating over the card's own corner or drawn inside/below its
  face — `board/layout.ts`'s `hpStripHeight`/`HP_STRIP_GAP`/`slotRects` reserve exactly that much
  row space so it can never land on a neighboring row, the divider or the turn banner (checked by
  `board/layout.test.ts`, which asserts no two slots' card/strip rects — nor the divider band —
  ever intersect), and `board/cardArt.ts` draws it at the matching offset. See the plan doc's
  "Round 4" notes.
- The board is a flat, near-black fill (`scene.ts`'s `background`), not a gradient/vignette — a
  `PIXI.Graphics` rect can't band the way even a subtle canvas gradient can once the board is big.
- An attack's board-wide effects (screen shake, a full-stage `ShockwaveFilter`) were removed in
  Round 4: only the two cards involved animate (the attacker's lunge; the defender's small recoil, a
  brief white flash via `scene.spawnFlash`, the damage number, and the ghost HP bar drain) — see
  `anim/buildTimeline.ts`'s attack-sequence comment. The win-screen bloom/confetti are unaffected
  (still whole-board, deliberately — see the plan's "Round 4 deviations").
- An active↔bench swap (retreat, a switch effect, a post-KO promotion) is detected *before* the
  generic per-slot diff (`anim/diff.ts`'s `detectActiveBenchMove`) and excluded from it entirely, so
  it produces exactly one `activeSwitch` change instead of the per-slot loop misreading "which
  Pokemon is in this slot changed" as an evolution plus bogus HP/energy/tools/status deltas (the
  root cause of HP bars animating on a swap where neither Pokemon's HP changed — see the plan's
  "Round 4" notes and `anim/diff.test.ts`'s retreat/promotion cases). `buildTimeline.ts`'s handler
  cross-fades each slot's *content* (not position — slots are fixed per index, see "Known
  simplifications" below) from the correct snapshot either direction, so stepping backward reverses
  it cleanly too.

## Known simplifications (see the plan doc's "Deviations" section for the full list)

- Deck and hand *counts* (not identities) drive most of the board — `ViewState`'s hand/deck are
  unordered id lists with no stable per-card instance identity across snapshots when there are
  duplicates, so cards flying out of/into a board slot (play, evolve, KO, attach a tool) are
  individually animated because the action that caused them names the exact card, while other
  hand/deck count changes (a draw, a search effect) just update the count. The discard pile is the
  one exception: since it's a stack, its actual top card (`discard.at(-1)`) is shown small.
- Decorative one-shot flourishes (particle bursts, the defender's hit-flash, confetti) animate
  independently of the reversible per-step timeline, so they don't un-play when you step backward
  through them — only the state-bearing tweens (position, alpha, scale, HP bar, card content) do.
- An active↔bench swap cross-fades each slot's *content* in place rather than flying the two cards
  to each other's positions — slots are persistent objects keyed by index (0 = active, 1-3 = bench),
  not by which Pokemon currently occupies them (see `board/scene.ts`'s header comment), so "moving"
  a Pokemon to a new slot means swapping what that fixed-position slot displays, the same way every
  other slot update already works.

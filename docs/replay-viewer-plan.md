# Replay Viewer + External Bots Implementation Plan

## Summary
Build a way to inspect games played by the CLI in a good-looking web UI that can step forward and
backward through game states with animated transitions, and let bots written outside this repo
(in any language) play games through a small JSON protocol.

Three parts, sharing one display-oriented state schema (`ViewState`):
1. **Replay recording** – the CLI writes one replay file per game: a full `ViewState` snapshot at
   every decision point plus the legal options and the chosen action.
2. **External bots** – an `ExternalPlayer` that talks JSON lines over stdin/stdout to a subprocess.
3. **Viewer** – a static web app (`viewer/`) that loads replay files and animates transitions.

## Decisions (already made)
- **Snapshots, not re-simulation.** Each step stores a full `ViewState`. Stepping backward is an
  index lookup; the browser never runs the engine; replays survive engine changes.
- **No engine-emitted event log.** The viewer derives transitions from
  `(prevSnapshot, nextSnapshot, chosenAction)`. The chosen action says *why* (which attack, which
  trainer), the snapshot diff says *what changed*. Coin flips are not visualized (only their result).
- **Viewer stack:** React 19 shell (panels, log, scrubber) + **PixiJS v8** board rendered via
  **`@pixi/react` v8** + **GSAP** timelines (with `PixiPlugin`, `MotionPathPlugin`, `CustomEase`)
  + **`pixi-filters`** (glow, bloom, shockwave, godray). Vite + TypeScript.
- **Viewer lives in this repo** under `viewer/`.
- **No card art.** Cards are drawn procedurally (type-colored frame, name, HP, attacks, gradient
  "art" area, holo shimmer for ex cards).
- **Bots see only their own information** (opponent hand as a count, deck order hidden).
  Replays keep the omniscient view.

## Part 1: Replay recording (Rust)

### `ViewState` (`src/replay/view.rs`)
A stable, serializable, display-oriented projection of `State`. It decouples the viewer and the
bot protocol from engine internals (`pub(crate)` turn flags etc.). Cards are referenced by id
(`Card::get_id()`), with a card table in the replay header.

```rust
pub struct ViewState {
    pub turn: u8,
    pub current_player: usize,
    pub points: [u8; 2],
    pub winner: Option<GameOutcome>,
    pub players: [PlayerView; 2],
    pub stadium: Option<String>, // card id
}
pub struct PlayerView {
    pub hand: Option<Vec<String>>, // None when redacted (bot view of opponent)
    pub hand_count: usize,
    pub deck_count: usize,
    pub discard: Vec<String>,
    pub discard_energies: Vec<EnergyType>,
    pub energy_zone: EnergyZone,
    pub in_play: [Option<SlotView>; 4], // 0 = active, 1..=3 bench
}
pub struct SlotView {
    pub card: String,
    pub hp: u32, pub max_hp: u32,
    pub energy: Vec<EnergyType>,
    pub tools: Vec<String>,
    pub status: Vec<StatusCondition>, // poisoned/asleep/paralyzed/burned/confused
    pub played_this_turn: bool,
}
impl ViewState {
    pub fn omniscient(state: &State) -> Self;
    pub fn for_player(state: &State, player: usize) -> Self; // redacts opponent hand
}
```
Add `ts-rs` (dev/optional feature is fine) to export TypeScript types for `ViewState` and the
replay file into `viewer/src/types/generated/`; alternatively hand-write TS types that mirror the
Rust structs and add a Rust test that round-trips a sample replay so drift is caught.

### Replay file format (`src/replay/mod.rs`)
One gzip-free, compact JSON file per game: `<replay_dir>/<game_id>.json`.
```jsonc
{
  "version": 1,
  "game_id": "…", "seed": 123,
  "players": [{ "name": "e3", "deck": ["A1 001", …] }, { "name": "mybot.py", "deck": […] }],
  "cards": { "A1 001": { /* full Card JSON as in database.json */ } },
  "steps": [
    { "ply": 0, "turn": 1, "actor": 0,
      "state": { /* ViewState before the action */ },
      "options": [{ "text": "EndTurn", "action": { /* SimpleAction JSON */ } }, …],
      "chosen": 3,
      "note": null }
  ],
  "final_state": { /* ViewState */ },
  "outcome": { "Win": 0 }
}
```
`options[i].text` is the `Display` of `SimpleAction`. `note` is an optional string a bot may attach
to its decision (Part 2).

### `ReplayRecorder` (`src/replay/recorder.rs`)
A `SimulationEventHandler` following the `DataExporter` pattern:
- `on_game_start` → begin a replay. Extend the hook (or add a new default-implemented hook, e.g.
  `on_game_start_with_metadata`) so it receives the decks, player names (`Debug` of the player /
  bot name) and the seed. Keep existing handlers compiling.
- `on_action` → push a step (state before, options, chosen index).
- `on_game_end` → set `final_state`/`outcome`, write the file.
- `--replay-sample N`: only write the first N games (per simulation run).

### CLI
`cargo run simulate A.txt B.txt -n 100 --replay-dir replays/ [--replay-sample 10]`
(also wire it through `simulate_against_folder`).

### Tests
- Record a seeded game via the public `Game`/simulation API; assert the replay JSON round-trips,
  `steps.len()` equals the number of `play_tick` calls, and `final_state.winner` matches.
- `ViewState::for_player` hides the opponent hand but keeps `hand_count`.

## Part 2: External bots (Rust + docs + example)

### Protocol (JSON lines over stdin/stdout, versioned)
```
engine → bot  {"type":"hello","protocol":1}
bot → engine  {"type":"hello","name":"MyBot v2"}
engine → bot  {"type":"new_game","game_id":"…","you":1,"deck":["A1 001",…]}
engine → bot  {"type":"decide","ply":41,"state":{…ViewState for this bot…},
               "actions":[{"i":0,"text":"EndTurn","action":{…}}, …]}
bot → engine  {"i":3,"note":"attack wins race"}      // note optional; stored in the replay
engine → bot  {"type":"game_over","outcome":{"Win":1}}
```
- Bot stderr is passed through (for debugging); only stdout is protocol.
- Invalid JSON, out-of-range index, timeout (default 10s, `--bot-timeout-ms`), or crash → log a
  warning and fall back to the first legal action (document this; do not panic the simulation).

### `ExternalPlayer` (`src/players/external_player.rs`)
- Implements `Player`. Spawns the command once (`sh -c <cmd>` on unix) and keeps it alive across
  games; sends `new_game` when it sees a new game, `game_over` via the replay/end hook if available
  (acceptable to skip `game_over` if the `Player` trait has no end hook—document it).
- Exposes the bot's `note` so the replay can store it (e.g. via a `last_note()` method on the
  `Player` trait with a default `None` implementation).
- `PlayerCode::X` (code `x`), with commands from `--bot-a "<cmd>"` / `--bot-b "<cmd>"`.
- Parallel simulations: each game gets its own `ExternalPlayer` (one subprocess per game/thread) –
  acceptable for now.

### Deliverables
- `docs/bot-protocol.md` – the protocol, message schemas, failure behavior, example session.
- `examples/bots/random_bot.py` – stdlib-only Python bot that picks a random action and attaches a
  short note.
- Integration test: play a full game against a tiny scripted bot (e.g. the Python example or a
  `sh`/`printf`-free Rust test binary) and assert the game finishes. If depending on `python3` in
  tests is a problem, gate that test behind a check for `python3` availability.
- README section describing how to run with an external bot.

## Part 3: Viewer (`viewer/`)

### Stack
Vite + React 19 + TypeScript, `pixi.js` v8, `@pixi/react` v8, `gsap` (PixiPlugin, MotionPathPlugin,
CustomEase registered once), `pixi-filters`. No other UI framework needed.

### Loading
- Drag-and-drop or file-picker for a replay `.json`; also `?url=<path>` to fetch one.
- Ship `viewer/public/sample-replay.json` (generated by the CLI) so the app demos out of the box.

### Layout
```
┌──────────────────────────────────────────────────────────┬──────────────────────┐
│ P1 name   hand ▢▢▢▢  deck 11  discard 4   ★★☆            │ LOG (one row / step) │
│   bench  [card] [card] [    ]                            │▶ T7 P1 Play Sabrina  │
│                 ACTIVE  card                             ├──────────────────────┤
│  ─────────────────── stadium ──────────────────────────  │ OPTIONS @ this step  │
│                 ACTIVE  card                             │ ○ EndTurn            │
│   bench  [card] [card] [    ]                            │ ● Attack Giant Bloom │
│ P0 name   hand (face up)  deck 9    ★☆☆                  │ bot note: "…"        │
├──────────────────────────────────────────────────────────┴──────────────────────┤
│ ⏮  ◀  ▶  ⏭   ▶ 1x/2x/4x  ───────●──────────  ply 41/118 · turn 7   (◆ = KO)    │
└─────────────────────────────────────────────────────────────────────────────────┘
```
- Board = one Pixi `<Application>` canvas; everything else is React DOM.
- Keyboard: ←/→ step, Shift+←/→ step a whole turn, Home/End, Space autoplay.
- Scrubber with turn tick marks and ◆ markers on steps where points changed.
- Clicking a log row seeks there. Options panel shows all legal actions with the chosen one
  highlighted and the bot note.

### Transition engine
- `buildTimeline(prev: ViewState, next: ViewState, chosen: SimpleAction): gsap.core.Timeline` –
  a pure function (plus handles to Pixi display objects) living in `viewer/src/anim/`.
- Diff into semantic changes first (`diff.ts`, unit-tested with vitest): card left/entered a slot,
  evolution (same slot, different card), HP delta per slot, energy added/removed, tools, status
  changes, hand/deck/discard count deltas, points delta, KO (slot emptied while HP hit 0 / points
  rose), active switch (active and bench swapped), turn change, stadium change.
- Then choreograph using the chosen action (e.g. `Attack(attack)` → attack sequence targeting the
  opponent's slots whose HP dropped).
- Forward step = `tl.play()`; backward step = build the timeline for (i-1 → i) and `tl.reverse()`
  from its end; scrubber drags beyond one step jump directly to the snapshot (no animation).
- Playback speed via `gsap.globalTimeline.timeScale()`.
- At the end of every step, reconcile the Pixi scene to `next` exactly (snapshot is the source of
  truth; animations never accumulate drift).
- Respect `prefers-reduced-motion` (skip to snapshot, keep simple fades).

### Effects ("make it look cool")
- **Attack:** attacker anticipation pull-back → lunge → ~70 ms hit-stop → screen shake →
  `ShockwaveFilter` ripple at target → floating damage number with overshoot ease.
- **HP bar:** "ghost bar" – a red remnant that lingers then drains after the real bar drops.
- **Energy attach:** orb flies from the energy-zone indicator along a bezier path
  (MotionPathPlugin); type-specific particle burst on arrival (fire embers, water droplets,
  lightning sparks, grass leaves, psychic sparkles, etc.).
- **KO:** card shatters into shards (particles), brief slow-motion, then the points star fills
  with a glow.
- **Cards:** subtle 3D-ish tilt on hover (skew/scale), holo shimmer (animated gradient/`GlowFilter`
  + additive blend) on ex cards; Flip-style flights between hand/bench/discard/deck.
- **Evolution:** flash + scale pop into the new card.
- **Turn change:** banner swipe with the active player's energy color flooding the board edge.
- **Win:** `AdvancedBloomFilter`/`GodrayFilter` + confetti burst.
- Procedural card frames colored by energy type; readable text at 1× and on HiDPI
  (`resolution: devicePixelRatio`, `autoDensity: true`).

### Quality bar
- `npm run build` and `npm run typecheck` (tsc --noEmit) pass; `vitest` tests for `diff.ts`.
- Verify rendering with Playwright (Chromium is preinstalled at `/opt/pw-browsers`, do not run
  `playwright install`): load the sample replay, step forward/backward, take screenshots.
- `viewer/README.md`: how to generate replays and run the viewer (`npm install && npm run dev`).
- `viewer/node_modules` and `viewer/dist` are git-ignored.

## Part 4 (optional, only if time allows)
- `cargo run -- view replays/` subcommand serving `viewer/dist` + a replay index page.
- Toggle to view a replay from one player's perspective (hide the other hand).

## Order of work
1. Part 1 (ViewState, recorder, CLI flag, tests). Commit.
2. Part 3 viewer MVP: load, static board render, stepping, log, options, scrubber. Commit.
3. Part 3 transitions + effects. Commit.
4. Part 2 external bots + docs + example + test. Commit.
5. Part 4 if time allows.

## Deviations

- **TS types are hand-written, not `ts-rs`-generated.** The plan allowed either. `viewer/src/types/replay.ts`
  hand-mirrors `ViewState`/`Replay`/etc., and `src/replay/recorder.rs`'s integration test
  (`tests/replay_test.rs`) round-trips a real replay through `serde_json`, so a shape drift in the
  Rust side is still caught by CI even though the TS side isn't generated. Revisit if the schema
  grows enough that this becomes error-prone.
- **`--replay-sample N` caps per call to `simulate()`, not per whole folder run.** When
  `deck_b_or_folder` is a folder, `simulate_against_folder` calls `simulate()` once per opponent
  deck; each call gets its own sample counter, so with `-n 1000 --replay-sample 10` against a
  10-deck folder you get up to 10 replays *per opponent deck* (100 total), not 10 total. Documented
  here rather than threading a shared counter through the folder sweep, which would have meant
  passing an `Arc` through a CLI-level function that otherwise only takes plain config structs.

- **Part 3's MVP and transition-engine steps landed in one commit, not two.** The board's
  persistent Pixi scene graph (`scene.ts`) and its GSAP timelines (`buildTimeline.ts`) turned out to
  be tightly coupled from the start (e.g. the HP bar's "ghost" bar only makes sense once the
  animation system exists), so splitting "static board" from "animated board" into separate commits
  would have meant writing throwaway code. Both are in the "Add replay viewer" commit.
- **`@pixi/react` hosts the canvas; the scene graph itself is built imperatively**, not as nested
  `@pixi/react` JSX (`board/scene.ts`'s header comment has the reasoning: a persistent, GSAP-driven
  scene fights a declarative reconciler for ownership of node state). `@pixi/react`'s `<Application
  onInit>` still does the real job the plan wanted from it — mounting/sizing/disposing the Pixi
  canvas within React's lifecycle.
- **Hand-writing the reconciled state into two rows, not one.** The plan's ASCII mock draws each
  player's name/hand/deck/discard/points on a single header line; in practice that many elements
  collided (name text, hand cards, and the energy-zone dot all fighting for the same ~150px of
  width). `board/layout.ts` splits it into an "info" row (name, energy zone, points, deck/discard)
  and a "hand" row one step further from the board's center.
- **Card-identity flights are only rendered where the action names the exact card** (`Place`,
  `Evolve`, `Play`, `AttachTool`); other hand/deck/discard count changes (draws, searches, etc.) get
  a lightweight count update with no flight animation, since `ViewState`'s hand/deck/discard are
  unordered id lists with no stable per-card instance identity across snapshots when there are
  duplicate cards. See `viewer/README.md`'s "Known simplifications".
- **Decorative flourishes (particle bursts, screen shake, the shockwave filter, confetti) aren't
  part of the reversible per-step timeline** — they animate independently via their own `gsap.to`
  calls, triggered by a `tl.call` inside the timeline. Only the state-bearing tweens (position,
  alpha, scale, HP-bar fraction, a slot's card content) are true children of the timeline, so
  stepping backward correctly un-plays those; a flourish just plays again rather than reversing. One
  visible consequence: the *first* time a step is traversed backward without ever having been played
  forward, `playbackController.ts` has to "prime" that step's timeline (reconcile to its `prev`
  snapshot, build it, then fast-forward to its end so the board doesn't visually flash) — a
  decorative flourish can fire for real during that fast-forward, so you may see e.g. KO shards or
  confetti appear an instant before the reverse animation itself plays. Harmless, but worth knowing
  about if it looks like a duplicate effect.
- **No `game_over` protocol message, and one bot subprocess per game rather than truly persistent
  across games.** See `docs/bot-protocol.md`'s own "Deviations" section for the reasoning (no
  `Player` end-of-game hook to send it from; the CLI already constructs a fresh `Player` per game
  for every player type, external bots included).
- **A replay's recorded player name for an external bot is the `--bot-a`/`--bot-b` command string,
  not the bot's self-reported `hello` name.** `GameStartMetadata` (used to name players in the
  replay) is built from `Debug`, right when a game starts — before `ExternalPlayer` has lazily
  spawned its subprocess and received the bot's `hello` reply. The name is still visible in the
  engine's own logs mid-game (`ExternalPlayer(<name>)` once spawned) and in the bot's own stderr.

## Round 2 (viewer follow-up)

A second pass on the viewer, after the user reviewed Round 1's screenshots, covering three things:

1. **Optional "bring your own" card images.** `viewer/src/board/cardImages.ts` resolves a URL
   pattern (`?cards=`/`?lang=` params → a settings-popover choice saved in `localStorage` →
   `VITE_CARD_IMAGE_URL` at build time → off) and loads textures by id with `PIXI.Assets.load`,
   falling back to the procedural card on a 404/error. The repo still ships zero images and images
   stay off until configured — see `viewer/README.md`'s "Card images" section for the placeholder
   syntax and the header's ⚙ Images popover (`ui/SettingsPopover.tsx`).
2. **Board layout overhaul** (`board/layout.ts`, `board/scene.ts`) to mirror the official app's
   spatial layout instead of Round 1's ad hoc landscape strip: a portrait column, mirrored
   opponent-half-above-divider-above-viewer's-own-half, with name bars, hand rows, bench, active
   spots, and deck/discard/energy-zone icons at the outer corners on each side. Points are now
   filling pips (with a glow on the filled ones) instead of `**-` text. `board/cardArt.ts` was
   rewritten to lay out its overlays (HP number, HP bar, energy pips, status badges) as *fractions*
   of the card's own size rather than fixed pixel offsets — the fixed-offset version was the actual
   cause of Round 1's "hands render as empty outlines" bug (a card small enough that a fixed
   pixel band reserved for overlays left a negative-height "art" rectangle).
3. **Fixes**: human-readable log/options text (`anim/formatAction.ts`, unit-tested), the hand
   rendering bug above, and a subtle hover tilt on interactive cards (GSAP rotation + scale on
   `pointerover`/`pointerout`).

### Round 2 deviations
- **The board's exact proportions are an interpretation of the official app's layout, not a
  pixel-accurate copy** — the request described the layout in words (a numbered top-to-bottom list
  of bands) rather than from a shared screenshot Claude could measure, so `layout.ts`'s row heights
  and corner placements are a reasonable reading of that description, verified visually with
  Playwright rather than diffed against a reference image.
- **Card-image loading uses `PIXI.Assets.load`'s default texture loader**, which prefers a
  `fetch()`-based `createImageBitmap` path over the `<img crossOrigin>` path in browsers that
  support it; the explicit `{data: {crossOrigin: "anonymous"}}` passed to `Assets.load` only
  affects the fallback path, but is included per the request for explicitness/robustness on
  browsers where the fallback is what actually runs.
- **Deck piles stay an abstract "stack" graphic** (layered semi-transparent rounded rects + a
  count), not a `CardVisual` — unlike the discard pile, which now shows its real top card
  (`discard.at(-1)`) small, per the request. A deck's contents are meant to stay hidden, so there's
  no "top card" to show.
- **The settings popover applies a new image pattern by saving to `localStorage` and reloading the
  page** (reflecting the choice in `?cards=`/`?lang=` too) rather than live-reconfiguring an
  already-running `CardImageStore`. Simpler and more robust than clearing every in-flight texture
  subscription and forcing a redraw of every currently-visible card.

## Round 3 (viewer follow-up, with a reference screenshot)

The user shared an actual screenshot of the official app this time (Round 2 only had a text
description) and reported card images looking horizontally stretched. Three things:

1. **Fixed the card-image stretch — root cause, not just the aspect-ratio constant.** The
   requested `CARD_ASPECT = 367/512` (deckgym's real image size) replaces 63:88 everywhere
   (`layout.ts`), and `cardArt.ts`'s image sprite now sizes itself with "contain" semantics
   (`board/imageFit.ts`'s `fitContain`, unit-tested) — scaled uniformly by
   `min(boxW/texW, boxH/texH)` and centered, never setting width/height independently. But neither
   of those alone explains a *visible* stretch (63:88 and 367:512 are within 0.2% of each other).
   The actual cause: `<Application autoDensity>` sets the canvas element's CSS width/height as
   fixed **inline** pixel values (`style.width = "640px"`), which beats any plain external
   stylesheet rule. The viewer's `width:auto;height:auto;max-width;max-height` rule was therefore
   dead code — only `max-width`/`max-height` (independent per axis) actually applied, so whenever
   the wrap's available width and height weren't in the board's exact ratio, one axis got clamped
   without the other following, stretching *everything* on the canvas non-uniformly (worst-case,
   confirmed by testing four very different viewport shapes and finding the canvas's rendered CSS
   ratio drifting from its intrinsic ratio). Fixed with a two-level box (`Board.tsx`,
   `index.css`): an outer frame carries the correct `aspect-ratio` (computed inline from
   `BOARD_WIDTH`/`BOARD_HEIGHT`, so it can't drift out of sync) and fits the wrap; the canvas then
   fills `100% !important` of that already-correctly-shaped frame — `!important` is the only thing
   that reliably beats Pixi's inline style.
2. **Board now fills the canvas, matching the reference's density**: `layout.ts` was retuned with
   much bigger cards, tight gaps, actives overlapping the divider, bench snug against actives, and
   a large, fanned, rotated player hand (`handCardPos` now returns a rotation and a small arc
   y-offset per card, min/max-clamped so even a full 10-card hand stays on-canvas). Deck/discard/
   energy-zone icons stay in the corners, resized to be legible at the new scale.
3. **Procedural cards show real card info** instead of a blank colored block: `cardArt.ts` gained
   a `DetailTier` (`minimal`/`compact`/`full`, picked from the card's own rendered height) that
   draws a name+HP header, a stage badge and an `EX` marker, a boxed type-colored art band, up to
   two attacks (energy cost pips, name, damage) with the ability name at the largest tier, and a
   weakness/retreat-cost footer — or, for a Trainer, its subtype and a word-wrapped, height-clipped
   effect blurb. Every `Text` gets its own capped-`devicePixelRatio` `resolution` so it stays crisp
   at any of these sizes.

### Round 3 deviations
- **The layout is a careful visual match to the reference screenshot, not a pixel-identical
  copy** — that screenshot is itself a promotional/summary shot (a circular vignette scene with
  Pokémon artwork, a "battle result"-style overlay) rather than the live gameplay HUD, so what was
  actually ported is its *spatial relationships* (tight bench-to-active adjacency, actives
  overlapping the divider, a large fanned bottom hand) rather than its background art, which the
  request explicitly asked to leave out in favor of the dark/minimal theme.
- **The player's hand doesn't literally bleed past the canvas edge.** The request's reference showed
  hand cards cropped by the *photo's* own bottom edge, which reads ambiguously between "intentional
  overflow" and "the screenshot just ends there." The safer reading implemented here: a large hand
  sitting close to the bottom name bar with minimal gap, clamped so every card's outer edge (even a
  fully rotated 10-card fan) stays on-canvas — overflow felt riskier to get right without a live
  reference to compare against, and a clipped card is harder to read, not more dramatic.
- **`PIXI.Text.resolution` is set per-instance, capped at 3x**, rather than left at Pixi's shared
  default — the same reasoning as the Application's own `resolution` cap (Round 1), balancing
  crispness against the texture memory a great many small on-card text objects would otherwise use
  at an uncapped HiDPI resolution.

## Acceptance criteria
1. `cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt -n 3 --players r,r --replay-dir replays/` writes 3 replay files.
2. Opening one in the viewer shows the board; ←/→ step with animated transitions both directions.
3. `--players r,x --bot-b "python3 examples/bots/random_bot.py"` plays games to completion and the
   bot's notes appear in the viewer.
4. `cargo fmt`, `cargo clippy --features tui -- -D warnings`, `cargo test --features "tui test-utils"`
   pass; viewer build/typecheck/tests pass.

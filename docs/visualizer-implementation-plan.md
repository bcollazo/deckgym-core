# Game Visualizer Implementation Plan (Rust Replay Server + PixiJS Front End)

## Summary
Build a browser visualizer that plays back a bot-vs-bot game with real animation, replacing the TUI as the way to *watch* a game (the TUI stays as the way to *inspect* one).

The engine stays where it is. A new `ReplayBundle` projects a finished game into a compact JSON contract; a small feature-gated Rust HTTP server hands the browser **an entire game in one response**; a PixiJS front end diffs consecutive board snapshots into a tweened animation timeline and renders procedurally-drawn cards on WebGL2/WebGPU.

No WebAssembly, no React.

## Prior Art: What Catanatron Does, and What We Change

| | Catanatron | This plan |
|---|---|---|
| Engine | Python | Rust (unchanged, `deckgym-core`) |
| Transport | `GET /api/games/:id/states/:index` per step | One `POST /api/replays` per **game** |
| Front end | React 19 + MUI + SCSS | TypeScript + PixiJS v8, no framework |
| Board | SVG / DOM nodes | WebGL2 (WebGPU-capable) sprites |
| Replay | `stateIndex` in React state, refetch on every step | Whole bundle in memory, scrub is a pointer move |

Catanatron's `ReplayScreen.tsx` refetches `/states/:index` on every prev/next, making scrubbing network-bound. A deckgym game is small once projected (card **ids** plus a board snapshot per ply, rather than two full decks per ply), so shipping the whole game at once removes that round trip entirely — the main benefit of an in-browser engine, without compiling one.

Catanatron's `feature/catanatron-rust` branch is engine-only and contains no UI, so there is nothing to port from it.

## In Scope (Milestone 1)
- `ReplayBundle` format + `replay` CLI producer.
- Feature-gated HTTP server producing the same bundle on demand.
- Deck and bot-strategy pickers, seed input.
- Full playback of a recorded game: animation timeline, transport controls, battle log.
- Procedurally drawn cards (no image assets).

## Out of Scope (Milestone 1)
- Human-playable mode (the TUI's `AppMode::Interactive` equivalent).
- Exposing `simulate` / `optimize` over HTTP.
- Real card art from a CDN — the renderer leaves a seam for it.
- Deck building / editing.
- Any change to engine rules, move generation, or player strategies.

## Data Model

### `ReplayBundle` (`src/replay.rs`)
Already drafted on this branch, not yet declared in `lib.rs`.

- `version: u32` — refuse to render a bundle the front end cannot read.
- `game_id: String`, `seed: u64` — the seed is always reported, so a random game is reproducible.
- `players: Vec<PlayerInfo>` — deck label, strategy code, energy types, deck list.
- `outcome: Option<GameOutcome>` — `None` if the recording hit the ply cap.
- `cards: BTreeMap<String, CardInfo>` — **one shared dictionary**, holding only the cards this game touched (tens, not the ~3,800 in `database.json`).
- `plies: Vec<Ply>` — one entry per decision point.
- `final_state: StateView` — the position the game ended in.

### `Ply`
- `ply`, `actor`, `is_stack`
- `kind: String` — the `SimpleAction` variant name, taken from its serde encoding so it cannot drift out of sync with the enum.
- `label: String` — from `SimpleAction`'s existing `Display`.
- `choices: usize` — `1` means the engine auto-played it; the front end uses this for beat grouping.
- `state: StateView` — the board **before** this action was applied.

### `StateView` / `SideView` / `SlotView`
Card **ids** only; bodies live in the dictionary. Per side: hand, deck count, discard pile, discarded energy, energy zone (`current` / `next`), and four in-play slots (index `0` Active, `1..=3` Bench). Per slot: id, damage, `max_hp` **after** tool/stadium/ability bonuses so `damage/max_hp` is directly the bar to draw, attached energy, tool, status conditions, and `behind` (the pre-evolution stack).

### Rationale
Serializing `State` directly would carry both decks in full plus search-oriented bookkeeping at every ply — tens of megabytes for a game watched in thirty seconds.

## Transport

- `POST /api/replays` `{deck_a, deck_b, players, seed?}` → full `ReplayBundle`
- `GET /api/decks` → example decks, to populate the picker
- `GET /api/players` → bot strategy codes and descriptions
- `GET /*` → static serving of the built front end

The `replay` CLI writes the identical format to a file, so the visualizer also runs fully static with no server.

## Open Decisions

These are recorded as decisions to make, not decisions made.

1. **Server crate.** `tiny_http` is ~5 crates and synchronous, matching a codebase with no async anywhere; it needs a small thread pool so one slow ExpectiMiniMax game does not block the next request. `axum` is ~120 crates and an async runtime, but is what anyone extending this later would expect. Either way it sits behind a `server` feature, like `tui`, so default builds and CI are untouched.
2. **Playback unit: beats vs plies** (see below). This is the decision that most shapes the animation layer.
3. **Front-end location.** Pixi v8 pulls 11 small transitive deps, so this puts a `node_modules` toolchain in a Rust repo. Alternative: the front end lives in a separate repo alongside deckgym.com, with `ReplayBundle` as the contract between them. (Note: `cdn.jsdelivr.net` is blocked by this environment's egress policy, so a runtime-CDN `<script>` variant could not be verified here; npm + Vite can.)
4. **Spectator visibility.** Both hands face-up for bot-vs-bot, presumably — but it is a choice.
5. **Perspective.** Fixed player-0-on-bottom, or a flip control.

## The Animation Layer

The engine emits **states, not events**. `play_tick()` gives a before and an after; nothing says "Venusaur attacked for 80 and knocked out Koffing." Two things make deriving that tractable.

### 1. `kind` is the hint, the diff is the payload
The action says *what the player did*; the diff says *what moved where*. Neither alone suffices — `Attack` does not say who took damage, and a damage delta does not say whether it came from an attack, poison, or a self-inflicted cost. Together they are unambiguous.

The diff surface is finite and is the actual spec of this layer:

| Signal | Visual |
|---|---|
| hand length, deck count | draw / play / discard |
| slot occupancy appears | Place |
| slot id changes, `behind` grows | Evolve |
| slot empties | KO / discard / return-to-hand, disambiguated by which pile grew |
| damage delta | damage number, hit shake, HP bar tween |
| energy array delta | attach; a **move** is a simultaneous −1/+1 of the same type |
| tool, status flags | attach/detach, condition badges |
| `points` delta | prize counter |
| slot 0 ↔ slot *k* identity swap | Retreat / Activate |
| energy zone, stadium | zone rotation, stadium swap |

### 2. Two-level timeline: plies → beats → tweens
A player "turn" is many engine plies — the move-generation stack fires sub-actions, and `choices == 1` plies are auto-played. One-ply-per-step playback would stutter through bookkeeping.

So plies coalesce into **beats** (break when the actor changes, or at a non-stack boundary), and each beat expands into overlapping tweens: anticipation → impact → reaction → settle. The scrub bar indexes beats; a modifier steps raw plies for debugging.

This is a departure from the TUI, which is ply-indexed (`AppMode::Replay` steps `current_index` over `Vec<State>`) — hence open decision 2.

**Known limitation:** if events are pure functions of `t`, then play/pause/speed/seek come free, but scrubbing *backwards* through particle effects does not. Reverse snaps to the beat boundary rather than un-exploding a KO.

### 3. Presentation state is separate from board state
The scene holds its own interpolated state that eases toward the target board; it never re-reads the whole snapshot per frame. Seeking snaps both.

## Rendering

Each distinct card face is drawn **once** — type-colored frame, name, HP bar, energy pips, attack lines — into a cached Pixi `RenderTexture`. Everything after that is a cheap sprite. This is the texture-atlas trick, with Pixi doing the bookkeeping, and it leaves a clean seam for swapping in real art later.

At ~40 visible cards nothing here is draw-call bound. The reason for WebGL is not throughput but **control**: shader filters for hit flash, KO burn-away, holo shimmer and energy glow, plus particles and screen shake, none of which are pleasant in CSS.

## File-Level Implementation Steps

1. `src/replay.rs` — declare `pub mod replay;` in `src/lib.rs`, re-export `ReplayBundle` and `record_game`. *(File drafted; currently uncompiled.)*
2. `src/bin/replay.rs` — CLI writing a bundle to JSON. Mirrors the `--data-output` flag's ergonomics.
3. `Cargo.toml` — add a `server` feature and the chosen HTTP crate as an optional dependency; register the `visualizer` bin with `required-features = ["server"]`, following the existing `tui` pattern.
4. `src/bin/visualizer.rs` — endpoints above, plus static serving and a request thread pool.
5. `visualizer/` — Vite + TypeScript + PixiJS scaffold.
   - `replay/bundle.ts` — load and validate a bundle.
   - `replay/beats.ts` — ply → beat grouping.
   - `replay/diff.ts` — board diff → visual events (the table above).
   - `replay/timeline.ts` — events → tweens; play/pause/speed/seek.
   - `render/cardTexture.ts` — procedural card faces into cached `RenderTexture`s.
   - `render/board.ts` — mat layout, slots, piles, energy zone.
   - `render/effects.ts` — filters, particles, damage numbers, shake.
   - `ui/transport.ts`, `ui/log.ts`, `ui/setup.ts` — controls, battle log, pickers.
6. `README.md` — a "Game Visualizer" section beside the existing TUI section.

## Tests

### Existing (must pass unchanged)
- `cargo test --features "tui test-utils"` — the engine is not being modified.

### New
1. **Rust, `src/replay.rs`** *(drafted)*: a seeded game records plies and an outcome; the same seed replays identically; every referenced card id resolves in the dictionary; `kind` is the variant name for both serde encodings (unit and struct variants); the bundle round-trips through JSON.
2. **Rust, server**: each endpoint returns well-formed JSON; an unknown deck is a 4xx, not a panic; a bundle survives the HTTP round trip.
3. **TypeScript, diff layer**: hand-built before/after board pairs for each row of the diff table, asserting the expected events — including the ones a naive differ gets wrong (energy *move* vs attach+detach, evolve vs place, KO vs return-to-hand).
4. **TypeScript, beats**: a recorded bundle groups into the expected beat boundaries; `choices == 1` plies never start a beat.
5. **Browser smoke test**: headless Chromium (pre-installed) loads a bundle, advances the timeline, and screenshots, to confirm the renderer actually draws.

## Acceptance Criteria
1. `cargo run --bin replay …` writes a bundle that the front end renders with no server.
2. `cargo run --bin visualizer --features server` serves a playable visualizer; picking two decks and two strategies deals a game and plays it back.
3. Default `cargo build` / `cargo test` dependency graph and runtime are unchanged — all new Rust code is feature-gated or inert.
4. Playback covers every row of the diff table with a distinct, legible animation.
5. Transport works: play, pause, speed, step, scrub, and seed replay of the same game.
6. Holds 60fps during the busiest beat of a full game.

## Assumptions
1. The engine gains no animation hooks; coupling a simulator that runs 10,000 games in 3 seconds to presentation concerns is the thing being avoided.
2. `database.json` has no image URLs and the repo ships no card art, so procedural rendering is the default rather than a fallback.
3. `ReplayBundle` is the contract; the front end can be replaced, relocated, or rewritten without touching the engine.
4. Milestone 1 is spectator-only — bots play, the viewer watches.

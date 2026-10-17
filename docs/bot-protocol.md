# External bot protocol

deckgym can hand decisions for one or both players to an external process (any language) instead
of a built-in `Player`. The engine talks to it over the process's stdin/stdout with one JSON object
per line ("JSON lines"). This is what `PlayerCode::X` (`src/players/external_player.rs`) and the
CLI's `--bot-a` / `--bot-b` flags use.

## Running a bot

```bash
cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt \
  -n 1 --players r,x --bot-b "python3 examples/bots/random_bot.py"
```

- `--players` needs an `x` in the slot the bot is playing.
- `--bot-a "<command>"` / `--bot-b "<command>"` give the shell command for that slot (run via
  `sh -c <command>`, so pipes/args work normally).
- `--bot-timeout-ms <ms>` (default 10000) caps how long the engine waits for one reply.

One subprocess is spawned per `ExternalPlayer`, lazily, on its first decision. In the current CLI,
that's one subprocess per game (see "Deviations" below) — it isn't restarted between decisions
within a game, only stdin/stdout stay open the whole game.

## Messages

Every message is a single JSON object on its own line (no embedded newlines). `engine → bot` lines
are written to the bot's stdin; `bot → engine` lines are read from its stdout.

### 1. Handshake (once, right after spawning)

```jsonc
engine → bot   {"type": "hello", "protocol": 1}
bot → engine   {"type": "hello", "name": "MyBot v2"}
```

`name` is optional; if given, it's used for the bot's display name (in logs and in a
`ReplayPlayerInfo`/log-panel sense) instead of the raw command string.

### 2. New game (once, right after the handshake)

```jsonc
engine → bot   {"type": "new_game", "game_id": "…", "you": 1, "deck": ["A1 001", "A1 001", …]}
```

- `you`: which player index (`0` or `1`) the bot is playing.
- `deck`: the bot's own deck as a flat list of card ids (one entry per copy, so a 20-card deck is
  20 entries) — see `database.json` / `cargo run --bin search` for what a card id looks like.

No reply is expected.

### 3. Decide (once per decision point)

```jsonc
engine → bot   {
  "type": "decide",
  "ply": 41,
  "state": { /* a ViewState, see below */ },
  "actions": [
    {"i": 0, "text": "EndTurn", "action": {"EndTurn": null} /* see note below */},
    {"i": 1, "text": "Attack(Giant Bloom)", "action": {"Attack": {"title": "Giant Bloom", …}}},
    …
  ]
}
bot → engine   {"i": 1, "note": "attack wins the race", "scores": [0.05, 0.9, 0.05]}
```

- `ply` is a per-game counter for this bot's own decisions (starts at 1); it does **not**
  necessarily match a replay file's global `ply` numbering.
- `state` is the engine's `ViewState` (`src/replay/view.rs`) **from this bot's own point of view**:
  `ViewState::for_player`, so the opponent's `hand` is `null` (only `hand_count` is visible) while
  everything else — both boards, discard piles, the stadium, deck counts — is fully visible, same
  as what a real player sees across the table.
- `actions[i].action` is the engine's internal `SimpleAction` (`src/actions/types.rs`), serialized
  with serde's default (externally-tagged) representation: a unit variant like `EndTurn` serializes
  to the bare string `"EndTurn"`, not `{"EndTurn": null}` — the example above is illustrative only.
  Most bots only need `actions[i].text` (`SimpleAction`'s `Display`, human-readable) and the index;
  `action` is there for a bot sophisticated enough to reason about the structured action.
- The bot's reply: `i` (required) is the index into `actions` to take. `note` (optional) is a short
  string stored in the replay next to that decision, shown in the viewer's options panel — use it
  for "why" (e.g. "attack wins the race", "saving retreat for next turn").
- `scores` (optional) is an array with exactly one finite number per entry of `actions`, in the same
  order — how much the bot likes each option (higher is better). It is stored in the replay and the
  viewer draws it next to each option: values that are non-negative and sum to 1 are shown as
  percentages (use this for a policy's probabilities), anything else as raw numbers with a bar
  scaled to the range. A `scores` array of the wrong length, or with non-numeric entries, is
  ignored; it never invalidates the move.

Only one decision is in flight at a time — the engine waits for a reply before sending the next
`decide`.

### Game end

The engine does **not** send a `game_over` message (see "Deviations" below). A bot can tell a game
ended because either it stops receiving `decide` messages, or (if the process is reused) it
receives a new `new_game`.

## Failure handling

If the bot's reply is missing, isn't valid JSON, has an `i` outside `0..actions.len()`, doesn't
arrive within `--bot-timeout-ms`, or the process has died, the engine:
1. logs a `warn!` describing what went wrong, and
2. falls back to `actions[0]` (the first legal action) so the simulation keeps going rather than
   panicking, and
3. records a synthetic note like `[bot error: bot timed out]` in the replay for that decision, so
   it's visible after the fact.

The bot's own stderr is passed straight through to the engine's stderr (not captured), so
`print("...", file=sys.stderr)` in a bot is a fine way to debug it live.

## Deviations from the original plan

- **No `game_over` message.** The `Player` trait has no "a game just ended" hook to call it from,
  and adding one just for this would touch every existing `Player` implementation. The plan
  explicitly allowed skipping it ("acceptable to skip `game_over` if the `Player` trait has no end
  hook—document it").
- **One subprocess per game, not truly "kept alive across games."** The CLI constructs a fresh set
  of `Player`s for every game it simulates (`Simulation::run`'s `run_single_simulation`, matching
  every other `Player` implementation, e.g. a fresh `MctsPlayer` per game), so in practice each
  `ExternalPlayer` instance's lifetime is exactly one game, and it spawns exactly one subprocess,
  cleaned up (stdin closed, then killed) when that `Player` is dropped at the end of the game. The
  protocol itself (repeatable `new_game` over one connection) doesn't assume this, so a caller
  embedding `ExternalPlayer` directly and reusing one instance across games would work too — the
  plan's own fallback ("each game gets its own `ExternalPlayer` — acceptable for now") already
  anticipated this.

## Optional rule-complete agent snapshots

A bot can reply to `hello` with `"agent_snapshot": true`. It then receives its
original `Deck` as `new_game.deck_definition` (including configured energy types),
and `decide.agent_snapshot` plus `decide.legal_actions` (complete `Action` values).
The snapshot preserves public rule effects/history and the bot's own hand. Both
deck orders and the opponent's hand identities are replaced with `UNKNOWN`
placeholder cards, preserving only zone lengths; future stacked choices are
redacted. It is a feature/input snapshot, **not a state suitable for simulation**.
Use the supplied legal actions rather than regenerating them from placeholders.
Bots that do not opt in retain the original compact `ViewState` protocol.

On Windows external commands run through `cmd /D /S /C` without a visible console;
other platforms use `sh -c`.

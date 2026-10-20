# First-turn Basic availability table: methods

Lookup table for the DeckGym deck page: for a Basic species in a 20-card deck, the chance that at
least one copy (and, for 2 copies, both) is **in play (Active or Bench) by the end of the player's
own first turn**, going first, going second and on average, next to the opening-hand baseline.

| | |
|---|---|
| Engine | deckgym-core, branch `first-turn-basics`, commit `b11fb701` (main `f239a4e` + the sim example and a borrowed `Game::state()` accessor; no rule changes) |
| Seed | `20261020` (per-game seeds are splitmix64 of seed, cell, policy and game index) |
| Main run | 3,436 cells, 143.8M games, 3,536 s (59 min), ~40k games/s |
| Hardware | AMD Ryzen 9 9950X (16 cores / 32 threads), 32 GB RAM, Windows 11, rustc 1.90, rayon |
| Date | 2026-10-20 |

## Files

| File | What |
|---|---|
| `../examples/first_turn_basics.rs` | Simulation: cell decks, bot, early-stopping game loop, JSONL checkpointing, `--trace`, non-mirror modes |
| `coverage.py` | Pulls real decks from the DeckGym analytics API, reports grid coverage → `coverage.json` (API snapshot in `data/`) |
| `make_plan.py`, `make_side_plans.py` | Simulation plans (`plan*.jsonl`) |
| `analyze.py` | Validation, Tools study, sensitivity, builds `table.json` / `table.csv` / `analysis.json` |
| `results.jsonl` | Main run, one line per cell (raw counts per seat) |
| `side.jsonl`, `sensitivity.jsonl` | Tools isolation, fixed-opponent and policy sensitivity runs |
| `table.json`, `table.csv` | The shipped table |

Reproduce (from deckgym-core root):

```bash
python first-turn-basics/coverage.py          # uses the cached data/ snapshot; --refresh to re-fetch
python first-turn-basics/make_plan.py > first-turn-basics/plan.jsonl
cargo run --release --example first_turn_basics -- --plan first-turn-basics/plan.jsonl --out first-turn-basics/results.jsonl --seed 20261020
python first-turn-basics/make_side_plans.py
cargo run --release --example first_turn_basics -- --plan first-turn-basics/plan_tools.jsonl --out first-turn-basics/side.jsonl --opp-t 0
cargo run --release --example first_turn_basics -- --plan first-turn-basics/plan_opponent.jsonl --out first-turn-basics/side.jsonl --opp-cell 2,4,2,2,1,1
cargo run --release --example first_turn_basics -- --plan first-turn-basics/plan_sensitivity.jsonl --out first-turn-basics/sensitivity.jsonl --policy random   # also greedy, everything
python first-turn-basics/analyze.py
```

The run is resumable: cells already in the output file (same policy, opponent, seed, games) are skipped.

## Engine facts checked

| Claim in the brief | Engine (commit above) | Status |
|---|---|---|
| Opening hand: 1 random Basic forced on top, rest shuffled, draw 5 | `Deck::shuffle(true)` pops one uniformly random Basic card to the top, shuffles the other 19; `draw()` takes from the top | Confirmed |
| First player is a coin flip | `State::initialize` | Confirmed |
| **First player does not draw on turn 1** | **Wrong.** When setup ends, `forecast_end_turn` sets `turn_count = 1` and calls `queue_draw_action(current_player, 1)`. `advance_turn` then queues the draw for turn 2 onwards. Both players draw 1 at the start of their first turn | **Discrepancy** |
| `turn_count` 1 / 2 = first / second player's first turn; `is_users_first_turn()` is `turn_count <= 2` | Yes (`turn_count` 0 is setup) | Confirmed |
| Professor's Research draws 2 | `professor_oak_effect` | Confirmed |
| Copycat shuffles hand into deck and draws the opponent's hand size | `copycat_effect`; the opponent's hand is counted before the shuffle | Confirmed |
| Poké Ball: random Basic from deck to hand | `pokemon_search_outcomes(.., basic_only = true)`, uniform over Basic *cards* in the deck (not species), deck reshuffled | Confirmed |
| Supporter once per turn, Items unlimited | `can_play_support` / `has_played_support`; Supporters allowed on turn 1 for both players | Confirmed |
| Max 2 copies per name | **Not enforced by the engine.** `Deck::is_valid` only checks 20 cards and >= 1 Basic. The cell decks respect the rule by construction | Note |
| Tools | Playing a Tool moves it from hand to an in-play Pokémon (`attach_tool` → `AttachTool`). No Tool draws or changes draw counts. The only route to an effect is hand size, and only through Copycat | See Tools finding |
| Other | Fossils are Trainers in the engine: they don't count as Basics for the opening-hand guarantee or for Poké Ball. Hand limit is 10 (`maybe_draw_card`). Bench is 3 | Note |

## Setup

**Question.** The target species has `c` copies. `m` = other Basic Pokémon cards (distinct species,
<= 2 each). `p` Poké Ball, `r` Professor's Research, `cc` Copycat, `t` Tools. The rest of the 20
cards are inert.

**Cell decks.** Target is Bulbasaur (A1 001). Others: Paras, Tangela, Pinsir, Scyther, Cottonee,
Skiddo (no abilities). Inert slots: 11 distinct Stage 1 Pokémon (they can't be played on either
first turn). Tools: Giant Cape, Rocky Helmet. None of these affects turn-1 draws.

**Grid run.** `c` 1–2, `m` 0–12, `p`/`r`/`cc` 0–2, `t` 0–4, `c+m+p+r+cc+t <= 20`: 3,436 cells,
all at 40,000 games. The 56 cells holding >= 0.5% of real (deck, Basic) pairs got 160,000. Each
mirror game yields one going-first and one going-second sample. 95% Wilson half-width <= 0.49 pp
per seat (<= 0.35 pp for the average).

**Game loop.** `State::initialize` with the cell deck for both players, then `Game::play_tick`
until the second player's first turn ends (`turn_count` 2 → 3). "In play" is read when each
player's first turn ends; "opening" is the 5-card hand before setup.

**Bot (greedy, goal: maximise target copies in play).** Checked in each decision, top rule first:

1. Place a target from hand: Active if the Active Spot is empty (setup), else on the Bench.
2. Setup only: no target in hand → any other Basic as Active.
3. Play a Tool if one is playable (attach anywhere). Never competes with the goal.
4. If a copy is still missing and none is in hand: play Poké Ball.
5. Bench other Basics only while free Bench slots > missing target copies, so a slot always stays
   open for each copy still to find. This also keeps them from being shuffled back by Copycat.
6. If a copy is still missing: one Supporter, whichever draws more right now. Professor's
   Research = 2; Copycat = opponent's current hand. Copycat only if the opponent holds > 2 cards
   (tie → Research). Never Copycat while holding an unbenched target.
7. End turn. The bot never attaches energy, attacks, retreats or evolves.

Because the rules are re-checked after every action, Poké Ball runs again after a Supporter and
anything new gets benched. Both players use the bot (mirror). `--trace N` prints full games;
traced games behave as above.

## Coverage on real decks

Source: DeckGym `meta-share` + `meta-deck-detail` (Limitless tournament data), STANDARD,
MONTHLY window 2026-09-21 → 2026-10-20, top 100 archetypes, every exact variant weighted by deck
count. That is **5,191 decks** (equal to the API's reported total, so nothing is lost to variant
truncation) and **15,680 (deck, Basic species) pairs**. Snapshot in `data/`, full counts in
`coverage.json`.

| Dimension | Real distribution (pairs) |
|---|---|
| c | 1: 6,151 · 2: 9,529 |
| m | 0: 191 · 1: 367 · 2: 2,491 · 3: 4,795 · 4: 4,366 · 5: 2,596 · 6: 769 · 7: 8 · 8: 85 · 9: 10 · 10: 2 |
| p | 0: 1,380 · 1: 912 · 2: 13,388 |
| r | 0: 158 · 1: 358 · 2: 15,164 |
| cc | 0: 1,991 · 1: 4,950 · 2: 8,739 |
| t | 0: 4,356 · 1: 4,907 · 2: 4,151 · 3: 1,442 · 4: 128 · 5: 33 · 6: 510 · 7: 153 |

- With `t` in the grid (0–4): **95.6%** of pairs, 96.2% of decks. All misses are 5–7 Tool decks
  (Dedenne ex Indeedee ex / Dedenne ex Pichu).
- **Shipped table (no `t`): 100.0%** of pairs. Real decks only use 396 distinct cells; `m` never
  exceeds 10.

**Most common other draw/search cards** (share of decks; not in the table):

| Card | Decks | Turn-1 relevance |
|---|---|---|
| Lisia (2 random Basics with <= 50 HP) | 9.3% | High, but HP-dependent: needs the target's HP |
| Clemont (Magneton / Heliolisk / Backpack) | 7.0% | Archetype-specific search, doesn't find Basics in general |
| May (2 random Pokémon, shuffle back as many) | 3.3% | High: random Pokémon, Basics or not |
| Juliana (random Stage 2) | 1.7% | None for Basics |
| Order Pad (coin flip: random Item) | 1.3% | Indirect: can find Poké Ball |

Also common and relevant to Copycat: Mars 11.5% and Red Card 4.2% (both reset the *opponent's*
hand), Hiking Trail 9.5% (end-of-turn refill, too late for turn 1). Pokémon Communication, Great
Ball, Mythical Slab and Dawn are not in this month's top-100 decks at all.

## Validation

**1. Opening hand vs exact enumeration: pass.** For `p=r=cc=t=0`, sim vs exact for the engine
model `P = c/B + (m/B)·(1 − C(19−c,4)/C(19,4))` (B = c+m). I also checked top-6 (opening + first
draw) for `inPlay1` and both-copies-in-top-6 for `inPlay2`. 26 cells × up to 3 metrics, worst
|z| = 2.00. Example, c=2 m=4: opening 0.5895 vs 0.5906, inPlay1 0.6445 vs 0.6452, inPlay2 0.1257
vs 0.1267.

**2. Zeraora post: matches where the decks match; the post's 5–8 Basic decks have 18 cards.**

| Basics | Blog | This table (c=2 p=2 r=2 cc=0, avg) | Blog code re-run on today's engine |
|---|---|---|---|
| 2 | 100 | 100.0 | 100.0 |
| 3 | 93.9 | 93.8 | 93.8 |
| 4 | 88.2 | 88.2 | 88.3 |
| 5 | 87.5 | 83.5 | 87.2 |
| 6 | 83.8 | 79.7 | 84.1 |
| 7 | 80.5 | 76.8 | 80.8 |
| 8 | 77.7 | 74.2 | 77.8 |

The blog's own code still reproduces its numbers, so the engine hasn't moved. The 4 pp gap from
5 Basics on comes from `example_decks/zeraora_basics/{5..8}_basics.txt` (branch
`zeraora-triggering-probability`): they hold **18 cards** (the "Trainer: N" headers overstate the
lists), and `Deck::from_file` doesn't reject them. A 2-card-smaller deck finds Zeraora more often.
**The published post's 5–8 Basic numbers are about 4 pp too high.** The other differences (the
blog measured only the going-first seat of player 0, its bot filled the Bench with other Basics
and then played randomly, and the decks had Tools) don't matter at 2–4 Basics, as the table shows.

**3. Monotonicity: pass.** Every neighbour pair in `p`, `r`, `cc` (6,788 comparisons each) and `m`
(9,466), per seat, for inPlay1 and inPlay2. Zero cases where adding Poké Ball / Research / Copycat
lowers the probability, or adding a Basic raises it, by more than 2 standard errors. The real
effects of one more card are large compared with the noise, so no false alarms show up.

## Tools finding

The hypothesis to test was that attached Tools make Copycat stronger. Main grid, each `t > 0` cell
vs the same cell at `t = 0`, Stouffer-combined z across cells:

| | going first | going second |
|---|---|---|
| `cc = 0` | +0.00 to +0.01 pp, \|z\| < 1 (noise) | +0.01 to +0.02 pp, \|z\| < 1.1 (noise) |
| `cc > 0`, t=1→4 | +0.05 → +0.22 pp (z +3.5 → +14) | **−0.27 → −0.98 pp** (z −18 → −63) |

Two mechanisms, separated by a non-mirror run (our `t` varies, the opponent keeps 0 Tools; the 30
most common real Copycat combinations, 40k samples per seat):

- **Own Tools help a little (the owner is right, but the effect is small).** A Tool played before
  Copycat is no longer in the hand Copycat shuffles back, so the deck you redraw from has one fewer
  dead card. Isolated effect: **+0.02 to +0.29 pp** going first, **+0.12 to +0.26 pp** going
  second (z 2–5).
- **The opponent's Tools hurt your Copycat.** Tools the first player attaches leave their hand,
  so the going-second Copycat draws fewer cards. In a mirror this is tied to *your* `t`, but in a
  real game it depends on the opponent's deck. It's an artifact of the mirror model.

**Decision: `t` dropped.** The own-side effect is real but below the 0.5 pp precision target, and
smaller than the uncertainty from the opponent model (next section). The table ships the **t = 0
cells**, not a pool over `t`, which would carry the mirror artifact into the going-second numbers.
The app looks up every deck ignoring Tools, which also covers the 6–7 Tool Dedenne decks.
`analyze.py --keep-t` builds the 5-D table if needed.

## Sensitivity

**Opponent model.** The 53 most common real cells against one fixed typical opponent (c=2 m=4
p=2 r=2 cc=1 t=1) instead of the mirror: inPlay1 moves −1.05 to +0.89 pp, mostly in `cc > 0`
cells. `cc = 0` cells, which can't depend on the opponent, move up to ±0.5 pp, so that is the noise
floor of this comparison. **Copycat values depend on the opponent's hand by up to ~1 pp.** The
mirror is a reasonable middle.

**Policy.** 10 representative cells, inPlay1 (first / second):

| Cell (c,m,p,r,cc,t) | greedy | play everything | random |
|---|---|---|---|
| 2,4,2,2,0,0 | .795 / .797 | .796 / .791 | .660 / .661 |
| 2,4,2,2,2,0 | .838 / .834 | .826 / .829 | .664 / .673 |
| 2,3,2,2,1,1 | .853 / .856 | .855 / .853 | .710 / .710 |
| 2,6,2,2,2,4 | .785 / .778 | .741 / .725 | .596 / .597 |
| 2,8,1,1,1,0 | .678 / .682 | .632 / .634 | .535 / .528 |
| 1,4,2,2,2,2 | .624 / .623 | .611 / .602 | .442 / .444 |
| 1,6,2,2,0,0 | .497 / .505 | .476 / .480 | .375 / .372 |
| 1,10,2,2,2,0 | .464 / .478 | .371 / .371 | .300 / .298 |
| 1,2,0,2,2,0 | .577 / .587 | .574 / .586 | .511 / .516 |
| 2,0,0,0,0,0 | 1 / 1 | 1 / 1 | 1 / 1 |

Play-everything matches greedy (±1 pp) in Basic-light decks and falls behind as `m` grows (−9 pp
at m=10), because it fills the Bench with other Basics and has no room left for the target. Random
play is 10–19 pp lower. The table assumes a player who is trying to get the target out.

## Opening-hand model: engine vs real game

The engine's model (1 uniformly random Basic + 4 of the other 19) differs from "uniform over
5-card hands with >= 1 Basic". For one target copy and 4 other Basics: 36.8% (engine) vs 31.0%
(conditional uniform). For 1-copy targets the gap peaks at ~6 pp with 2–5 other Basics and
shrinks to ~2 pp at 12. The `opening` section of `analysis.json` lists both for every c/m.

I can't verify which model the real game uses. Pokémon TCG Pocket guarantees at least one Basic in
the opening hand, but it doesn't say how. "Deal one Basic, then 4 random cards" (the engine) is a
plausible, cheap implementation and fits the absence of mulligans in Pocket. But "redraw until the
hand has a Basic" would also give no visible mulligans, and it produces the conditional-uniform
distribution. If the client does the latter, the table overstates opening and inPlay values by up
to ~6 pp, mostly for 1-copy targets. This is the assumption most worth confirming, for example by
logging a few hundred real opening hands of a 1-copy-target deck.

## Assumptions to double-check against the real game

1. **Opening-hand guarantee mechanism** (above). The biggest possible error, up to ~6 pp on
   opening and inPlay values.
2. **The first player draws on turn 1** (engine behaviour; the brief said otherwise). As far as I
   know Pocket does give the first player a turn-1 draw (its first-turn restriction is no energy),
   but please confirm. If not, going-first values fall by about the value of one draw (~4 pp for
   a 1-copy target with 4 other Basics).
3. **Supporters on the first player's turn 1** are legal in the engine. If the real game blocked
   them, going-first Research/Copycat gains would vanish.
4. **Copycat counts the opponent's hand before your shuffle** and draws up to the 10-card limit.
5. **Poké Ball picks uniformly among Basic cards**, not species.
6. **Mirror opponent** for Copycat (above): ~1 pp.
7. **Fossils aren't Basics** in the engine; a deck counting fossils as Basics should leave them
   out of `m`.

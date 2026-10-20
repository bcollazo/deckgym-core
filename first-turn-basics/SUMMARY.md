# First-turn Basic availability: 2-minute summary

**What shipped.** `table.json` (159 KB minified, 702 cells) and `table.csv`. Key: `c` (1–2 copies
of the species), `m` (0–12 other Basic cards), `p` / `r` / `cc` (0–2 Poké Ball / Professor's
Research / Copycat). Per cell, going first / going second / average: `opening`, `inPlay1`
(>= 1 copy in play by the end of your own first turn), `inPlay2` (both, for c = 2), 95% half-width
`ci` (<= 0.49 pp per seat, <= 0.35 pp average). 143.8M simulated games, 59 min on the 9950X.
Details in `METHODS.md`.

**Coverage: 100%** of 15,680 (deck, Basic species) pairs from 5,191 tournament decks (top 100
archetypes, 2026-09-21 → 10-20). With Tools as a key it would be 95.6%: the Dedenne ex decks run
5–7 Tools.

**Tools: dropped.** No effect without Copycat. With Copycat, your own Tools help **+0.1 to
+0.3 pp**: a Tool played before Copycat isn't shuffled back into your deck. So you're right in
direction, but the effect is below the table's precision. The larger −1 pp going second in the
mirror run comes from the *opponent's* Tools shrinking the hand you copy, an artifact of mirror
matches. The table uses the t = 0 cells, so the app can ignore Tools.

**Validation.** Opening hand and top-6 match exact enumeration of the engine model (worst |z| 2.0).
No monotonicity violations. The Zeraora post matches at 2–4 Basics (100 / 93.8 / 88.2 vs 100 /
93.9 / 88.2).

## 5 numbers that surprised me

1. **Zeraora post, 5–8 Basics: about 4 pp too high.** Those deck files have 18 cards, not 20, and
   the engine accepted them. Correct values: 83.5 / 79.7 / 76.8 / 74.2 (post: 87.5 / 83.8 / 80.5 /
   77.7).
2. **2 Copycat beat 2 Professor's Research** on their own: 46.8% vs 45.2% (1 copy, 4 other
   Basics). That holds even going first, because the opponent still has ~3–4 cards after setup.
3. **Poké Ball is the best single card**: 2 Poké Ball take that 1-copy target from 40.9% to 51.5%
   (+10.6 pp), against +4.3 pp for 2 Research.
4. **Typical 1-copy target, typical deck** (4 other Basics, 2/2/2): 36.8% in the opening hand,
   **62.5%** by the end of turn 1. Getting *both* copies of a 2-copy target out is rare, 31.9%,
   against 83.6% for at least one.
5. **Going first vs going second differs by at most 2 pp** anywhere in the grid. In the engine
   both players draw on turn 1, and only Copycat cares about the seat.

## Double-check against the real game

1. **How the "at least one Basic" opening is dealt.** The engine deals 1 random Basic + 4 random
   cards. If the real client redraws until a Basic appears, opening and turn-1 values for 1-copy
   targets are up to ~6 pp too high. This matters most.
2. **The first player draws on turn 1** in the engine (the brief assumed not). If the real game
   skips it, going-first values drop ~4 pp.
3. **Supporters on turn 1 going first** are allowed in the engine.
4. **Copycat depends on the opponent's hand.** The table assumes a mirror; a fixed typical
   opponent moves Copycat cells by up to ~1 pp.
5. **Bot plays to get the target out**, keeping a Bench slot for each missing copy.
   "Play everything" loses up to 9 pp in Basic-heavy decks.

**Next table candidates** (top other draw/search cards, share of decks): Lisia 9.3% (Basics with
<= 50 HP, so it needs HP), Clemont 7.0% (archetype-specific), May 3.3% (random Pokémon), Juliana
1.7%, Order Pad 1.3%. Mars (11.5%) and Red Card (4.2%) change the opponent's hand and therefore
Copycat.

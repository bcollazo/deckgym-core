# Pokémon TCG Pocket — Complete Rules Reference for LLMs

## Purpose

This document is a self-contained rules reference for reasoning about Pokémon TCG Pocket battles. It is intended for LLMs, agents, simulators, deck-analysis tools, and strategy-writing systems that need a compact but complete description of how the game works.

This document focuses on battle rules and game-state reasoning. It does not catalog every card, monetization system, pack-opening system, trading rule, event reward, or ranked ladder detail.

---

## Quick Context Paragraph

Pokémon TCG Pocket is a fast digital Pokémon card game using 20-card decks, no Energy cards in the deck, and an Energy Zone that generates one Energy per turn. Players have one Active Pokémon and up to three Benched Pokémon. The goal is to score 3 points by Knocking Out opposing Pokémon. Regular Pokémon are worth 1 point; Pokémon ex are worth 2 points; Mega Pokémon are worth 3 points. Weakness is +20 damage (not ×2), Resistance is absent, and there are no Prize cards. The first player does not draw or attach Energy on their first turn but can generally play Supporters. Pocket differs from traditional Pokémon TCG, which uses 60-card decks, Energy cards in the deck, 6 Prize cards, a 5-Pokémon Bench, 7-card opening hands, ×2 Weakness, possible Resistance, and deck-out loss.

---

## High-Level Summary

Pokémon TCG Pocket is a simplified, faster digital adaptation of the Pokémon Trading Card Game. Two players use 20-card decks made of Pokémon and Trainer cards. Energy cards are not in the deck; instead, an Energy Zone generates one Energy for the active player each turn.

The main objective is to score **3 points** by Knocking Out opponent Pokémon. Most Pokémon award 1 point; Pokémon ex award 2 points; Mega Pokémon award 3 points.

Several tabletop rules are compressed or removed: smaller decks, smaller opening hands, smaller Benches, no Prize cards, no Energy cards in the deck, no Resistance, fixed +20 Weakness, and no loss for failing to draw from an empty deck. A game that has not ended after 30 turns is declared a tie.

---

## Core Concepts

### Players

A battle is played by two players. Each player has:

- A 20-card deck.
- A hand (maximum 10 cards).
- One Active Pokémon slot and up to three Bench slots.
- A discard pile.
- A discard energy pile (discarded Energy tracked separately from cards).
- A point total.
- An Energy Zone with a `current` slot and a `next` (preview) slot.

### Pokémon

Pokémon cards represent creatures that battle. A Pokémon can be:

- **Basic Pokémon** (Stage 0): Can be played directly from hand to an empty Active or Bench slot.
- **Stage 1 Pokémon**: Evolves from a specified Basic Pokémon.
- **Stage 2 Pokémon**: Evolves from a specified Stage 1 Pokémon.
- **Pokémon ex**: Stronger variant, worth 2 points when Knocked Out.
- **Mega Pokémon**: Even stronger variant, worth 3 points when Knocked Out.

Each Pokémon card has: HP, Type, one or more attacks, optionally an Ability, Retreat Cost, optionally a Weakness, and Evolution stage.

### Trainer Cards

Trainer cards are non-Pokémon cards. Categories used in Pocket:

- **Supporter**: Powerful effect; limited to one per turn. Generally can be played by both players, including the first player on their first turn (unlike traditional Pokémon TCG).
- **Item**: Can typically be played freely multiple times per turn (unless an effect prevents it).
- **Tool**: Attaches to a Pokémon and provides a persistent effect. One Tool per Pokémon.
- **Fossil**: A special Trainer card that can be placed as a Pokémon in the Active or Bench slot with 40 HP. Fossils cannot retreat and can be discarded voluntarily at any point during the owning player's turn.
- **Stadium**: A global modifier affecting both players. Only one Stadium can be in play at a time; playing a new Stadium discards the current one.

Card text overrides general rules when a specific effect differs.

---

## Deck Construction

A legal Pokémon TCG Pocket deck:

- Contains exactly **20 cards**.
- Contains **at least one Basic Pokémon**.
- Contains **no more than 2 copies** of any card with the same name.
- Contains Pokémon and Trainer cards only — **no Energy cards**.
- Has an associated Energy Zone configuration (up to 3 declared Energy types) that determines what Energy the Zone generates. If the deck's Pokémon all share one type, the Zone generates only that type. Multi-type decks generate each declared type randomly.

**Deck ratios note:** Because decks are 20 cards and the duplicate limit is 2, a single copy is 5% of the deck and a two-of line is 10%. Card count analysis differs substantially from the 60-card traditional game.

---

## Battle Setup

1. Each player uses a legal 20-card deck. Decks are shuffled.
2. Each player draws **5 cards** as their opening hand. If a player has no Basic Pokémon in their hand, the game performs a mulligan (in the digital implementation, setup is automated to guarantee at least one Basic Pokémon in the starting hand).
3. Each player places a Basic Pokémon (or Fossil Trainer) in the Active slot.
4. Each player may place additional Basic Pokémon on the Bench (up to 3 Bench slots).
5. Turn order is determined randomly.
6. The Energy Zone is pre-populated: both players start with `current = None` and `next = <rolled energy>`. The player going second rotates their Energy Zone on turn 2 (their first turn); the player going first rotates on turn 3 (their second turn).

---

## Turn Structure

Turn order increments with a global `turn_count` (starting at 1 once setup ends). On turn 1, the randomly selected first player acts; on turn 2, the other player acts; and so on, alternating.

A player's turn proceeds in this order:

1. **Start of turn** — start-of-turn Ability effects trigger (e.g., drawing a Pokémon to hand automatically).
2. **Draw step** — the active player draws 1 card (except the first player on turn 1, who does not draw).
3. **Energy Zone rotation** — the player's Energy Zone advances: `next` becomes `current`, and a new `next` is rolled from the deck's declared Energy types. (Exception: the first player on turn 1 gets no energy; their `current` stays `None`.)
4. **Main phase** — the player may freely perform any of the following actions in any order (subject to restrictions):
   - Play Basic Pokémon from hand to an empty Bench slot (or empty Active slot if Active is vacant).
   - Evolve Pokémon (subject to evolution timing rules).
   - Play Trainer cards (Supporters limited to one per turn; Items unlimited unless blocked; Tools attach to a Pokémon).
   - Use an Ability (subject to once-per-turn or conditional restrictions).
   - Attach one Energy from the Energy Zone to one Pokémon (Active or Bench). Consuming the turn's energy clears `current`.
   - Retreat the Active Pokémon once per turn (by discarding attached Energy equal to Retreat Cost and switching to a Benched Pokémon).
   - Use an eligible Stadium ability (for Stadiums with activated effects).
   - Discard a Fossil from the Active or Bench slot voluntarily.
5. **Attack** (optional, usually ending the turn) — the Active Pokémon attacks if the player has enough Energy attached and no effect blocks it. Attacking ends the main phase.
6. **Pokémon Checkup** (between turns) — after an attack and before the next player's turn, status conditions are resolved in this order: Poisoned → Burned → Asleep → Paralyzed (see Status Conditions).
7. **End-of-turn effects** — turn-scoped effects (e.g., damage modifiers lasting "until end of your next turn") expire.

---

## First-Turn Rules

| Rule | Player going first (turn 1) | Player going second (turn 2) |
|---|---|---|
| Draw a card | No | Yes |
| Energy Zone rotation | No (`current` stays `None`) | Yes (first energy available) |
| Attach Energy from Zone | No | Yes |
| Play Supporter | Yes | Yes |
| Play Items | Yes | Yes |
| Evolve Pokémon | No | No |
| Attack | No (no energy to pay; first-turn attack restriction) | Generally yes, if energy requirements are met |

**Key reasoning note:** "Going first" means acting on turn 1 of the global turn count. The player going first cannot attach energy on their first turn, making their first real energy available on turn 3 (their second turn). The player going second gets energy starting on turn 2 (their first turn), giving them a one-turn energy advantage.

---

## Evolution Rules

- A Pokémon generally **cannot evolve on turn 1 or turn 2** (the first two global turns).
- A Pokémon **cannot evolve on the same turn it was played** (i.e., `played_this_turn = true` prevents evolution).
- A Pokémon can only evolve **once per turn**.
- Evolving a Pokémon replaces its card with the evolved card, **preserving damage counters, attached Energy, and attached Tools**.
- The evolved Pokémon loses its `played_this_turn` flag and status conditions that would be cleared by leaving the Active spot are generally not cleared by evolution alone.
- Special Abilities can override these restrictions (e.g., Eevee's Boosted Evolution ability allows evolution on the first turn or the turn it was played if it is in the Active spot).

---

## Energy Zone

The Energy Zone is the defining mechanical difference from traditional Pokémon TCG.

### How It Works

- **No Energy cards in the deck.** Energy is generated externally by the Energy Zone.
- Each player's Energy Zone has two slots: `current` (attachable this turn) and `next` (visible preview for next turn).
- At the start of each player's turn (after turn 1 of the player going first), the Energy Zone rotates: `next` becomes `current`, and a fresh `next` is randomly rolled from the deck's declared Energy types.
- The player may attach the `current` energy to **any one of their Pokémon** (Active or Bench) during their main phase. Attaching consumes `current`, setting it to `None` until the next rotation.
- If a deck uses only **one Energy type**, the Zone always produces that type — no variance.
- If a deck uses **two or three Energy types**, the Zone rolls randomly each turn, creating sequencing variance.

### Energy Type List

The valid Energy types in Pocket are: Grass, Fire, Water, Lightning, Psychic, Fighting, Darkness, Metal, Dragon, Colorless.

**Colorless Energy** is a wildcard: any Energy type satisfies a Colorless cost requirement.

### Energy Acceleration

Some cards can attach additional Energy beyond the one-per-turn Zone attachment. Examples include:
- Supporter cards like **Misty** (attach multiple Water Energy via coin flip).
- Pokémon attacks or Abilities that move Energy from the discard pile or from Benched Pokémon (e.g., Magnezone's Magnetic Circuit, Gardevoir converting Zone energy to Psychic).
- Cards that reduce attack cost (counted as a form of acceleration).

These effects are distinct from the Zone attachment and do not consume the turn's Energy Zone charge unless the card text says so.

### LLM Reasoning Notes for Energy

- Do not model Energy cards as deck slots.
- Do not assume the player must draw Energy.
- Do not count Energy cards when evaluating deck ratios.
- Do consider whether attack costs require specific Energy types.
- Do consider whether a multi-type Energy Zone can produce the needed type on the relevant turn.
- Do distinguish between Zone energy and Energy from card effects (acceleration).

---

## Attacks and Damage

### Declaring an Attack

A player may attack with their Active Pokémon if:
- The Active Pokémon has an available attack.
- Sufficient Energy of the correct types (or Colorless wildcards) is attached.
- No effect prevents attacking (e.g., Paralysis, Confusion tails flip, `CannotAttack` card effect).
- The player chooses to end the main phase by attacking.

Attacking ends the turn (after checkup resolution).

### Damage Resolution Order

1. Start with the attack's printed base damage.
2. Apply attack text effects (coin flips, conditional bonuses, etc.).
3. Apply relevant damage modifiers from turn effects (e.g., Giovanni +10, Oricorio type bonus).
4. Apply Weakness if the defending Pokémon is weak to the attacker's type (+20 damage flat).
5. Apply damage reductions (e.g., Rocky Helmet reduces incoming damage, Heavy Helmet reduces damage from low-retreat-cost Pokémon, Metal Core Barrier reduces by 50).
6. Apply Special Condition effects that block or modify the attack.
7. Place damage counters on the target(s).
8. Resolve counterattack effects (e.g., Rocky Helmet dealing damage back to the attacker).
9. Check for Knock Outs and award points.
10. Trigger on-knockout Ability effects.
11. Queue promotion decisions if an Active slot becomes empty.

### Weakness

Pocket Weakness is **+20 damage** flat, not ×2. For example, an attack dealing 50 damage against a Pokémon weak to that type deals 70 total.

**Exception:** The Bounded Field Stadium reverses Weakness to ×2 instead of +20.

### Resistance

Pocket **does not use Resistance** as a standard stat. No Resistance reduction is applied unless a specific card or future rule explicitly introduces it.

### Bench Damage

By default, attacks only hit the Active Pokémon. Many attacks have explicit effects that also damage Benched Pokémon. Bench damage does **not** get modified by Weakness, damage reductions from tools, or most damage modifiers (unless card text specifies otherwise).

---

## Knock Outs and Points

A Pokémon is Knocked Out when its accumulated damage equals or exceeds its HP.

When a Pokémon is Knocked Out:
1. The Knocked Out Pokémon, all cards beneath it (evolution chain), and its attached Tool go to the discard pile.
2. Attached Energy goes to the energy discard pile (tracked separately).
3. The **opponent scores points** based on the Knocked Out Pokémon's type:
   - **Regular Pokémon**: 1 point.
   - **Pokémon ex**: 2 points.
   - **Mega Pokémon**: 3 points.
4. If the Active slot is now empty, the owning player must choose a Benched Pokémon to promote to Active. If there is no Bench, that player loses immediately (regardless of points).
5. If a player reaches **3 points**, they win immediately.

**Simultaneous knock-out edge cases:**
- If both players reach 3 or more points simultaneously, the game is a tie.
- If both players have no Pokémon remaining simultaneously, the game is a tie.

### Win Conditions

A player wins by:
1. Accumulating **3 points** (most common).
2. The opponent having **no Pokémon remaining** in play after a Knock Out (immediate loss even if points < 3).

### Tie Conditions

A tie occurs if:
- Both players reach 3 points at the same moment.
- Both players simultaneously have no Pokémon remaining.
- The global turn count exceeds **30 turns** without a winner (game timeout).

### No Deck-Out Loss

Unlike traditional Pokémon TCG, **a player does not lose for failing to draw from an empty deck**. Drawing from an empty deck is a no-op. Deck depletion is not a meaningful win condition or control strategy in Pocket.

---

## Status Conditions

Pocket uses five status conditions: **Poisoned, Burned, Asleep, Paralyzed, Confused**.

Stacking rules:
- **Poisoned** and **Burned** can coexist on the same Pokémon.
- **Asleep, Paralyzed, and Confused** are mutually exclusive; applying one replaces the others.

Status conditions are generally resolved at Pokémon Checkup (between turns), in this order: **Poisoned → Burned → Asleep → Paralyzed**. Confused is resolved when the Pokémon attempts to attack.

### Poisoned

- During Pokémon Checkup, a Poisoned Pokémon takes **10 damage** (base).
- Certain Abilities (e.g., Nihilego's More Poison) increase poison damage by +10 per Nihilego in play, but only for the opponent's Active Pokémon.
- Poison damage does not apply Weakness or other attack modifiers.
- Cured by: retreating to Bench, evolving, or card effects (e.g., Lum Berry Tool).

### Burned

- During Pokémon Checkup, a Burned Pokémon takes **20 damage**.
- After taking damage, a coin is flipped: heads = the Burn condition is removed; tails = Burn persists into the next turn.
- Burn damage does not apply Weakness or other attack modifiers.
- Cured by: retreating to Bench, evolving, coin flip (heads at checkup), or card effects.

### Asleep

- An Asleep Pokémon **cannot attack or retreat**.
- During Pokémon Checkup, a coin is flipped: heads = the Pokémon wakes up; tails = remains Asleep.
- Cured by: coin flip (heads at checkup), retreating to Bench (if an effect allows it), evolving, or card effects.

### Paralyzed

- A Paralyzed Pokémon **cannot attack or retreat** for one full turn cycle.
- Paralysis is **automatically removed** at the end of Pokémon Checkup (i.e., it lasts exactly one opponent turn cycle).
- Cured by: Pokémon Checkup (automatic), evolving, or card effects.

### Confused

- When a Confused Pokémon **attempts to attack**, a coin is flipped: heads = the attack proceeds normally; tails = the attack fails and the turn ends (no damage dealt, no effect).
- Cured by: retreating to Bench, evolving, or card effects.

### Removing Status Conditions

The following actions remove **all** status conditions (and most card effects tied to the Active spot):
- Retreating the Pokémon to the Bench.
- Evolving the Pokémon.
- Specific card effects (e.g., Lum Berry Tool, Full Heal Item).

---

## Retreat

- A player may retreat their Active Pokémon **once per turn**.
- To retreat, discard Energy attached to the Active Pokémon equal to its Retreat Cost, then choose a Benched Pokémon to become the new Active.
- **Fossils cannot retreat.**
- Effects can prevent retreat (e.g., `NoRetreat` card effect).
- Retreating removes all status conditions and most `CardEffect` modifiers from the retreated Pokémon.
- Switching via card effects (Trainer cards like Switch, Sabrina, etc.) is **not** treated as retreating and does not consume the once-per-turn retreat action, unless the card text says otherwise.
- Some Abilities and Tools reduce Retreat Cost (e.g., Shaymin's Sky Support reduces Active Basic Pokémon cost by 1; Air Balloon / Inflatable Boat / Big Air Balloon can reduce or eliminate Retreat Cost for specific Pokémon).

---

## Abilities

- Abilities are special effects printed on Pokémon cards. They are not attacks.
- Some Abilities are **passive** (always active while the Pokémon is in the required zone): e.g., Serperior's Jungle Totem (doubles Grass Energy while Serperior is in play).
- Some Abilities are **activated** (the player chooses when to use them): e.g., Weezing's Noxious Fumes (once per turn, poisons the opponent's Active).
- Activated Abilities are generally once per turn per Pokémon (`ability_used` flag resets at end of turn).
- Some Abilities trigger **at the start of the opponent's turn** or **on energy attachment** or **on Knock Out**.
- A Pokémon that just moved to the Active spot this turn (`moved_to_active_this_turn`) may be restricted from using certain Abilities immediately.

---

## Stadiums

- Only **one Stadium** can be active at a time. Playing a new Stadium discards the previous one.
- Stadiums affect **both players** unless the card text specifies otherwise.
- Some Stadiums are passive (always-on effects): e.g., Starting Plains (+20 HP to Basic Pokémon), Snowy Terrain / Sand Slammer (checkup damage).
- Some Stadiums are **activated** (each player may use them once per turn): e.g., Mesagoza, Fragrant Forest, Area Zero.
- Each player may use an activated Stadium's effect once per turn (`has_used_stadium` flag, per player).

---

## Fossils

- Fossil Trainer cards can be placed in the Active or Bench slot as if they were Basic Pokémon.
- A Fossil placed in play has **40 HP**.
- Fossils **cannot retreat**.
- Fossils can be **voluntarily discarded** by the owning player at any point during their turn.
- Certain Pokémon evolve from Fossils (e.g., Kabuto evolves from Old Amber Fossil).

---

## Special Pokémon Categories

Beyond Pokémon ex and Mega Pokémon, the game includes Pokémon with special type classifications that some cards or Abilities reference:

- **Ancient Pokémon** (e.g., Koraidon ex, Roaring Moon, Great Tusk): interacts with cards like Ancient Booster Energy Capsule.
- **Future Pokémon** (e.g., Miraidon ex, Iron Hands, Iron Valiant): interacts with Future System ability effects (cost reduction).
- **Ultra Beasts** (e.g., Nihilego, Buzzwole ex, Naganadel): interacts with Beast-specific cards and Abilities.

---

## Important Game-State Variables for LLMs

When reasoning about a Pocket battle, track at minimum:

| Variable | Notes |
|---|---|
| `turn_count` | Global turn number; 1 = first player's first turn |
| `current_player` | Which player (0 or 1) is acting |
| `points[0]`, `points[1]` | Points scored; first to 3 wins |
| `hands[0]`, `hands[1]` | Cards in each player's hand (max 10) |
| `decks[0]`, `decks[1]` | Remaining deck cards |
| `discard_piles[0]`, `discard_piles[1]` | Discarded cards |
| `discard_energies[0]`, `discard_energies[1]` | Discarded Energy (separate from card discard) |
| `in_play_pokemon[p][0]` | Active Pokémon for player p |
| `in_play_pokemon[p][1..3]` | Bench Pokémon for player p (up to 3) |
| `damage_counters` on each Pokémon | Accumulated damage |
| `attached_energy` on each Pokémon | Energy available for attacks and retreat |
| `attached_tool` on each Pokémon | At most one Tool per Pokémon |
| `energy_zone[p].current` | Energy attachable this turn (None if used or first turn) |
| `energy_zone[p].next` | Preview of next turn's energy |
| `has_played_support` | Whether current player used a Supporter this turn |
| `has_retreated` | Whether current player retreated this turn |
| `ability_used` on each Pokémon | Whether activated Ability was used this turn |
| `played_this_turn` on each Pokémon | Prevents evolution |
| `moved_to_active_this_turn` | Some Abilities check this |
| Status conditions on each Pokémon | Poisoned, Burned, Asleep, Paralyzed, Confused |
| Active card effects on each Pokémon | CardEffect list; cleared on retreat |
| Active turn effects | TurnEffect list; scoped to specific turn(s) |
| `active_stadium` | The current Stadium card in play |
| `winner` | None until game over |

---

## Common Reasoning Pitfalls

1. **Do not use traditional 60-card deck assumptions.** Deck sizes, ratios, and probabilities all differ.
2. **Do not add Energy cards to Pocket decklists.** Energy comes from the Energy Zone.
3. **Do not assume 6 Prize cards exist.** Pocket uses a 3-point system.
4. **Do not assume the Bench holds 5 Pokémon.** Pocket Bench limit is 3.
5. **Do not double damage for Weakness.** Pocket Weakness is +20 flat (×2 only with Bounded Field Stadium).
6. **Do not apply Resistance.** Pocket generally has no Resistance stat.
7. **Do not assume the first player draws on their first turn.** They do not.
8. **Do not assume the first player attaches Energy on their first turn.** They cannot.
9. **Do not assume deck-out is a loss condition.** It is not. Drawing from an empty deck is a no-op.
10. **Do not assume Supporters are banned on the first player's first turn.** They can generally play Supporters on turn 1.
11. **Do not treat switching effects as retreating.** Using a Switch or similar Trainer is not a retreat and does not consume the once-per-turn retreat.
12. **Do not ignore the 10-card hand limit.** Drawing beyond 10 is a no-op.
13. **Do not assume Mega Pokémon behave like Pokémon ex for point value.** Mega Pokémon are worth 3 points.
14. **Do not assume Pokémon ex gives up only 1 point.** Knocking Out a Pokémon ex gives the opponent 2 points — nearly the entire winning condition.
15. **Do not assume Fossils can retreat.** They cannot.
16. **Do not assume all bench-targeting effects apply Weakness.** Bench damage generally does not get Weakness applied.
17. **Do not assume the game continues past 30 turns.** It is declared a tie after turn 30.

---

## Differences from Traditional Pokémon TCG

| Topic | Pokémon TCG Pocket | Traditional Pokémon TCG |
|---|---|---|
| Deck size | 20 cards | 60 cards |
| Energy source | Energy Zone (generated each turn) | Energy cards in the deck |
| Energy in deck | No Energy card slots | Energy cards consume deck slots |
| Duplicate limit | 2 copies per card name | 4 copies per card name (basic Energy is unlimited) |
| Opening hand | 5 cards | 7 cards |
| Hand limit | 10 cards | No hand-size limit in standard play |
| Prize/Point system | No Prize cards; 3-point score system | 6 Prize cards set aside at game start |
| Win condition | First to 3 points, or opponent has no Pokémon | Take all 6 Prizes, deck-out opponent, or opponent has no Pokémon |
| KO reward | 1 point (regular), 2 points (ex), 3 points (Mega) | Draw Prize cards; certain Pokémon give multiple Prizes |
| Bench size | Up to 3 Benched Pokémon | Up to 5 Benched Pokémon |
| Weakness | +20 damage (flat) | ×2 damage (modern tabletop rules) |
| Resistance | Generally absent | Often −30 damage when present |
| First player draw | No draw on turn 1 | First player draws on turn 1 |
| First player Supporter | Can play Supporter on turn 1 | Cannot play Supporter on turn 1 (traditional rule) |
| First player Energy | Cannot attach Energy from Zone on turn 1 | Can attach Energy from hand on turn 1 |
| Deck-out loss | No — drawing from empty deck is a no-op | Yes — player loses if they cannot draw at start of turn |
| Game timeout | Tie after 30 turns | No formal timeout in standard play |
| Fossil mechanic | Fossil Trainer cards placed as Pokémon with 40 HP | Fossil mechanic also exists but details differ |
| Trainer types | Supporter, Item, Tool, Fossil, Stadium | Item, Supporter, Stadium, Tool, Pokémon Tool |
| Match length | Short digital battles (~5–10 minutes) | Longer tabletop battles |
| Energy draw variance | None (Zone is deterministic for mono-type) | High variance (Energy cards mixed in deck) |

---

## Strategic Implications

### Fast Point Pressure

Because the game ends at 3 points, each Knock Out represents a large fraction of the game. Knocking Out a Pokémon ex while keeping your own Pokémon alive can put you 2/3 of the way to winning in a single exchange.

### Pokémon ex Are Powerful but Risky

Pokémon ex often have superior HP, attacks, or Abilities, but giving up 2 points is a decisive liability in a 3-point game. Losing one Pokémon ex and one regular Pokémon usually means losing the game.

### Mega Pokémon Are Even Riskier

Mega Pokémon award 3 points — the entire winning condition — when Knocked Out. Playing a Mega Pokémon means the opponent only needs to KO it to win outright.

### Bench Space Is Scarce

With only 3 Bench slots, setup is more constrained than in the traditional game. A player cannot freely develop many backup attackers or support Pokémon. Filling the Bench requires careful prioritization.

### Energy Consistency

Mono-type decks always get the Energy type they need from the Zone. Multi-type decks gain flexibility at the cost of sequencing reliability — the needed type may not appear at the critical turn.

### Deck-Out Is Not a Strategy

Since drawing from an empty deck is not a loss condition, pure stall or mill strategies are much weaker than in games where deck-out is fatal.

### Energy Acceleration Is Impactful

Because the Zone provides only one Energy per turn, any card that provides additional Energy attachment (Misty, Moltres ex, Magnezone, Gardevoir, Erika, etc.) can dramatically accelerate the game plan and enable attacks turns earlier than the opponent can respond.

---

## Update Policy

This document should be reviewed whenever Pokémon TCG Pocket receives:

- A major rules update.
- Introduction of a new special Pokémon category (new point values, new rule-box types).
- Changes to Energy Zone behavior.
- Changes to Bench size, hand size, or win/loss conditions.
- Introduction of Resistance or other normally-absent mechanics.

For card-specific reasoning, always consult the current in-game card text. Card text overrides all general rules described in this document.

// Pure diffing of two ViewState snapshots into semantic changes. See
// docs/replay-viewer-plan.md's "Transition engine" section. Kept dependency-free (no Pixi/GSAP
// imports) so it's cheaply unit-testable.

import type { EnergyType, EnergyZone, GameOutcome, PlayerView, StatusCondition, ViewState } from "../types/replay";

export type PlayerIndex = 0 | 1;
export type SlotIndex = 0 | 1 | 2 | 3;
/** `"<player>-<slot>"`, e.g. `"0-0"` is player 0's active spot. */
export type SlotKey = `${PlayerIndex}-${SlotIndex}`;

export function slotKey(player: number, slot: number): SlotKey {
  return `${player}-${slot}` as SlotKey;
}

export type SemanticChange =
  | { type: "turnChange"; from: number; to: number; fromPlayer: number; toPlayer: number }
  | { type: "pointsChange"; player: PlayerIndex; from: number; to: number }
  | { type: "stadiumChange"; from: string | null; to: string | null }
  | { type: "winnerDecided"; winner: GameOutcome }
  | { type: "cardEntered"; slot: SlotKey; card: string }
  | { type: "cardLeft"; slot: SlotKey; card: string }
  | { type: "knockOut"; slot: SlotKey; card: string }
  | { type: "evolution"; slot: SlotKey; from: string; to: string }
  | { type: "hpChange"; slot: SlotKey; from: number; to: number; maxHp: number }
  | { type: "energyChange"; slot: SlotKey; added: EnergyType[]; removed: EnergyType[] }
  | { type: "toolsChange"; slot: SlotKey; added: string[]; removed: string[] }
  | { type: "statusChange"; slot: SlotKey; added: StatusCondition[]; removed: StatusCondition[] }
  | {
      type: "activeSwitch";
      player: PlayerIndex;
      benchSlot: SlotKey;
      activeFrom: string | null;
      activeTo: string | null;
    }
  | { type: "handCountChange"; player: PlayerIndex; from: number; to: number }
  | { type: "deckCountChange"; player: PlayerIndex; from: number; to: number }
  | { type: "discardCountChange"; player: PlayerIndex; from: number; to: number }
  | { type: "energyZoneChange"; player: PlayerIndex; from: EnergyZone; to: EnergyZone };

/** Diffs a multiset (order doesn't matter, duplicates do) into what was added/removed. */
function diffMultiset<T>(prev: T[], next: T[]): { added: T[]; removed: T[] } {
  const removed = [...prev];
  const added: T[] = [];
  for (const item of next) {
    const idx = removed.indexOf(item);
    if (idx >= 0) {
      removed.splice(idx, 1);
    } else {
      added.push(item);
    }
  }
  return { added, removed };
}

function sameEnergyZone(a: EnergyZone, b: EnergyZone): boolean {
  return a.current === b.current && a.next === b.next;
}

/** Detects a whole Pokemon moving between the active slot and a bench slot — a retreat, a
 * "switch" effect, or a promotion after a knock-out (in which case `prevActive` is absent: there's
 * no old active to place on the bench, just a bench Pokemon taking the empty active spot).
 *
 * Root cause this fixes: the per-slot loop below diffs slot *index* 0 against slot index 0
 * (old active's card vs. new active's card), and the bench slot the same way — so a swap, which
 * changes *which Pokemon* sits in each slot without changing either Pokemon's own HP, energy,
 * tools or status, used to be misread as an "evolution" (slot 0's card id changed) plus bogus
 * `hpChange`/`energyChange`/`toolsChange`/`statusChange` entries (e.g. "HP dropped from 70 to 60")
 * that were really just the two different Pokemon's own values. Detecting the move up front and
 * excluding both slots from the per-slot diff (see `swappedSlots` below) fixes it at the source:
 * the only change emitted for these two slots is `activeSwitch`, whose handler in
 * `anim/buildTimeline.ts` moves each composite (card + HP + energy + tools + status) to its new
 * slot as a unit and re-`update()`s it from the target snapshot, never tweening an HP bar. */
function detectActiveBenchMove(
  prevPlayer: PlayerView,
  nextPlayer: PlayerView,
): { benchSlot: SlotIndex; activeFrom: string | null; activeTo: string } | null {
  const prevActive = prevPlayer.in_play[0];
  const nextActive = nextPlayer.in_play[0];
  if (!nextActive) return null; // active is empty (or stayed empty) — nothing moved into it
  if (prevActive && prevActive.card === nextActive.card) return null; // same Pokemon, no move

  for (const slot of [1, 2, 3] as SlotIndex[]) {
    const prevBench = prevPlayer.in_play[slot];
    if (prevBench?.card === nextActive.card) {
      return { benchSlot: slot, activeFrom: prevActive?.card ?? null, activeTo: nextActive.card };
    }
  }
  return null;
}

export function diffViewStates(prev: ViewState, next: ViewState): SemanticChange[] {
  const changes: SemanticChange[] = [];

  if (prev.turn !== next.turn) {
    changes.push({
      type: "turnChange",
      from: prev.turn,
      to: next.turn,
      fromPlayer: prev.current_player,
      toPlayer: next.current_player,
    });
  }

  if (prev.stadium !== next.stadium) {
    changes.push({ type: "stadiumChange", from: prev.stadium, to: next.stadium });
  }

  if (!prev.winner && next.winner) {
    changes.push({ type: "winnerDecided", winner: next.winner });
  }

  for (const p of [0, 1] as PlayerIndex[]) {
    if (prev.points[p] !== next.points[p]) {
      changes.push({ type: "pointsChange", player: p, from: prev.points[p], to: next.points[p] });
    }

    const prevPlayer = prev.players[p];
    const nextPlayer = next.players[p];

    if (prevPlayer.hand_count !== nextPlayer.hand_count) {
      changes.push({
        type: "handCountChange",
        player: p,
        from: prevPlayer.hand_count,
        to: nextPlayer.hand_count,
      });
    }
    if (prevPlayer.deck_count !== nextPlayer.deck_count) {
      changes.push({
        type: "deckCountChange",
        player: p,
        from: prevPlayer.deck_count,
        to: nextPlayer.deck_count,
      });
    }
    if (prevPlayer.discard.length !== nextPlayer.discard.length) {
      changes.push({
        type: "discardCountChange",
        player: p,
        from: prevPlayer.discard.length,
        to: nextPlayer.discard.length,
      });
    }
    if (!sameEnergyZone(prevPlayer.energy_zone, nextPlayer.energy_zone)) {
      changes.push({
        type: "energyZoneChange",
        player: p,
        from: prevPlayer.energy_zone,
        to: nextPlayer.energy_zone,
      });
    }

    // A knock-out this half-turn is inferred from the opponent's points rising: the true KO'd
    // slot(s) on this player's side are the ones that just emptied. Everything else that empties
    // (retreat to hand, self-discard effects, etc.) is a plain `cardLeft`.
    const opponentScored = next.points[(1 - p) as PlayerIndex] > prev.points[(1 - p) as PlayerIndex];

    // Detected *before* the per-slot loop so the active and bench slots it names can be excluded
    // from that loop entirely (see `detectActiveBenchMove`'s comment) — they get exactly one
    // `activeSwitch` change, not a same-slot-index diff.
    const activeBenchMove = detectActiveBenchMove(prevPlayer, nextPlayer);
    const swappedSlots = new Set<SlotIndex>();
    if (activeBenchMove) {
      changes.push({
        type: "activeSwitch",
        player: p,
        benchSlot: slotKey(p, activeBenchMove.benchSlot),
        activeFrom: activeBenchMove.activeFrom,
        activeTo: activeBenchMove.activeTo,
      });
      swappedSlots.add(0);
      swappedSlots.add(activeBenchMove.benchSlot);
    }

    for (const slot of [0, 1, 2, 3] as SlotIndex[]) {
      if (swappedSlots.has(slot)) continue;
      const key = slotKey(p, slot);
      const prevSlot = prevPlayer.in_play[slot];
      const nextSlot = nextPlayer.in_play[slot];

      if (!prevSlot && nextSlot) {
        changes.push({ type: "cardEntered", slot: key, card: nextSlot.card });
        continue;
      }
      if (prevSlot && !nextSlot) {
        if (opponentScored) {
          changes.push({ type: "knockOut", slot: key, card: prevSlot.card });
        } else {
          changes.push({ type: "cardLeft", slot: key, card: prevSlot.card });
        }
        continue;
      }
      if (!prevSlot || !nextSlot) continue;

      if (prevSlot.card !== nextSlot.card) {
        changes.push({ type: "evolution", slot: key, from: prevSlot.card, to: nextSlot.card });
        // An evolution can still gain energy/tools/status/hp changes in the same step (rare, but
        // e.g. a stadium HP bonus changing max HP); fall through to the shared per-slot diffs.
      }

      if (prevSlot.hp !== nextSlot.hp || prevSlot.max_hp !== nextSlot.max_hp) {
        changes.push({ type: "hpChange", slot: key, from: prevSlot.hp, to: nextSlot.hp, maxHp: nextSlot.max_hp });
      }

      const energyDiff = diffMultiset(prevSlot.energy, nextSlot.energy);
      if (energyDiff.added.length || energyDiff.removed.length) {
        changes.push({ type: "energyChange", slot: key, ...energyDiff });
      }

      const toolsDiff = diffMultiset(prevSlot.tools, nextSlot.tools);
      if (toolsDiff.added.length || toolsDiff.removed.length) {
        changes.push({ type: "toolsChange", slot: key, ...toolsDiff });
      }

      const statusDiff = diffMultiset(prevSlot.status, nextSlot.status);
      if (statusDiff.added.length || statusDiff.removed.length) {
        changes.push({ type: "statusChange", slot: key, ...statusDiff });
      }
    }
  }

  return changes;
}

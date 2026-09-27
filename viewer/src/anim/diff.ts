// Pure diffing of two ViewState snapshots into semantic changes. See
// docs/replay-viewer-plan.md's "Transition engine" section. Kept dependency-free (no Pixi/GSAP
// imports) so it's cheaply unit-testable.

import type { EnergyType, EnergyZone, GameOutcome, StatusCondition, ViewState } from "../types/replay";

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

    for (const slot of [0, 1, 2, 3] as SlotIndex[]) {
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

    // Active <-> bench swap: the active card identity changed, and the new active's card id
    // matches a card that was on the bench and is no longer at that bench slot (a retreat or a
    // forced switch), OR the old active's card reappears on a bench slot that used to hold what's
    // now active (a full swap). Card *ids* are the only identity we have (see the "Deviations"
    // note on hand/bench/discard instance identity), so with two identical duplicate copies this
    // picks a plausible pairing rather than a guaranteed-correct one.
    const prevActive = prevPlayer.in_play[0];
    const nextActive = nextPlayer.in_play[0];
    if (prevActive && nextActive && prevActive.card !== nextActive.card) {
      for (const slot of [1, 2, 3] as SlotIndex[]) {
        const prevBench = prevPlayer.in_play[slot];
        const nextBench = nextPlayer.in_play[slot];
        const benchNowHasOldActive = nextBench?.card === prevActive.card;
        const benchHadNewActive = prevBench?.card === nextActive.card;
        if (benchNowHasOldActive || benchHadNewActive) {
          changes.push({
            type: "activeSwitch",
            player: p,
            benchSlot: slotKey(p, slot),
            activeFrom: prevActive.card,
            activeTo: nextActive.card,
          });
          break;
        }
      }
    }
  }

  return changes;
}

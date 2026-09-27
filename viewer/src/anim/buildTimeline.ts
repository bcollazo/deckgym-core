// Turns (prevSnapshot, nextSnapshot, chosenAction) into one GSAP timeline. See
// docs/replay-viewer-plan.md's "Transition engine" section for the design this follows.
//
// Every tween that changes something the board must land on exactly (position, hp-bar fraction,
// alpha/scale of a persistent slot, a slot's card content) is added directly to the returned
// timeline, so it is self-contained: it can be played forward from a board already reconciled to
// `prev`, or (for a transition that was never played forward) primed by reconciling to `prev`,
// built, then jumped to progress(1) before `.reverse()` is called — see
// `store/playbackController.ts` for that priming step. Content swaps that happen instantaneously
// mid-tween (an evolution's new form appearing, a KO'd slot emptying, an active/bench swap) use
// `tl.call` with a direction check (`tl.reversed()`) so they apply the correct snapshot's content
// whichever way the timeline is currently playing.
//
// Decorative one-shot flourishes (particle bursts, screen shake, the shockwave filter) are
// triggered via `tl.call` too, but animate independently on the GSAP global timeline rather than as
// timeline children — see the "Deviations" note in the plan doc for why those aren't reversed.

import type { Card, EnergyType, SimpleAction, ViewState } from "../types/replay";
import { actionPayload, actionTag } from "../types/replay";
import { diffViewStates, type SemanticChange, type SlotKey } from "./diff";
import { energyColor } from "./colors";
import { gsap, prefersReducedMotion } from "./gsap";
import { shockwaveAt, type BoardScene } from "../board/scene";
import { CARD_H, CARD_W } from "../board/layout";

function parseSlotKey(key: SlotKey): { player: number; slot: number } {
  const [player, slot] = key.split("-").map(Number);
  return { player, slot };
}

export interface TimelineContext {
  scene: BoardScene;
  cards: Record<string, Card>;
  actor: number;
}

/** Builds (but does not play) the timeline for the transition `prev -> next` caused by `chosen`. */
export function buildTimeline(prev: ViewState, next: ViewState, chosen: SimpleAction, ctx: TimelineContext): gsap.core.Timeline {
  const { scene, cards, actor } = ctx;
  const reduced = prefersReducedMotion();
  const tl = gsap.timeline({ paused: true });
  const changes = diffViewStates(prev, next);
  const tag = actionTag(chosen);

  const dur = (base: number) => (reduced ? Math.min(0.06, base) : base);

  // Sets a slot's persistent visual to whichever snapshot the timeline is *currently* moving
  // towards — `next`'s content while playing forward, `prev`'s while `.reverse()`d — so a
  // backward step visually undoes the content swap, not just the position/alpha tween around it.
  const applySlot = (player: number, slot: number) => {
    const state = tl.reversed() ? prev : next;
    const visual = scene.slot(player, slot);
    const slotView = state.players[player].in_play[slot];
    visual.update(slotView ? cards[slotView.card] : undefined, slotView ?? undefined);
  };

  let cursor = 0;
  const advance = (by: number) => {
    cursor += by;
  };

  // ---- Turn change: a banner sweep, and update the HUD turn label at the flash point. ----
  const turnChange = changes.find((c): c is Extract<SemanticChange, { type: "turnChange" }> => c.type === "turnChange");
  if (turnChange) {
    tl.set(scene.turnBanner, { visible: true, alpha: 0 }, cursor);
    tl.call(
      () => {
        const toTurn = tl.reversed() ? turnChange.from : turnChange.to;
        const toPlayer = tl.reversed() ? turnChange.fromPlayer : turnChange.toPlayer;
        scene.turnText.text = `Turn ${toTurn}`;
        (scene.turnBanner.getChildAt(0) as import("pixi.js").Graphics).tint = toPlayer === 0 ? 0x4f8cff : 0xff6b6b;
      },
      [],
      cursor,
    );
    tl.fromTo(scene.turnBanner, { alpha: 0 }, { alpha: 1, duration: dur(0.2) }, cursor);
    tl.to(scene.turnBanner, { alpha: 0, duration: dur(0.3) }, cursor + dur(0.5));
    tl.set(scene.turnBanner, { visible: false }, cursor + dur(0.8));
    advance(reduced ? 0.1 : 0.45);
  }

  // ---- Stadium change: quick flash on the stadium visual, content swapped at the flash peak. ----
  const stadiumChange = changes.find((c): c is Extract<SemanticChange, { type: "stadiumChange" }> => c.type === "stadiumChange");
  if (stadiumChange) {
    tl.to(scene.stadium.container.scale, { x: 1.15, y: 1.15, duration: dur(0.15) }, cursor);
    tl.call(
      () => {
        const id = tl.reversed() ? stadiumChange.from : stadiumChange.to;
        scene.stadium.update(id ? cards[id] : undefined, undefined);
      },
      [],
      cursor + dur(0.15),
    );
    tl.to(scene.stadium.container.scale, { x: 1, y: 1, duration: dur(0.15) }, cursor + dur(0.15));
    advance(dur(0.3));
  }

  const consumedSlots = new Set<SlotKey>();

  // ---- Attack sequence: only the active Pokemon attacks, so the attacker slot is always (actor, 0). ----
  if (tag === "Attack") {
    const opponent = actor === 0 ? 1 : 0;
    const targets = changes.filter(
      (c): c is Extract<SemanticChange, { type: "hpChange" }> =>
        c.type === "hpChange" && parseSlotKey(c.slot).player === opponent && c.to < c.from,
    );
    const attackerVisual = scene.slot(actor, 0);
    const attackerPos = scene.slotPosition(actor, 0);
    const primaryTarget: { slot: SlotKey } | undefined =
      targets[0] ?? changes.find((c): c is Extract<SemanticChange, { type: "knockOut" }> => c.type === "knockOut" && parseSlotKey(c.slot).player === opponent);
    const targetPos = primaryTarget
      ? scene.slotPosition(parseSlotKey(primaryTarget.slot).player, parseSlotKey(primaryTarget.slot).slot)
      : scene.slotPosition(opponent, 0);
    const dx = targetPos.x - attackerPos.x;
    const dy = targetPos.y - attackerPos.y;
    const dist = Math.hypot(dx, dy) || 1;
    const lungeX = attackerPos.x + (dx / dist) * 26;
    const lungeY = attackerPos.y + (dy / dist) * 26;
    const pullX = attackerPos.x - (dx / dist) * 10;
    const pullY = attackerPos.y - (dy / dist) * 10;

    tl.to(attackerVisual.container, { x: pullX, y: pullY, duration: dur(0.12), ease: "power1.out" }, cursor);
    tl.to(attackerVisual.container, { x: lungeX, y: lungeY, duration: dur(0.14), ease: "attackLunge" }, cursor + dur(0.12));
    advance(dur(0.12) + dur(0.14));
    if (!reduced) advance(0.07); // hit-stop: a brief hold at the moment of impact
    tl.call(
      () => {
        if (!reduced) {
          shockwaveAt(scene, targetPos);
          scene.screenShake(7, 0.3);
        }
      },
      [],
      cursor,
    );
    tl.to(attackerVisual.container, { x: attackerPos.x, y: attackerPos.y, duration: dur(0.2), ease: "power2.out" }, cursor);
    advance(dur(0.2));

    for (const t of targets) {
      consumedSlots.add(t.slot);
      const { player, slot } = parseSlotKey(t.slot);
      const pos = scene.slotPosition(player, slot);
      const visual = scene.slot(player, slot);
      animateHpBar(tl, visual, t.from, t.to, t.maxHp, cursor, dur);
      if (!reduced) {
        tl.call(
          () => {
            const dmg = t.from - t.to;
            const label = scene.spawnFloatingText({ x: pos.x, y: pos.y - 30 }, `-${dmg}`, 0xff5c5c, 22);
            gsap.to(label, { y: pos.y - 70, alpha: 0, duration: 0.9, ease: "popOvershoot" });
          },
          [],
          cursor,
        );
      }
    }
  }

  // ---- Everything else: generic per-change handling. ----
  for (const change of changes) {
    switch (change.type) {
      case "cardEntered": {
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        const size = slot === 0 ? { w: visual.width, h: visual.height } : { w: CARD_W, h: CARD_H };
        const from = scene.handAnchorPosition(player);
        const to = scene.slotPosition(player, slot);
        const card = cards[change.card];
        tl.call(() => visual.update(undefined, undefined), [], cursor);
        const flight = scene.spawnFlightCard(card, from, to, size);
        tl.fromTo(flight, { x: from.x, y: from.y, alpha: 1 }, { x: to.x, y: to.y, duration: dur(0.35), ease: "power2.inOut" }, cursor);
        tl.fromTo(flight.scale, { x: 0.6, y: 0.6 }, { x: 1, y: 1, duration: dur(0.35) }, cursor);
        tl.call(
          () => {
            applySlot(player, slot);
            visual.container.alpha = 1;
            visual.container.scale.set(1);
            scene.removeFx(flight);
          },
          [],
          cursor + dur(0.35),
        );
        break;
      }
      case "evolution": {
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        const fromDeck = tag === "Evolve" && (actionPayload(chosen) as { from_deck?: boolean } | undefined)?.from_deck === true;
        const anchor = fromDeck ? scene.deckAnchorPosition(player) : scene.handAnchorPosition(player);
        const to = scene.slotPosition(player, slot);
        if (!reduced) {
          const flight = scene.spawnFlightCard(cards[change.to], anchor, to, { w: visual.width, h: visual.height });
          flight.alpha = 0.9;
          tl.fromTo(flight, { x: anchor.x, y: anchor.y }, { x: to.x, y: to.y, duration: dur(0.3), ease: "power2.in" }, cursor);
          tl.call(() => scene.removeFx(flight), [], cursor + dur(0.3));
        }
        tl.to(visual.container.scale, { x: 1.25, y: 1.25, duration: dur(0.15) }, cursor + dur(0.3));
        tl.call(() => applySlot(player, slot), [], cursor + dur(0.3) + dur(0.075));
        tl.to(visual.container.scale, { x: 1, y: 1, duration: dur(0.15) }, cursor + dur(0.3) + dur(0.15));
        if (!reduced) scene.spawnBurst(to, 0xffffff, 10);
        break;
      }
      case "knockOut": {
        consumedSlots.add(change.slot);
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        const pos = scene.slotPosition(player, slot);
        tl.to(visual.container, { x: pos.x + 6, duration: 0.04, yoyo: true, repeat: 3 }, cursor);
        tl.to(visual.container.scale, { x: 0.2, y: 0.2, duration: dur(0.35), ease: "power2.in" }, cursor + dur(0.16));
        tl.to(visual.container, { alpha: 0, duration: dur(0.35) }, cursor + dur(0.16));
        tl.call(
          () => {
            if (!reduced) scene.spawnShards(pos, 0xdddddd, 12);
            applySlot(player, slot);
            visual.container.x = pos.x;
            visual.container.alpha = 1;
            visual.container.scale.set(1);
          },
          [],
          cursor + dur(0.16) + dur(0.35),
        );
        const opponent = player === 0 ? 1 : 0;
        tl.to(scene.pointsContainer(opponent).scale, { x: 1.5, y: 1.5, duration: dur(0.15), yoyo: true, repeat: 1 }, cursor + dur(0.5));
        advance(dur(0.16) + dur(0.35) + dur(0.15));
        break;
      }
      case "cardLeft": {
        if (consumedSlots.has(change.slot)) break;
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        const from = scene.slotPosition(player, slot);
        const to = scene.discardAnchorPosition(player);
        if (!reduced) {
          const flight = scene.spawnFlightCard(cards[change.card], from, to, { w: visual.width, h: visual.height });
          tl.fromTo(flight, { x: from.x, y: from.y }, { x: to.x, y: to.y, duration: dur(0.3) }, cursor);
          tl.to(flight, { alpha: 0, duration: dur(0.1) }, cursor + dur(0.3));
          tl.call(() => scene.removeFx(flight), [], cursor + dur(0.4));
        }
        tl.to(visual.container, { alpha: 0, duration: dur(0.2) }, cursor);
        tl.call(
          () => {
            applySlot(player, slot);
            visual.container.alpha = 1;
          },
          [],
          cursor + dur(0.3),
        );
        break;
      }
      case "activeSwitch": {
        const { player } = change;
        const activeVisual = scene.slot(player, 0);
        const { slot: benchSlotIdx } = parseSlotKey(change.benchSlot);
        const benchVisual = scene.slot(player, benchSlotIdx);
        tl.to([activeVisual.container, benchVisual.container], { alpha: 0, duration: dur(0.15) }, cursor);
        tl.call(
          () => {
            applySlot(player, 0);
            applySlot(player, benchSlotIdx);
          },
          [],
          cursor + dur(0.15),
        );
        tl.to([activeVisual.container, benchVisual.container], { alpha: 1, duration: dur(0.15) }, cursor + dur(0.15));
        advance(dur(0.3));
        break;
      }
      case "hpChange": {
        if (consumedSlots.has(change.slot)) break;
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        animateHpBar(tl, visual, change.from, change.to, change.maxHp, cursor, dur);
        break;
      }
      case "energyChange": {
        const { player, slot } = parseSlotKey(change.slot);
        if (reduced || change.added.length === 0) {
          tl.call(() => applySlot(player, slot), [], cursor);
          break;
        }
        const to = scene.slotPosition(player, slot);
        const from = scene.energyZoneAnchorPosition(player);
        change.added.forEach((type: EnergyType, i: number) => {
          const orb = scene.spawnEnergyOrb(from, to, type);
          const t0 = cursor + i * 0.06;
          tl.to(
            orb,
            {
              motionPath: {
                path: [{ x: (from.x + to.x) / 2, y: Math.min(from.y, to.y) - 50 }, { x: to.x, y: to.y }],
                curviness: 1.25,
              },
              duration: dur(0.5),
              ease: "power1.in",
            },
            t0,
          );
          tl.call(
            () => {
              if (!reduced) scene.spawnBurst(to, energyColor(type), 6);
              scene.removeFx(orb);
              applySlot(player, slot);
            },
            [],
            t0 + dur(0.5),
          );
        });
        advance(dur(0.5) + change.added.length * 0.06);
        break;
      }
      case "toolsChange":
      case "statusChange": {
        const { player, slot } = parseSlotKey(change.slot);
        const visual = scene.slot(player, slot);
        tl.to(visual.container.scale, { x: 1.08, y: 1.08, duration: dur(0.12), yoyo: true, repeat: 1 }, cursor);
        tl.call(() => applySlot(player, slot), [], cursor + dur(0.06));
        break;
      }
      default:
        break;
    }
  }

  // ---- Winner: bloom + confetti at the very end. ----
  const winnerChange = changes.find((c): c is Extract<SemanticChange, { type: "winnerDecided" }> => c.type === "winnerDecided");
  if (winnerChange && !reduced) {
    tl.call(() => scene.winPulse(), [], cursor + 0.2);
  }

  return tl;
}

function animateHpBar(
  tl: gsap.core.Timeline,
  visual: { hpBarInnerWidth: number; setHpBar: (fraction: number, which: "fg" | "ghost") => void },
  from: number,
  to: number,
  maxHp: number,
  at: number,
  dur: (base: number) => number,
) {
  const fromFraction = maxHp > 0 ? from / maxHp : 0;
  const toFraction = maxHp > 0 ? to / maxHp : 0;
  const fg = { v: fromFraction };
  tl.to(fg, { v: toFraction, duration: dur(0.25), ease: "power1.out", onUpdate: () => visual.setHpBar(fg.v, "fg") }, at);
  const ghost = { v: fromFraction };
  tl.to(
    ghost,
    { v: toFraction, duration: dur(0.5), ease: "power1.in", onUpdate: () => visual.setHpBar(ghost.v, "ghost") },
    at + dur(0.35),
  );
}

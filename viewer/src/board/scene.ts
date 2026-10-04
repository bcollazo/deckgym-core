// The persistent Pixi scene graph for one board, plus small "fx" helpers (ephemeral sprites,
// screen shake, particle bursts) that `anim/buildTimeline.ts` composes into GSAP timelines.
//
// Design: every persistent object (a slot's CardVisual, the stadium, hand rows, HUD text) lives at
// a fixed layout position and is never itself tweened in x/y across steps — `reconcile()` always
// snaps every persistent object to exactly match a ViewState. "Flight" animations (a card leaving
// the hand, an energy orb travelling to a slot, KO shards) are done with throwaway sprites added to
// a dedicated `fx` layer and destroyed when their tween finishes. That keeps reconciliation trivial
// (no risk of drift) no matter which direction playback moves, per the plan's "reconcile to
// snapshot at end of each step" rule.

import * as PIXI from "pixi.js";
import { AdvancedBloomFilter, GlowFilter } from "pixi-filters";
import { energyColor, PLAYER_COLORS } from "../anim/colors";
import { gsap } from "../anim/gsap";
import type { Card, EnergyType, PlayerView, ReplayPlayerInfo, ViewState } from "../types/replay";
import type { CardImageStore } from "./cardImages";
import { createCardVisual, type CardVisual } from "./cardArt";
import {
  ACTIVE_SIZE,
  BENCH_SIZE,
  BOARD_HEIGHT,
  BOARD_WIDTH,
  deckAnchor,
  DECK_DISCARD_SIZE,
  discardAnchor,
  dividerLineY,
  energyZoneAnchor,
  handAnchor,
  handCardPos,
  handCardSize,
  handCountAnchor,
  nameAnchor,
  pointsRowAnchor,
  slotPos,
  STADIUM_SIZE,
  stadiumPos,
} from "./layout";

export interface BoardScene {
  app: PIXI.Application;
  root: PIXI.Container;
  fx: PIXI.Container;
  slot(player: number, idx: number): CardVisual;
  slotPosition(player: number, idx: number): { x: number; y: number };
  handAnchorPosition(player: number): { x: number; y: number };
  deckAnchorPosition(player: number): { x: number; y: number };
  discardAnchorPosition(player: number): { x: number; y: number };
  energyZoneAnchorPosition(player: number): { x: number; y: number };
  /** The container holding a player's 3 point pips — animate the whole thing (scale/pulse) rather
   * than an individual pip, which keeps the KO/scoring flourish simple. */
  pointsContainer(player: number): PIXI.Container;
  energyZoneGraphic(player: number): PIXI.Graphics;
  stadium: CardVisual;
  turnBanner: PIXI.Container;
  turnText: PIXI.Text;
  reconcile(state: ViewState, cards: Record<string, Card>): void;
  setPlayerNames(players: [ReplayPlayerInfo, ReplayPlayerInfo]): void;
  spawnFlightCard(card: Card | undefined, from: { x: number; y: number }, to: { x: number; y: number }, size: { w: number; h: number }): PIXI.Container;
  spawnEnergyOrb(from: { x: number; y: number }, to: { x: number; y: number }, type: EnergyType): PIXI.Container;
  spawnBurst(pos: { x: number; y: number }, color: number, count?: number): void;
  spawnShards(pos: { x: number; y: number }, color: number, count?: number): void;
  spawnConfetti(): void;
  spawnFloatingText(pos: { x: number; y: number }, text: string, color: number, size?: number): PIXI.Text;
  /** A brief white flash over a single card (the attack's defender, say) — an fx-layer overlay
   * sized/positioned to match it, not a whole-board effect. Caller fades/removes it (see
   * `anim/buildTimeline.ts`'s hit-reaction handling). */
  spawnFlash(pos: { x: number; y: number }, w: number, h: number): PIXI.Graphics;
  winPulse(): void;
  removeFx(node: PIXI.Container): void;
  /** Kills and destroys every ephemeral fx-layer child. Called before (re)building a transition's
   * timeline and on a hard scrubber jump, so decorative flourishes never outlive the step they
   * belong to (see this file's header comment). */
  clearFx(): void;
  destroy(): void;
}

function makeHudText(text = "", size = 13): PIXI.Text {
  return new PIXI.Text({
    text,
    style: { fontFamily: "Inter, sans-serif", fontSize: size, fill: 0xe8eaf2, fontWeight: "600" },
  });
}

export function createBoardScene(app: PIXI.Application, images?: CardImageStore): BoardScene {
  const root = new PIXI.Container();
  root.sortableChildren = true;
  const fx = new PIXI.Container();
  app.stage.addChild(root);

  // Round 4: a flat near-black board, not the earlier radial vignette — the request was for
  // "darker and cleaner", and a gradient (even a subtle one) is exactly the kind of thing that
  // banded visibly once the board got bigger. A plain fill can't band.
  const background = new PIXI.Graphics().rect(0, 0, BOARD_WIDTH, BOARD_HEIGHT).fill({ color: 0x07090d });
  root.addChild(background);

  // A faint center divider line (no illustrated playmat — just enough to separate the two halves).
  const dividerY = dividerLineY();
  const divider = new PIXI.Graphics()
    .moveTo(20, dividerY)
    .lineTo(BOARD_WIDTH - 20, dividerY)
    .stroke({ width: 1, color: 0xffffff, alpha: 0.08 });
  root.addChild(divider);

  function slotVisual(size: { w: number; h: number }): CardVisual {
    return createCardVisual(size.w, size.h, { images, interactive: true });
  }

  const slots: CardVisual[][] = [[], []];
  for (const player of [0, 1]) {
    for (let idx = 0; idx < 4; idx++) {
      const size = idx === 0 ? ACTIVE_SIZE : BENCH_SIZE;
      // Faint rounded slot outline, drawn once, under everything — reads as an empty spot when
      // there's no Pokemon there, matching the "faint rounded slot outlines" look.
      const outline = new PIXI.Graphics().roundRect(-size.w / 2, -size.h / 2, size.w, size.h, size.w * 0.1).stroke({ width: 1.5, color: 0xffffff, alpha: 0.08 });
      const pos = slotPos(player, idx);
      outline.x = pos.x;
      outline.y = pos.y;
      root.addChild(outline);

      const visual = slotVisual(size);
      visual.container.pivot.set(size.w / 2, size.h / 2);
      visual.container.x = pos.x;
      visual.container.y = pos.y;
      visual.update(undefined, undefined);
      root.addChild(visual.container);
      slots[player][idx] = visual;
    }
  }

  const stadium = createCardVisual(STADIUM_SIZE.w, STADIUM_SIZE.h, { images, interactive: true });
  const stPos = stadiumPos();
  stadium.container.pivot.set(STADIUM_SIZE.w / 2, STADIUM_SIZE.h / 2);
  stadium.container.x = stPos.x;
  stadium.container.y = stPos.y;
  stadium.update(undefined, undefined);
  root.addChild(stadium.container);

  const nameTexts: PIXI.Text[] = [];
  const pointsContainers: PIXI.Container[] = [];
  const deckStacks: PIXI.Graphics[] = [];
  const deckTexts: PIXI.Text[] = [];
  const discardVisuals: CardVisual[] = [];
  const discardCountTexts: PIXI.Text[] = [];
  const energyZoneGraphics: PIXI.Graphics[] = [];
  const handContainers: PIXI.Container[] = [];
  const handVisuals: CardVisual[][] = [[], []];
  let handCountText: PIXI.Text | null = null;

  for (const player of [0, 1]) {
    const name = makeHudText();
    const nAnchor = nameAnchor(player);
    name.anchor.set(0, 0.5);
    name.x = nAnchor.x;
    name.y = nAnchor.y;
    root.addChild(name);
    nameTexts[player] = name;

    const points = new PIXI.Container();
    const pAnchor = pointsRowAnchor(player);
    points.x = pAnchor.x;
    points.y = pAnchor.y;
    root.addChild(points);
    pointsContainers[player] = points;

    const ez = new PIXI.Graphics();
    const ezAnchor = energyZoneAnchor(player);
    ez.x = ezAnchor.x;
    ez.y = ezAnchor.y;
    root.addChild(ez);
    energyZoneGraphics[player] = ez;

    const deckAnc = deckAnchor(player);
    const deckStack = new PIXI.Graphics();
    deckStack.x = deckAnc.x;
    deckStack.y = deckAnc.y;
    root.addChild(deckStack);
    const deckText = makeHudText("", 15);
    deckText.anchor.set(0.5);
    deckText.y = DECK_DISCARD_SIZE.h / 2 + 14;
    deckStack.addChild(deckText);
    deckStacks[player] = deckStack;
    deckTexts[player] = deckText;

    // The discard pile shows its actual top card, small, with a count badge — not just an
    // abstract stack (unlike the deck, whose order/identity is meant to stay hidden).
    const discAnc = discardAnchor(player);
    const discardVisual = createCardVisual(DECK_DISCARD_SIZE.w, DECK_DISCARD_SIZE.h, { images });
    discardVisual.container.pivot.set(DECK_DISCARD_SIZE.w / 2, DECK_DISCARD_SIZE.h / 2);
    discardVisual.container.x = discAnc.x;
    discardVisual.container.y = discAnc.y;
    discardVisual.container.alpha = 0.9;
    root.addChild(discardVisual.container);
    discardVisuals[player] = discardVisual;
    const discardCount = makeHudText("", 15);
    discardCount.anchor.set(0.5);
    discardCount.x = discAnc.x;
    discardCount.y = discAnc.y + DECK_DISCARD_SIZE.h / 2 + 14;
    root.addChild(discardCount);
    discardCountTexts[player] = discardCount;

    const hand = new PIXI.Container();
    root.addChild(hand);
    handContainers[player] = hand;
  }

  const countAnc = handCountAnchor();
  handCountText = makeHudText("", 12);
  handCountText.anchor.set(1, 0.5);
  handCountText.x = countAnc.x;
  handCountText.y = countAnc.y;
  root.addChild(handCountText);

  const turnBanner = new PIXI.Container();
  const bannerBg = new PIXI.Graphics().rect(-BOARD_WIDTH, -15, BOARD_WIDTH * 2, 30).fill({ color: 0xffffff, alpha: 0.12 });
  const turnText = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: 20, fill: 0xffffff, fontWeight: "800" },
  });
  turnText.anchor.set(0.5);
  turnBanner.addChild(bannerBg, turnText);
  turnBanner.x = BOARD_WIDTH / 2;
  turnBanner.y = dividerLineY();
  turnBanner.alpha = 0;
  turnBanner.visible = false;
  root.addChild(turnBanner);

  root.addChild(fx);

  function drawStack(g: PIXI.Graphics, count: number, color: number) {
    g.clear();
    const layers = Math.min(3, count > 0 ? 3 : 0);
    const { w, h } = DECK_DISCARD_SIZE;
    for (let i = layers - 1; i >= 0; i--) {
      g.roundRect(-w / 2 - i * 2, -h / 2 - i * 2, w, h, 4).fill({ color, alpha: 0.3 + i * 0.12 }).stroke({ width: 1, color: 0x000000, alpha: 0.3 });
    }
  }

  function drawPointsPips(container: PIXI.Container, points: number) {
    container.removeChildren();
    const radius = 7;
    const spacing = 20;
    for (let i = 0; i < 3; i++) {
      const filled = i < points;
      const pip = new PIXI.Graphics();
      if (filled) {
        pip.circle(0, 0, radius).fill({ color: 0xffd54f });
        pip.filters = [new GlowFilter({ color: 0xffd54f, distance: 5, outerStrength: 0.5, innerStrength: 0 })];
      } else {
        pip.circle(0, 0, radius).stroke({ width: 1.5, color: 0xffffff, alpha: 0.35 });
      }
      pip.x = (i - 1) * spacing;
      container.addChild(pip);
    }
  }

  function reconcileHand(player: number, hand: string[] | null, cards: Record<string, Card>) {
    const container = handContainers[player];
    const list = hand ?? [];
    const size = handCardSize(player);

    for (const old of handVisuals[player]) old.destroy();
    handVisuals[player] = [];
    container.removeChildren();

    list.forEach((cardId, i) => {
      const pos = handCardPos(player, i, list.length);
      const visual = createCardVisual(size.w, size.h, { images, interactive: player === 0 });
      visual.container.x = pos.x;
      visual.container.y = pos.y;
      visual.container.pivot.set(size.w / 2, size.h / 2);
      visual.setBaseRotation(pos.rotation);
      visual.container.zIndex = i;
      if (player === 1) visual.container.alpha = 0.85; // opponent hand reads as "visible but theirs"
      visual.update(cards[cardId], undefined);
      container.addChild(visual.container);
      handVisuals[player].push(visual);
    });
    container.sortableChildren = true;
  }

  const scene: BoardScene = {
    app,
    root,
    fx,
    slot: (player, idx) => slots[player][idx],
    slotPosition: (player, idx) => slotPos(player, idx),
    handAnchorPosition: (player) => handAnchor(player),
    deckAnchorPosition: (player) => deckAnchor(player),
    discardAnchorPosition: (player) => discardAnchor(player),
    energyZoneAnchorPosition: (player) => energyZoneAnchor(player),
    pointsContainer: (player) => pointsContainers[player],
    energyZoneGraphic: (player) => energyZoneGraphics[player],
    stadium,
    turnBanner,
    turnText,

    setPlayerNames(players) {
      nameTexts[0].text = players[0].name;
      nameTexts[1].text = players[1].name;
    },

    reconcile(state, cards) {
      for (const player of [0, 1] as const) {
        const pv: PlayerView = state.players[player];
        for (let idx = 0; idx < 4; idx++) {
          const slotView = pv.in_play[idx];
          const visual = slots[player][idx];
          visual.update(slotView ? cards[slotView.card] : undefined, slotView ?? undefined);
          visual.setTools((slotView?.tools ?? []).map((id) => cards[id]).filter((card) => card !== undefined));
          visual.container.alpha = 1;
          visual.container.scale.set(1);
          visual.container.rotation = 0;
        }
        drawPointsPips(pointsContainers[player], state.points[player]);
        drawStack(deckStacks[player], pv.deck_count, PLAYER_COLORS[player]);
        deckTexts[player].text = String(pv.deck_count);

        const topDiscardId = pv.discard.at(-1);
        discardVisuals[player].update(topDiscardId ? cards[topDiscardId] : undefined, undefined);
        discardVisuals[player].container.alpha = topDiscardId ? 0.9 : 0;
        discardCountTexts[player].text = pv.discard.length > 0 ? String(pv.discard.length) : "";

        const ez = energyZoneGraphics[player];
        ez.clear();
        ez.circle(0, 0, 18).fill({ color: energyColor(pv.energy_zone.current) }).stroke({ width: 2.5, color: 0xffffff, alpha: pv.energy_zone.current ? 0.8 : 0.15 });
        ez.circle(28, 18, 9).fill({ color: energyColor(pv.energy_zone.next), alpha: 0.7 });

        reconcileHand(player, pv.hand, cards);
      }
      if (handCountText) handCountText.text = `${state.players[1].hand_count} cards`;
      stadium.update(state.stadium ? cards[state.stadium] : undefined, undefined);
      stadium.container.alpha = state.stadium ? 1 : 0;
      turnText.text = `Turn ${state.turn}`;
      root.x = 0;
      root.y = 0;
    },

    // Both `spawn*` helpers below only *create* an ephemeral fx-layer object positioned at `from`;
    // they deliberately don't animate it themselves. The caller (`anim/buildTimeline.ts`) adds the
    // movement tween directly to its transition timeline, so that timeline stays the single source
    // of truth for forward/backward playback and speed control (see that file's header comment).
    spawnFlightCard(card, from, _to, size) {
      const visual = createCardVisual(size.w, size.h, { images });
      visual.update(card, undefined);
      visual.container.pivot.set(size.w / 2, size.h / 2);
      visual.container.x = from.x;
      visual.container.y = from.y;
      visual.container.scale.set(0.7);
      fx.addChild(visual.container);
      return visual.container;
    },

    spawnEnergyOrb(from, _to, type) {
      const orb = new PIXI.Graphics().circle(0, 0, 9).fill({ color: energyColor(type) }).stroke({ width: 2, color: 0xffffff, alpha: 0.8 });
      orb.x = from.x;
      orb.y = from.y;
      fx.addChild(orb);
      return orb;
    },

    spawnBurst(pos, color, count = 8) {
      for (let i = 0; i < count; i++) {
        const angle = (i / count) * Math.PI * 2;
        const p = new PIXI.Graphics().circle(0, 0, 3).fill({ color });
        p.x = pos.x;
        p.y = pos.y;
        fx.addChild(p);
        gsap.to(p, {
          x: pos.x + Math.cos(angle) * 24,
          y: pos.y + Math.sin(angle) * 24,
          alpha: 0,
          duration: 0.4,
          ease: "power1.out",
          onComplete: () => p.destroy(),
        });
      }
    },

    spawnShards(pos, color, count = 10) {
      for (let i = 0; i < count; i++) {
        const shard = new PIXI.Graphics().rect(-4, -4, 8, 8).fill({ color });
        shard.x = pos.x;
        shard.y = pos.y;
        fx.addChild(shard);
        const angle = Math.random() * Math.PI * 2;
        const dist = 30 + Math.random() * 40;
        gsap.to(shard, {
          x: pos.x + Math.cos(angle) * dist,
          y: pos.y + Math.sin(angle) * dist,
          rotation: (Math.random() - 0.5) * 6,
          alpha: 0,
          duration: 0.6 + Math.random() * 0.3,
          ease: "power2.out",
          onComplete: () => shard.destroy(),
        });
      }
    },

    spawnConfetti() {
      const colors = [0xffd54f, 0x4f8cff, 0xff6b6b, 0x66d97a, 0xe040fb];
      for (let i = 0; i < 40; i++) {
        const piece = new PIXI.Graphics().rect(-4, -6, 8, 12).fill({ color: colors[i % colors.length] });
        piece.x = Math.random() * BOARD_WIDTH;
        piece.y = -20;
        fx.addChild(piece);
        gsap.to(piece, {
          y: BOARD_HEIGHT + 40,
          x: piece.x + (Math.random() - 0.5) * 120,
          rotation: Math.random() * 10,
          duration: 1.6 + Math.random() * 1.2,
          delay: Math.random() * 0.5,
          ease: "power1.in",
          onComplete: () => piece.destroy(),
        });
      }
    },

    spawnFloatingText(pos, text, color, size = 20) {
      const t = new PIXI.Text({
        text,
        style: { fontFamily: "Inter, sans-serif", fontSize: size, fill: color, fontWeight: "800", stroke: { color: 0x000000, width: 3 } },
      });
      t.anchor.set(0.5);
      t.x = pos.x;
      t.y = pos.y;
      fx.addChild(t);
      return t;
    },

    spawnFlash(pos, w, h) {
      const g = new PIXI.Graphics().roundRect(-w / 2, -h / 2, w, h, w * 0.08).fill({ color: 0xffffff });
      g.x = pos.x;
      g.y = pos.y;
      g.alpha = 0.85;
      g.blendMode = "add";
      fx.addChild(g);
      return g;
    },

    winPulse() {
      const bloom = new AdvancedBloomFilter({ threshold: 0.2, bloomScale: 1.4, brightness: 1.1 });
      const existing = root.filters ? (Array.isArray(root.filters) ? root.filters : [root.filters]) : [];
      root.filters = [...existing, bloom];
      gsap.fromTo(bloom, { bloomScale: 0 }, { bloomScale: 1.6, duration: 0.6, yoyo: true, repeat: 1, ease: "sine.inOut" });
      scene.spawnConfetti();
    },

    removeFx(node) {
      gsap.killTweensOf(node);
      if (node.parent) node.parent.removeChild(node);
      node.destroy({ children: true });
    },

    clearFx() {
      const children = [...fx.children];
      for (const child of children) {
        gsap.killTweensOf(child);
      }
      fx.removeChildren();
      for (const child of children) {
        child.destroy({ children: true });
      }
    },

    destroy() {
      app.stage.removeChild(root);
      root.destroy({ children: true });
    },
  };

  return scene;
}

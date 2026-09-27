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
import { AdvancedBloomFilter, ShockwaveFilter } from "pixi-filters";
import { energyColor, PLAYER_COLORS } from "../anim/colors";
import { gsap } from "../anim/gsap";
import type { Card, EnergyType, PlayerView, ReplayPlayerInfo, ViewState } from "../types/replay";
import { cardName } from "../types/replay";
import { createCardVisual, type CardVisual } from "./cardArt";
import {
  ACTIVE_CARD_H,
  ACTIVE_CARD_W,
  BOARD_HEIGHT,
  BOARD_WIDTH,
  CARD_H,
  CARD_W,
  deckAnchor,
  discardAnchor,
  energyZoneAnchor,
  HAND_CARD_H,
  HAND_CARD_W,
  handAnchor,
  handCardPos,
  nameAnchor,
  pointsAnchor,
  slotPos,
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
  pointsText(player: number): PIXI.Text;
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
  screenShake(intensity?: number, duration?: number): gsap.core.Timeline;
  winPulse(): void;
  removeFx(node: PIXI.Container): void;
  /** Kills and destroys every ephemeral fx-layer child. Called before (re)building a transition's
   * timeline and on a hard scrubber jump, so decorative flourishes never outlive the step they
   * belong to (see this file's header comment). */
  clearFx(): void;
  destroy(): void;
}

function makeHudText(text = ""): PIXI.Text {
  return new PIXI.Text({
    text,
    style: { fontFamily: "Inter, sans-serif", fontSize: 13, fill: 0xe8eaf2, fontWeight: "600" },
  });
}

export function createBoardScene(app: PIXI.Application): BoardScene {
  const root = new PIXI.Container();
  const fx = new PIXI.Container();
  app.stage.addChild(root);

  const background = new PIXI.Graphics()
    .rect(0, 0, BOARD_WIDTH, BOARD_HEIGHT)
    .fill({ color: 0x0e1018 });
  root.addChild(background);

  // A soft center divider ("stadium row").
  const divider = new PIXI.Graphics()
    .rect(0, BOARD_HEIGHT / 2 - 46, BOARD_WIDTH, 92)
    .fill({ color: 0x171a26 });
  root.addChild(divider);

  const slots: CardVisual[][] = [[], []];
  for (const player of [0, 1]) {
    for (let idx = 0; idx < 4; idx++) {
      const size = idx === 0 ? { w: ACTIVE_CARD_W, h: ACTIVE_CARD_H } : { w: CARD_W, h: CARD_H };
      const visual = createCardVisual(size.w, size.h);
      const pos = slotPos(player, idx);
      visual.container.pivot.set(size.w / 2, size.h / 2);
      visual.container.x = pos.x;
      visual.container.y = pos.y;
      visual.update(undefined, undefined);
      root.addChild(visual.container);
      slots[player][idx] = visual;
    }
  }

  const stadium = createCardVisual(96, 72);
  const stPos = stadiumPos();
  stadium.container.pivot.set(48, 36);
  stadium.container.x = stPos.x;
  stadium.container.y = stPos.y;
  stadium.update(undefined, undefined);
  root.addChild(stadium.container);

  const nameTexts: PIXI.Text[] = [];
  const pointsTexts: PIXI.Text[] = [];
  const deckStacks: PIXI.Graphics[] = [];
  const deckTexts: PIXI.Text[] = [];
  const discardStacks: PIXI.Graphics[] = [];
  const discardTexts: PIXI.Text[] = [];
  const energyZoneGraphics: PIXI.Graphics[] = [];
  const handContainers: PIXI.Container[] = [];

  for (const player of [0, 1]) {
    const name = makeHudText();
    const nAnchor = nameAnchor(player);
    name.x = nAnchor.x;
    name.y = nAnchor.y - 8;
    root.addChild(name);
    nameTexts[player] = name;

    const points = makeHudText();
    const pAnchor = pointsAnchor(player);
    points.x = pAnchor.x;
    points.y = pAnchor.y - 8;
    root.addChild(points);
    pointsTexts[player] = points;

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
    const deckText = makeHudText();
    deckText.anchor.set(0.5);
    deckStack.addChild(deckText);
    deckStacks[player] = deckStack;
    deckTexts[player] = deckText;

    const discAnc = discardAnchor(player);
    const discStack = new PIXI.Graphics();
    discStack.x = discAnc.x;
    discStack.y = discAnc.y;
    root.addChild(discStack);
    const discText = makeHudText();
    discText.anchor.set(0.5);
    discStack.addChild(discText);
    discardStacks[player] = discStack;
    discardTexts[player] = discText;

    const hand = new PIXI.Container();
    root.addChild(hand);
    handContainers[player] = hand;
  }

  const turnBanner = new PIXI.Container();
  const bannerBg = new PIXI.Graphics().rect(-BOARD_WIDTH, -18, BOARD_WIDTH * 2, 36).fill({ color: 0xffffff, alpha: 0.12 });
  const turnText = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: 22, fill: 0xffffff, fontWeight: "800" },
  });
  turnText.anchor.set(0.5);
  turnBanner.addChild(bannerBg, turnText);
  turnBanner.x = BOARD_WIDTH / 2;
  turnBanner.y = BOARD_HEIGHT / 2;
  turnBanner.alpha = 0;
  turnBanner.visible = false;
  root.addChild(turnBanner);

  root.addChild(fx);

  function drawStack(g: PIXI.Graphics, count: number, color: number) {
    g.clear();
    const layers = Math.min(3, count > 0 ? 3 : 0);
    for (let i = layers - 1; i >= 0; i--) {
      g.roundRect(-20 - i * 2, -28 - i * 2, 40, 56, 4).fill({ color, alpha: 0.25 + i * 0.1 }).stroke({ width: 1, color: 0x000000, alpha: 0.3 });
    }
  }

  function reconcileHand(player: number, hand: string[] | null, cards: Record<string, Card>) {
    const container = handContainers[player];
    container.removeChildren();
    const list = hand ?? [];
    list.forEach((cardId, i) => {
      const pos = handCardPos(player, i, list.length);
      const visual = createCardVisual(HAND_CARD_W, HAND_CARD_H);
      visual.container.x = pos.x;
      visual.container.y = pos.y;
      visual.container.pivot.set(HAND_CARD_W / 2, HAND_CARD_H / 2);
      visual.update(cards[cardId], undefined);
      container.addChild(visual.container);
    });
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
    pointsText: (player) => pointsTexts[player],
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
          visual.container.alpha = 1;
          visual.container.scale.set(1);
          visual.container.rotation = 0;
        }
        pointsTexts[player].text = "*".repeat(state.points[player]) + "-".repeat(Math.max(0, 3 - state.points[player]));
        drawStack(deckStacks[player], pv.deck_count, PLAYER_COLORS[player]);
        deckTexts[player].text = String(pv.deck_count);
        drawStack(discardStacks[player], pv.discard.length, 0x555555);
        discardTexts[player].text = String(pv.discard.length);

        const ez = energyZoneGraphics[player];
        ez.clear();
        ez.circle(0, 0, 12).fill({ color: energyColor(pv.energy_zone.current) }).stroke({ width: 2, color: 0xffffff, alpha: pv.energy_zone.current ? 0.8 : 0.15 });
        ez.circle(20, 0, 7).fill({ color: energyColor(pv.energy_zone.next), alpha: 0.7 });

        reconcileHand(player, pv.hand, cards);
      }
      stadium.update(state.stadium ? cards[state.stadium] : undefined, undefined);
      turnText.text = `Turn ${state.turn}`;
      root.x = 0;
      root.y = 0;
    },

    // Both `spawn*` helpers below only *create* an ephemeral fx-layer object positioned at `from`;
    // they deliberately don't animate it themselves. The caller (`anim/buildTimeline.ts`) adds the
    // movement tween directly to its transition timeline, so that timeline stays the single source
    // of truth for forward/backward playback and speed control (see that file's header comment).
    spawnFlightCard(card, from, _to, size) {
      const visual = createCardVisual(size.w, size.h);
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

    screenShake(intensity = 8, duration = 0.35) {
      const tl = gsap.timeline();
      const steps = 6;
      for (let i = 0; i < steps; i++) {
        const decay = 1 - i / steps;
        tl.to(root, {
          x: (Math.random() - 0.5) * intensity * decay,
          y: (Math.random() - 0.5) * intensity * decay,
          duration: duration / steps,
          ease: "none",
        });
      }
      tl.to(root, { x: 0, y: 0, duration: duration / steps });
      return tl;
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

export function shockwaveAt(scene: BoardScene, pos: { x: number; y: number }) {
  const filter = new ShockwaveFilter({ amplitude: 24, wavelength: 140, speed: 900 });
  filter.center = [pos.x / BOARD_WIDTH, pos.y / BOARD_HEIGHT];
  const existing = scene.root.filters ? (Array.isArray(scene.root.filters) ? scene.root.filters : [scene.root.filters]) : [];
  scene.root.filters = [...existing, filter];
  const state = { time: 0 };
  gsap.fromTo(
    state,
    { time: 0 },
    {
      time: 1,
      duration: 0.5,
      ease: "power1.out",
      onUpdate: () => {
        filter.time = state.time;
      },
      onComplete: () => {
        scene.root.filters = (scene.root.filters as PIXI.Filter[]).filter((f) => f !== filter);
      },
    },
  );
}

export function nameOrId(cards: Record<string, Card>, id: string | null | undefined): string {
  if (!id) return "";
  const c = cards[id];
  return c ? cardName(c) : id;
}

// Card rendering: procedurally-drawn by default (no card art ships with the viewer — see the
// plan's "No card art" decision), showing the same information a real card would (name, HP, stage,
// attacks with their energy cost, ability, weakness/retreat), or, when a `CardImageStore` is
// supplied and has a texture for a card's id, a real image with the same overlays composited on
// top (see `docs/replay-viewer-plan.md`'s "Round 2"/"Round 3" sections on "bring your own assets").
// Every card — procedural or image — keeps the physical card's real aspect ratio
// (`layout.ts`'s `CARD_ASPECT`, matching deckgym's own 367x512 card images).

import * as PIXI from "pixi.js";
import { GlowFilter } from "pixi-filters";
import { energyColor } from "../anim/colors";
import { gsap } from "../anim/gsap";
import type { Attack, Card, EnergyType, SlotView, StatusCondition } from "../types/replay";
import { cardEnergyType, cardId, cardName, isExCard, isPokemonCard, stageLabel } from "../types/replay";
import type { CardImageStore } from "./cardImages";
import { fitContain } from "./imageFit";
import { HP_STRIP_GAP, TOOL_PEEK_FRACTION, hpStripHeight } from "./layout";

const STATUS_GLYPH: Record<StatusCondition, { glyph: string; color: number }> = {
  Poisoned: { glyph: "PS", color: 0xab47bc },
  Paralyzed: { glyph: "PR", color: 0xffd54f },
  Asleep: { glyph: "SL", color: 0x90caf9 },
  Burned: { glyph: "BN", color: 0xff7043 },
  Confused: { glyph: "CF", color: 0xff8a80 },
};

/** How much procedural detail fits legibly at a given rendered height — see the plan doc's
 * "Round 3" notes. Opponent-hand-sized cards get name+HP only; bench/player-hand-sized cards get a
 * compact single attack line; active-sized cards get the full attack list + ability. */
type DetailTier = "minimal" | "compact" | "full";
function tierFor(height: number): DetailTier {
  if (height < 100) return "minimal";
  if (height < 200) return "compact";
  return "full";
}

// Crisp text at HiDPI: each Text's own resolution, capped like the Application's (see Board.tsx),
// rather than relying on whatever the shared default happens to be.
const TEXT_RESOLUTION = typeof window === "undefined" ? 1 : Math.min(window.devicePixelRatio || 1, 3);

function text(value: string, style: Partial<PIXI.TextStyleOptions>): PIXI.Text {
  const t = new PIXI.Text({ text: value, style: { fontFamily: "Inter, sans-serif", ...style } });
  t.resolution = TEXT_RESOLUTION;
  return t;
}

function truncate(value: string, max: number): string {
  return value.length > max ? `${value.slice(0, max - 1)}…` : value;
}

function pipRow(types: EnergyType[], radius: number, fallbackColor = 0x9aa0b4): PIXI.Container {
  const row = new PIXI.Container();
  types.forEach((t, i) => {
    const pip = new PIXI.Graphics()
      .circle(i * (radius * 2 + radius * 0.4), 0, radius)
      .fill({ color: t ? energyColor(t) : fallbackColor })
      .stroke({ width: Math.max(1, radius * 0.15), color: 0x0b0d14 });
    row.addChild(pip);
  });
  return row;
}

export interface CardVisual {
  container: PIXI.Container;
  width: number;
  height: number;
  hpBarGhost: PIXI.Graphics;
  hpBarFg: PIXI.Graphics;
  hpBarInnerWidth: number;
  /** Redraws the card face for the given card/slot. Cheap enough to call on every reconcile. */
  update(card: Card | undefined, slot: SlotView | undefined, faceDown?: boolean): void;
  setHpBar(fraction: number, which: "fg" | "ghost"): void;
  /** Shows the Pokemon's attached Tool cards tucked behind it, each peeking out below by
   * `TOOL_PEEK_FRACTION` of the card height with its name on the visible sliver. Pass [] to clear. */
  setTools(tools: Card[]): void;
  /** Sets the card's rest-state rotation (a fanned hand card's tilt). Hover tilt (if interactive)
   * animates relative to this rather than assuming zero, so hovering a fanned card doesn't snap it
   * upright. */
  setBaseRotation(rotation: number): void;
  destroy(): void;
}

export interface CardVisualOptions {
  /** Shared image lookup/loader; omit to always use the procedural face. */
  images?: CardImageStore;
  /** Subtle hover tilt (skew + lift). Off by default for throwaway fx sprites that don't want
   * pointer events fighting an in-flight tween. */
  interactive?: boolean;
}

export function createCardVisual(width: number, height: number, opts: CardVisualOptions = {}): CardVisual {
  const { images, interactive = false } = opts;

  const container = new PIXI.Container();
  const bg = new PIXI.Graphics();
  const art = new PIXI.Graphics();
  const imageSprite = new PIXI.Sprite();
  imageSprite.visible = false;
  imageSprite.anchor.set(0.5);
  imageSprite.x = width / 2;
  imageSprite.y = height / 2;
  const frame = new PIXI.Graphics();
  const shimmer = new PIXI.Graphics();
  shimmer.visible = false;

  // Proportional margins (fractions of the card's own size) rather than fixed pixel offsets, so
  // this scales cleanly from a small opponent-hand card up to a large active card — a fixed-pixel
  // "reserved band" was the Round 2 bug that made small hand cards render as empty outlines (the
  // art rect's height went negative once the card was smaller than the reserved bands).
  const marginX = width * 0.06;
  const cornerRadius = width * 0.09;

  // Rounded-corner clip for the real-image face (Round 4: an image card draws with no frame/border
  // stroke at all — see `redrawStatic` — so this mask is what keeps its corners matching every
  // other card's rounded shape instead of a hard rectangle; it also clips a "contain"-fit BYO image
  // whose aspect ratio doesn't match the card's own, so a letterboxed edge never pokes past the
  // rounding). `includeInBuild`/`measurable` get set false on it by Pixi itself once it's used as a
  // mask, so adding it as a plain child (needed for it to pick up the container's transform) doesn't
  // also render it as a visible white rectangle.
  const imageMask = new PIXI.Graphics().roundRect(0, 0, width, height, cornerRadius).fill(0xffffff);
  imageSprite.mask = imageMask;

  // Rich procedural content (name/HP header, stage/ex badges, ability, attacks, weakness/retreat)
  // — hidden when showing a real image, and rebuilt each `update()` since its content and the
  // detail tier both vary per card. Confined above `overlayTop`; the live overlays below that
  // (attached energy, status) never fight it for space.
  const procInfo = new PIXI.Container();

  // ---- HP strip: a compact bar (left) + number (right) drawn *above* the card, not inside/below
  // its face (Round 4 fix — the HP number/bar used to float over the card's own top-right corner
  // with no reserved space, landing on whichever row/divider/turn-banner happened to be above it;
  // see the plan doc's "Round 4" notes and `layout.ts`'s `slotRects`). `layout.ts` reserves exactly
  // `hpStripHeight(height)` of space above every in-play slot for this — same formula, so the two
  // can never drift out of sync. Only in-play slots (a `slot` passed to `update()`) show it; hand/
  // discard/stadium cards leave it invisible (and layout.ts reserves no strip space for those). ----
  const stripH = hpStripHeight(height);
  const stripBarH = Math.max(4, stripH * 0.4);
  const stripMidY = -HP_STRIP_GAP - stripH / 2;
  const stripBarW = width * 0.64;
  const hpNumber = text("", {
    fontSize: Math.max(10, stripH * 0.78),
    fill: 0x66d97a,
    fontWeight: "800",
    stroke: { color: 0x0b0d14, width: 3 },
  });
  hpNumber.anchor.set(1, 0.5);
  hpNumber.x = width;
  hpNumber.y = stripMidY;
  const hpBarBg = new PIXI.Graphics();
  const hpBarGhost = new PIXI.Graphics();
  const hpBarFg = new PIXI.Graphics();
  const energyRow = new PIXI.Container();
  const statusRow = new PIXI.Container();
  const toolBadge = text("", { fontSize: 9, fill: 0xffe08a, fontWeight: "700" });

  // Tool cards tucked behind this one: drawn first so the card face covers all but a sliver.
  const toolLayer = new PIXI.Container();
  let toolVisuals: CardVisual[] = [];

  container.addChild(toolLayer, bg, art, imageSprite, imageMask, frame, shimmer, procInfo, energyRow, statusRow, toolBadge, hpBarBg, hpBarGhost, hpBarFg, hpNumber);

  let baseRotation = 0;
  if (interactive) {
    container.eventMode = "static";
    container.cursor = "pointer";
    container.on("pointerover", () => {
      gsap.to(container, { rotation: baseRotation - 0.07, duration: 0.18, ease: "power2.out" });
      gsap.to(container.scale, { x: 1.06, y: 1.06, duration: 0.18, ease: "power2.out" });
      container.zIndex = 1000;
    });
    container.on("pointerout", () => {
      gsap.to(container, { rotation: baseRotation, duration: 0.22, ease: "power2.out" });
      gsap.to(container.scale, { x: 1, y: 1, duration: 0.22, ease: "power2.out" });
      container.zIndex = 0;
    });
  }

  // Attached-energy and status overlays now sit at the very bottom of the card face (the HP bar
  // that used to anchor this stack lives in the strip above the card now — see above), with room
  // above them (`overlayTop`) for procedural attack lines/footer/effect text.
  const energyY = height - height * 0.09;
  const statusY = energyY - height * 0.13;
  const overlayTop = statusY - height * 0.07;

  function redrawStatic(type: EnergyType | null, ex: boolean, useImage: boolean, tier: DetailTier) {
    const color = energyColor(type);
    bg.clear();
    art.clear();
    imageSprite.visible = useImage;

    if (!useImage) {
      bg.roundRect(0, 0, width, height, cornerRadius).fill({ color: 0x14161f });
      const artMargin = width * 0.05;
      // A real-card-like layout: a boxed art band near the top, plain panel below for text, at
      // any tier with enough room for that text; at the smallest (hand) sizes the art simply
      // fills the whole face, matching a plain colored card back/face.
      const artY = tier === "minimal" ? height * 0.1 : height * 0.11;
      const artH = tier === "minimal" ? height - artY - height * 0.05 : height * 0.32;
      const artW = width - artMargin * 2;
      art.roundRect(artMargin, artY, artW, artH, cornerRadius * 0.5).fill({ color });
      // A soft vertical gradient "art" look, faked with a couple of translucent overlays (Pixi v8
      // has no built-in gradient fill helper here without a canvas texture, so this keeps it cheap).
      art.roundRect(artMargin, artY, artW, artH / 2, cornerRadius * 0.5).fill({ color: 0xffffff, alpha: 0.14 });
      art.roundRect(artMargin, artY + artH * 0.6, artW, artH * 0.4, cornerRadius * 0.5).fill({ color: 0x000000, alpha: 0.2 });
    }

    // Round 4: a real image draws with no frame/border at all — "the image only" (plus the rounded-
    // corner mask above, and the live status/energy/tool overlays, which stay either way). Only
    // procedural cards (which need *something* to read as a card edge around a flat color fill)
    // keep the stroke.
    frame.clear();
    if (!useImage) {
      frame.roundRect(0, 0, width, height, cornerRadius).stroke({ width: ex ? 2.5 : 1.5, color: ex ? 0xffd54f : color, alpha: 0.9 });
    }

    // Round 4: an image card draws "the image only" (plus the required overlays) — no holo shimmer
    // either, since it reads as another border/outline treatment. Procedural ex cards keep it.
    if (ex && !useImage) {
      const bandY = tier === "minimal" ? height * 0.1 : height * 0.11;
      const bandH = tier === "minimal" ? height * 0.75 : height * 0.32;
      shimmer.clear().roundRect(marginX, bandY, width - marginX * 2, bandH, cornerRadius * 0.5).fill({ color: 0xffffff, alpha: 0.2 });
      shimmer.visible = true;
      shimmer.filters ??= [new GlowFilter({ color: 0xffe08a, distance: 6, outerStrength: 0.6, innerStrength: 0 })];
    } else {
      shimmer.visible = false;
    }
  }

  function layoutOverlayRow(row: PIXI.Container, count: number, y: number, spacing: number) {
    row.removeChildren();
    row.y = y;
    row.x = width / 2 - ((count - 1) * spacing) / 2;
  }

  function destroyChildren(c: PIXI.Container) {
    for (const child of [...c.children]) child.destroy({ children: true });
    c.removeChildren();
  }

  // ---- Rich procedural content (only when not showing a real image) ----

  function addHeader(name: string, printedHp: number | null, tier: DetailTier) {
    const row = new PIXI.Container();
    row.y = height * 0.03;
    const nameSize = Math.max(7, width * (tier === "minimal" ? 0.13 : 0.1));
    const nameT = text(truncate(name, tier === "minimal" ? 8 : 14), { fontSize: nameSize, fill: 0xffffff, fontWeight: "700" });
    nameT.x = marginX;
    row.addChild(nameT);
    if (printedHp !== null) {
      const hpT = text(`HP${printedHp}`, { fontSize: nameSize * 0.85, fill: 0xd7dbe8, fontWeight: "600" });
      hpT.anchor.set(1, 0);
      hpT.x = width - marginX;
      row.addChild(hpT);
    }
    procInfo.addChild(row);
    return row.y + nameT.height;
  }

  function addBadges(stage: number, ex: boolean, y: number): number {
    const badgeH = Math.max(10, height * 0.055);
    const stageT = text(stageLabel(stage), { fontSize: badgeH * 0.62, fill: 0xd7dbe8, fontWeight: "600" });
    const pill = new PIXI.Graphics().roundRect(0, 0, stageT.width + 8, badgeH, badgeH / 2).fill({ color: 0x000000, alpha: 0.35 });
    stageT.x = 4;
    stageT.y = (badgeH - stageT.height) / 2;
    const badge = new PIXI.Container();
    badge.addChild(pill, stageT);
    badge.x = marginX;
    badge.y = y;
    procInfo.addChild(badge);
    if (ex) {
      const exT = text("EX", { fontSize: badgeH * 0.7, fill: 0x1a1408, fontWeight: "800" });
      const exPill = new PIXI.Graphics().roundRect(0, 0, exT.width + 10, badgeH, badgeH / 2).fill({ color: 0xffd54f });
      exT.x = 5;
      exT.y = (badgeH - exT.height) / 2;
      const exBadge = new PIXI.Container();
      exBadge.addChild(exPill, exT);
      exBadge.x = width - marginX - exPill.width;
      exBadge.y = y;
      procInfo.addChild(exBadge);
    }
    return y + badgeH;
  }

  function addAbilityLine(title: string, y: number): number {
    const size = Math.max(7, width * 0.07);
    const row = new PIXI.Container();
    row.y = y;
    const label = text("Ability", { fontSize: size * 0.85, fill: 0x8ec5ff, fontWeight: "700" });
    row.addChild(label);
    const nameT = text(truncate(title, 18), { fontSize: size, fill: 0xffffff, fontWeight: "600" });
    nameT.x = label.width + 5;
    nameT.y = -1;
    row.addChild(nameT);
    procInfo.addChild(row);
    return y + Math.max(label.height, nameT.height) + height * 0.02;
  }

  function addAttackLine(attack: Attack, y: number, compact: boolean): number {
    const radius = Math.max(2.5, width * (compact ? 0.026 : 0.032));
    const row = new PIXI.Container();
    row.y = y;
    const costs = attack.energy_required.slice(0, 4);
    const pips = pipRow(costs, radius);
    pips.x = marginX;
    row.addChild(pips);
    const pipsWidth = costs.length > 0 ? (costs.length - 1) * (radius * 2 + radius * 0.4) + radius * 2 : 0;
    const nameSize = Math.max(7, width * (compact ? 0.082 : 0.075));
    const nameT = text(truncate(attack.title, compact ? 11 : 15), { fontSize: nameSize, fill: 0xffffff, fontWeight: "600" });
    nameT.x = marginX + pipsWidth + 6;
    nameT.y = -nameT.height / 2 + radius;
    row.addChild(nameT);
    if (attack.fixed_damage > 0) {
      const dmgT = text(String(attack.fixed_damage), { fontSize: nameSize * 1.15, fill: 0xffe08a, fontWeight: "800" });
      dmgT.anchor.set(1, 0.5);
      dmgT.x = width - marginX;
      dmgT.y = radius;
      row.addChild(dmgT);
    }
    procInfo.addChild(row);
    return y + radius * 2 + height * 0.045;
  }

  function addFooter(weakness: EnergyType | null, retreatCost: EnergyType[], y: number) {
    const radius = Math.max(2.5, width * 0.028);
    if (weakness) {
      const row = new PIXI.Container();
      row.y = y;
      const label = text("Weak", { fontSize: Math.max(7, width * 0.065), fill: 0x9aa0b4 });
      row.addChild(label);
      const pip = pipRow([weakness], radius);
      pip.x = label.width + 6 + radius;
      row.addChild(pip);
      procInfo.addChild(row);
    }
    if (retreatCost.length > 0) {
      const row = new PIXI.Container();
      row.y = y;
      const label = text("Retreat", { fontSize: Math.max(7, width * 0.065), fill: 0x9aa0b4 });
      label.anchor.set(1, 0);
      const pips = pipRow(retreatCost.slice(0, 5), radius, 0x9aa0b4);
      const pipsWidth = (retreatCost.length - 1) * (radius * 2 + radius * 0.4) + radius * 2;
      pips.x = width - marginX - pipsWidth + radius;
      label.x = pips.x - radius - 6;
      row.addChild(pips, label);
      procInfo.addChild(row);
    }
  }

  function addTrainerBody(subtype: string, effect: string, y: number, tier: DetailTier) {
    const badgeH = Math.max(10, height * 0.055);
    const subT = text(subtype, { fontSize: badgeH * 0.62, fill: 0xd7dbe8, fontWeight: "600" });
    const pill = new PIXI.Graphics().roundRect(0, 0, subT.width + 8, badgeH, badgeH / 2).fill({ color: 0x000000, alpha: 0.35 });
    subT.x = 4;
    subT.y = (badgeH - subT.height) / 2;
    const badge = new PIXI.Container();
    badge.addChild(pill, subT);
    badge.x = marginX;
    badge.y = y;
    procInfo.addChild(badge);

    if (tier === "full" || tier === "compact") {
      const effectT = text(effect, {
        fontSize: Math.max(7, width * 0.068),
        fill: 0xc7cbe0,
        wordWrap: true,
        wordWrapWidth: width - marginX * 2,
        lineHeight: Math.max(9, width * 0.09),
      });
      effectT.x = marginX;
      effectT.y = y + badgeH + height * 0.02;
      // Clip long effect text to the space above the overlay strip rather than overflowing it.
      const maxH = overlayTop - effectT.y;
      if (effectT.height > maxH) {
        const mask = new PIXI.Graphics().rect(0, 0, width, maxH).fill(0xffffff);
        mask.x = effectT.x;
        mask.y = effectT.y;
        procInfo.addChild(mask);
        effectT.mask = mask;
      }
      procInfo.addChild(effectT);
    }
  }

  function buildProceduralInfo(card: Card, tier: DetailTier) {
    destroyChildren(procInfo);
    if (tier === "minimal") {
      if (isPokemonCard(card)) {
        addHeader(card.Pokemon.name, card.Pokemon.hp, tier);
      } else {
        addHeader(card.Trainer.name, null, tier);
      }
      return;
    }

    if (isPokemonCard(card)) {
      const p = card.Pokemon;
      let y = addHeader(p.name, p.hp, tier);
      y = addBadges(p.stage, isExCard(card), y + height * 0.02);
      y = height * 0.47; // below the boxed art band, regardless of how tall the badge row was
      if (tier === "full" && p.ability) {
        y = addAbilityLine(p.ability.title, y);
      }
      const maxAttacks = tier === "full" ? 2 : 1;
      for (const attack of p.attacks.slice(0, maxAttacks)) {
        if (y > overlayTop - height * 0.06) break;
        y = addAttackLine(attack, y, tier === "compact");
      }
      addFooter(p.weakness, p.retreat_cost, Math.min(y + height * 0.01, overlayTop - height * 0.04));
    } else {
      const tCard = card.Trainer;
      addHeader(tCard.name, null, tier);
      addTrainerBody(tCard.trainer_card_type, tCard.effect, height * 0.16, tier);
    }
  }

  let lastCard: Card | undefined;
  let unsubscribeImage: (() => void) | null = null;

  function clearImageSubscription() {
    unsubscribeImage?.();
    unsubscribeImage = null;
  }

  const visual: CardVisual = {
    container,
    width,
    height,
    hpBarGhost,
    hpBarFg,
    hpBarInnerWidth: stripBarW,
    update(card, slot, faceDown = false) {
      lastCard = card;
      if (!card) {
        container.visible = false;
        clearImageSubscription();
        return;
      }
      container.visible = true;

      if (faceDown) {
        clearImageSubscription();
        destroyChildren(procInfo);
        redrawStatic(null, false, false, tierFor(height));
        hpNumber.text = "";
        energyRow.removeChildren();
        statusRow.removeChildren();
        toolBadge.text = "";
        hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = false;
        return;
      }

      const id = cardId(card);
      const type = cardEnergyType(card);
      const ex = isExCard(card);
      const tier = tierFor(height);
      const texture = images?.getTexture(id);

      if (texture) {
        clearImageSubscription();
        imageSprite.texture = texture;
        const fit = fitContain(width, height, texture.width, texture.height);
        imageSprite.width = fit.w;
        imageSprite.height = fit.h;
      } else if (images?.enabled) {
        unsubscribeImage = images.onSettled(id, () => {
          // Guard against a stale async callback: only redraw if we're still showing this card.
          if (lastCard && cardId(lastCard) === id) {
            visual.update(lastCard, slot, faceDown);
          }
        });
      }
      redrawStatic(type, ex, !!texture, tier);

      if (texture) {
        destroyChildren(procInfo);
      } else {
        buildProceduralInfo(card, tier);
      }

      if (!slot) {
        // A hand card: face only, no in-play stats.
        hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = false;
        hpNumber.text = "";
        energyRow.removeChildren();
        statusRow.removeChildren();
        toolBadge.text = "";
        return;
      }

      hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = true;
      hpBarBg.clear().roundRect(0, stripMidY - stripBarH / 2, stripBarW, stripBarH, stripBarH / 2).fill({ color: 0x2a2e3f });
      const fraction = slot.max_hp > 0 ? Math.max(0, Math.min(1, slot.hp / slot.max_hp)) : 0;
      visual.setHpBar(fraction, "ghost");
      visual.setHpBar(fraction, "fg");

      // The HP number sits in the strip above the card, to the right of the bar — this way it
      // reads the same whether the face below it is a procedural card or a real image, and never
      // fights the procedural header's printed HP for space (see the strip's own header comment for
      // why it's reserved space in `layout.ts` rather than floating over the card).
      hpNumber.text = `${slot.hp}`;
      hpNumber.style.fill = fraction > 0.5 ? 0x66d97a : fraction > 0.2 ? 0xffc94d : 0xff5c5c;

      const energyCount = slot.energy.length;
      layoutOverlayRow(energyRow, Math.max(energyCount, 1), energyY, Math.max(9, width * 0.13));
      slot.energy.slice(0, 8).forEach((e, i) => {
        const pip = new PIXI.Graphics()
          .circle(i * Math.max(9, width * 0.13), 0, Math.max(3.5, width * 0.05))
          .fill({ color: energyColor(e) })
          .stroke({ width: 1, color: 0x0b0d14 });
        energyRow.addChild(pip);
      });

      // Status conditions: round badges hugging the card's right edge, a bit above its vertical
      // middle (like the official app), stacked downward when a Pokemon has more than one.
      statusRow.removeChildren();
      const badgeR = Math.max(9, width * 0.1);
      statusRow.x = width - badgeR * 0.35;
      statusRow.y = height * 0.36;
      slot.status.forEach((s, i) => {
        const { glyph, color } = STATUS_GLYPH[s];
        const badge = new PIXI.Container();
        const disc = new PIXI.Graphics().circle(0, 0, badgeR).fill({ color }).stroke({ width: Math.max(1.5, badgeR * 0.14), color: 0xffffff, alpha: 0.9 });
        const label = text(glyph, { fontSize: badgeR * 0.82, fill: 0x111111, fontWeight: "800" });
        label.anchor.set(0.5);
        badge.addChild(disc, label);
        badge.y = i * badgeR * 2.25;
        statusRow.addChild(badge);
      });

      toolBadge.text = slot.tools.length > 0 ? `T×${slot.tools.length}` : "";
      toolBadge.x = 4;
      toolBadge.y = 4;
    },
    setTools(tools) {
      for (const visual of toolVisuals) visual.destroy();
      toolVisuals = [];
      for (const child of toolLayer.removeChildren()) child.destroy({ children: true });
      const peek = height * TOOL_PEEK_FRACTION;
      // A Pokemon holds one Tool, and the board layout reserves room for exactly one sliver.
      tools.slice(0, 1).forEach((tool, i) => {
        const visual = createCardVisual(width, height, { images });
        visual.update(tool, undefined);
        visual.container.y = peek * (i + 1);
        toolLayer.addChildAt(visual.container, 0);
        toolVisuals.push(visual);
      });
      tools.slice(0, 1).forEach((tool, i) => {
        const label = text(cardName(tool), {
          fontSize: Math.max(8, Math.round(peek * 0.6)),
          fill: 0xffffff,
          fontWeight: "700",
          stroke: { color: 0x0b0d14, width: 3 },
        });
        label.anchor.set(0.5);
        label.x = width / 2;
        label.y = height + peek * (i + 0.5);
        toolLayer.addChild(label);
      });
    },
    setHpBar(fraction, which) {
      const bar = which === "fg" ? hpBarFg : hpBarGhost;
      const color = which === "fg" ? (fraction > 0.5 ? 0x66d97a : fraction > 0.2 ? 0xffc94d : 0xff5c5c) : 0xff5c5c;
      bar.clear();
      if (fraction > 0) {
        bar.roundRect(0, stripMidY - stripBarH / 2, Math.max(2, stripBarW * fraction), stripBarH, stripBarH / 2).fill({ color, alpha: which === "ghost" ? 0.55 : 1 });
      }
    },
    setBaseRotation(rotation) {
      baseRotation = rotation;
      container.rotation = rotation;
    },
    destroy() {
      clearImageSubscription();
      container.destroy({ children: true });
    },
  };

  return visual;
}

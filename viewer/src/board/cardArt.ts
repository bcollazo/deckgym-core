// Card rendering: procedurally-drawn by default (no card art ships with the viewer — see the
// plan's "No card art" decision), or, when a `CardImageStore` is supplied and has a texture for a
// card's id, a real image with the same overlays (see `docs/replay-viewer-plan.md`'s "Round 2"
// section on "bring your own assets"). Every card — procedural or image — keeps the physical
// card's 63:88 aspect ratio (`layout.ts`'s `CARD_ASPECT`).

import * as PIXI from "pixi.js";
import { GlowFilter } from "pixi-filters";
import { energyColor } from "../anim/colors";
import { gsap } from "../anim/gsap";
import type { Card, EnergyType, SlotView, StatusCondition } from "../types/replay";
import { cardEnergyType, cardId, cardName, isExCard } from "../types/replay";
import type { CardImageStore } from "./cardImages";

const STATUS_GLYPH: Record<StatusCondition, { glyph: string; color: number }> = {
  Poisoned: { glyph: "PSN", color: 0xab47bc },
  Paralyzed: { glyph: "PRZ", color: 0xffd54f },
  Asleep: { glyph: "SLP", color: 0x90a4ae },
  Burned: { glyph: "BRN", color: 0xff7043 },
  Confused: { glyph: "CNF", color: 0xff8a80 },
};

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
  destroy(): void;
}

export interface CardVisualOptions {
  /** Shared image lookup/loader; omit to always use the procedural face. */
  images?: CardImageStore;
  /** Subtle hover tilt (skew + lift). Off by default for throwaway fx sprites that don't want
   * pointer events fighting an in-flight tween. */
  interactive?: boolean;
}

function truncate(text: string, max: number): string {
  return text.length > max ? `${text.slice(0, max - 1)}…` : text;
}

export function createCardVisual(width: number, height: number, opts: CardVisualOptions = {}): CardVisual {
  const { images, interactive = false } = opts;

  const container = new PIXI.Container();
  const bg = new PIXI.Graphics();
  const art = new PIXI.Graphics();
  const imageSprite = new PIXI.Sprite();
  imageSprite.visible = false;
  imageSprite.width = width;
  imageSprite.height = height;
  const frame = new PIXI.Graphics();
  // Scaled to the card's own size: a full-size board card gets normal-ish text, but a tiny hand
  // card (see layout.ts's OPP_HAND_SIZE) would render illegible mush at that size, so it shrinks
  // the font instead — and `update()` hides the name entirely below a legibility floor.
  const nameFontSize = Math.max(7, Math.min(11, Math.round(width / 8)));
  const nameText = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: nameFontSize, fill: 0xffffff, fontWeight: "700" },
  });
  const hpNumber = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: 12, fill: 0x66d97a, fontWeight: "800", stroke: { color: 0x0b0d14, width: 3 } },
  });
  hpNumber.anchor.set(1, 1);
  const hpBarBg = new PIXI.Graphics();
  const hpBarGhost = new PIXI.Graphics();
  const hpBarFg = new PIXI.Graphics();
  const energyRow = new PIXI.Container();
  const statusRow = new PIXI.Container();
  const toolBadge = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: 9, fill: 0xffe08a, fontWeight: "700" },
  });
  const shimmer = new PIXI.Graphics();
  shimmer.visible = false;

  container.addChild(
    bg,
    art,
    imageSprite,
    frame,
    shimmer,
    nameText,
    energyRow,
    statusRow,
    toolBadge,
    hpBarBg,
    hpBarGhost,
    hpBarFg,
    hpNumber,
  );

  if (interactive) {
    container.eventMode = "static";
    container.cursor = "pointer";
    const baseScale = { x: 1, y: 1 };
    container.on("pointerover", () => {
      gsap.to(container, { rotation: -0.045, duration: 0.18, ease: "power2.out" });
      gsap.to(container.scale, { x: baseScale.x * 1.06, y: baseScale.y * 1.06, duration: 0.18, ease: "power2.out" });
      container.zIndex = 1000;
    });
    container.on("pointerout", () => {
      gsap.to(container, { rotation: 0, duration: 0.22, ease: "power2.out" });
      gsap.to(container.scale, { x: baseScale.x, y: baseScale.y, duration: 0.22, ease: "power2.out" });
      container.zIndex = 0;
    });
  }

  // Proportional margins (fractions of the card's own size) rather than fixed pixel offsets, so
  // this scales cleanly from a ~35px opponent hand card up to a ~180px active card — a fixed-pixel
  // "reserved band" was the earlier bug that made small hand cards render as empty outlines
  // (the art rect's height went negative once the card was smaller than the reserved bands).
  const marginX = width * 0.07;
  const cornerRadius = width * 0.1;
  const hpBarH = Math.max(3, height * 0.045);
  const hpBarY = height - height * 0.09;
  const hpBarWidth = width - marginX * 2;
  const energyY = hpBarY - height * 0.11;
  const statusY = energyY - height * 0.12;

  function redrawStatic(type: EnergyType | null, ex: boolean, useImage: boolean) {
    const color = energyColor(type);
    bg.clear();
    art.clear();
    imageSprite.visible = useImage;

    if (!useImage) {
      bg.roundRect(0, 0, width, height, cornerRadius).fill({ color: 0x1a1d29 });
      const artMargin = width * 0.065;
      const artY = height * 0.12;
      const artH = height - artY - height * 0.06;
      const artW = width - artMargin * 2;
      art.roundRect(artMargin, artY, artW, artH, cornerRadius * 0.6).fill({ color });
      // A soft vertical gradient "art" look, faked with a couple of translucent overlays (Pixi v8
      // has no built-in gradient fill helper here without a canvas texture, so this keeps it cheap).
      art.roundRect(artMargin, artY, artW, artH / 2, cornerRadius * 0.6).fill({ color: 0xffffff, alpha: 0.12 });
      art.roundRect(artMargin, artY + artH * 0.6, artW, artH * 0.4, cornerRadius * 0.6).fill({ color: 0x000000, alpha: 0.18 });
    }

    frame
      .clear()
      .roundRect(0, 0, width, height, cornerRadius)
      .stroke({ width: ex ? 2.5 : 1.5, color: ex ? 0xffd54f : useImage ? 0x30354a : color, alpha: 0.9 });

    if (ex) {
      shimmer.clear().roundRect(marginX, height * 0.1, width - marginX * 2, height * 0.6, cornerRadius * 0.6).fill({ color: 0xffffff, alpha: 0.22 });
      shimmer.visible = true;
      shimmer.filters ??= [new GlowFilter({ color: 0xffe08a, distance: 6, outerStrength: 0.6, innerStrength: 0 })];
    } else {
      shimmer.visible = false;
    }
  }

  function layoutRow(row: PIXI.Container, count: number, y: number, spacing: number) {
    row.removeChildren();
    row.y = y;
    row.x = width / 2 - ((count - 1) * spacing) / 2;
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
    hpBarInnerWidth: hpBarWidth,
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
        redrawStatic(null, false, false);
        nameText.text = "";
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
      const texture = images?.getTexture(id);

      if (texture) {
        clearImageSubscription();
        imageSprite.texture = texture;
      } else {
        if (images?.enabled) {
          unsubscribeImage = images.onSettled(id, () => {
            // Guard against a stale async callback: only redraw if we're still showing this card.
            if (lastCard && cardId(lastCard) === id) {
              visual.update(lastCard, slot, faceDown);
            }
          });
        }
      }
      redrawStatic(type, ex, !!texture);

      const showName = !texture && width >= 40;
      nameText.visible = showName;
      nameText.text = showName ? truncate(cardName(card), Math.floor(width / 6)) : "";
      nameText.x = width / 2 - nameText.width / 2;
      nameText.y = height * 0.02;

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
      hpBarBg.clear().roundRect(marginX, hpBarY, hpBarWidth, hpBarH, hpBarH / 2).fill({ color: 0x2a2e3f });
      const fraction = slot.max_hp > 0 ? Math.max(0, Math.min(1, slot.hp / slot.max_hp)) : 0;
      visual.setHpBar(fraction, "ghost");
      visual.setHpBar(fraction, "fg");

      // The HP number sits just above the card's own top-right corner, not on the face — this way
      // it reads the same whether the face below it is a procedural card or a real image.
      hpNumber.text = `${slot.hp}`;
      hpNumber.style.fill = fraction > 0.5 ? 0x66d97a : fraction > 0.2 ? 0xffc94d : 0xff5c5c;
      hpNumber.x = width;
      hpNumber.y = -2;

      const energyCount = slot.energy.length;
      layoutRow(energyRow, Math.max(energyCount, 1), energyY, Math.max(9, width * 0.13));
      slot.energy.slice(0, 8).forEach((e, i) => {
        const pip = new PIXI.Graphics()
          .circle(i * Math.max(9, width * 0.13), 0, Math.max(3.5, width * 0.05))
          .fill({ color: energyColor(e) })
          .stroke({ width: 1, color: 0x0b0d14 });
        energyRow.addChild(pip);
      });

      const statuses = slot.status.slice(0, 2);
      layoutRow(statusRow, Math.max(statuses.length, 1), statusY, width * 0.32);
      statuses.forEach((s, i) => {
        const { glyph, color } = STATUS_GLYPH[s];
        const badge = new PIXI.Container();
        const pill = new PIXI.Graphics().roundRect(-12, -6, 24, 12, 6).fill({ color, alpha: 0.85 });
        const label = new PIXI.Text({
          text: glyph,
          style: { fontFamily: "Inter, sans-serif", fontSize: 7, fill: 0x111111, fontWeight: "700" },
        });
        label.x = -label.width / 2;
        label.y = -label.height / 2;
        badge.addChild(pill, label);
        badge.x = i * width * 0.32;
        statusRow.addChild(badge);
      });

      toolBadge.text = slot.tools.length > 0 ? `T×${slot.tools.length}` : "";
      toolBadge.x = 4;
      toolBadge.y = 4;
    },
    setHpBar(fraction, which) {
      const bar = which === "fg" ? hpBarFg : hpBarGhost;
      const color = which === "fg" ? (fraction > 0.5 ? 0x66d97a : fraction > 0.2 ? 0xffc94d : 0xff5c5c) : 0xff5c5c;
      bar.clear();
      if (fraction > 0) {
        bar.roundRect(marginX, hpBarY, Math.max(2, hpBarWidth * fraction), hpBarH, hpBarH / 2).fill({ color, alpha: which === "ghost" ? 0.55 : 1 });
      }
    },
    destroy() {
      clearImageSubscription();
      container.destroy({ children: true });
    },
  };

  return visual;
}

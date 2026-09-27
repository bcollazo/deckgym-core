// Procedural card rendering: no card art ships with the viewer (see the plan's "Decisions"), so
// every card is a type-colored frame with a name, HP bar, energy pips, status badges and a
// gradient-look "art" placeholder, drawn with Pixi Graphics/Text.

import * as PIXI from "pixi.js";
import { GlowFilter } from "pixi-filters";
import { energyColor } from "../anim/colors";
import type { Card, EnergyType, SlotView, StatusCondition } from "../types/replay";
import { cardEnergyType, cardName, isExCard } from "../types/replay";

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

function truncate(text: string, max: number): string {
  return text.length > max ? `${text.slice(0, max - 1)}…` : text;
}

export function createCardVisual(width: number, height: number): CardVisual {
  const container = new PIXI.Container();
  const bg = new PIXI.Graphics();
  const art = new PIXI.Graphics();
  const frame = new PIXI.Graphics();
  // Scaled to the card's own size: a full-size board card gets normal 11px text, but a tiny hand
  // card (see layout.ts's HAND_CARD_W/H) would render illegible mush at that size, so it shrinks
  // the font instead — and `update()` hides the name entirely below a legibility floor.
  const nameFontSize = Math.max(7, Math.min(11, Math.round(width / 8)));
  const nameText = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: nameFontSize, fill: 0xffffff, fontWeight: "700" },
  });
  const hpText = new PIXI.Text({
    text: "",
    style: { fontFamily: "Inter, sans-serif", fontSize: 10, fill: 0xffffff, fontWeight: "600" },
  });
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

  container.addChild(bg, art, frame, shimmer, nameText, energyRow, statusRow, toolBadge, hpBarBg, hpBarGhost, hpBarFg, hpText);

  const hpBarWidth = width - 12;
  const hpBarY = height - 16;

  function redrawStatic(type: EnergyType | null, ex: boolean) {
    const color = energyColor(type);
    bg.clear().roundRect(0, 0, width, height, 8).fill({ color: 0x1a1d29 }).stroke({ width: 2, color });

    art.clear();
    const artY = 14;
    const artH = height - 44;
    art
      .roundRect(6, artY, width - 12, artH, 5)
      .fill({ color })
      .stroke({ width: 0 });
    // A soft vertical gradient "art" look, faked with a couple of translucent overlays (Pixi v8 has
    // no built-in gradient fill helper here without a canvas texture, so this keeps it cheap).
    art
      .roundRect(6, artY, width - 12, artH / 2, 5)
      .fill({ color: 0xffffff, alpha: 0.12 });
    art
      .roundRect(6, artY + artH * 0.6, width - 12, artH * 0.4, 5)
      .fill({ color: 0x000000, alpha: 0.18 });

    frame.clear().roundRect(0, 0, width, height, 8).stroke({ width: ex ? 2.5 : 1.5, color: ex ? 0xffd54f : color, alpha: 0.9 });

    if (ex) {
      shimmer.clear().roundRect(6, artY, width - 12, artH, 5).fill({ color: 0xffffff, alpha: 0.25 });
      shimmer.visible = true;
      if (!shimmer.filters) {
        shimmer.filters = [new GlowFilter({ color: 0xffe08a, distance: 6, outerStrength: 0.6, innerStrength: 0 })];
      }
    } else {
      shimmer.visible = false;
    }
  }

  function layoutRow(row: PIXI.Container, count: number, y: number, spacing: number) {
    row.removeChildren();
    row.y = y;
    row.x = width / 2 - ((count - 1) * spacing) / 2;
  }

  const visual: CardVisual = {
    container,
    width,
    height,
    hpBarGhost,
    hpBarFg,
    hpBarInnerWidth: hpBarWidth,
    update(card, slot, faceDown = false) {
      if (!card) {
        container.visible = false;
        return;
      }
      container.visible = true;

      if (faceDown) {
        redrawStatic(null, false);
        nameText.text = "";
        hpText.text = "";
        energyRow.removeChildren();
        statusRow.removeChildren();
        toolBadge.text = "";
        hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = false;
        return;
      }

      const type = cardEnergyType(card);
      const ex = isExCard(card);
      redrawStatic(type, ex);

      nameText.visible = width >= 40;
      nameText.text = nameText.visible ? truncate(cardName(card), Math.floor(width / 6)) : "";
      nameText.x = width / 2 - nameText.width / 2;
      nameText.y = 2;

      if (!slot) {
        // A hand card: name only, no in-play stats.
        hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = false;
        hpText.text = "";
        energyRow.removeChildren();
        statusRow.removeChildren();
        toolBadge.text = "";
        return;
      }

      hpBarBg.visible = hpBarFg.visible = hpBarGhost.visible = true;
      hpBarBg.clear().roundRect(6, hpBarY, hpBarWidth, 6, 3).fill({ color: 0x2a2e3f });
      const fraction = slot.max_hp > 0 ? Math.max(0, Math.min(1, slot.hp / slot.max_hp)) : 0;
      visual.setHpBar(fraction, "ghost");
      visual.setHpBar(fraction, "fg");
      hpText.text = `${slot.hp}/${slot.max_hp}`;
      hpText.x = width / 2 - hpText.width / 2;
      hpText.y = hpBarY - 12;

      const energyCount = slot.energy.length;
      layoutRow(energyRow, Math.max(energyCount, 1), height - 30, 11);
      slot.energy.slice(0, 8).forEach((e, i) => {
        const pip = new PIXI.Graphics()
          .circle(i * 11, 0, 4.5)
          .fill({ color: energyColor(e) })
          .stroke({ width: 1, color: 0x0b0d14 });
        energyRow.addChild(pip);
      });

      const statuses = slot.status.slice(0, 2);
      layoutRow(statusRow, Math.max(statuses.length, 1), height - 42, 26);
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
        badge.x = i * 26;
        statusRow.addChild(badge);
      });

      toolBadge.text = slot.tools.length > 0 ? `T×${slot.tools.length}` : "";
      toolBadge.x = width - toolBadge.width - 4;
      toolBadge.y = height - 44;
    },
    setHpBar(fraction, which) {
      const bar = which === "fg" ? hpBarFg : hpBarGhost;
      const color = which === "fg" ? (fraction > 0.5 ? 0x66d97a : fraction > 0.2 ? 0xffc94d : 0xff5c5c) : 0xff5c5c;
      bar.clear();
      if (fraction > 0) {
        bar.roundRect(6, hpBarY, Math.max(2, hpBarWidth * fraction), 6, 3).fill({ color, alpha: which === "ghost" ? 0.55 : 1 });
      }
    },
    destroy() {
      container.destroy({ children: true });
    },
  };

  return visual;
}

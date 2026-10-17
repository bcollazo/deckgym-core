// Round 4: HP numbers/bars used to float above each card with no reserved space, so they landed on
// neighboring rows, the divider or the turn banner (see the plan doc's "Round 4" notes). This test
// asserts the fix holds structurally: every in-play slot's card rect and its HP strip rect (see
// `slotRects`) never intersects any other slot's card/strip rects, nor the divider band (which also
// bounds the turn banner — see `scene.ts`'s `bannerBg`, sized well within `DIVIDER_H`).

import { describe, expect, it } from "vitest";
import { BENCH_SIZE, BOARD_HEIGHT, dividerBandRect, type Rect, slotRects, STADIUM_SIZE, TOOL_PEEK_FRACTION } from "./layout";

function overlaps(a: Rect, b: Rect): boolean {
  return a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;
}

interface NamedRect {
  name: string;
  rect: Rect;
}

function allRects(): NamedRect[] {
  const rects: NamedRect[] = [];
  for (const player of [0, 1]) {
    for (let slot = 0; slot < 4; slot++) {
      const { card, strip } = slotRects(player, slot);
      rects.push({ name: `p${player}-slot${slot}-card`, rect: card });
      rects.push({ name: `p${player}-slot${slot}-strip`, rect: strip });
    }
  }
  rects.push({ name: "divider-band", rect: dividerBandRect() });
  return rects;
}

describe("board layout: HP strip reservation", () => {
  it("gives every slot's card and strip rects a positive size", () => {
    for (const { name, rect } of allRects()) {
      expect(rect.w, `${name}.w`).toBeGreaterThan(0);
      expect(rect.h, `${name}.h`).toBeGreaterThan(0);
    }
  });

  it("sits each slot's strip directly above its own card, with no gap-eating overlap", () => {
    for (const player of [0, 1]) {
      for (let slot = 0; slot < 4; slot++) {
        const { card, strip } = slotRects(player, slot);
        expect(strip.y + strip.h, `p${player}-slot${slot} strip bottom vs card top`).toBeLessThanOrEqual(card.y);
        // Same horizontal span (the strip spans exactly the card's width, centered the same).
        expect(strip.x).toBeCloseTo(card.x, 5);
        expect(strip.w).toBeCloseTo(card.w, 5);
      }
    }
  });

  it("never intersects any two rects belonging to different slots (or the divider band)", () => {
    const rects = allRects();
    const offenders: string[] = [];
    for (let i = 0; i < rects.length; i++) {
      for (let j = i + 1; j < rects.length; j++) {
        const a = rects[i];
        const b = rects[j];
        // A slot's own card/strip pair is checked separately above (they're meant to be adjacent).
        const sameSlot = a.name.split("-").slice(0, 2).join("-") === b.name.split("-").slice(0, 2).join("-");
        if (sameSlot) continue;
        if (overlaps(a.rect, b.rect)) offenders.push(`${a.name} <-> ${b.name}`);
      }
    }
    expect(offenders).toEqual([]);
  });
});

describe("board layout: tool peek and stadium", () => {
  // A tool card sits behind its Pokemon and shows a sliver below it; that sliver must not touch
  // any other slot's card/strip, the divider band, or the hand rows' cards.
  function peekRect(player: number, slot: number): Rect {
    const { card } = slotRects(player, slot);
    return { x: card.x, y: card.y + card.h, w: card.w, h: card.h * TOOL_PEEK_FRACTION };
  }

  it("keeps every slot's tool sliver clear of every other slot and the divider", () => {
    const rects = allRects();
    const offenders: string[] = [];
    for (const player of [0, 1]) {
      for (let slot = 0; slot < 4; slot++) {
        const peek = peekRect(player, slot);
        for (const { name, rect } of rects) {
          if (name.startsWith(`p${player}-slot${slot}-`)) continue;
          if (overlaps(peek, rect)) offenders.push(`p${player}-slot${slot}-peek <-> ${name}`);
        }
      }
    }
    expect(offenders).toEqual([]);
  });

  it("keeps tool slivers on the board", () => {
    for (const player of [0, 1]) {
      for (let slot = 0; slot < 4; slot++) {
        const peek = peekRect(player, slot);
        expect(peek.y + peek.h, `p${player}-slot${slot}`).toBeLessThanOrEqual(BOARD_HEIGHT);
      }
    }
  });

  it("draws the stadium as large as a bench card", () => {
    expect(STADIUM_SIZE).toEqual(BENCH_SIZE);
  });
});

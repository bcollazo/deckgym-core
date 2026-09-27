import { describe, expect, it } from "vitest";
import { fitContain } from "./imageFit";

describe("fitContain", () => {
  it("keeps the exact size when the box already matches the texture's aspect ratio", () => {
    // deckgym's real card images are 367x512 (aspect 0.716796875).
    expect(fitContain(367, 512, 367, 512)).toEqual({ w: 367, h: 512 });
    // A box already at the same aspect ratio, just scaled down, should hit the box exactly.
    const box = { w: 108, h: (108 / 367) * 512 };
    const result = fitContain(box.w, box.h, 367, 512);
    expect(result.w).toBeCloseTo(box.w, 5);
    expect(result.h).toBeCloseTo(box.h, 5);
  });

  it("letterboxes a wider-than-box texture by shrinking to the box's width", () => {
    // A box taller than the texture's own aspect ratio implies width is the binding constraint.
    const result = fitContain(100, 200, 367, 512);
    expect(result.w).toBeCloseTo(100, 5);
    expect(result.h).toBeCloseTo((100 / 367) * 512, 5);
    expect(result.h).toBeLessThan(200); // never exceeds the box
  });

  it("letterboxes a taller-than-box texture by shrinking to the box's height", () => {
    // A very wide, short box implies height is the binding constraint.
    const result = fitContain(400, 100, 367, 512);
    expect(result.h).toBeCloseTo(100, 5);
    expect(result.w).toBeCloseTo((100 / 512) * 367, 5);
    expect(result.w).toBeLessThan(400);
  });

  it("never distorts the texture's own aspect ratio", () => {
    for (const [boxW, boxH] of [
      [50, 400],
      [400, 50],
      [92, 128],
      [640, 1072],
    ]) {
      const { w, h } = fitContain(boxW, boxH, 367, 512);
      expect(w / h).toBeCloseTo(367 / 512, 6);
    }
  });

  it("falls back to the box size for degenerate input instead of dividing by zero", () => {
    expect(fitContain(100, 50, 0, 0)).toEqual({ w: 100, h: 50 });
    expect(fitContain(0, 0, 367, 512)).toEqual({ w: 0, h: 0 });
  });
});

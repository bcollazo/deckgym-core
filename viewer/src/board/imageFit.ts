// Pure "contain" fit math, kept dependency-free (no Pixi import) so it's cheaply unit-testable —
// see cardArt.ts, which uses this to size a real card image inside its box without distorting it.

export interface Size {
  w: number;
  h: number;
}

/**
 * Scales `(texW, texH)` uniformly (by `min(boxW/texW, boxH/texH)`) so it fits entirely inside
 * `(boxW, boxH)` without distortion — the same semantics as CSS `object-fit: contain`. Falls back
 * to the box's own size for degenerate (zero/negative) input rather than dividing by zero.
 */
export function fitContain(boxW: number, boxH: number, texW: number, texH: number): Size {
  if (boxW <= 0 || boxH <= 0 || texW <= 0 || texH <= 0) {
    return { w: Math.max(0, boxW), h: Math.max(0, boxH) };
  }
  const scale = Math.min(boxW / texW, boxH / texH);
  return { w: texW * scale, h: texH * scale };
}

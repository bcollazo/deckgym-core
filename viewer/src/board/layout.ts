// Board geometry. Everything is laid out in a fixed logical 960x640 space; the canvas is scaled
// to fit its container via CSS, so these numbers never need to react to window size.

export const BOARD_WIDTH = 960;
export const BOARD_HEIGHT = 640;

export const CARD_W = 92;
export const CARD_H = 128;
export const ACTIVE_CARD_W = 108;
export const ACTIVE_CARD_H = 150;

const BENCH_GAP = 16;
// Small enough that up to a full 10-card hand (the engine's hand-size cap) never overlaps — see
// handCardPos's spacing below. Below cardArt.ts's 40px legibility floor, so hand cards render as a
// plain type-colored frame with no name text (matches the plan mock's "hand ▢▢▢▢").
const HAND_CARD_W = 22;
const HAND_CARD_H = 30;
const HAND_SPREAD = 300;

const ACTIVE_X = 210;
const BENCH_GROUP_CENTER_X = 620;

// Each player gets two HUD rows at the outer edge of their half: an "info" row (name, energy
// zone, points, deck/discard counts) and, one step further out, the hand row. Player 0's rows sit
// at the very bottom, player 1's (mirrored) at the very top.
const INFO_ROW_INSET = 16;
const HAND_ROW_INSET = 46;

function infoRowY(player: number): number {
  return player === 0 ? BOARD_HEIGHT - INFO_ROW_INSET : INFO_ROW_INSET;
}

function handRowY(player: number): number {
  return player === 0 ? BOARD_HEIGHT - HAND_ROW_INSET : HAND_ROW_INSET;
}

/** Center of the active spot for a player. Player 0 sits in the bottom half, player 1 (mirrored)
 * in the top half. Active sits to the left, clear of the bench group, so it reads as visually
 * distinct rather than "a fourth bench slot". */
export function activePos(player: number): { x: number; y: number } {
  const y = player === 0 ? BOARD_HEIGHT / 2 + 140 : BOARD_HEIGHT / 2 - 140;
  return { x: ACTIVE_X, y };
}

/** Center of bench slot `i` (0, 1, 2) for a player. */
export function benchPos(player: number, i: number): { x: number; y: number } {
  const totalWidth = 3 * CARD_W + 2 * BENCH_GAP;
  const startX = BENCH_GROUP_CENTER_X - totalWidth / 2;
  const x = startX + i * (CARD_W + BENCH_GAP) + CARD_W / 2;
  const y = player === 0 ? BOARD_HEIGHT / 2 + 140 : BOARD_HEIGHT / 2 - 140;
  return { x, y };
}

/** Center for in-play slot index (0 = active, 1..3 = bench). */
export function slotPos(player: number, slot: number): { x: number; y: number } {
  return slot === 0 ? activePos(player) : benchPos(player, slot - 1);
}

export function slotSize(slot: number): { w: number; h: number } {
  return slot === 0 ? { w: ACTIVE_CARD_W, h: ACTIVE_CARD_H } : { w: CARD_W, h: CARD_H };
}

/** Anchor for a card animating out of/into a player's hand — the middle of their hand row. */
export function handAnchor(player: number): { x: number; y: number } {
  return { x: BOARD_WIDTH / 2, y: handRowY(player) };
}

export function handCardPos(player: number, index: number, count: number): { x: number; y: number } {
  const anchor = handAnchor(player);
  const spacing = Math.min(HAND_CARD_W + 2, HAND_SPREAD / Math.max(count, 1));
  const startX = anchor.x - ((count - 1) * spacing) / 2;
  return { x: startX + index * spacing, y: anchor.y };
}

export function nameAnchor(player: number): { x: number; y: number } {
  return { x: 16, y: infoRowY(player) };
}

export function energyZoneAnchor(player: number): { x: number; y: number } {
  return { x: 210, y: infoRowY(player) };
}

export function pointsAnchor(player: number): { x: number; y: number } {
  return { x: 270, y: infoRowY(player) };
}

export function deckAnchor(player: number): { x: number; y: number } {
  return { x: BOARD_WIDTH - 130, y: infoRowY(player) };
}

export function discardAnchor(player: number): { x: number; y: number } {
  return { x: BOARD_WIDTH - 60, y: infoRowY(player) };
}

export function stadiumPos(): { x: number; y: number } {
  return { x: BOARD_WIDTH / 2, y: BOARD_HEIGHT / 2 };
}

export { HAND_CARD_W, HAND_CARD_H };

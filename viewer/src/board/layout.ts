// Board geometry: a portrait column (mirroring the official app's spatial layout — see the plan's
// "Round 2" notes) laid out in a fixed logical space; the canvas is scaled to fit its container via
// CSS, so these numbers never need to react to window size.

/** Every card, procedural or a real image, keeps this aspect ratio (a physical TCG card is 63x88mm). */
export const CARD_ASPECT = 63 / 88;

export const BOARD_WIDTH = 640;

const NAME_BAR_H = 40;
const HAND_ROW_H_OPP = 70;
const HAND_ROW_H_PLAYER = 110;
const DECK_STRIP_H = 54;
const BENCH_ROW_H = 130;
const ACTIVE_ROW_H = 190;
const DIVIDER_H = 64;

export const BOARD_HEIGHT =
  NAME_BAR_H * 2 + HAND_ROW_H_OPP + HAND_ROW_H_PLAYER + DECK_STRIP_H * 2 + BENCH_ROW_H * 2 + ACTIVE_ROW_H * 2 + DIVIDER_H;

// Row top-edges, opponent (player 1) from the board's top edge down to the divider, player
// (player 0) mirrored from the divider down to the bottom edge.
const nameBarOppTop = 0;
const handRowOppTop = nameBarOppTop + NAME_BAR_H;
const deckStripOppTop = handRowOppTop + HAND_ROW_H_OPP;
const benchRowOppTop = deckStripOppTop + DECK_STRIP_H;
const activeRowOppTop = benchRowOppTop + BENCH_ROW_H;
const dividerTop = activeRowOppTop + ACTIVE_ROW_H;
const activeRowPlayerTop = dividerTop + DIVIDER_H;
const benchRowPlayerTop = activeRowPlayerTop + ACTIVE_ROW_H;
const deckStripPlayerTop = benchRowPlayerTop + BENCH_ROW_H;
const handRowPlayerTop = deckStripPlayerTop + DECK_STRIP_H;
const nameBarPlayerTop = handRowPlayerTop + HAND_ROW_H_PLAYER;

function rowCenterY(top: number, height: number): number {
  return top + height / 2;
}

const BENCH_H = 108;
export const BENCH_W = Math.round(BENCH_H * CARD_ASPECT);
export const BENCH_SIZE = { w: BENCH_W, h: BENCH_H };
const ACTIVE_H = Math.round(BENCH_H * 1.6);
export const ACTIVE_W = Math.round(ACTIVE_H * CARD_ASPECT);
export const ACTIVE_SIZE = { w: ACTIVE_W, h: ACTIVE_H };
// Kept for callers that used the old flat names.
export const CARD_W = BENCH_W;
export const CARD_H = BENCH_H;
export const ACTIVE_CARD_W = ACTIVE_W;
export const ACTIVE_CARD_H = ACTIVE_H;

const OPP_HAND_H = 52;
export const OPP_HAND_SIZE = { w: Math.round(OPP_HAND_H * CARD_ASPECT), h: OPP_HAND_H };
const PLAYER_HAND_H = 92;
export const PLAYER_HAND_SIZE = { w: Math.round(PLAYER_HAND_H * CARD_ASPECT), h: PLAYER_HAND_H };
// Kept for callers that used the old flat names (the player's own hand card size).
export const HAND_CARD_W = PLAYER_HAND_SIZE.w;
export const HAND_CARD_H = PLAYER_HAND_SIZE.h;

const STADIUM_H = 56;
export const STADIUM_SIZE = { w: Math.round(STADIUM_H * CARD_ASPECT), h: STADIUM_H };

const DECK_DISCARD_H = 40;
export const DECK_DISCARD_SIZE = { w: Math.round(DECK_DISCARD_H * CARD_ASPECT), h: DECK_DISCARD_H };

const BENCH_GAP = 14;

/** Center of the active spot for a player. Player 0 (bottom half) is the person viewing the
 * replay; player 1 (mirrored, top half) is their opponent — matching the official app. */
export function activePos(player: number): { x: number; y: number } {
  const top = player === 0 ? activeRowPlayerTop : activeRowOppTop;
  return { x: BOARD_WIDTH / 2, y: rowCenterY(top, ACTIVE_ROW_H) };
}

/** Center of bench slot `i` (0, 1, 2) for a player. */
export function benchPos(player: number, i: number): { x: number; y: number } {
  const totalWidth = 3 * BENCH_W + 2 * BENCH_GAP;
  const startX = BOARD_WIDTH / 2 - totalWidth / 2;
  const x = startX + i * (BENCH_W + BENCH_GAP) + BENCH_W / 2;
  const top = player === 0 ? benchRowPlayerTop : benchRowOppTop;
  return { x, y: rowCenterY(top, BENCH_ROW_H) };
}

/** Center for in-play slot index (0 = active, 1..3 = bench). */
export function slotPos(player: number, slot: number): { x: number; y: number } {
  return slot === 0 ? activePos(player) : benchPos(player, slot - 1);
}

export function slotSize(slot: number): { w: number; h: number } {
  return slot === 0 ? ACTIVE_SIZE : BENCH_SIZE;
}

function handRowInfo(player: number): { top: number; height: number; size: { w: number; h: number } } {
  return player === 0
    ? { top: handRowPlayerTop, height: HAND_ROW_H_PLAYER, size: PLAYER_HAND_SIZE }
    : { top: handRowOppTop, height: HAND_ROW_H_OPP, size: OPP_HAND_SIZE };
}

/** Anchor for a card animating out of/into a player's hand — the middle of their hand row. */
export function handAnchor(player: number): { x: number; y: number } {
  const { top, height } = handRowInfo(player);
  return { x: BOARD_WIDTH / 2, y: rowCenterY(top, height) };
}

export function handCardSize(player: number): { w: number; h: number } {
  return handRowInfo(player).size;
}

/** The player's own hand (player 0) is larger and fans with overlap, like a hand of cards held up;
 * the opponent's (player 1) is a compact, non-overlapping row (both are face-up: the replay is
 * omniscient — see the plan's "Decisions"). */
export function handCardPos(player: number, index: number, count: number): { x: number; y: number } {
  const { size } = handRowInfo(player);
  const anchor = handAnchor(player);
  const maxSpread = BOARD_WIDTH - 40;
  const naturalSpacing = player === 0 ? size.w * 0.62 : size.w + 4;
  const spacing = Math.min(naturalSpacing, maxSpread / Math.max(count, 1));
  const startX = anchor.x - ((count - 1) * spacing) / 2;
  return { x: startX + index * spacing, y: anchor.y };
}

function deckDiscardStripInfo(player: number): { top: number } {
  return { top: player === 0 ? deckStripPlayerTop : deckStripOppTop };
}

/** The player's deck/discard sit in a thin strip between their bench and hand rows, at the outer
 * edge (right for the viewer's own side, mirrored left for the opponent's), per the plan. */
export function deckAnchor(player: number): { x: number; y: number } {
  const { top } = deckDiscardStripInfo(player);
  const y = rowCenterY(top, DECK_DISCARD_H);
  return player === 0 ? { x: BOARD_WIDTH - 34, y } : { x: 34, y };
}

export function discardAnchor(player: number): { x: number; y: number } {
  const { top } = deckDiscardStripInfo(player);
  const y = rowCenterY(top, DECK_DISCARD_H);
  return player === 0 ? { x: BOARD_WIDTH - 34 - DECK_DISCARD_SIZE.w - 14, y } : { x: 34 + DECK_DISCARD_SIZE.w + 14, y };
}

/** The energy zone sits in the board's outer corner nearest that player's name bar — bottom-right
 * for the viewer's own side, mirrored top-left for the opponent's. */
export function energyZoneAnchor(player: number): { x: number; y: number } {
  return player === 0
    ? { x: BOARD_WIDTH - 26, y: rowCenterY(nameBarPlayerTop, NAME_BAR_H) }
    : { x: 26, y: rowCenterY(nameBarOppTop, NAME_BAR_H) };
}

export function nameAnchor(player: number): { x: number; y: number } {
  const top = player === 0 ? nameBarPlayerTop : nameBarOppTop;
  const y = rowCenterY(top, NAME_BAR_H);
  return player === 0 ? { x: 16, y } : { x: 50, y };
}

export function pointsRowAnchor(player: number): { x: number; y: number } {
  const top = player === 0 ? nameBarPlayerTop : nameBarOppTop;
  return { x: BOARD_WIDTH / 2, y: rowCenterY(top, NAME_BAR_H) };
}

/** Where the opponent's hand-size (card) count is shown — only the opponent's name bar carries
 * one; the player's own hand is drawn face-up in full so a count would be redundant. */
export function handCountAnchor(): { x: number; y: number } {
  return { x: BOARD_WIDTH - 50, y: rowCenterY(nameBarOppTop, NAME_BAR_H) };
}

export function stadiumPos(): { x: number; y: number } {
  return { x: BOARD_WIDTH / 2 - 90, y: dividerTop + DIVIDER_H / 2 };
}

export function dividerLineY(): number {
  return dividerTop + DIVIDER_H / 2;
}

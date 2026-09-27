// Board geometry: a portrait column mirroring the official app's spatial composition (see the
// user's reference screenshot, described in docs/replay-viewer-plan.md's "Round 3" notes) — tight
// gaps, big cards filling the available height, actives overlapping the center divider slightly,
// bench snug against the active, and a large fanned player hand — but kept dark/minimal (no
// illustrated playmat). Laid out in a fixed logical space; the canvas is scaled to fit its
// container via CSS, so these numbers never need to react to window size.
//
// Round 4: every in-play slot (active or bench, either player) now reserves a small "HP strip"
// directly above its card (see `hpStripHeight`/`HP_STRIP_GAP` and `slotRects` below) — a compact
// bar+number header drawn by `cardArt.ts` — so the HP number/bar can never land on a neighboring
// row, the divider or the turn banner (see `layout.test.ts`, which checks every slot's card+strip
// rects against every other slot's, the divider band and the turn banner for intersections). The
// board and its cards were also both made bigger (`BOARD_WIDTH` and the bench/active/hand sizes),
// since the previous, narrower board left much of a wide viewport empty — see this file's
// `BOARD_WIDTH` comment and the plan doc's "Round 4" notes.

/** Every card, procedural or a real image, keeps this ratio: deckgym's card images are all
 * 367x512px (Round 3 fix — 63:88, the physical card's mm ratio, was close enough to *look* right
 * but not identical, and combined with another bug made images visibly stretched; see cardArt.ts
 * and the plan doc). */
export const CARD_ASPECT = 367 / 512;

// Wider than the bench/active card block actually needs (see `benchPos`), so there's real breathing
// room for the deck/discard/energy-zone corner icons instead of them crowding the cards — "a wider
// center gap for deck/discard/energy is fine" per the Round 4 request — and so a wide viewport uses
// more of its width before the board's own aspect ratio caps it (see the plan doc's "Round 4
// deviations" for why this is a bigger static board rather than a dynamically-reflowing one).
export const BOARD_WIDTH = 1000;

// Every other row/gap constant below is kept as tight as it can be while still fitting its own
// content (see each one's own comment) — the HP strip (below) and the bigger card sizes both add
// real height, and the only way to make the *cards* bigger on screen (not just the canvas wider) is
// to claw that back everywhere else, since a viewport that's bound by the board's height scales
// every logical pixel by the same factor.
const NAME_BAR_H = 26;
const GAP_NAME_HAND = 2;
const GAP_HAND_DECKSTRIP = 2;
const DECK_STRIP_H = 52;
const GAP_DECKSTRIP_BENCH = 2;
const GAP_BENCH_ACTIVE = 2;
const DIVIDER_H = 38; // fits the 36px-tall turn banner (scene.ts) with a 1px margin each side
const GAP_DECKSTRIP_HAND = 2;

const BENCH_H = 210;
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

/** Gap between a slot's HP strip and the top of its card. */
export const HP_STRIP_GAP = 3;

/** How tall a slot's HP strip (bar + number, drawn by `cardArt.ts`) is, given its card's height —
 * one shared formula so the row math here and the drawing in `cardArt.ts` can never drift apart. */
export function hpStripHeight(cardHeight: number): number {
  return Math.max(14, Math.round(cardHeight * 0.1));
}

const BENCH_STRIP_H = hpStripHeight(BENCH_H);
const ACTIVE_STRIP_H = hpStripHeight(ACTIVE_H);

// A slot's "row" is its HP strip plus the gap plus the card itself, top to bottom — the strip is
// *inside* this height, not extra space around it, so no separate gap constant is needed to keep it
// clear of the row above (see the header comment).
const BENCH_ROW_H = BENCH_STRIP_H + HP_STRIP_GAP + BENCH_H;
const ACTIVE_ROW_H = ACTIVE_STRIP_H + HP_STRIP_GAP + ACTIVE_H;

// The opponent's hand is sized as a fraction of a bench card (60-70%, per the Round 4 request) —
// big enough to read as real cards, not a row of dots — while the player's own hand stays as big as
// a bench card or a little bigger, fanned out (see `handCardPos`'s rotation/arc below).
const OPP_HAND_H = Math.round(BENCH_H * 0.65);
export const OPP_HAND_SIZE = { w: Math.round(OPP_HAND_H * CARD_ASPECT), h: OPP_HAND_H };
const PLAYER_HAND_H = Math.round(BENCH_H * 1.05);
export const PLAYER_HAND_SIZE = { w: Math.round(PLAYER_HAND_H * CARD_ASPECT), h: PLAYER_HAND_H };
// Kept for callers that used the old flat names (the player's own hand card size).
export const HAND_CARD_W = PLAYER_HAND_SIZE.w;
export const HAND_CARD_H = PLAYER_HAND_SIZE.h;
const HAND_ROW_H_OPP = OPP_HAND_H; // flat, non-overlapping row — no arc/rotation to clear
const HAND_ROW_H_PLAYER = PLAYER_HAND_H + 22; // + room for the fan's rotation/arc to clear neighbors

const STADIUM_H = 48;
export const STADIUM_SIZE = { w: Math.round(STADIUM_H * CARD_ASPECT), h: STADIUM_H };

const DECK_DISCARD_H = 48;
export const DECK_DISCARD_SIZE = { w: Math.round(DECK_DISCARD_H * CARD_ASPECT), h: DECK_DISCARD_H };

const BENCH_GAP = 10;

// Row top-edges, opponent (player 1) from the board's top edge down to the divider, player
// (player 0) mirrored from the divider down to the bottom edge. Walked cumulatively so the gap
// constants above are the only place row spacing is tuned.
const nameBarOppTop = 0;
const handRowOppTop = nameBarOppTop + NAME_BAR_H + GAP_NAME_HAND;
const deckStripOppTop = handRowOppTop + HAND_ROW_H_OPP + GAP_HAND_DECKSTRIP;
const benchRowOppTop = deckStripOppTop + DECK_STRIP_H + GAP_DECKSTRIP_BENCH;
const activeRowOppTop = benchRowOppTop + BENCH_ROW_H + GAP_BENCH_ACTIVE;
const dividerTop = activeRowOppTop + ACTIVE_ROW_H;
const activeRowPlayerTop = dividerTop + DIVIDER_H;
const benchRowPlayerTop = activeRowPlayerTop + ACTIVE_ROW_H + GAP_BENCH_ACTIVE;
const deckStripPlayerTop = benchRowPlayerTop + BENCH_ROW_H + GAP_DECKSTRIP_BENCH;
const handRowPlayerTop = deckStripPlayerTop + DECK_STRIP_H + GAP_DECKSTRIP_HAND;
const nameBarPlayerTop = handRowPlayerTop + HAND_ROW_H_PLAYER + GAP_NAME_HAND;

export const BOARD_HEIGHT = nameBarPlayerTop + NAME_BAR_H;

function rowCenterY(top: number, height: number): number {
  return top + height / 2;
}

/** Center of the active spot for a player. Player 0 (bottom half) is the person viewing the
 * replay; player 1 (mirrored, top half) is their opponent — matching the official app. The card
 * sits at the *bottom* of its row (the strip occupies the top, see the header comment), not
 * centered in it. */
export function activePos(player: number): { x: number; y: number } {
  const top = player === 0 ? activeRowPlayerTop : activeRowOppTop;
  const cardTop = top + ACTIVE_STRIP_H + HP_STRIP_GAP;
  return { x: BOARD_WIDTH / 2, y: cardTop + ACTIVE_H / 2 };
}

/** Center of bench slot `i` (0, 1, 2) for a player — tight against its neighbors and against the
 * active row above/below it, per the reference. Like `activePos`, the card sits at the bottom of
 * its row, below its own HP strip. */
export function benchPos(player: number, i: number): { x: number; y: number } {
  const totalWidth = 3 * BENCH_W + 2 * BENCH_GAP;
  const startX = BOARD_WIDTH / 2 - totalWidth / 2;
  const x = startX + i * (BENCH_W + BENCH_GAP) + BENCH_W / 2;
  const top = player === 0 ? benchRowPlayerTop : benchRowOppTop;
  const cardTop = top + BENCH_STRIP_H + HP_STRIP_GAP;
  return { x, y: cardTop + BENCH_H / 2 };
}

/** Center for in-play slot index (0 = active, 1..3 = bench). */
export function slotPos(player: number, slot: number): { x: number; y: number } {
  return slot === 0 ? activePos(player) : benchPos(player, slot - 1);
}

export function slotSize(slot: number): { w: number; h: number } {
  return slot === 0 ? ACTIVE_SIZE : BENCH_SIZE;
}

/** This slot's HP strip height (bar + number), matching what `cardArt.ts` draws for a `CardVisual`
 * created with this slot's size. */
export function slotStripHeight(slot: number): number {
  return slot === 0 ? ACTIVE_STRIP_H : BENCH_STRIP_H;
}

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

/** The card rect and its HP strip rect for a given in-play slot, in board coordinates — used to
 * draw the strip (`cardArt.ts` mirrors this via `slotStripHeight`/`HP_STRIP_GAP`) and, in
 * `layout.test.ts`, to assert no two slots' rects (card or strip) ever intersect. */
export function slotRects(player: number, slot: number): { card: Rect; strip: Rect } {
  const size = slotSize(slot);
  const pos = slotPos(player, slot);
  const stripH = slotStripHeight(slot);
  const cardTop = pos.y - size.h / 2;
  const stripTop = cardTop - HP_STRIP_GAP - stripH;
  const x = pos.x - size.w / 2;
  return {
    card: { x, y: cardTop, w: size.w, h: size.h },
    strip: { x, y: stripTop, w: size.w, h: stripH },
  };
}

/** The divider band's rect (the center line plus the turn banner that flashes on it) — every
 * slot's card/strip rects must stay clear of this too. */
export function dividerBandRect(): Rect {
  return { x: 0, y: dividerTop, w: BOARD_WIDTH, h: DIVIDER_H };
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

/** The player's own hand (player 0) is as big as a bench card or bigger, and fans out along a
 * gentle arc with a little rotation per card, like a hand of cards held up — the center card sits
 * highest, the outer ones droop and tilt outward. The opponent's (player 1) stays a small, flat,
 * non-overlapping row (both are face-up: the replay is omniscient — see the plan's "Decisions"). */
export function handCardPos(player: number, index: number, count: number): { x: number; y: number; rotation: number } {
  const { size } = handRowInfo(player);
  const anchor = handAnchor(player);
  const margin = 12;
  // The fan's total footprint is `(count-1)*spacing + size.w` (the spacing between card centers,
  // plus the last card's own half-width on each end) — capping spacing by that, not just by
  // `count`, keeps every card's outer edge on-canvas even at a full 10-card hand.
  const maxSpacingForWidth = (n: number) => (n > 1 ? Math.max(0, BOARD_WIDTH - margin * 2 - size.w) / (n - 1) : Infinity);

  if (player === 1) {
    const spacing = Math.min(size.w + 3, maxSpacingForWidth(count));
    const startX = anchor.x - ((count - 1) * spacing) / 2;
    return { x: startX + index * spacing, y: anchor.y, rotation: 0 };
  }

  // Fanned, overlapping hand: t goes from -1 (leftmost) to 1 (rightmost).
  const spacing = Math.min(size.w * 0.56, maxSpacingForWidth(count));
  const startX = anchor.x - ((count - 1) * spacing) / 2;
  const t = count > 1 ? (index - (count - 1) / 2) / ((count - 1) / 2) : 0;
  const maxRotation = 0.16; // ~9 degrees at the fan's outer edge
  const arcDrop = 16; // px the outer cards sit below the center card
  return {
    x: startX + index * spacing,
    y: anchor.y + t * t * arcDrop,
    rotation: t * maxRotation,
  };
}

function deckDiscardStripInfo(player: number): { top: number } {
  return { top: player === 0 ? deckStripPlayerTop : deckStripOppTop };
}

/** The player's deck/discard sit in a thin strip between their bench and hand rows, at the outer
 * edge (right for the viewer's own side, mirrored left for the opponent's), per the plan. */
export function deckAnchor(player: number): { x: number; y: number } {
  const { top } = deckDiscardStripInfo(player);
  const y = rowCenterY(top, DECK_DISCARD_H);
  return player === 0 ? { x: BOARD_WIDTH - 36, y } : { x: 36, y };
}

export function discardAnchor(player: number): { x: number; y: number } {
  const { top } = deckDiscardStripInfo(player);
  const y = rowCenterY(top, DECK_DISCARD_H);
  return player === 0 ? { x: BOARD_WIDTH - 36 - DECK_DISCARD_SIZE.w - 16, y } : { x: 36 + DECK_DISCARD_SIZE.w + 16, y };
}

/** The energy zone sits in the board's outer corner nearest that player's name bar — bottom-right
 * for the viewer's own side, mirrored top-left for the opponent's. */
export function energyZoneAnchor(player: number): { x: number; y: number } {
  return player === 0
    ? { x: BOARD_WIDTH - 28, y: rowCenterY(nameBarPlayerTop, NAME_BAR_H) }
    : { x: 28, y: rowCenterY(nameBarOppTop, NAME_BAR_H) };
}

export function nameAnchor(player: number): { x: number; y: number } {
  const top = player === 0 ? nameBarPlayerTop : nameBarOppTop;
  const y = rowCenterY(top, NAME_BAR_H);
  return player === 0 ? { x: 16, y } : { x: 52, y };
}

export function pointsRowAnchor(player: number): { x: number; y: number } {
  const top = player === 0 ? nameBarPlayerTop : nameBarOppTop;
  return { x: BOARD_WIDTH / 2, y: rowCenterY(top, NAME_BAR_H) };
}

/** Where the opponent's hand-size (card) count is shown — only the opponent's name bar carries
 * one; the player's own hand is drawn face-up in full so a count would be redundant. */
export function handCountAnchor(): { x: number; y: number } {
  return { x: BOARD_WIDTH - 52, y: rowCenterY(nameBarOppTop, NAME_BAR_H) };
}

export function stadiumPos(): { x: number; y: number } {
  // Clear of the (wider, centered) active card's own horizontal span, so a stadium in play never
  // overlaps it — active cards are the widest thing centered on this row.
  return { x: BOARD_WIDTH / 2 - ACTIVE_W / 2 - STADIUM_SIZE.w / 2 - 10, y: dividerTop + DIVIDER_H / 2 };
}

export function dividerLineY(): number {
  return dividerTop + DIVIDER_H / 2;
}

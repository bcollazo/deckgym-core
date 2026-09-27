// Hand-written mirror of the Rust replay types in `src/replay/*.rs` and `src/actions/types.rs`.
//
// These are NOT generated (see docs/replay-viewer-plan.md's "Deviations" section): the Rust side
// is Serialize/Deserialize with serde's default (externally-tagged) enum representation, and
// `tests/replay_test.rs` round-trips a real replay through serde_json, so a shape drift on the
// Rust side is still caught by CI even without codegen.
//
// Field names intentionally stay snake_case to match the JSON 1:1 with the Rust struct fields,
// rather than translating to camelCase, so this file is easy to diff against the Rust source.

export type EnergyType =
  | "Grass"
  | "Fire"
  | "Water"
  | "Lightning"
  | "Psychic"
  | "Fighting"
  | "Darkness"
  | "Metal"
  | "Dragon"
  | "Colorless";

export type StatusCondition = "Poisoned" | "Paralyzed" | "Asleep" | "Burned" | "Confused";

export type GameOutcome = { Win: number } | "Tie";

export interface EnergyZone {
  current: EnergyType | null;
  next: EnergyType | null;
}

export interface SlotView {
  card: string;
  hp: number;
  max_hp: number;
  energy: EnergyType[];
  tools: string[];
  status: StatusCondition[];
  played_this_turn: boolean;
}

export interface PlayerView {
  /** `null` when redacted (a bot's view of its opponent's hand). Always present in a replay. */
  hand: string[] | null;
  hand_count: number;
  deck_count: number;
  discard: string[];
  discard_energies: EnergyType[];
  energy_zone: EnergyZone;
  /** index 0 = active, 1..=3 = bench */
  in_play: [SlotView | null, SlotView | null, SlotView | null, SlotView | null];
}

export interface ViewState {
  turn: number;
  current_player: number;
  points: [number, number];
  winner: GameOutcome | null;
  players: [PlayerView, PlayerView];
  stadium: string | null;
}

export interface Attack {
  energy_required: EnergyType[];
  title: string;
  fixed_damage: number;
  effect: string | null;
}

export interface Ability {
  title: string;
  effect: string;
}

export interface PokemonCardData {
  id: string;
  name: string;
  stage: number;
  evolves_from: string | null;
  hp: number;
  energy_type: EnergyType;
  ability: Ability | null;
  attacks: Attack[];
  weakness: EnergyType | null;
  retreat_cost: EnergyType[];
  rarity: string;
  booster_pack: string;
}

export type TrainerType = "Supporter" | "Item" | "Tool" | "Fossil" | "Stadium";

export interface TrainerCardData {
  id: string;
  trainer_card_type: TrainerType;
  name: string;
  effect: string;
  rarity: string;
  booster_pack: string;
}

export type Card = { Pokemon: PokemonCardData } | { Trainer: TrainerCardData };

export function isPokemonCard(card: Card): card is { Pokemon: PokemonCardData } {
  return "Pokemon" in card;
}

export function cardName(card: Card | undefined): string {
  if (!card) return "?";
  return isPokemonCard(card) ? card.Pokemon.name : card.Trainer.name;
}

export function cardEnergyType(card: Card | undefined): EnergyType | null {
  if (!card || !isPokemonCard(card)) return null;
  return card.Pokemon.energy_type;
}

export function isExCard(card: Card | undefined): boolean {
  if (!card || !isPokemonCard(card)) return false;
  return card.Pokemon.name.toLowerCase().split(" ").at(-1) === "ex";
}

export function cardId(card: Card): string {
  return isPokemonCard(card) ? card.Pokemon.id : card.Trainer.id;
}

/**
 * `SimpleAction` (`src/actions/types.rs`) has ~50 variants and serde's default externally-tagged
 * representation: a unit variant like `EndTurn` serializes to the string `"EndTurn"`; every other
 * variant serializes to `{ "<Variant>": <payload> }`. Rather than typing all fifty (most of which
 * the viewer never choreographs specially), this stays a loose union and `actionTag`/
 * `actionPayload` below pull out the tag/payload generically. The variants the animation engine
 * *does* choreograph specially get their payload shape asserted at the point of use.
 */
export type SimpleAction = string | Record<string, unknown>;

export function actionTag(action: SimpleAction): string {
  return typeof action === "string" ? action : Object.keys(action)[0];
}

export function actionPayload(action: SimpleAction): unknown {
  if (typeof action === "string") return undefined;
  const tag = actionTag(action);
  return action[tag];
}

export interface ReplayPlayerInfo {
  name: string;
  deck: string[];
}

export interface ReplayOption {
  text: string;
  action: SimpleAction;
}

export interface ReplayStep {
  ply: number;
  turn: number;
  actor: number;
  state: ViewState;
  options: ReplayOption[];
  chosen: number;
  note: string | null;
}

export interface Replay {
  version: number;
  game_id: string;
  seed: number;
  players: [ReplayPlayerInfo, ReplayPlayerInfo];
  cards: Record<string, Card>;
  steps: ReplayStep[];
  final_state: ViewState | null;
  outcome: GameOutcome | null;
}

/** The full sequence of rest-snapshots in a replay: `steps[0].state, ..., steps[N-1].state,
 * final_state` — `N + 1` snapshots for `N` steps. */
export function snapshotAt(replay: Replay, index: number): ViewState {
  if (index < replay.steps.length) return replay.steps[index].state;
  if (index === replay.steps.length && replay.final_state) return replay.final_state;
  throw new Error(`snapshot index ${index} out of range`);
}

export function snapshotCount(replay: Replay): number {
  return replay.steps.length + (replay.final_state ? 1 : 0);
}

export function outcomeLabel(outcome: GameOutcome | null, players: [ReplayPlayerInfo, ReplayPlayerInfo]): string {
  if (!outcome) return "In progress";
  if (outcome === "Tie") return "Tie";
  return `${players[outcome.Win]?.name ?? `Player ${outcome.Win}`} wins`;
}

import { describe, expect, it } from "vitest";
import { formatAction, type FormatActionContext } from "./formatAction";
import type { Card, PlayerView, ViewState } from "../types/replay";

const bulbasaur: Card = {
  Pokemon: {
    id: "A1 001",
    name: "Bulbasaur",
    stage: 0,
    evolves_from: null,
    hp: 70,
    energy_type: "Grass",
    ability: null,
    attacks: [],
    weakness: null,
    retreat_cost: [],
    rarity: "Common",
    booster_pack: "A1",
  },
};

const ivysaur: Card = { Pokemon: { ...bulbasaur.Pokemon, id: "A1 002", name: "Ivysaur" } };
const venusaur: Card = { Pokemon: { ...bulbasaur.Pokemon, id: "A1 004", name: "Venusaur" } };
const ekans: Card = { Pokemon: { ...bulbasaur.Pokemon, id: "A1 176", name: "Ekans" } };
const potion: Card = {
  Trainer: { id: "P-A 001", trainer_card_type: "Item", name: "Potion", effect: "", rarity: "Common", booster_pack: "P-A" },
};

const cards: Record<string, Card> = {
  "A1 001": bulbasaur,
  "A1 002": ivysaur,
  "A1 004": venusaur,
  "A1 176": ekans,
  "P-A 001": potion,
};

function emptyPlayer(overrides: Partial<PlayerView> = {}): PlayerView {
  return {
    hand: [],
    hand_count: 0,
    deck_count: 10,
    discard: [],
    discard_energies: [],
    energy_zone: { current: null, next: null },
    in_play: [null, null, null, null],
    ...overrides,
  };
}

function state(overrides: Partial<ViewState> = {}): ViewState {
  return {
    turn: 3,
    current_player: 0,
    points: [0, 0],
    winner: null,
    players: [emptyPlayer(), emptyPlayer()],
    stadium: null,
    ...overrides,
  };
}

const ctx = (extra: Partial<FormatActionContext> = {}): FormatActionContext => ({ cards, ...extra });

describe("formatAction", () => {
  it("formats EndTurn", () => {
    expect(formatAction("EndTurn", "EndTurn", ctx())).toBe("End turn");
  });

  it("formats DrawCard", () => {
    expect(formatAction({ DrawCard: { amount: 1 } }, "DrawCard(1)", ctx())).toBe("Draw 1");
  });

  it("formats Place", () => {
    expect(formatAction({ Place: [bulbasaur, 2] }, "Place(Bulbasaur, 2)", ctx())).toBe("Place Bulbasaur on Bench 2");
  });

  it("formats Place onto the active slot", () => {
    expect(formatAction({ Place: [bulbasaur, 0] }, "Place(Bulbasaur, 0)", ctx())).toBe("Place Bulbasaur on Active");
  });

  it("formats Evolve with the pre-evolution name from beforeState", () => {
    const before = state({
      players: [
        emptyPlayer({ in_play: [{ card: "A1 002", hp: 90, max_hp: 90, energy: [], tools: [], status: [], played_this_turn: false }, null, null, null] }),
        emptyPlayer(),
      ],
    });
    const action = formatAction(
      { Evolve: { evolution: venusaur, in_play_idx: 0, from_deck: false } },
      "Evolve(Venusaur, 0, from_deck: false)",
      ctx({ beforeState: before, actor: 0 }),
    );
    expect(action).toBe("Evolve Ivysaur → Venusaur (Active)");
  });

  it("formats Evolve without beforeState by omitting the 'from' name", () => {
    const action = formatAction(
      { Evolve: { evolution: venusaur, in_play_idx: 0, from_deck: false } },
      "Evolve(Venusaur, 0, from_deck: false)",
      ctx(),
    );
    expect(action).toBe("Evolve → Venusaur (Active)");
  });

  it("formats Play", () => {
    expect(formatAction({ Play: { trainer_card: { name: "Potion" } } }, "Play(Potion)", ctx())).toBe("Play Potion");
  });

  it("formats Attack", () => {
    expect(formatAction({ Attack: { title: "Vine Whip" } }, "Attack(Vine Whip)", ctx())).toBe("Attack: Vine Whip");
  });

  it("formats Retreat", () => {
    expect(formatAction({ Retreat: 1 }, "Retreat(1)", ctx())).toBe("Retreat to Bench 1");
  });

  it("formats Attach with the occupant's name from beforeState", () => {
    const before = state({
      players: [
        emptyPlayer({ in_play: [{ card: "A1 176", hp: 60, max_hp: 60, energy: [], tools: [], status: [], played_this_turn: false }, null, null, null] }),
        emptyPlayer(),
      ],
    });
    const action = formatAction(
      { Attach: { attachments: [[1, "Darkness", 0]], is_turn_energy: true } },
      'Attach("(1, Darkness, 0)", true)',
      ctx({ beforeState: before, actor: 0 }),
    );
    expect(action).toBe("Attach Darkness → Active (Ekans)");
  });

  it("formats a multi-energy Attach with an amount prefix", () => {
    const action = formatAction(
      { Attach: { attachments: [[2, "Water", 1]], is_turn_energy: false } },
      "Attach(...)",
      ctx(),
    );
    expect(action).toBe("Attach 2x Water → Bench 1");
  });

  it("falls back to the provided text for an unhandled variant", () => {
    expect(formatAction({ SomeFutureVariant: { whatever: 1 } }, "SomeFutureVariant(...)", ctx())).toBe("SomeFutureVariant(...)");
  });
});

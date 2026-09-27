import { describe, expect, it } from "vitest";
import { diffViewStates, slotKey } from "./diff";
import type { PlayerView, ViewState } from "../types/replay";

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
    turn: 1,
    current_player: 0,
    points: [0, 0],
    winner: null,
    players: [emptyPlayer(), emptyPlayer()],
    stadium: null,
    ...overrides,
  };
}

describe("diffViewStates", () => {
  it("detects a turn change", () => {
    const prev = state({ turn: 3, current_player: 0 });
    const next = state({ turn: 4, current_player: 1 });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "turnChange", from: 3, to: 4, fromPlayer: 0, toPlayer: 1 });
  });

  it("detects a card entering an empty slot", () => {
    const prev = state();
    const next = state({
      players: [
        emptyPlayer({
          in_play: [
            { card: "A1 001", hp: 70, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: true },
            null,
            null,
            null,
          ],
        }),
        emptyPlayer(),
      ],
    });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "cardEntered", slot: slotKey(0, 0), card: "A1 001" });
  });

  it("detects hp changes on an existing card", () => {
    const slot = { card: "A1 001", hp: 70, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: false };
    const prev = state({ players: [emptyPlayer({ in_play: [slot, null, null, null] }), emptyPlayer()] });
    const next = state({
      players: [emptyPlayer({ in_play: [{ ...slot, hp: 40 }, null, null, null] }), emptyPlayer()],
    });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "hpChange", slot: slotKey(0, 0), from: 70, to: 40, maxHp: 70 });
  });

  it("classifies a slot emptying as a knock-out when the opponent scored", () => {
    const slot = { card: "A1 001", hp: 10, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: false };
    const prev = state({
      points: [0, 0],
      players: [emptyPlayer({ in_play: [slot, null, null, null] }), emptyPlayer()],
    });
    const next = state({
      points: [0, 1],
      players: [emptyPlayer({ in_play: [null, null, null, null] }), emptyPlayer()],
    });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "knockOut", slot: slotKey(0, 0), card: "A1 001" });
  });

  it("classifies a slot emptying without a score change as a plain cardLeft", () => {
    const slot = { card: "A1 001", hp: 70, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: false };
    const prev = state({ players: [emptyPlayer({ in_play: [slot, null, null, null] }), emptyPlayer()] });
    const next = state({ players: [emptyPlayer({ in_play: [null, null, null, null] }), emptyPlayer()] });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "cardLeft", slot: slotKey(0, 0), card: "A1 001" });
  });

  it("detects an evolution as a same-slot card id change", () => {
    const bulbasaur = {
      card: "A1 001",
      hp: 70,
      max_hp: 70,
      energy: [],
      tools: [],
      status: [],
      played_this_turn: false,
    };
    const ivysaur = { ...bulbasaur, card: "A1 002", max_hp: 90, hp: 90 };
    const prev = state({ players: [emptyPlayer({ in_play: [bulbasaur, null, null, null] }), emptyPlayer()] });
    const next = state({ players: [emptyPlayer({ in_play: [ivysaur, null, null, null] }), emptyPlayer()] });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "evolution", slot: slotKey(0, 0), from: "A1 001", to: "A1 002" });
    // The HP delta from the evolution's higher max HP is reported too.
    expect(changes).toContainEqual({ type: "hpChange", slot: slotKey(0, 0), from: 70, to: 90, maxHp: 90 });
  });

  it("detects energy attach as an add-only multiset diff", () => {
    const slot = { card: "A1 001", hp: 70, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: false };
    const prev = state({ players: [emptyPlayer({ in_play: [slot, null, null, null] }), emptyPlayer()] });
    const next = state({
      players: [emptyPlayer({ in_play: [{ ...slot, energy: ["Grass" as const] }, null, null, null] }), emptyPlayer()],
    });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({ type: "energyChange", slot: slotKey(0, 0), added: ["Grass"], removed: [] });
  });

  it("detects a retreat as an activeSwitch between the active slot and a bench slot", () => {
    const active = { card: "A1 001", hp: 70, max_hp: 70, energy: [], tools: [], status: [], played_this_turn: false };
    const bench = { card: "A1 003", hp: 60, max_hp: 60, energy: [], tools: [], status: [], played_this_turn: false };
    const prev = state({
      players: [emptyPlayer({ in_play: [active, bench, null, null] }), emptyPlayer()],
    });
    const next = state({
      players: [emptyPlayer({ in_play: [bench, active, null, null] }), emptyPlayer()],
    });
    const changes = diffViewStates(prev, next);
    expect(changes).toContainEqual({
      type: "activeSwitch",
      player: 0,
      benchSlot: slotKey(0, 1),
      activeFrom: "A1 001",
      activeTo: "A1 003",
    });
  });

  it("reports no changes for two identical snapshots", () => {
    const s = state({ turn: 5 });
    expect(diffViewStates(s, s)).toEqual([]);
  });
});

import type { Replay } from "../types/replay";

export class ReplayLoadError extends Error {}

/** Minimal structural check — enough to catch "wrong file" without re-implementing a schema
 * validator for a format that already has a Rust-side round-trip test (tests/replay_test.rs). */
function assertLooksLikeReplay(value: unknown): asserts value is Replay {
  if (
    !value ||
    typeof value !== "object" ||
    !("steps" in value) ||
    !Array.isArray((value as Replay).steps) ||
    !("players" in value) ||
    !("cards" in value)
  ) {
    throw new ReplayLoadError("This file doesn't look like a deckgym replay (missing steps/players/cards).");
  }
}

export function parseReplayJson(text: string): Replay {
  let value: unknown;
  try {
    value = JSON.parse(text);
  } catch {
    throw new ReplayLoadError("That file isn't valid JSON.");
  }
  assertLooksLikeReplay(value);
  return value;
}

export async function loadReplayFromFile(file: File): Promise<Replay> {
  const text = await file.text();
  return parseReplayJson(text);
}

export async function loadReplayFromUrl(url: string): Promise<Replay> {
  const res = await fetch(url);
  if (!res.ok) {
    throw new ReplayLoadError(`Failed to fetch ${url}: HTTP ${res.status}`);
  }
  return parseReplayJson(await res.text());
}

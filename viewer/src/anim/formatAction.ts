// Turns a `SimpleAction` (`src/actions/types.rs`, ~50 variants) into a short, human-readable
// string for the log/options panels — e.g. `Attach Darkness → Active (Ekans)` instead of the raw
// Rust `Debug`-ish text deckgym's `Display` impl produces for that same action
// (`Attach("(1, Darkness, 0)", true)`). Only the common variants are special-cased; anything else
// falls back to `fallbackText` (the replay's own `options[i].text`, i.e. `SimpleAction`'s
// `Display`), so this never needs to keep up with all fifty variants to stay useful.

import type { Card, EnergyType, SimpleAction, ViewState } from "../types/replay";
import { actionPayload, actionTag, cardName } from "../types/replay";

export interface FormatActionContext {
  cards: Record<string, Card>;
  /** The ViewState right before this action (a replay step's own `state`) — used to name the
   * Pokemon already occupying a slot the action targets (e.g. what's being attached to). */
  beforeState?: ViewState;
  /** Whose action this is (a replay step's own `actor`). */
  actor?: number;
}

function slotLabel(idx: number): string {
  return idx === 0 ? "Active" : `Bench ${idx}`;
}

function occupantName(ctx: FormatActionContext, idx: number, player = ctx.actor): string | null {
  if (player === undefined || !ctx.beforeState) return null;
  const slot = ctx.beforeState.players[player]?.in_play[idx];
  if (!slot) return null;
  const card = ctx.cards[slot.card];
  return card ? cardName(card) : null;
}

function withOccupant(base: string, occupant: string | null): string {
  return occupant ? `${base} (${occupant})` : base;
}

/** `SimpleAction::Attach`'s single-attachment tuple, `(amount, energy_type, in_play_idx)`,
 * serialized as a 3-element JSON array. */
type AttachmentTuple = [number, EnergyType, number];

export function formatAction(action: SimpleAction, fallbackText: string, ctx: FormatActionContext): string {
  const tag = actionTag(action);
  const payload = actionPayload(action);

  switch (tag) {
    case "EndTurn":
      return "End turn";

    case "Noop":
      return "Pass";

    case "DrawCard": {
      const amount = (payload as { amount?: number } | undefined)?.amount;
      return amount === undefined ? fallbackText : `Draw ${amount}`;
    }

    case "Place": {
      const tuple = payload as [Card, number] | undefined;
      if (!tuple) return fallbackText;
      const [card, idx] = tuple;
      return `Place ${cardName(card)} on ${slotLabel(idx)}`;
    }

    case "Evolve": {
      const p = payload as { evolution: Card; in_play_idx: number; from_deck?: boolean } | undefined;
      if (!p) return fallbackText;
      const from = occupantName(ctx, p.in_play_idx);
      const to = cardName(p.evolution);
      const base = from ? `Evolve ${from} → ${to} (${slotLabel(p.in_play_idx)})` : `Evolve → ${to} (${slotLabel(p.in_play_idx)})`;
      return p.from_deck ? `${base} (from deck)` : base;
    }

    case "Play": {
      const p = payload as { trainer_card?: { name?: string } } | undefined;
      return p?.trainer_card?.name ? `Play ${p.trainer_card.name}` : fallbackText;
    }

    case "Attack": {
      const p = payload as { title?: string } | undefined;
      return p?.title ? `Attack: ${p.title}` : fallbackText;
    }

    case "Retreat": {
      if (typeof payload !== "number") return fallbackText;
      return `Retreat to ${slotLabel(payload)}`;
    }

    case "Attach": {
      const p = payload as { attachments?: AttachmentTuple[] } | undefined;
      if (!p?.attachments?.length) return fallbackText;
      const parts = p.attachments.map(([amount, energyType, idx]) => {
        const prefix = amount > 1 ? `${amount}x ` : "";
        return withOccupant(`${prefix}${energyType} → ${slotLabel(idx)}`, occupantName(ctx, idx));
      });
      return `Attach ${parts.join(", ")}`;
    }

    case "AttachTool": {
      const p = payload as { in_play_idx: number; tool_card: Card } | undefined;
      if (!p) return fallbackText;
      return withOccupant(`Attach ${cardName(p.tool_card)} → ${slotLabel(p.in_play_idx)}`, occupantName(ctx, p.in_play_idx));
    }

    case "UseAbility": {
      const p = payload as { in_play_idx: number } | undefined;
      if (!p) return fallbackText;
      const occ = occupantName(ctx, p.in_play_idx);
      return occ ? `Use Ability: ${occ}` : fallbackText;
    }

    case "Heal": {
      const p = payload as { in_play_idx: number; amount: number; cure_status?: boolean } | undefined;
      if (!p) return fallbackText;
      const suffix = p.cure_status ? " + cure status" : "";
      return `${withOccupant(`Heal ${p.amount}`, occupantName(ctx, p.in_play_idx))}${suffix}`;
    }

    case "Activate": {
      const p = payload as { in_play_idx: number } | undefined;
      if (!p) return fallbackText;
      return `Promote ${slotLabel(p.in_play_idx)} to Active`;
    }

    default:
      return fallbackText;
  }
}

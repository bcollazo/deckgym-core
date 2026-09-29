//! Read-only public rule context for agents; deliberately excludes hands and deck contents.
use super::{PendingCoinReflip, State};
use crate::effects::{CardEffect, TurnEffect};
use crate::models::{Attack, Card, EnergyType};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize)]
pub struct SlotDetails {
    pub max_hp: u32,
    pub damage: u32,
    pub effects: Vec<(CardEffect, u8)>,
    pub effective_effects: Vec<CardEffect>,
    pub poison_damage: u32,
    pub paralyzed_on_turn: Option<u8>,
    pub heal_blocked: bool,
    pub abilities_disabled: bool,
    pub damaged_this_turn: bool,
    pub damaged_last_turn: bool,
}

#[derive(Debug, Serialize)]
pub struct PublicContext<'a> {
    pub end_turn_pending: bool,
    pub has_played_support: bool,
    pub has_retreated: bool,
    pub has_used_stadium: [bool; 2],
    pub has_used_victory_star: [bool; 2],
    pub has_used_luxury_coin: [bool; 2],
    pub knocked_out_this_turn: bool,
    pub knocked_out_last_turn: bool,
    pub knocked_out_types_this_turn: &'a BTreeSet<EnergyType>,
    pub knocked_out_types_last_turn: &'a BTreeSet<EnergyType>,
    pub attack_name_used_this_turn: [&'a Option<String>; 2],
    pub attack_name_used_last_turn: [&'a Option<String>; 2],
    pub attack_name_used_count: [&'a BTreeMap<String, u32>; 2],
    pub own_knocked_out_count: [u32; 2],
    pub points_at_own_turn_start: [u8; 2],
    pub points_gained_during_own_last_turn: [u8; 2],
}

impl State {
    /// The decision actor can differ from the turn owner during a stacked effect.
    pub fn decision_actor(&self) -> usize {
        if self.turn_count > 0 {
            self.move_generation_stack
                .last()
                .map(|(actor, _)| *actor)
                .unwrap_or(self.current_player)
        } else {
            self.current_player
        }
    }

    /// Arrays are ordered [viewer, opponent]. Only public historical information is exposed.
    pub fn public_context(&self, viewer: usize) -> PublicContext<'_> {
        assert!(viewer < 2);
        let order = [viewer, 1 - viewer];
        PublicContext {
            end_turn_pending: self.end_turn_pending,
            has_played_support: self.has_played_support,
            has_retreated: self.has_retreated,
            has_used_stadium: order.map(|p| self.has_used_stadium[p]),
            has_used_victory_star: order.map(|p| self.has_used_victory_star[p]),
            has_used_luxury_coin: order.map(|p| self.has_used_luxury_coin[p]),
            knocked_out_this_turn: self.knocked_out_by_opponent_attack_this_turn,
            knocked_out_last_turn: self.knocked_out_by_opponent_attack_last_turn,
            knocked_out_types_this_turn: &self.knocked_out_types_this_turn,
            knocked_out_types_last_turn: &self.knocked_out_types_last_turn,
            attack_name_used_this_turn: order.map(|p| &self.attack_name_used_this_turn[p]),
            attack_name_used_last_turn: order.map(|p| &self.attack_name_used_last_turn[p]),
            attack_name_used_count: order.map(|p| &self.attack_name_used_count[p]),
            own_knocked_out_count: order.map(|p| self.own_knocked_out_count[p]),
            points_at_own_turn_start: order.map(|p| self.points_at_own_turn_start[p]),
            points_gained_during_own_last_turn: order
                .map(|p| self.points_gained_during_own_last_turn[p]),
        }
    }

    /// Scheduled public effects, keyed by absolute turn. Consumers should encode time remaining.
    pub fn public_turn_effects(&self) -> &BTreeMap<u8, Vec<TurnEffect>> {
        &self.turn_effects
    }

    /// The actor has already observed these flips. Never exposes future rolls.
    pub fn observed_coin_reflip(&self, viewer: usize) -> Option<&PendingCoinReflip> {
        self.pending_coin_reflip
            .as_ref()
            .filter(|pending| pending.actor == viewer)
    }

    /// Authoritative cost of an active Pokemon's attack, including cost substitutions/modifiers.
    pub fn effective_attack_cost(&self, player: usize, attack: &Attack) -> Vec<EnergyType> {
        crate::hooks::get_effective_attack_cost(attack, self, player)
    }

    pub fn effective_attached_energy(&self, player: usize, slot: usize) -> Vec<EnergyType> {
        self.in_play_pokemon[player][slot]
            .as_ref()
            .map(|p| p.get_effective_attached_energy(self, player))
            .unwrap_or_default()
    }
}

/// Printed knockout value, using the same rules as knockout resolution.
pub fn knockout_points(card: &Card) -> u8 {
    card.get_knockout_points()
}

impl State {
    /// Agent transport snapshot, not a simulatable state. Hidden card identities are
    /// replaced by placeholders, preserving zone lengths. Only the current legal
    /// choices are retained; future stacked choices may contain undisclosed cards.
    pub fn agent_snapshot(&self, viewer: usize, actions: &[crate::actions::Action]) -> State {
        assert!(viewer < 2 && actions.iter().all(|a| a.actor == viewer));
        let mut snapshot = self.clone();
        let unknown = Card::Trainer(crate::models::TrainerCard {
            id: "UNKNOWN".into(),
            name: "Unknown".into(),
            trainer_card_type: crate::models::TrainerType::Item,
            effect: String::new(),
            rarity: String::new(),
            booster_pack: String::new(),
        });
        snapshot.hands[1 - viewer].fill(unknown.clone());
        for deck in &mut snapshot.decks {
            deck.cards.fill(unknown.clone());
        }
        snapshot.decks[1 - viewer].energy_types.clear();
        for (_, choices) in &mut snapshot.move_generation_stack {
            choices.clear();
        }
        if let Some((actor, choices)) = snapshot.move_generation_stack.last_mut() {
            *actor = viewer;
            *choices = actions.iter().map(|a| a.action.clone()).collect();
        }
        if snapshot
            .pending_coin_reflip
            .as_ref()
            .is_some_and(|p| p.actor != viewer)
        {
            snapshot.pending_coin_reflip = None;
        }
        snapshot
    }
}

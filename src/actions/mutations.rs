use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

use crate::{
    actions::apply_action_helpers::{handle_damage_only, handle_knockouts},
    models::StatusCondition,
    State,
};

use super::{
    apply_action_helpers::{FnMutation, Mutation},
    outcomes::Outcomes,
    Action, SimpleAction,
};

// These functions should share the common code of
// forcing the end of the turn, applying damage with calculations, forcing enemy
// to promote pokemon after knockout, etc... apply to all attacks.

/// A damage target expressed relative to the attacking player. Resolved to an
/// absolute `(player, in_play_idx)` pair once the acting player is known, so
/// that self-targeting effects (e.g. Great Tusk's Shaking Stomp hitting its
/// own Bench) and opponent-targeting effects can share the same plumbing
/// without one being mistaken for the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageTarget {
    /// A Pokémon on the attacker's own side, at the given in-play index.
    SelfPlayer(usize),
    /// A Pokémon on the attacker's opponent's side, at the given in-play index.
    Opponent(usize),
}

impl DamageTarget {
    pub(crate) fn resolve(self, actor: usize) -> (usize, usize) {
        match self {
            DamageTarget::SelfPlayer(idx) => (actor, idx),
            DamageTarget::Opponent(idx) => ((actor + 1) % 2, idx),
        }
    }
}

// Useful for deterministic attacks
pub(crate) fn active_damage_doutcome(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, |_, _, _| {})
}

pub(crate) fn active_damage_effect_doutcome(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> Outcomes {
    damage_effect_doutcome(vec![(damage, DamageTarget::Opponent(0))], additional_effect)
}

pub(crate) fn damage_effect_doutcome<F>(
    targets: Vec<(u32, DamageTarget)>,
    additional_effect: F,
) -> Outcomes
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    Outcomes::single(damage_effect_mutation(targets, additional_effect))
}

// ===== Helper functions for building Mutations
pub(crate) fn active_damage_mutation(damage: u32) -> Mutation {
    damage_effect_mutation(vec![(damage, DamageTarget::Opponent(0))], |_, _, _| {})
}

pub(crate) fn active_damage_effect_mutation(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> Mutation {
    damage_effect_mutation(vec![(damage, DamageTarget::Opponent(0))], additional_effect)
}

pub(crate) fn damage_effect_mutation<F>(
    targets: Vec<(u32, DamageTarget)>,
    additional_effect: F,
) -> Mutation
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    Box::new({
        move |rng, state, action| {
            let attack_name: String = match &action.action {
                SimpleAction::Attack(attack_index) => state.in_play_pokemon[action.actor][0]
                    .as_ref()
                    .expect("Attacking Pokemon must be there if attacking")
                    .card
                    .get_attacks()
                    .get(*attack_index)
                    .unwrap_or_else(|| {
                        panic!("Index must exist if attacking with {}", attack_index)
                    })
                    .title
                    .clone(),
                SimpleAction::UseCopiedAttack {
                    source_player,
                    source_in_play_idx,
                    attack_index,
                    ..
                } => state.in_play_pokemon[*source_player][*source_in_play_idx]
                    .as_ref()
                    .expect("Copied-attack source Pokemon must exist")
                    .card
                    .get_attacks()
                    .get(*attack_index)
                    .unwrap_or_else(|| {
                        panic!(
                            "Copied attack index must exist for source {}:{}",
                            source_player, source_in_play_idx
                        )
                    })
                    .title
                    .clone(),
                _ => panic!("This codepath should come from an attack."),
            };

            let attacking_ref = (action.actor, 0);
            let is_from_active_attack = true;
            handle_damage_only(
                state,
                attacking_ref,
                &targets,
                is_from_active_attack,
                Some(&attack_name),
            );
            additional_effect(rng, state, action);
            handle_knockouts(state, attacking_ref, is_from_active_attack);
        }
    })
}

// ===== Other Helper Functions
pub(crate) fn build_status_effect(status: StatusCondition) -> FnMutation {
    Box::new({
        move |_, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            state.apply_status_condition(opponent, 0, status);
        }
    })
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use crate::{
        actions::SimpleAction, card_ids::CardId, database::get_card_by_enum,
        hooks::to_playable_card,
    };

    use super::*;

    #[test]
    fn test_build_status_effect() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();
        let action = Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        };
        let bulbasuar = get_card_by_enum(CardId::A1001Bulbasaur);
        state.in_play_pokemon[1][0] = Some(to_playable_card(&bulbasuar, false));
        let effect = build_status_effect(StatusCondition::Asleep);
        effect(&mut rng, &mut state, &action);
        assert!(state.get_active(1).is_asleep());
    }

    #[test]
    fn test_arceus_avoids_status() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();
        let action = Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        };
        let arceus = get_card_by_enum(CardId::A2a071ArceusEx);
        state.in_play_pokemon[1][0] = Some(to_playable_card(&arceus, false));
        let effect = build_status_effect(StatusCondition::Asleep);
        effect(&mut rng, &mut state, &action);
        assert!(!state.get_active(1).is_asleep());
    }
}

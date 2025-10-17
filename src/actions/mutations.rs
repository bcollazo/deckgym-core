use log::trace;
use rand::rngs::StdRng;

use crate::{card_ids::CardId, hooks::get_damage_from_attack, models::StatusCondition, State};

use super::{
    apply_action_helpers::{handle_damage, FnMutation, Mutation, Mutations, Probabilities},
    Action,
};

// These functions should share the common code of
// forcing the end of the turn, applying damage with calculations, forcing enemy
// to promote pokemon after knockout, etc... apply to all attacks.

// === Helper functions to build Outcomes (Probabilities, Mutations)
// Doutcome means deterministic outcome
pub(crate) fn doutcome(
    mutation: fn(&mut StdRng, &mut State, &Action),
) -> (Probabilities, Mutations) {
    (
        vec![1.0],
        vec![Box::new(move |rng, state, action| {
            mutation(rng, state, action);
        })],
    )
}

// Useful for abilities
pub(crate) fn ability_doutcome(mutation: Mutation) -> (Probabilities, Mutations) {
    (vec![1.0], vec![mutation])
}

pub(crate) fn ability_mutation<F>(additional_effect: F) -> Mutation
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    Box::new({
        move |rng, state, action| {
            additional_effect(rng, state, action);
        }
    })
}

// Useful for attacks
pub(crate) fn active_damage_doutcome(damage: u32) -> (Probabilities, Mutations) {
    damage_doutcome(vec![(damage, 0)])
}

pub(crate) fn damage_doutcome(targets: Vec<(u32, usize)>) -> (Probabilities, Mutations) {
    (vec![1.0], vec![damage_mutation(targets)])
}

pub(crate) fn active_damage_effect_doutcome(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> (Probabilities, Mutations) {
    (
        vec![1.0],
        vec![active_damage_effect_mutation(damage, additional_effect)],
    )
}

// TODO: Ask for state so that we can get damage via index, before the mutation,
//  and reuse the common mutation code.
pub(crate) fn index_active_damage_doutcome<F>(
    attack_index: usize,
    additional_effect: F,
) -> (Probabilities, Mutations)
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    (
        vec![1.0],
        vec![Box::new(move |rng, state, action| {
            additional_effect(rng, state, action);

            let damage = get_damage_from_attack(state, action.actor, attack_index, 0);
            handle_damage(state, action.actor, &vec![(damage, 0)]);
        })],
    )
}

pub(crate) fn damage_effect_doutcome<F>(
    targets: Vec<(u32, usize)>,
    additional_effect: F,
) -> (Probabilities, Mutations)
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    (
        vec![1.0],
        vec![damage_effect_mutation(targets, additional_effect)],
    )
}

// ===== Helper functions for building Mutations
pub(crate) fn active_damage_mutation(damage: u32) -> Mutation {
    damage_mutation(vec![(damage, 0)])
}

pub(crate) fn damage_mutation(targets: Vec<(u32, usize)>) -> Mutation {
    damage_effect_mutation(targets, |_, _, _| {})
}

pub(crate) fn active_damage_effect_mutation(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> Mutation {
    damage_effect_mutation(vec![(damage, 0)], additional_effect)
}

pub(crate) fn damage_effect_mutation<F>(
    targets: Vec<(u32, usize)>,
    additional_effect: F,
) -> Mutation
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    Box::new({
        move |rng, state, action| {
            additional_effect(rng, state, action);
            handle_damage(state, action.actor, &targets);
        }
    })
}

// ===== Other Helper Functions
pub(crate) fn build_status_effect(status: StatusCondition) -> FnMutation {
    Box::new({
        move |_, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            let opponent_active = state.get_active_mut(opponent);

            // Arceus Ex avoids status effects
            let string_id = opponent_active.get_id();
            let arceus_ids = [
                CardId::A2a071ArceusEx,
                CardId::A2a086ArceusEx,
                CardId::A2a095ArceusEx,
                CardId::A2a096ArceusEx,
            ];
            let card_id = CardId::from_card_id(&string_id).unwrap();
            if arceus_ids.contains(&card_id) {
                trace!("Arceus Ex avoids status effect");
                return;
            }

            match status {
                StatusCondition::Asleep => opponent_active.asleep = true,
                StatusCondition::Paralyzed => opponent_active.paralyzed = true,
                StatusCondition::Poisoned => opponent_active.poisoned = true,
            }
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
        assert!(state.get_active(1).asleep);
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
        assert!(!state.get_active(1).asleep);
    }
}

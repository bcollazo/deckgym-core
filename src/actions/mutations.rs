use rand::rngs::StdRng;

use crate::{models::StatusCondition, State};

use super::{
    apply_action_helpers::{handle_damage, FnMutation, Mutation, Mutations, Probabilities},
    Action,
};

// These functions should share the common code of
// forcing the end of the turn, applying damage with calculations, forcing enemy
// to promote pokemon after knockout, etc... apply to all attacks.

// === Helper functions to build Outcomes = (Probabilities, Mutations)
// Doutcome means deterministic outcome
pub(crate) fn doutcome(
    mutation: fn(&mut StdRng, &mut State, &Action),
) -> (Probabilities, Mutations) {
    doutcome_from_mutation(Box::new(mutation))
}

pub(crate) fn doutcome_from_mutation(mutation: Mutation) -> (Probabilities, Mutations) {
    (vec![1.0], vec![mutation])
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
            let opponent = (action.actor + 1) % 2;
            let targets: Vec<(u32, usize, usize)> = targets
                .iter()
                .map(|(damage, in_play_idx)| (*damage, opponent, *in_play_idx))
                .collect();
            handle_damage(state, (action.actor, 0), &targets, true);
        }
    })
}

// ===== Other Helper Functions
pub(crate) fn build_status_effect(status: StatusCondition) -> FnMutation {
    Box::new({
        move |_, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            let opponent_active = state.get_active_mut(opponent);
            opponent_active.apply_status_condition(status);
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

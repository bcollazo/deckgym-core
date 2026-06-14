use rand::rngs::StdRng;

use crate::{models::StatusCondition, State};

use super::{
    apply_action_helpers::FnMutation,
    attack_outcome::{AttackOutcome, AttackOutcomes, DamageTarget},
    Action,
};

// These functions build the structured `AttackOutcome`/`AttackOutcomes` for attacks. Damage is
// carried as data (in the `damage` field) rather than baked into the effect closures, so that the
// shared resolution path (`AttackOutcome::into_mutation`) can apply damage with the usual
// modifiers/counterattacks/knockouts, and so that the defender's coin-flip damage prevention can
// strip only the active Pokémon's damage while still running effects.

// Useful for deterministic attacks
pub(crate) fn active_damage_doutcome(damage: u32) -> AttackOutcomes {
    AttackOutcomes::single(active_damage_outcome(damage))
}

pub(crate) fn active_damage_effect_doutcome(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> AttackOutcomes {
    AttackOutcomes::single(active_damage_effect_outcome(damage, additional_effect))
}

/// `targets` is a list of `(damage, is_opponent_target, in_play_idx)`, where
/// `is_opponent_target` indicates whether `in_play_idx` refers to a slot on
/// the attacker's opponent's side (true) or the attacker's own side (false).
pub(crate) fn damage_effect_doutcome<F>(
    targets: Vec<DamageTarget>,
    additional_effect: F,
) -> AttackOutcomes
where
    F: Fn(&mut StdRng, &mut State, &Action) + 'static,
{
    AttackOutcomes::single(AttackOutcome::damage_then_effect(
        targets,
        additional_effect,
    ))
}

// ===== Helper functions for building single AttackOutcome branches
pub(crate) fn active_damage_outcome(damage: u32) -> AttackOutcome {
    AttackOutcome::damage(vec![(damage, true, 0)])
}

pub(crate) fn active_damage_effect_outcome(
    damage: u32,
    additional_effect: impl Fn(&mut StdRng, &mut State, &Action) + 'static,
) -> AttackOutcome {
    AttackOutcome::damage_then_effect(vec![(damage, true, 0)], additional_effect)
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

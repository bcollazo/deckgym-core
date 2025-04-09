use std::panic;

use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::StdRng};

use crate::{
    hooks::{get_retreat_cost, on_attach_tool, to_playable_card},
    state::State,
    types::{Card, PlayedCard},
};

use super::{
    apply_abilities_action::apply_abilities_action,
    apply_action_helpers::{
        apply_common_mutation, forecast_end_turn, handle_attack_damage, Mutations, Probabilities,
    },
    apply_attack_action::forecast_attack,
    apply_trainer_action::forecast_trainer_action,
    Action, SimpleAction,
};

/// Main function to mutate the state based on the action. It forecasts the possible outcomes
/// and then chooses one of them to apply. This is so that bot implementations can re-use the
/// `forecast_action` function.
pub fn apply_action(rng: &mut StdRng, state: &mut State, action: &Action) {
    let (probabilities, mut lazy_mutations) = forecast_action(state, action);
    if probabilities.len() == 1 {
        lazy_mutations.remove(0)(rng, state, action);
    } else {
        let dist = WeightedIndex::new(&probabilities).unwrap();
        let chosen_index = dist.sample(rng);
        lazy_mutations.remove(chosen_index)(rng, state, action);
    }
}

/// This should be mostly a "router" function that calls the appropriate forecast function
/// based on the action type.
pub fn forecast_action(state: &State, action: &Action) -> (Probabilities, Mutations) {
    match &action.action {
        // Deterministic Actions
        SimpleAction::DrawCard // TODO: DrawCard should return actual deck probabilities.
        | SimpleAction::Place(_, _)
        | SimpleAction::Attach { .. }
        | SimpleAction::AttachTool { .. }
        | SimpleAction::Evolve(_, _)
        | SimpleAction::UseAbility(_)
        | SimpleAction::Activate { .. }
        | SimpleAction::Retreat(_)
        | SimpleAction::ApplyDamage { .. }
        | SimpleAction::Heal { .. } => (
            vec![1.0],
            vec![Box::new({
                |_, mutable_state, action| {
                    apply_deterministic_action(mutable_state, action);
                }
            })],
        ),
        SimpleAction::Attack(index) => forecast_attack(action.actor, state, *index),
        SimpleAction::Play { trainer_card } => {
            forecast_trainer_action(action.actor, state, trainer_card)
        }
        // acting_player is not passed here, because there is only 1 turn to end. The current turn.
        SimpleAction::EndTurn => forecast_end_turn(state),
    }
}

fn apply_deterministic_action(state: &mut State, action: &Action) {
    apply_common_mutation(state, action);

    match &action.action {
        SimpleAction::DrawCard => {
            state.maybe_draw_card(action.actor);
        }
        SimpleAction::Attach {
            attachments,
            is_turn_energy,
        } => {
            for (amount, energy, in_play_idx) in attachments {
                state.in_play_pokemon[action.actor][*in_play_idx]
                    .as_mut()
                    .expect("Pokemon should be there if attaching energy to it")
                    .attached_energy
                    .extend(std::iter::repeat_n(*energy, *amount as usize));
            }
            if *is_turn_energy {
                state.current_energy = None;
            }
        }
        SimpleAction::AttachTool {
            in_play_idx,
            tool_id,
        } => {
            state.in_play_pokemon[action.actor][*in_play_idx]
                .as_mut()
                .expect("Pokemon should be there if attaching tool to it")
                .attached_tool = Some(*tool_id);
            on_attach_tool(state, action.actor, *in_play_idx, *tool_id);
        }
        SimpleAction::Place(card, index) => {
            let played_card = to_playable_card(card, true);
            state.in_play_pokemon[action.actor][*index] = Some(played_card);
            state.remove_card_from_hand(action.actor, card);
        }
        SimpleAction::Evolve(card, position) => {
            apply_evolve(action.actor, state, card, *position);
        }
        SimpleAction::UseAbility(position) => {
            apply_abilities_action(action.actor, state, *position);
        }
        SimpleAction::Activate { in_play_idx } => {
            apply_retreat(action.actor, state, *in_play_idx, true);
        }
        SimpleAction::Retreat(position) => {
            apply_retreat(action.actor, state, *position, false);
        }
        SimpleAction::ApplyDamage { targets } => {
            handle_attack_damage(state, action.actor, targets);
        }
        // Trainer-Specific Actions
        SimpleAction::Heal {
            in_play_idx,
            amount,
        } => {
            apply_healing(action.actor, state, *in_play_idx, *amount);
        }
        _ => panic!("Deterministic Action expected"),
    }
}

fn apply_healing(acting_player: usize, state: &mut State, position: usize, amount: u32) {
    let active = state.in_play_pokemon[acting_player][position]
        .as_mut()
        .expect("Pokemon should be there if applying potion to it");
    active.heal(amount);
}

fn apply_retreat(acting_player: usize, state: &mut State, bench_idx: usize, is_free: bool) {
    if !is_free {
        let active = state.in_play_pokemon[acting_player][0]
            .as_ref()
            .expect("Active Pokemon should be there if paid retreating");
        let retreat_cost = get_retreat_cost(state, active);
        let attached_energy: &mut Vec<_> = state.in_play_pokemon[acting_player][0]
            .as_mut()
            .expect("Active Pokemon should be there if paid retreating")
            .attached_energy
            .as_mut();

        // TODO: Maybe give option to user to select which energy to discard
        let count = retreat_cost.len();
        attached_energy.truncate(attached_energy.len() - count);
    }

    state.in_play_pokemon[acting_player].swap(0, bench_idx);

    // Cure any status conditions
    if let Some(pokemon) = &state.in_play_pokemon[acting_player][bench_idx] {
        state.in_play_pokemon[acting_player][bench_idx] = Some(PlayedCard {
            poisoned: false,
            paralyzed: false,
            asleep: false,
            ..pokemon.clone()
        });
    }

    state.has_retreated = true;
}

// We will replace the PlayedCard, but taking into account the attached energy
//  and the remaining HP.
fn apply_evolve(acting_player: usize, state: &mut State, card: &Card, position: usize) {
    // This removes status conditions
    let mut played_card = to_playable_card(card, true);

    let old_pokemon = state.in_play_pokemon[acting_player][position]
        .as_ref()
        .expect("Pokemon should be there if evolving it");
    if let Card::Pokemon(pokemon_card) = &played_card.card {
        if pokemon_card.stage == 0 {
            panic!("Only stage 1 or 2 pokemons can be evolved");
        }

        let damage_taken = old_pokemon.total_hp - old_pokemon.remaining_hp;
        played_card.remaining_hp -= damage_taken;
        played_card.attached_energy = old_pokemon.attached_energy.clone();
        played_card.cards_behind = old_pokemon.cards_behind.clone();
        played_card.cards_behind.push(old_pokemon.card.clone());
        state.in_play_pokemon[acting_player][position] = Some(played_card);
    } else {
        panic!("Only Pokemon cards can be evolved");
    }
    state.remove_card_from_hand(acting_player, card);
    // NOTE: Phantomly leave the Stage 0 card behind the newly evolved card
}

// Test that when evolving a damanged pokemon, damage stays.
#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::card_ids::CardId;
    use crate::database::get_card_by_enum;
    use crate::types::PlayedCard;
    use crate::{types::EnergyType, Deck};

    #[test]
    fn test_apply_evolve() {
        let mut state = State::new(&Deck::default(), &Deck::default());
        let energy = EnergyType::Colorless;
        let mankey = get_card_by_enum(CardId::PA017Mankey);
        let primeape = get_card_by_enum(CardId::A1142Primeape);
        let mut base_played_card = to_playable_card(&mankey, false);
        base_played_card.remaining_hp = 20; // 30 damage taken
        base_played_card.attached_energy = vec![energy];
        state.in_play_pokemon[0][0] = Some(base_played_card.clone());
        let mut healthy_bench = base_played_card.clone();
        healthy_bench.remaining_hp = 50;
        healthy_bench.attached_energy = vec![energy, energy, energy];
        state.in_play_pokemon[0][2] = Some(healthy_bench);
        state.hands[0] = vec![primeape.clone(), primeape.clone()];

        // Evolve Active
        apply_evolve(0, &mut state, &primeape, 0);
        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(PlayedCard {
                card: primeape.clone(),
                remaining_hp: 60, // 90 - 30 = 60
                total_hp: 90,
                attached_energy: vec![energy],
                attached_tool: None,
                played_this_turn: true,
                ability_used: false,
                poisoned: false,
                paralyzed: false,
                asleep: false,
                cards_behind: vec![mankey.clone()]
            })
        );

        // Evolve Bench
        apply_evolve(0, &mut state, &primeape, 2);
        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(PlayedCard {
                card: primeape.clone(),
                remaining_hp: 60, // 90 - 30 = 60
                total_hp: 90,
                attached_energy: vec![energy],
                attached_tool: None,
                played_this_turn: true,
                ability_used: false,
                poisoned: false,
                paralyzed: false,
                asleep: false,
                cards_behind: vec![mankey.clone()]
            })
        );
        assert_eq!(
            state.in_play_pokemon[0][2],
            Some(PlayedCard {
                card: primeape.clone(),
                remaining_hp: 90, // 90 - 0 = 90
                total_hp: 90,
                attached_energy: vec![energy, energy, energy],
                attached_tool: None,
                played_this_turn: true,
                ability_used: false,
                poisoned: false,
                paralyzed: false,
                asleep: false,
                cards_behind: vec![mankey.clone()]
            })
        );
    }

    #[test]
    fn test_forcefully_retreat() {
        let mut state = State::new(&Deck::default(), &Deck::default());
        // PUT Mankey in Active and Primeape in Bench 2
        let mankey = get_card_by_enum(CardId::A1141Mankey);
        let primeape = get_card_by_enum(CardId::A1142Primeape);
        state.in_play_pokemon[0][0] = Some(to_playable_card(&mankey, false));
        state.in_play_pokemon[0][2] = Some(to_playable_card(&primeape, false));

        // Forcefully Activate Primeape
        let mut rng: StdRng = StdRng::seed_from_u64(rand::random());
        let action = Action {
            actor: 0,
            action: SimpleAction::Activate { in_play_idx: 2 },
            is_stack: false,
        };
        apply_action(&mut rng, &mut state, &action);

        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(to_playable_card(&primeape, false))
        );
        assert_eq!(
            state.in_play_pokemon[0][2],
            Some(to_playable_card(&mankey, false))
        );
    }
}

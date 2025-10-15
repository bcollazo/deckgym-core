mod attacks;
mod move_generation_abilities;
mod move_generation_trainer;

use crate::actions::{Action, SimpleAction};
use crate::hooks::{can_retreat, contains_energy, get_retreat_cost};
use crate::models::Card;
use crate::state::State;

use attacks::generate_attack_actions;
use move_generation_abilities::generate_ability_actions;
pub use move_generation_trainer::{
    generate_possible_trainer_actions, trainer_move_generation_implementation,
};

/// Generates a list of possible moves for the current player.
///
/// # Arguments
/// * `state` - The current state of the game.
///
/// # Returns
/// * A tuple containing the current player and a list of possible actions
pub fn generate_possible_actions(state: &State) -> (usize, Vec<Action>) {
    let in_initial_setup_phase = state.turn_count == 0;
    if in_initial_setup_phase {
        let possible_actions = generate_initial_setup_actions(state)
            .iter()
            .map(|action| Action {
                actor: state.current_player,
                action: action.clone(),
                is_stack: false,
            })
            .collect();
        return (state.current_player, possible_actions);
    }

    // If there are moves in the generation stack, short-circuit to that
    if let Some((actor, possible_actions)) = state.move_generation_stack.last() {
        let actions = possible_actions
            .iter()
            .map(|action| Action {
                actor: *actor,
                action: action.clone(),
                is_stack: true,
            })
            .collect();
        return (*actor, actions);
    }

    // Free play actions. User can always end turn.
    let current_player = state.current_player;
    let mut actions = vec![SimpleAction::EndTurn];

    // Hand actions (Play Support Cards, Trainer, or Place Pokemons in mat)
    let hand_actions = generate_hand_actions(state);
    actions.extend(hand_actions);

    // Maybe attach energy to in play cards
    if let Some(energy) = state.current_energy {
        state.in_play_pokemon[current_player]
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                if x.is_some() {
                    actions.push(SimpleAction::Attach {
                        attachments: vec![(1, energy, i)],
                        is_turn_energy: true,
                    });
                }
            })
    }

    // Maybe retreat pokemon
    if let Some(card) = &state.in_play_pokemon[current_player][0] {
        if can_retreat(state)
            && contains_energy(card, &get_retreat_cost(state, card), state, current_player)
        {
            state
                .enumerate_bench_pokemon(current_player)
                .for_each(|(i, _)| {
                    actions.push(SimpleAction::Retreat(i));
                });
        }
    }

    // Maybe attack (only starting on turn 2)
    let attack_actions = generate_attack_actions(state);
    actions.extend(attack_actions);

    // Add actions given by abilities
    let ability_actions = generate_ability_actions(state);
    actions.extend(ability_actions);

    let possible_actions = actions
        .iter()
        .map(|action| Action {
            actor: current_player,
            action: action.clone(),
            is_stack: false,
        })
        .collect();
    (current_player, possible_actions)
}

fn generate_initial_setup_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let hand_actions = generate_hand_actions(state);
    if state.in_play_pokemon[current_player][0].is_none() {
        let place_active_actions: Vec<SimpleAction> = hand_actions
            .iter()
            .filter(|x| matches!(x, SimpleAction::Place(_, 0)))
            .cloned()
            .collect();
        place_active_actions
    } else {
        let mut actions = Vec::new();
        let place_bench_actions: Vec<SimpleAction> = hand_actions
            .iter()
            .filter(|x| {
                if let SimpleAction::Place(_, position) = x {
                    *position != 0
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        actions.extend(place_bench_actions);
        actions.push(SimpleAction::EndTurn);
        actions
    }
}

fn generate_hand_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let mut actions = Vec::new();

    get_current_hand(state)
        .iter()
        .for_each(|hand_card| match hand_card {
            Card::Pokemon(pokemon_card) => {
                // Basic pokemons can be placed in empty Active or Bench slots
                if pokemon_card.stage == 0 {
                    state.in_play_pokemon[current_player]
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| {
                            if x.is_none() {
                                actions.push(SimpleAction::Place(hand_card.clone(), i));
                            }
                        });
                } else {
                    // Evolutions can only be played if previous stage
                    // is there, and wasn't played this turn, and isn't the first 2 turns.
                    if state.is_users_first_turn() {
                        return;
                    }
                    // For each non-zero stage pokemon in hand, check if the evolves_from is in play
                    // if so, add evolve action
                    if let Some(evolves_from) = &pokemon_card.evolves_from {
                        state.in_play_pokemon[current_player]
                            .iter()
                            .enumerate()
                            .for_each(|(i, x)| {
                                if let Some(pokemon) = x {
                                    if !pokemon.played_this_turn
                                        && pokemon.get_name() == *evolves_from
                                    {
                                        actions.push(SimpleAction::Evolve(hand_card.clone(), i));
                                    }
                                }
                            });
                    }
                }
            }
            Card::Trainer(trainer_card) => {
                let trainer_actions = generate_possible_trainer_actions(state, trainer_card)
                    .expect("Trainer card not implemented");
                actions.extend(trainer_actions);
            }
        });
    actions
}

fn get_current_hand(state: &State) -> &Vec<Card> {
    &state.hands[state.current_player]
}

mod attacks;
mod move_generation_abilities;
mod move_generation_trainer;

use crate::actions::{Action, SimpleAction};
use crate::hooks::{can_evolve_into, can_retreat, contains_energy, get_retreat_cost};
use crate::models::Card;
use crate::state::State;
use crate::AbilityId;

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

    // Maybe discard fossils (fossils can be discarded at any time during your turn)
    generate_discard_fossil_actions(state, &mut actions);

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
                    // Exception: Eevee with Boosted Evolution ability can evolve on first turn
                    // or turn it was played, if it's in the active spot.

                    // Check if we should skip evolution checks due to first turn
                    // (unless there's a Boosted Evolution Eevee in active spot)
                    let has_boosted_evolution_in_active = state.in_play_pokemon[current_player][0]
                        .as_ref()
                        .and_then(|active| AbilityId::from_pokemon_id(&active.card.get_id()[..]))
                        .map(|id| id == AbilityId::B1184EeveeBoostedEvolution)
                        .unwrap_or(false);

                    if state.is_users_first_turn() && !has_boosted_evolution_in_active {
                        return;
                    }

                    // For each non-zero stage pokemon in hand, check if it can evolve
                    // from any pokemon in play (using can_evolve_into which handles special abilities)
                    state
                        .enumerate_in_play_pokemon(current_player)
                        .for_each(|(i, pokemon)| {
                            // Check if this pokemon has Boosted Evolution and is in active spot
                            let can_bypass_timing = i == 0
                                && AbilityId::from_pokemon_id(&pokemon.card.get_id()[..])
                                    .map(|id| id == AbilityId::B1184EeveeBoostedEvolution)
                                    .unwrap_or(false);

                            if (!pokemon.played_this_turn || can_bypass_timing)
                                && can_evolve_into(hand_card, pokemon)
                                && can_evolve_at_position(state, current_player, i)
                            {
                                actions.push(SimpleAction::Evolve {
                                    evolution: hand_card.clone(),
                                    in_play_idx: i,
                                    from_deck: false, // Normal evolution from hand
                                });
                            }
                        });
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

fn generate_discard_fossil_actions(state: &State, actions: &mut Vec<SimpleAction>) {
    let current_player = state.current_player;

    // Check all in-play pokemon to see if any are fossils
    state
        .enumerate_in_play_pokemon(current_player)
        .for_each(|(i, pokemon)| {
            if pokemon.is_fossil() {
                actions.push(SimpleAction::DiscardFossil { in_play_idx: i });
            }
        });
}

/// Checks if evolution is allowed at the given position, considering ability restrictions
fn can_evolve_at_position(state: &State, player: usize, position: usize) -> bool {
    // Aerodactyl Ex's Primeval Law blocks evolution of opponent's active Pokemon
    if position == 0 && has_opponent_aerodactyl_ex_primeval_law(state, player) {
        return false;
    }
    true
}

fn has_opponent_aerodactyl_ex_primeval_law(state: &State, player: usize) -> bool {
    let opponent = (player + 1) % 2;

    // Check if opponent has any Aerodactyl Ex with Primeval Law ability in play
    state
        .enumerate_in_play_pokemon(opponent)
        .any(|(_, pokemon)| {
            AbilityId::from_pokemon_id(&pokemon.get_id()[..])
                .map(|id| id == AbilityId::A1a046AerodactylExPrimevalLaw)
                .unwrap_or(false)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{card_ids::CardId, database::get_card_by_enum, hooks::to_playable_card};

    #[test]
    fn test_aerodactyl_evolve_action_appears_with_fossil_in_mat() {
        // Setup: Player has Old Amber in active spot and Aerodactyl in hand
        let mut state = State::default();
        state.turn_count = 3; // After first 2 turns (evolution allowed)

        // Place Old Amber fossil in active spot
        let old_amber = get_card_by_enum(CardId::A1218OldAmber);
        let played_old_amber = to_playable_card(&old_amber, false);
        state.in_play_pokemon[0][0] = Some(played_old_amber.clone());

        // Add Aerodactyl to hand
        let aerodactyl = get_card_by_enum(CardId::A1210Aerodactyl);
        state.hands[0].push(aerodactyl.clone());

        // Debug output
        println!("Old Amber name: {}", played_old_amber.get_name());
        println!(
            "Old Amber played_this_turn: {}",
            played_old_amber.played_this_turn
        );
        if let Card::Pokemon(poke) = &aerodactyl {
            println!("Aerodactyl evolves_from: {:?}", poke.evolves_from);
        }
        println!(
            "can_evolve_into result: {}",
            can_evolve_into(&aerodactyl, &played_old_amber)
        );

        // Generate actions
        let hand_actions = generate_hand_actions(&state);

        println!("Generated {} hand actions", hand_actions.len());
        for action in &hand_actions {
            println!("Action: {}", action);
        }

        // Check that an Evolve action is present
        let has_evolve_action = hand_actions.iter().any(|action| {
            matches!(
                action,
                SimpleAction::Evolve { evolution, in_play_idx: 0, .. } if evolution.get_id() == aerodactyl.get_id()
            )
        });

        assert!(
            has_evolve_action,
            "Should be able to evolve Old Amber into Aerodactyl"
        );
    }

    #[test]
    fn test_aerodactyl_ex_evolve_action_appears_with_fossil_in_mat() {
        // Setup: Player has Old Amber in bench and Aerodactyl ex in hand
        let mut state = State::default();
        state.turn_count = 3; // After first 2 turns (evolution allowed)

        // Place Old Amber fossil in bench slot 1
        let old_amber = get_card_by_enum(CardId::A1218OldAmber);
        let played_old_amber = to_playable_card(&old_amber, false);
        state.in_play_pokemon[0][1] = Some(played_old_amber);

        // Add Aerodactyl ex to hand
        let aerodactyl_ex = get_card_by_enum(CardId::A1a046AerodactylEx);
        state.hands[0].push(aerodactyl_ex.clone());

        // Generate actions
        let hand_actions = generate_hand_actions(&state);

        // Check that an Evolve action is present for bench position
        let has_evolve_action = hand_actions.iter().any(|action| {
            matches!(
                action,
                SimpleAction::Evolve { evolution, in_play_idx: 1, .. } if evolution.get_id() == aerodactyl_ex.get_id()
            )
        });

        assert!(
            has_evolve_action,
            "Should be able to evolve Old Amber into Aerodactyl ex"
        );
    }

    #[test]
    fn test_aerodactyl_ex_blocks_opponent_active_evolution() {
        // Setup: Player 0 has Aerodactyl ex in play, Player 1 has Bulbasaur and Ivysaur in hand
        let mut state = State::default();
        state.turn_count = 3; // After first 2 turns
        state.current_player = 1; // Player 1's turn

        // Place Aerodactyl ex for player 0
        let aerodactyl_ex = get_card_by_enum(CardId::A1a046AerodactylEx);
        let played_aerodactyl = to_playable_card(&aerodactyl_ex, false);
        state.in_play_pokemon[0][0] = Some(played_aerodactyl);

        // Place Bulbasaur in player 1's active spot
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        let played_bulbasaur = to_playable_card(&bulbasaur, false);
        state.in_play_pokemon[1][0] = Some(played_bulbasaur);

        // Add Ivysaur to player 1's hand
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
        state.hands[1].push(ivysaur.clone());

        // Generate actions for player 1
        let hand_actions = generate_hand_actions(&state);

        // Check that NO Evolve action is present for active position
        let has_active_evolve = hand_actions.iter().any(|action| {
            matches!(
                action,
                SimpleAction::Evolve { evolution, in_play_idx: 0, .. } if evolution.get_id() == ivysaur.get_id()
            )
        });

        assert!(
            !has_active_evolve,
            "Aerodactyl ex's Primeval Law should block active Pokemon evolution"
        );
    }

    #[test]
    fn test_aerodactyl_ex_does_not_block_bench_evolution() {
        // Setup: Player 0 has Aerodactyl ex, Player 1 has Bulbasaur on bench
        let mut state = State::default();
        state.turn_count = 3; // After first 2 turns
        state.current_player = 1;

        // Place Aerodactyl ex for player 0
        let aerodactyl_ex = get_card_by_enum(CardId::A1a046AerodactylEx);
        let played_aerodactyl = to_playable_card(&aerodactyl_ex, false);
        state.in_play_pokemon[0][0] = Some(played_aerodactyl);

        // Place a different pokemon in active for player 1
        let charmander = get_card_by_enum(CardId::A1033Charmander);
        state.in_play_pokemon[1][0] = Some(to_playable_card(&charmander, false));

        // Place Bulbasaur on player 1's bench
        let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
        let played_bulbasaur = to_playable_card(&bulbasaur, false);
        state.in_play_pokemon[1][1] = Some(played_bulbasaur);

        // Add Ivysaur to player 1's hand
        let ivysaur = get_card_by_enum(CardId::A1002Ivysaur);
        state.hands[1].push(ivysaur.clone());

        // Generate actions
        let hand_actions = generate_hand_actions(&state);

        // Check that Evolve action IS present for bench position
        let has_bench_evolve = hand_actions.iter().any(|action| {
            matches!(
                action,
                SimpleAction::Evolve { evolution, in_play_idx: 1, .. } if evolution.get_id() == ivysaur.get_id()
            )
        });

        assert!(
            has_bench_evolve,
            "Aerodactyl ex's Primeval Law should NOT block bench Pokemon evolution"
        );
    }
}

use std::{collections::HashMap, panic};

use log::debug;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::StdRng};

use crate::{
    actions::{
        apply_abilities_action::forecast_ability,
        apply_action_helpers::{apply_common_mutation, Mutation},
    },
    hooks::{get_retreat_cost, on_attach_energy, on_attach_tool, on_evolve, to_playable_card},
    models::{Card, EnergyType},
    state::State,
};

use super::{
    apply_action_helpers::{forecast_end_turn, handle_damage, Mutations, Probabilities},
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
    let (proba, mutas) = match &action.action {
        // Deterministic Actions
        SimpleAction::DrawCard { .. } // TODO: DrawCard should return actual deck probabilities.
        | SimpleAction::Place(_, _)
        | SimpleAction::Attach { .. }
        | SimpleAction::MoveEnergy { .. }
        | SimpleAction::AttachTool { .. }
        | SimpleAction::Evolve(_, _)
        | SimpleAction::Activate { .. }
        | SimpleAction::Retreat(_)
        | SimpleAction::ApplyDamage { .. }
        | SimpleAction::Heal { .. }
        | SimpleAction::ApplyEeveeBagDamageBoost
        | SimpleAction::HealAllEeveeEvolutions
        | SimpleAction::Noop => forecast_deterministic_action(),
        SimpleAction::UseAbility { in_play_idx } => forecast_ability(state, action, *in_play_idx),
        SimpleAction::Attack(index) => forecast_attack(action.actor, state, *index),
        SimpleAction::Play { trainer_card } => {
            forecast_trainer_action(action.actor, state, trainer_card)
        }
        SimpleAction::CommunicatePokemon { hand_pokemon } => {
            forecast_pokemon_communication(action.actor, state, hand_pokemon)
        }
        SimpleAction::ShuffleOpponentSupporter { supporter_card } => {
            forecast_shuffle_opponent_supporter(action.actor, supporter_card)
        }
        SimpleAction::AttachFromDiscard {
            in_play_idx,
            num_random_energies,
        } => forecast_attach_from_discard(state, action.actor, *in_play_idx, *num_random_energies),
        // acting_player is not passed here, because there is only 1 turn to end. The current turn.
        SimpleAction::EndTurn => forecast_end_turn(state),
    };

    let mut wrapped_mutations: Mutations = vec![];
    for original_mutation in mutas {
        let mutation_closure: Mutation = Box::new(original_mutation);
        wrapped_mutations.push(Box::new(move |rng, state, action| {
            apply_common_mutation(state, action);
            mutation_closure(rng, state, action);
        }));
    }
    (proba, wrapped_mutations)
}

fn forecast_deterministic_action() -> (Probabilities, Mutations) {
    (
        vec![1.0],
        vec![Box::new(move |_, state, action| {
            apply_deterministic_action(state, action);
        })],
    )
}

fn apply_deterministic_action(state: &mut State, action: &Action) {
    match &action.action {
        SimpleAction::DrawCard { .. } => {
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
                // Call hook for each energy attached
                for _ in 0..*amount {
                    on_attach_energy(state, action.actor, *in_play_idx, *energy, *is_turn_energy);
                }
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
        SimpleAction::MoveEnergy {
            from_in_play_idx,
            to_in_play_idx,
            energy,
        } => {
            let actor_board = &mut state.in_play_pokemon[action.actor];
            let mut removed = false;
            if let Some(from_card) = actor_board[*from_in_play_idx].as_mut() {
                if let Some(pos) = from_card.attached_energy.iter().position(|e| e == energy) {
                    from_card.attached_energy.swap_remove(pos);
                    removed = true;
                }
            }
            if removed {
                if let Some(to_card) = actor_board[*to_in_play_idx].as_mut() {
                    to_card.attached_energy.push(*energy);
                } else if let Some(from_card) = actor_board[*from_in_play_idx].as_mut() {
                    // Put energy back if destination vanished (should not normally happen)
                    from_card.attached_energy.push(*energy);
                }
            }
        }
        SimpleAction::Place(card, index) => {
            let played_card = to_playable_card(card, true);
            state.in_play_pokemon[action.actor][*index] = Some(played_card);
            state.remove_card_from_hand(action.actor, card);
        }
        SimpleAction::Evolve(card, position) => {
            apply_evolve(action.actor, state, card, *position);
        }
        SimpleAction::Activate { in_play_idx } => {
            apply_retreat(action.actor, state, *in_play_idx, true);
        }
        SimpleAction::Retreat(position) => {
            apply_retreat(action.actor, state, *position, false);
        }
        SimpleAction::ApplyDamage {
            target_player,
            targets,
            is_from_active_attack,
        } => {
            handle_damage(state, *target_player, targets, *is_from_active_attack);
        }
        // Trainer-Specific Actions
        SimpleAction::Heal {
            in_play_idx,
            amount,
            cure_status,
        } => {
            apply_healing(action.actor, state, *in_play_idx, *amount, *cure_status);
        }
        SimpleAction::ApplyEeveeBagDamageBoost => {
            apply_eevee_bag_damage_boost(state);
        }
        SimpleAction::HealAllEeveeEvolutions => {
            apply_heal_all_eevee_evolutions(action.actor, state);
        }
        SimpleAction::Noop => {}
        _ => panic!("Deterministic Action expected"),
    }
}

fn apply_healing(
    acting_player: usize,
    state: &mut State,
    position: usize,
    amount: u32,
    cure_status: bool,
) {
    let pokemon = state.in_play_pokemon[acting_player][position]
        .as_mut()
        .expect("Pokemon should be there if healing it");
    pokemon.heal(amount);
    if cure_status {
        pokemon.cure_status_conditions();
    }
}

/// is_free is analogous to "via retreat". If false, its because this comes from an Activate.
/// Note: This might be called when a K.O. happens, so can't assume there is an active...
fn apply_retreat(acting_player: usize, state: &mut State, bench_idx: usize, is_free: bool) {
    if !is_free {
        let active = state.in_play_pokemon[acting_player][0]
            .as_ref()
            .expect("Active Pokemon should be there if paid retreating");
        let double_grass = active.has_double_grass(state, acting_player);
        let retreat_cost = get_retreat_cost(state, active).len();
        let attached_energy: &mut Vec<_> = state.in_play_pokemon[acting_player][0]
            .as_mut()
            .expect("Active Pokemon should be there if paid retreating")
            .attached_energy
            .as_mut();

        // TODO: Maybe give option to user to select which energy to discard

        // Some energies are worth more than others... For now decide the ordering
        // that keeps as much Grass energy as possible (since possibly worth more).

        // Re-order energies so that Grass are at the beginning
        attached_energy.sort_by(|a, b| {
            if *a == EnergyType::Grass && *b != EnergyType::Grass {
                std::cmp::Ordering::Less
            } else if *a != EnergyType::Grass && *b == EnergyType::Grass {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // Start walking from the back in the attached, removing energies until retreat cost is paid
        let mut remaining_cost = retreat_cost;
        while remaining_cost > 0 && !attached_energy.is_empty() {
            let energy = attached_energy.pop().unwrap();
            if energy == EnergyType::Grass && double_grass {
                remaining_cost = remaining_cost.saturating_sub(2);
            } else {
                remaining_cost = remaining_cost.saturating_sub(1);
            }
        }
        if remaining_cost > 0 {
            panic!("Not enough energy to pay retreat cost");
        }
    }

    state.in_play_pokemon[acting_player].swap(0, bench_idx);

    // Clear status and effects of the new bench Pokemon
    if let Some(pokemon) = state.in_play_pokemon[acting_player][bench_idx].as_mut() {
        pokemon.clear_status_and_effects();
    }

    state.has_retreated = true;
}

// We will replace the PlayedCard, but taking into account the attached energy
//  and the remaining HP.
fn apply_evolve(acting_player: usize, state: &mut State, to_card: &Card, position: usize) {
    // This removes status conditions
    let mut played_card = to_playable_card(to_card, true);

    let from_pokemon = state.in_play_pokemon[acting_player][position]
        .as_ref()
        .expect("Pokemon should be there if evolving it");
    if let Card::Pokemon(to_pokemon) = &played_card.card {
        if to_pokemon.stage == 0 {
            panic!("Basic pokemon do not evolve from others...");
        }

        let damage_taken = from_pokemon.total_hp - from_pokemon.remaining_hp;
        played_card.remaining_hp -= damage_taken;
        played_card.attached_energy = from_pokemon.attached_energy.clone();
        played_card.cards_behind = from_pokemon.cards_behind.clone();
        played_card.cards_behind.push(from_pokemon.card.clone());
        state.in_play_pokemon[acting_player][position] = Some(played_card);
    } else {
        panic!("Only Pokemon cards can be evolved");
    }
    state.remove_card_from_hand(acting_player, to_card);

    // Run special logic hooks on evolution
    on_evolve(acting_player, state, to_card)
}

fn forecast_pokemon_communication(
    acting_player: usize,
    state: &State,
    hand_pokemon: &Card,
) -> (Probabilities, Mutations) {
    let deck_pokemon: Vec<_> = state.decks[acting_player]
        .cards
        .iter()
        .filter(|card| matches!(card, Card::Pokemon(_)))
        .collect();

    let num_deck_pokemon = deck_pokemon.len();
    if num_deck_pokemon == 0 {
        // Should not happen if move generation is correct, but just shuffle deck
        return (
            vec![1.0],
            vec![Box::new(|rng, state, action| {
                state.decks[action.actor].shuffle(false, rng);
            })],
        );
    }

    // Create uniform probability for each deck Pokemon (1/N for each)
    let probabilities = vec![1.0 / (num_deck_pokemon as f64); num_deck_pokemon];
    let mut outcomes: Mutations = vec![];
    for i in 0..num_deck_pokemon {
        let hand_pokemon_clone = hand_pokemon.clone();
        outcomes.push(Box::new(move |rng, state, action| {
            // Get the i-th Pokemon from deck
            let deck_pokemon_card = state.decks[action.actor]
                .cards
                .iter()
                .filter(|card| matches!(card, Card::Pokemon(_)))
                .nth(i)
                .cloned()
                .expect("Deck Pokemon should exist");

            // Perform the swap
            // 1. Remove hand Pokemon from hand
            if let Some(pos) = state.hands[action.actor]
                .iter()
                .position(|c| c == &hand_pokemon_clone)
            {
                state.hands[action.actor].remove(pos);
            }
            // 2. Add deck Pokemon to hand
            state.hands[action.actor].push(deck_pokemon_card.clone());
            // 3. Remove deck Pokemon from deck
            if let Some(pos) = state.decks[action.actor]
                .cards
                .iter()
                .position(|c| c == &deck_pokemon_card)
            {
                state.decks[action.actor].cards.remove(pos);
            }
            // 4. Add hand Pokemon to deck
            state.decks[action.actor]
                .cards
                .push(hand_pokemon_clone.clone());
            // 5. Shuffle deck
            state.decks[action.actor].shuffle(false, rng);

            debug!(
                "Pokemon Communication: Swapped {:?} from hand with {:?} from deck",
                hand_pokemon_clone, deck_pokemon_card
            );
        }));
    }

    (probabilities, outcomes)
}

fn forecast_shuffle_opponent_supporter(
    acting_player: usize,
    supporter_card: &Card,
) -> (Probabilities, Mutations) {
    let supporter_clone = supporter_card.clone();
    (
        vec![1.0],
        vec![Box::new(move |rng, state, _action| {
            let opponent = (acting_player + 1) % 2;
            // Remove Supporter from opponent's hand
            if let Some(pos) = state.hands[opponent]
                .iter()
                .position(|c| c == &supporter_clone)
            {
                state.hands[opponent].remove(pos);
            }
            // Add Supporter to opponent's deck
            state.decks[opponent].cards.push(supporter_clone.clone());
            // Shuffle opponent's deck
            state.decks[opponent].shuffle(false, rng);
            debug!(
                "Silver: Shuffled {:?} from opponent's hand into their deck",
                supporter_clone
            );
        })],
    )
}

fn forecast_attach_from_discard(
    state: &State,
    acting_player: usize,
    in_play_idx: usize,
    num_random_energies: usize,
) -> (Probabilities, Mutations) {
    let discard_energies = &state.discard_energies[acting_player];
    let actual_num = std::cmp::min(num_random_energies, discard_energies.len());

    if actual_num == 0 {
        return (vec![1.0], vec![Box::new(|_, _, _| {})]);
    }
    if actual_num == 1 {
        // Deterministic: just attach the first energy
        let energy = discard_energies[0];
        return (
            vec![1.0],
            vec![Box::new(move |_rng, state, action| {
                state.attach_energies_from_discard(action.actor, in_play_idx, &[energy]);
                debug!(
                    "Lusamine: Attached {:?} from discard to Pokemon at index {}",
                    energy, in_play_idx
                );
            })],
        );
    }

    // For 2 energies, generate all combinations and deduplicate
    let combinations = generate_energy_combinations(discard_energies);
    let total_combinations: usize = combinations.iter().map(|(_, count)| count).sum();

    let mut probabilities = Vec::new();
    let mut mutations: Mutations = Vec::new();
    for (combo, count) in combinations {
        let probability = count as f64 / total_combinations as f64;
        probabilities.push(probability);
        mutations.push(Box::new(move |_rng, state, action| {
            state.attach_energies_from_discard(action.actor, in_play_idx, &combo);
            debug!(
                "Lusamine: Attached {:?} from discard to Pokemon at index {}",
                combo, in_play_idx
            );
        }));
    }

    (probabilities, mutations)
}

/// Generate all unique 2-energy combinations from a list of energies in discard pile.
/// Returns a vector of (combination, count) tuples where count is how many times
/// this combination appears when considering all possible pairs.
fn generate_energy_combinations(energies: &[EnergyType]) -> Vec<(Vec<EnergyType>, usize)> {
    let mut combination_counts: HashMap<Vec<EnergyType>, usize> = HashMap::new();
    for i in 0..energies.len() {
        for j in (i + 1)..energies.len() {
            let mut combo = vec![energies[i], energies[j]];
            combo.sort(); // Sort to treat [Grass, Fire] same as [Fire, Grass]
            *combination_counts.entry(combo).or_insert(0) += 1;
        }
    }
    combination_counts.into_iter().collect()
}

fn apply_eevee_bag_damage_boost(state: &mut State) {
    use crate::effects::TurnEffect;
    state.add_turn_effect(
        TurnEffect::IncreasedDamageForEeveeEvolutions { amount: 10 },
        0,
    );
}

fn apply_heal_all_eevee_evolutions(acting_player: usize, state: &mut State) {
    for pokemon in state.in_play_pokemon[acting_player].iter_mut().flatten() {
        if pokemon.evolved_from("Eevee") {
            pokemon.heal(20);
        }
    }
}

// Test that when evolving a damanged pokemon, damage stays.
#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::card_ids::CardId;
    use crate::database::get_card_by_enum;
    use crate::{
        models::{EnergyType, PlayedCard},
        Deck,
    };

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
            Some(PlayedCard::new(
                primeape.clone(),
                60, // 90 - 30 = 60
                90,
                vec![energy],
                true,
                vec![mankey.clone()]
            ))
        );

        // Evolve Bench
        apply_evolve(0, &mut state, &primeape, 2);
        assert_eq!(
            state.in_play_pokemon[0][0],
            Some(PlayedCard::new(
                primeape.clone(),
                60, // 90 - 30 = 60
                90,
                vec![energy],
                true,
                vec![mankey.clone()]
            ))
        );
        assert_eq!(
            state.in_play_pokemon[0][2],
            Some(PlayedCard::new(
                primeape.clone(),
                90, // 90 - 0 = 90
                90,
                vec![energy, energy, energy],
                true,
                vec![mankey.clone()]
            ))
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

    #[test]
    fn test_generate_energy_combinations_all_same_type() {
        // [Grass, Grass, Grass] -> 1 unique combo [Grass, Grass] with count 3
        let energies = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Grass];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 1);
        let (combo, count) = &combinations[0];
        assert_eq!(combo, &vec![EnergyType::Grass, EnergyType::Grass]);
        assert_eq!(*count, 3);
    }

    #[test]
    fn test_generate_energy_combinations_mixed_types() {
        // [Grass, Grass, Fire] -> 2 unique combos:
        // [Grass, Grass] count 1, [Fire, Grass] or [Grass, Fire] count 2
        let energies = vec![EnergyType::Grass, EnergyType::Grass, EnergyType::Fire];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 2);

        // Find the mixed Fire-Grass combo (sorted, so could be either order)
        let fire_grass = combinations
            .iter()
            .find(|(combo, _)| {
                combo.len() == 2
                    && combo.contains(&EnergyType::Fire)
                    && combo.contains(&EnergyType::Grass)
                    && combo[0] != combo[1]
            })
            .expect("Should have Fire-Grass combo");
        assert_eq!(fire_grass.1, 2);

        let grass_grass = combinations
            .iter()
            .find(|(combo, _)| combo == &vec![EnergyType::Grass, EnergyType::Grass])
            .expect("Should have Grass-Grass combo");
        assert_eq!(grass_grass.1, 1);
    }

    #[test]
    fn test_generate_energy_combinations_all_different() {
        // [Grass, Fire, Water] -> 3 unique combos, each count 1
        let energies = vec![EnergyType::Grass, EnergyType::Fire, EnergyType::Water];
        let combinations = super::generate_energy_combinations(&energies);

        assert_eq!(combinations.len(), 3); // C(3,2) = 3
        for (_, count) in &combinations {
            assert_eq!(*count, 1);
        }
    }
}

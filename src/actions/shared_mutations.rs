use log::debug;
use std::cmp::min;

use crate::{
    actions::{
        apply_action_helpers::{Mutations, Probabilities},
        apply_place_card,
        mutations::doutcome,
    },
    combinatorics::generate_combinations,
    models::{Card, EnergyType},
    State,
};

pub(crate) fn pokemon_search_outcomes(
    acting_player: usize,
    state: &State,
    basic_only: bool,
) -> (Probabilities, Mutations) {
    card_search_outcomes_with_filter(acting_player, state, move |card: &&Card| {
        if basic_only {
            card.is_basic()
        } else {
            matches!(card, Card::Pokemon(_))
        }
    })
}

pub(crate) fn pokemon_search_outcomes_by_type(
    state: &State,
    basic_only: bool,
    energy_type: EnergyType,
) -> (Probabilities, Mutations) {
    pokemon_search_outcomes_by_type_for_player(state.current_player, state, basic_only, energy_type)
}

pub(crate) fn pokemon_search_outcomes_by_type_for_player(
    acting_player: usize,
    state: &State,
    basic_only: bool,
    energy_type: EnergyType,
) -> (Probabilities, Mutations) {
    card_search_outcomes_with_filter(acting_player, state, move |card: &&Card| {
        let type_matches = card.get_type().map(|t| t == energy_type).unwrap_or(false);
        let basic_check = !basic_only || card.is_basic();
        type_matches && basic_check
    })
}

pub(crate) fn gladion_search_outcomes(
    acting_player: usize,
    state: &State,
) -> (Probabilities, Mutations) {
    card_search_outcomes_with_filter(acting_player, state, move |card: &&Card| {
        let name = card.get_name();
        name == "Type: Null" || name == "Silvally"
    })
}

pub(crate) fn supporter_search_outcomes(
    acting_player: usize,
    state: &State,
) -> (Probabilities, Mutations) {
    card_search_outcomes_with_filter(
        acting_player,
        state,
        move |card: &&Card| matches!(card, Card::Trainer(trainer_card) if trainer_card.trainer_card_type == crate::models::TrainerType::Supporter),
    )
}

fn card_search_outcomes_with_filter<F>(
    acting_player: usize,
    state: &State,
    card_filter: F,
) -> (Probabilities, Mutations)
where
    F: Fn(&&Card) -> bool + Clone + 'static,
{
    card_search_outcomes_with_filter_multiple(acting_player, state, 1, card_filter)
}

/// Draw up to `num_to_draw` cards from deck that match the filter, using unordered combinations
pub(crate) fn card_search_outcomes_with_filter_multiple<F>(
    acting_player: usize,
    state: &State,
    num_to_draw: usize,
    card_filter: F,
) -> (Probabilities, Mutations)
where
    F: Fn(&&Card) -> bool + Clone + 'static,
{
    let eligible_pokemon: Vec<Card> = state.decks[acting_player]
        .cards
        .iter()
        .filter(|c| card_filter(c))
        .cloned()
        .collect();

    let num_eligible = eligible_pokemon.len();

    if num_eligible == 0 {
        // No eligible Pokemon in deck, just shuffle
        return doutcome(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    let actual_draw_count = min(num_to_draw, num_eligible);

    // Generate all possible unordered combinations
    let draw_combinations = generate_combinations(&eligible_pokemon, actual_draw_count);
    let num_outcomes = draw_combinations.len();
    let probabilities = vec![1.0 / (num_outcomes as f64); num_outcomes];
    let mut outcomes: Mutations = vec![];

    for combo in draw_combinations {
        outcomes.push(Box::new(move |rng, state, _action| {
            // Transfer each Pokemon from the combination to hand
            for pokemon in &combo {
                state.transfer_card_from_deck_to_hand(acting_player, pokemon);
            }

            state.decks[acting_player].shuffle(false, rng);
        }));
    }

    (probabilities, outcomes)
}

pub(crate) fn search_and_bench_by_name(
    state: &State,
    card_name: String,
) -> (Probabilities, Mutations) {
    search_and_bench_with_filter(
        state,
        move |card: &Card| card.get_name() == card_name,
        "Card should be in deck",
    )
}

pub(crate) fn search_and_bench_basic(state: &State) -> (Probabilities, Mutations) {
    search_and_bench_with_filter(
        state,
        |card: &Card| card.is_basic(),
        "Basic card should be in deck",
    )
}

fn search_and_bench_with_filter<F>(
    state: &State,
    card_filter: F,
    missing_card_msg: &'static str,
) -> (Probabilities, Mutations)
where
    F: Fn(&Card) -> bool + Clone + 'static,
{
    let num_cards_in_deck = state.decks[state.current_player]
        .cards
        .iter()
        .filter(|c| card_filter(c))
        .count();

    if num_cards_in_deck == 0 {
        doutcome({
            |rng, state, action| {
                // If there are no matching cards in the deck, just shuffle it
                state.decks[action.actor].shuffle(false, rng);
            }
        })
    } else {
        let probabilities = vec![1.0 / (num_cards_in_deck as f64); num_cards_in_deck];
        let mut outcomes: Mutations = vec![];

        for i in 0..num_cards_in_deck {
            let card_filter = card_filter.clone();
            outcomes.push(Box::new(move |rng, state, action| {
                let bench_space = state.in_play_pokemon[action.actor]
                    .iter()
                    .position(|x| x.is_none());
                if bench_space.is_none() {
                    debug!("No bench space available, shuffling deck without placing card");
                    state.decks[action.actor].shuffle(false, rng);
                    return;
                }

                let card = state.decks[action.actor]
                    .cards
                    .iter()
                    .filter(|c| card_filter(c))
                    .nth(i)
                    .cloned()
                    .expect(missing_card_msg);

                debug!(
                    "Fetched {card:?} from deck for player {} to place on bench",
                    action.actor
                );

                let bench_idx = bench_space.unwrap();
                apply_place_card(state, action.actor, &card, bench_idx, true);

                state.decks[action.actor].shuffle(false, rng);
            }));
        }
        (probabilities, outcomes)
    }
}

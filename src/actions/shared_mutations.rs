use log::debug;

use crate::{
    actions::{
        apply_action_helpers::{Mutations, Probabilities},
        mutations::doutcome,
    },
    hooks::to_playable_card,
    types::{Card, EnergyType},
    State,
};

pub(crate) fn pokemon_search_outcomes(
    acting_player: usize,
    state: &State,
    basic_only: bool,
) -> (Probabilities, Mutations) {
    pokemon_search_outcomes_with_filter(acting_player, state, move |card: &&Card| {
        if basic_only {
            card.is_basic()
        } else {
            matches!(card, Card::Pokemon(_))
        }
    })
}

pub(crate) fn pokemon_search_outcomes_by_type(
    acting_player: usize,
    state: &State,
    basic_only: bool,
    energy_type: EnergyType,
) -> (Probabilities, Mutations) {
    pokemon_search_outcomes_with_filter(acting_player, state, move |card: &&Card| {
        let type_matches = card.get_type().map(|t| t == energy_type).unwrap_or(false);
        let basic_check = !basic_only || card.is_basic();
        type_matches && basic_check
    })
}

fn pokemon_search_outcomes_with_filter<F>(
    acting_player: usize,
    state: &State,
    card_filter: F,
) -> (Probabilities, Mutations)
where
    F: Fn(&&Card) -> bool + Clone + 'static,
{
    let num_pokemon_in_deck = state.decks[acting_player]
        .cards
        .iter()
        .filter(|c| card_filter(c))
        .count();

    if num_pokemon_in_deck == 0 {
        doutcome(|rng, state, action| {
            // If there are no Pokemon in the deck, just shuffle it
            state.decks[action.actor].shuffle(false, rng);
        })
    } else {
        let probabilities = vec![1.0 / (num_pokemon_in_deck as f64); num_pokemon_in_deck];
        let mut outcomes: Mutations = vec![];

        for i in 0..num_pokemon_in_deck {
            let filter = card_filter.clone();
            outcomes.push(Box::new(move |rng, state, action| {
                let card = state.decks[action.actor]
                    .cards
                    .iter()
                    .filter(|c| filter(c))
                    .nth(i)
                    .cloned()
                    .expect("Card should be in deck");

                // Put 1 random Pokemon from your deck into your hand.
                let deck = &mut state.decks[action.actor];
                debug!("Fetched {card:?} from deck for player {}", action.actor);
                // Add it to hand and remove one of it from deck
                state.hands[action.actor].push(card.clone());
                if let Some(pos) = deck.cards.iter().position(|x| x == &card) {
                    deck.cards.remove(pos);
                } else {
                    panic!("Card should be in deck");
                }

                deck.shuffle(false, rng);
            }));
        }
        (probabilities, outcomes)
    }
}

pub(crate) fn search_and_bench_by_name(
    acting_player: usize,
    state: &State,
    card_name: &'static str,
) -> (Probabilities, Mutations) {
    let num_cards_in_deck = state.decks[acting_player]
        .cards
        .iter()
        .filter(|c| c.get_name() == card_name)
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
            outcomes.push(Box::new(move |rng, state, action| {
                // Check if there's bench space first
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
                    .filter(|c| c.get_name() == card_name)
                    .nth(i)
                    .cloned()
                    .expect("Card should be in deck");

                // Put the card onto the bench
                let deck = &mut state.decks[action.actor];
                debug!(
                    "Fetched {card:?} from deck for player {} to place on bench",
                    action.actor
                );

                // Remove card from deck
                if let Some(pos) = deck.cards.iter().position(|x| x == &card) {
                    deck.cards.remove(pos);
                } else {
                    panic!("Card should be in deck");
                }

                // Place on bench
                let bench_idx = bench_space.unwrap();
                let playable_card = to_playable_card(&card, true);
                state.in_play_pokemon[action.actor][bench_idx] = Some(playable_card);

                deck.shuffle(false, rng);
            }));
        }
        (probabilities, outcomes)
    }
}

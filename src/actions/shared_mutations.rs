use log::debug;

use crate::{
    actions::{
        apply_action_helpers::{apply_common_mutation, Mutations, Probabilities},
        mutations::doutcome,
    },
    types::Card,
    State,
};

pub(crate) fn pokemon_search_outcomes(
    acting_player: usize,
    state: &State,
    basic_only: bool,
) -> (Probabilities, Mutations) {
    let card_filter = if basic_only {
        |x: &&Card| x.is_basic()
    } else {
        |x: &&Card| matches!(x, Card::Pokemon(_))
    };
    let num_pokemon_in_deck = state.decks[acting_player]
        .cards
        .iter()
        .filter(card_filter)
        .count();
    if num_pokemon_in_deck == 0 {
        doutcome({
            |rng, state, action| {
                // If there are no Pokemon in the deck, just shuffle it
                state.decks[action.actor].shuffle(false, rng);
            }
        })
    } else {
        let probabilities = vec![1.0 / (num_pokemon_in_deck as f64); num_pokemon_in_deck];
        let mut outcomes: Mutations = vec![];
        for i in 0..num_pokemon_in_deck {
            outcomes.push(Box::new({
                move |rng, state, action| {
                    apply_common_mutation(state, action);

                    let card = state.decks[action.actor]
                        .cards
                        .iter()
                        .filter(card_filter)
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
                }
            }));
        }
        (probabilities, outcomes)
    }
}

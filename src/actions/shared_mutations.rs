use log::debug;

use crate::{
    actions::{
        apply_action_helpers::{apply_common_mutation, Mutations, Probabilities},
        mutations::doutcome,
    },
    State,
};

pub(crate) fn pokeball_outcomes(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    let num_basic_in_deck = state.decks[acting_player]
        .cards
        .iter()
        .filter(|x| x.is_basic())
        .count();
    if num_basic_in_deck == 0 {
        doutcome({
            |rng, state, action| {
                // IF FROM ABILITY, MARK IT DONE

                // If there are no basic Pokemon in the deck, just shuffle it
                state.decks[action.actor].shuffle(false, rng);
            }
        })
    } else {
        let probabilities = vec![1.0 / (num_basic_in_deck as f64); num_basic_in_deck];
        let mut outcomes: Mutations = vec![];
        for i in 0..num_basic_in_deck {
            outcomes.push(Box::new({
                move |rng, state, action| {
                    apply_common_mutation(state, action);

                    let card = state.decks[action.actor]
                        .cards
                        .iter()
                        .filter(|x| x.is_basic())
                        .nth(i)
                        .cloned()
                        .expect("Should be a basic card");

                    // Put 1 random Basic Pokemon from your deck into your hand.
                    let deck = &mut state.decks[action.actor];
                    debug!("Pokeball selected card: {card:?}");
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

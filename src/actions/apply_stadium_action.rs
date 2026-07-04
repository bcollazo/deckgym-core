use log::debug;

use crate::{
    actions::{
        apply_action_helpers::Mutation,
        shared_mutations::{pokemon_search_outcomes, pokemon_search_outcomes_by_type_for_player},
    },
    models::{Card, EnergyType, TrainerType},
    stadiums::{
        is_area_zero_active, is_fragrant_forest_active, is_kids_room_active, is_mesagoza_active,
    },
    State,
};

use super::{
    apply_action_helpers::Mutations,
    outcomes::{CoinSeq, Outcomes},
    SimpleAction,
};

/// Forecasts the UseStadium action for activated stadiums like Mesagoza, Fragrant Forest, Area
/// Zero, and Kid's Room.
pub(crate) fn forecast_use_stadium(state: &State, acting_player: usize) -> Outcomes {
    if is_mesagoza_active(state) {
        return forecast_mesagoza_effect(state, acting_player);
    }
    if is_fragrant_forest_active(state) {
        return forecast_fragrant_forest_effect(state, acting_player);
    }
    if is_area_zero_active(state) {
        return forecast_area_zero_effect(state, acting_player);
    }
    if is_kids_room_active(state) {
        return forecast_kids_room_effect(state, acting_player);
    }
    Outcomes::single_fn(|_, _, _| {})
}

/// Area Zero: Once during each player's turn, that player may shuffle a Basic Pokémon from their
/// hand into their deck. If they do, they draw a card.
fn forecast_area_zero_effect(state: &State, acting_player: usize) -> Outcomes {
    let choices: Vec<SimpleAction> = state.hands[acting_player]
        .iter()
        .filter(|card| card.is_basic())
        .map(|card| SimpleAction::ShuffleOwnCardsIntoDeck {
            cards: vec![card.clone()],
        })
        .collect();

    Outcomes::single_fn(move |_, state, action| {
        state.has_used_stadium[action.actor] = true;
        if !choices.is_empty() {
            state
                .move_generation_stack
                .push((action.actor, choices.clone()));
        }
    })
}

/// Mesagoza: Once during each player's turn, that player may flip a coin.
/// If heads, that player puts a random Pokémon from their deck into their hand.
fn forecast_mesagoza_effect(state: &State, acting_player: usize) -> Outcomes {
    // Get the search outcomes for any Pokemon (reusing existing logic)
    let (search_probs, search_mutations) =
        pokemon_search_outcomes(acting_player, state, false).into_branches();

    let mut branches: Vec<(f64, Mutation, Vec<CoinSeq>)> =
        Vec::with_capacity(search_probs.len() + 1);
    for (prob, mutation) in search_probs.into_iter().zip(search_mutations) {
        let wrapped: Mutation = Box::new(move |rng, state, action| {
            state.has_used_stadium[action.actor] = true;
            mutation(rng, state, action);
            debug!("Mesagoza: Flipped heads, searched for Pokemon");
        });
        branches.push((0.5 * prob, wrapped, vec![CoinSeq(vec![true])]));
    }
    let tails_mutation: Mutation = Box::new(move |_, state, action| {
        state.has_used_stadium[action.actor] = true;
        debug!("Mesagoza: Flipped tails, nothing happens");
    });
    branches.push((0.5, tails_mutation, vec![CoinSeq(vec![false])]));

    // Not `binary_coin`: heads fans out into many weighted search outcomes, not one mutation.
    Outcomes::from_coin_branches(branches).expect("Mesagoza coin branches should be valid")
}

/// Fragrant Forest: Once during each player's turn, that player may put a random Basic [G] Pokémon from their deck into their hand.
fn forecast_fragrant_forest_effect(state: &State, acting_player: usize) -> Outcomes {
    pokemon_search_outcomes_by_type_for_player(acting_player, state, true, EnergyType::Grass)
        .map_mutations(|mutation| {
            Box::new(move |rng, state, action| {
                state.has_used_stadium[action.actor] = true;
                mutation(rng, state, action);
            })
        })
}

/// Kid's Room: Once during each player's turn, that player may choose a card in their hand and
/// switch it with a random Pokémon Tool card in their deck.
fn forecast_kids_room_effect(state: &State, acting_player: usize) -> Outcomes {
    let choices: Vec<SimpleAction> = state.hands[acting_player]
        .iter()
        .map(|card| SimpleAction::SwitchHandCardForRandomTool {
            hand_card: card.clone(),
        })
        .collect();

    Outcomes::single_fn(move |_, state, action| {
        state.has_used_stadium[action.actor] = true;
        if !choices.is_empty() {
            state
                .move_generation_stack
                .push((action.actor, choices.clone()));
        }
    })
}

fn is_tool_card(card: &Card) -> bool {
    matches!(card, Card::Trainer(t) if t.trainer_card_type == TrainerType::Tool)
}

/// Kid's Room: switch the chosen hand card with a random Pokémon Tool card from the deck.
pub(crate) fn forecast_switch_hand_card_for_random_tool(
    state: &State,
    acting_player: usize,
    hand_card: &Card,
) -> Outcomes {
    let deck_tools: Vec<Card> = state.decks[acting_player]
        .cards
        .iter()
        .filter(|card| is_tool_card(card))
        .cloned()
        .collect();

    let num_deck_tools = deck_tools.len();
    if num_deck_tools == 0 {
        // Should not happen if move generation is correct, but just shuffle deck
        return Outcomes::single_fn(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    let probabilities = vec![1.0 / (num_deck_tools as f64); num_deck_tools];
    let mut outcomes: Mutations = vec![];
    for tool_card in deck_tools {
        let hand_card_clone = hand_card.clone();
        outcomes.push(Box::new(move |rng, state, action| {
            state.transfer_card_from_hand_to_deck(action.actor, &hand_card_clone);
            state.transfer_card_from_deck_to_hand(action.actor, &tool_card);
            state.decks[action.actor].shuffle(false, rng);
            debug!(
                "Kid's Room: Switched {:?} from hand with {:?} from deck",
                hand_card_clone, tool_card
            );
        }));
    }

    Outcomes::from_parts(probabilities, outcomes)
}

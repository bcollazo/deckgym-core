use log::debug;
use rand::rngs::StdRng;

use crate::{
    card_ids::CardId,
    state::GameOutcome,
    tool_ids::ToolId,
    types::{Card, EnergyType, TrainerCard},
    State,
};

use super::{
    apply_action_helpers::{apply_common_mutation, Mutations, Probabilities},
    Action, SimpleAction,
};

// This is a reducer of all actions relating to trainer cards.
pub fn forecast_trainer_action(
    acting_player: usize,
    state: &State,
    trainer_card: &TrainerCard,
) -> (Probabilities, Mutations) {
    let trainer_id =
        CardId::from_numeric_id(trainer_card.numeric_id).expect("CardId should be known");
    match trainer_id {
        CardId::PA001Potion => deterministic(potion_effect),
        CardId::PA002XSpeed => deterministic(turn_effect),
        CardId::PA005PokeBall => pokeball_outcomes(acting_player, state),
        CardId::PA006RedCard => deterministic(red_card_effect),
        CardId::PA007ProfessorsResearch => deterministic(professor_oak_effect),
        CardId::A1219Erika | CardId::A1266Erika => deterministic(erika_effect),
        CardId::A1220Misty | CardId::A1267Misty => misty_outcomes(),
        CardId::A1222Koga | CardId::A1269Koga => deterministic(koga_effect),
        CardId::A1223Giovanni | CardId::A1270Giovanni => deterministic(giovanni_effect),
        CardId::A1225Sabrina | CardId::A1272Sabrina => deterministic(sabrina_effect),
        CardId::A1a065MythicalSlab => deterministic(mythical_slab_effect),
        CardId::A1a068Leaf | CardId::A1a082Leaf => deterministic(turn_effect),
        CardId::A2150Cyrus | CardId::A2190Cyrus => deterministic(cyrus_effect),
        CardId::A2147GiantCape => deterministic(attach_tool),
        _ => panic!("Unsupported Trainer Card"),
    }
}

fn deterministic(mutation: fn(&mut StdRng, &mut State, &Action)) -> (Probabilities, Mutations) {
    (
        vec![1.0],
        vec![Box::new(move |rng, state, action| {
            apply_common_mutation(state, action);
            mutation(rng, state, action);
        })],
    )
}

fn erika_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    inner_healing_effect(rng, state, action, 50, Some(EnergyType::Grass));
}

fn potion_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    inner_healing_effect(rng, state, action, 20, None);
}

// Queues up the decision of healing an in_play pokemon that matches energy (if None, then any)
fn inner_healing_effect(
    _: &mut StdRng,
    state: &mut State,
    action: &Action,
    amount: u32,
    energy: Option<EnergyType>,
) {
    let possible_moves = state
        .enumerate_in_play_pokemon(action.actor)
        .filter(|(_, x)| energy.is_none() || x.get_energy_type() == Some(EnergyType::Grass))
        .map(|(i, _)| SimpleAction::Heal {
            in_play_idx: i,
            amount,
        })
        .collect::<Vec<_>>();
    if !possible_moves.is_empty() {
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }
}

// Will return 6 outputs, one that attaches no energy justruns apply_common, one that
//  queues decision of attaching 1 energy to in_play waters.
fn misty_outcomes() -> (Probabilities, Mutations) {
    // probabilistic attach energy to water pokemon
    // 50% no energy, 25% 1 energy, 12.5% 2 energy, 6.75% 3 energy, 3.125% 4 energy, 1.5625% 5 energy
    let probabilities = vec![0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625];
    let mut outcomes: Mutations = vec![];
    for j in 0..6 {
        outcomes.push(Box::new({
            move |_, state, action| {
                apply_common_mutation(state, action);

                // For each in_play water pokemon
                let possible_moves = state
                    .enumerate_in_play_pokemon(action.actor)
                    .filter(|(_, x)| x.get_energy_type() == Some(EnergyType::Water))
                    .map(|(i, _)| SimpleAction::Attach {
                        attachments: vec![(j, EnergyType::Water, i)],
                        is_turn_energy: false,
                    })
                    .collect::<Vec<_>>();
                if !possible_moves.is_empty() {
                    state
                        .move_generation_stack
                        .push((action.actor, possible_moves));
                }
            }
        }));
    }
    (probabilities, outcomes)
}

fn pokeball_outcomes(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    let num_basic_in_deck = state.decks[acting_player]
        .cards
        .iter()
        .filter(|x| x.is_basic())
        .count();
    if num_basic_in_deck == 0 {
        deterministic({
            |rng, state, action| {
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
                    // Select a random one
                    debug!("Pokeball selected card: {:?}", card);
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

// Remember to implement these in the main controller / hooks.
fn turn_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    if let SimpleAction::Play { trainer_card } = &action.action {
        let card = Card::Trainer(trainer_card.clone());
        state.add_turn_effect(card, 0);
    } else {
        panic!("Something went wrong. An action was played but couldnt get the card");
    }
}

fn sabrina_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)
    let opponent_player = (action.actor + 1) % 2;
    let possible_moves = state
        .enumerate_bench_pokemon(opponent_player)
        .map(|(i, _)| SimpleAction::Activate { in_play_idx: i })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((opponent_player, possible_moves));
}

fn cyrus_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Switch 1 of your opponent's Pokemon that has damage on it to the Active Spot.
    let opponent_player = (action.actor + 1) % 2;
    let possible_moves = state
        .enumerate_bench_pokemon(opponent_player)
        .filter(|(_, x)| x.is_damaged())
        .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
        .collect::<Vec<_>>();
    state
        .move_generation_stack
        .push((opponent_player, possible_moves));
}

fn giovanni_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    if let SimpleAction::Play { trainer_card } = &action.action {
        // During this turn, attacks used by your Pokémon do +10 damage to your opponent's Active Pokémon.
        let card = Card::Trainer(trainer_card.clone());
        state.add_turn_effect(card, 0);
    } else {
        panic!("XSpeed should be played");
    }
}

fn koga_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Put your Muk or Weezing in the Active Spot into your hand.
    let active_pokemon = state.in_play_pokemon[action.actor][0]
        .as_ref()
        .expect("Active Pokemon should be there if Koga is played");
    let mut cards_to_collect = active_pokemon.cards_behind.clone();
    cards_to_collect.push(active_pokemon.card.clone());
    state.hands[action.actor].extend(cards_to_collect);
    // Energy dissapears
    state.in_play_pokemon[action.actor][0] = None;

    // if no bench pokemon, finish game as a loss
    let bench_pokemon = state.enumerate_bench_pokemon(action.actor).count();
    if bench_pokemon == 0 {
        debug!("Player lost due to no bench pokemon after Koga");
        state.winner = Some(GameOutcome::Win((action.actor + 1) % 2))
    } else {
        // else force current_player to promote one of their bench pokemon
        let possible_moves = state
            .enumerate_bench_pokemon(action.actor)
            .map(|(i, _)| SimpleAction::Activate { in_play_idx: i })
            .collect::<Vec<_>>();
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }
}

// TODO: Problem. With doing 1.0, we are basically giving bots the ability to see the cards in deck.
// TODO: In theory this should give a probability distribution over cards in deck.
fn professor_oak_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Draw 2 cards.
    for _ in 0..2 {
        state.maybe_draw_card(action.actor);
    }
}

// TODO: Actually use distribution of possibilities to capture probabilities
// of pulling the different psychic left in deck vs pushing an item to the bottom.
fn mythical_slab_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Look at the top card of your deck. If that card is a Psychic Pokemon,\n        put it in your hand. If it is not a Psychic Pokemon, put it on the\n        bottom of your deck.
    if let Some(card) = state.decks[action.actor].cards.first() {
        if card.is_basic() {
            state.hands[action.actor].push(card.clone());
            state.decks[action.actor].cards.remove(0);
        } else {
            let card = state.decks[action.actor].cards.remove(0);
            state.decks[action.actor].cards.push(card);
        }
    } // else do nothing
}

// Here we will simplify the output possibilities, counting with the fact that value functions
// should not use the cards of the enemy as input.
fn red_card_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    // Your opponent shuffles their hand into their deck and draws 3 cards.
    let acting_player = action.actor;
    let opponent = (acting_player + 1) % 2;
    let opponent_hand = &mut state.hands[opponent];
    let opponent_deck = &mut state.decks[opponent];
    opponent_deck.cards.append(opponent_hand);
    opponent_deck.shuffle(false, rng);
    for _ in 0..3 {
        state.maybe_draw_card(opponent);
    }
}

// Give the choice to the player to attach a tool to one of their pokemon.
fn attach_tool(_: &mut StdRng, state: &mut State, action: &Action) {
    if let SimpleAction::Play { trainer_card } = &action.action {
        let &tool_id = ToolId::from_trainer_card(trainer_card).expect("ToolId should exist");
        let choices = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, x)| !x.has_tool_attached())
            .map(|(in_play_idx, _)| SimpleAction::AttachTool {
                in_play_idx,
                tool_id,
            })
            .collect::<Vec<_>>();
        state.move_generation_stack.push((action.actor, choices));
    } else {
        panic!("Tool should have been played");
    }
}

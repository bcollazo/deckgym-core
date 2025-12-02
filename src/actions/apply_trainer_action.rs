use std::cmp::min;

use log::debug;
use rand::rngs::StdRng;

use crate::{
    actions::{
        mutations::doutcome,
        shared_mutations::{
            gladion_search_outcomes, pokemon_search_outcomes,
            pokemon_search_outcomes_with_filter_multiple,
        },
    },
    card_ids::CardId,
    card_logic::can_rare_candy_evolve,
    combinatorics::generate_combinations,
    effects::TurnEffect,
    hooks::{get_stage, is_ultra_beast},
    models::{Card, EnergyType, TrainerCard},
    state::GameOutcome,
    tool_ids::ToolId,
    State,
};

use super::{
    apply_action_helpers::{Mutations, Probabilities},
    Action, SimpleAction,
};

// This is a reducer of all actions relating to trainer cards.
pub fn forecast_trainer_action(
    acting_player: usize,
    state: &State,
    trainer_card: &TrainerCard,
) -> (Probabilities, Mutations) {
    let trainer_id =
        CardId::from_card_id(trainer_card.id.as_str()).expect("CardId should be known");
    match trainer_id {
        CardId::PA001Potion => doutcome(potion_effect),
        CardId::PA002XSpeed => doutcome(x_speed_effect),
        CardId::PA005PokeBall | CardId::A2b111PokeBall => {
            pokemon_search_outcomes(acting_player, state, true)
        }
        CardId::PA006RedCard => doutcome(red_card_effect),
        CardId::PA007ProfessorsResearch | CardId::A4b373ProfessorsResearch => {
            doutcome(professor_oak_effect)
        }
        CardId::A1219Erika | CardId::A1266Erika | CardId::A4b328Erika | CardId::A4b329Erika => {
            doutcome(erika_effect)
        }
        CardId::A1220Misty | CardId::A1267Misty => misty_outcomes(),
        CardId::A1221Blaine | CardId::A1268Blaine => doutcome(blaine_effect),
        CardId::A2a072Irida | CardId::A2a087Irida | CardId::A4b330Irida | CardId::A4b331Irida => {
            doutcome(irida_effect)
        }
        CardId::A2b070PokemonCenterLady | CardId::A2b089PokemonCenterLady => {
            doutcome(pokemon_center_lady_effect)
        }
        CardId::A3155Lillie
        | CardId::A3197Lillie
        | CardId::A3209Lillie
        | CardId::A4b348Lillie
        | CardId::A4b349Lillie
        | CardId::A4b374Lillie => doutcome(lillie_effect),
        CardId::A1222Koga | CardId::A1269Koga => doutcome(koga_effect),
        CardId::A1223Giovanni
        | CardId::A1270Giovanni
        | CardId::A4b334Giovanni
        | CardId::A4b335Giovanni => doutcome(giovanni_effect),
        CardId::A2b071Red | CardId::A2b090Red | CardId::A4b352Red | CardId::A4b353Red => {
            doutcome(red_effect)
        }
        CardId::A1225Sabrina
        | CardId::A1272Sabrina
        | CardId::A4b338Sabrina
        | CardId::A4b339Sabrina => doutcome(sabrina_effect),
        CardId::A1a065MythicalSlab => doutcome(mythical_slab_effect),
        CardId::A1a068Leaf | CardId::A1a082Leaf | CardId::A4b346Leaf | CardId::A4b347Leaf => {
            doutcome(leaf_effect)
        }
        CardId::A2147GiantCape
        | CardId::A2148RockyHelmet
        | CardId::A3146PoisonBarb
        | CardId::A3147LeafCape
        | CardId::A3a065ElectricalCord
        | CardId::A4a067InflatableBoat
        | CardId::A4b318ElectricalCord
        | CardId::A4b319ElectricalCord
        | CardId::B1219HeavyHelmet => doutcome(attach_tool),
        CardId::A2150Cyrus | CardId::A2190Cyrus | CardId::A4b326Cyrus | CardId::A4b327Cyrus => {
            doutcome(cyrus_effect)
        }
        CardId::A2155Mars | CardId::A2195Mars | CardId::A4b344Mars | CardId::A4b345Mars => {
            doutcome(mars_effect)
        }
        CardId::A3144RareCandy
        | CardId::A4b314RareCandy
        | CardId::A4b315RareCandy
        | CardId::A4b379RareCandy => doutcome(rare_candy_effect),
        CardId::A3a064Repel => doutcome(repel_effect),
        CardId::A2146PokemonCommunication
        | CardId::A4b316PokemonCommunication
        | CardId::A4b317PokemonCommunication => doutcome(pokemon_communication_effect),
        CardId::A4151ElementalSwitch
        | CardId::A4b310ElementalSwitch
        | CardId::A4b311ElementalSwitch => doutcome(elemental_switch_effect),
        CardId::A3a067Gladion | CardId::A3a081Gladion => {
            gladion_search_outcomes(acting_player, state)
        }
        CardId::A3a069Lusamine
        | CardId::A3a083Lusamine
        | CardId::A4b350Lusamine
        | CardId::A4b351Lusamine
        | CardId::A4b375Lusamine => doutcome(lusamine_effect),
        CardId::A4157Lyra | CardId::A4197Lyra | CardId::A4b332Lyra | CardId::A4b333Lyra => {
            doutcome(lyra_effect)
        }
        CardId::A4158Silver | CardId::A4198Silver | CardId::A4b336Silver | CardId::A4b337Silver => {
            doutcome(silver_effect)
        }
        CardId::A3b066EeveeBag
        | CardId::A3b107EeveeBag
        | CardId::A4b308EeveeBag
        | CardId::A4b309EeveeBag => doutcome(eevee_bag_effect),
        CardId::B1217FlamePatch | CardId::B1331FlamePatch => doutcome(flame_patch_effect),
        CardId::B1225Copycat | CardId::B1270Copycat => doutcome(copycat_effect),
        CardId::A2b069Iono | CardId::A2b088Iono | CardId::A4b340Iono | CardId::A4b341Iono => {
            doutcome(iono_effect)
        }
        CardId::B1223May | CardId::B1268May => may_effect(acting_player, state),
        CardId::B1226Lisia | CardId::B1271Lisia => lisia_effect(acting_player, state),
        CardId::A2a073CelesticTownElder | CardId::A2a088CelesticTownElder => {
            celestic_town_elder_effect(acting_player, state)
        }
        _ => panic!("Unsupported Trainer Card"),
    }
}

fn erika_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    inner_healing_effect(rng, state, action, 50, Some(EnergyType::Grass));
}

fn irida_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Heal 40 damage from each of your Pokémon that has any Water Energy attached.
    debug!("Irida: Healing 40 damage from each Pokemon with Water Energy attached");
    for pokemon in state.in_play_pokemon[action.actor].iter_mut().flatten() {
        if pokemon.attached_energy.contains(&EnergyType::Water) {
            pokemon.heal(40);
        }
    }
}

fn pokemon_center_lady_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Heal 30 damage from 1 of your Pokémon, and it recovers from all Special Conditions.
    debug!("Pokemon Center Lady: Healing 30 damage and curing status conditions");
    let possible_moves = state
        .enumerate_in_play_pokemon(action.actor)
        .map(|(i, _)| SimpleAction::Heal {
            in_play_idx: i,
            amount: 30,
            cure_status: true,
        })
        .collect::<Vec<_>>();
    if !possible_moves.is_empty() {
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }
}

fn lillie_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let possible_moves = state
        .enumerate_in_play_pokemon(action.actor)
        .filter(|(_, x)| get_stage(x) == 2)
        .map(|(i, _)| SimpleAction::Heal {
            in_play_idx: i,
            amount: 60,
            cure_status: false,
        })
        .collect::<Vec<_>>();
    if !possible_moves.is_empty() {
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }
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
            cure_status: false,
        })
        .collect::<Vec<_>>();
    if !possible_moves.is_empty() {
        state
            .move_generation_stack
            .push((action.actor, possible_moves));
    }
}

// Will return 6 outputs, one that attaches no energy, one that
//  queues decision of attaching 1 energy to in_play waters.
fn misty_outcomes() -> (Probabilities, Mutations) {
    // probabilistic attach energy to water pokemon
    // 50% no energy, 25% 1 energy, 12.5% 2 energy, 6.75% 3 energy, 3.125% 4 energy, 1.5625% 5 energy
    let probabilities = vec![0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625];
    let mut outcomes: Mutations = vec![];
    for j in 0..6 {
        outcomes.push(Box::new({
            move |_, state, action| {
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

// Remember to implement these in the main controller / hooks.
fn x_speed_effect(_: &mut StdRng, state: &mut State, _: &Action) {
    state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 1 }, 0);
}
fn leaf_effect(_: &mut StdRng, state: &mut State, _: &Action) {
    state.add_turn_effect(TurnEffect::ReducedRetreatCost { amount: 2 }, 0);
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

fn repel_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    // Switch out your opponent's Active Basic Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)
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

fn mars_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    // Your opponent shuffles their hand into their deck and draws a card for each of their remaining points needed to win.
    let opponent_player = (action.actor + 1) % 2;
    let opponent_points = state.points[opponent_player];
    let cards_to_draw = (3 - opponent_points) as usize;

    debug!(
        "Mars: Opponent has {} points, shuffling hand and drawing {} cards",
        opponent_points, cards_to_draw
    );

    // Shuffle opponent's hand back into deck
    state.decks[opponent_player]
        .cards
        .append(&mut state.hands[opponent_player]);
    state.decks[opponent_player].shuffle(false, rng);

    // Draw cards
    for _ in 0..cards_to_draw {
        if let Some(card) = state.decks[opponent_player].draw() {
            state.hands[opponent_player].push(card);
        }
    }
}

fn giovanni_effect(_: &mut StdRng, state: &mut State, _: &Action) {
    // During this turn, attacks used by your Pokémon do +10 damage to your opponent's Active Pokémon.
    state.add_turn_effect(TurnEffect::IncreasedDamage { amount: 10 }, 0);
}

fn blaine_effect(_: &mut StdRng, state: &mut State, _: &Action) {
    // During this turn, attacks used by your Ninetales, Rapidash, or Magmar do +30 damage to your opponent's Active Pokémon.
    state.add_turn_effect(TurnEffect::BlaineEffect, 0);
}

fn red_effect(_: &mut StdRng, state: &mut State, _: &Action) {
    // During this turn, attacks used by your Pokémon do +20 damage to your opponent's Active Pokémon ex.
    state.add_turn_effect(TurnEffect::IncreasedDamageAgainstEx { amount: 20 }, 0);
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
        let choices = tool_id
            .enumerate_choices(state, action.actor)
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

/// Makes user select what Stage2-Basic pair to evolve.
fn rare_candy_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;
    let hand = &state.hands[player];

    // Flat-map basic in play with valid stage 2 in hand pairs
    let possible_candy_evolutions: Vec<SimpleAction> = state
        .enumerate_in_play_pokemon(player)
        .flat_map(|(in_play_idx, in_play)| {
            hand.iter()
                .filter(|card| can_rare_candy_evolve(card, in_play))
                .map(move |card| SimpleAction::Evolve(card.clone(), in_play_idx))
        })
        .collect();

    if !possible_candy_evolutions.is_empty() {
        state
            .move_generation_stack
            .push((player, possible_candy_evolutions));
    }
}

/// Queue the decision for user to select which Pokemon from hand to swap
fn pokemon_communication_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;
    let possible_swaps: Vec<SimpleAction> = state.hands[player]
        .iter()
        .filter(|card| matches!(card, Card::Pokemon(_)))
        .map(|card| SimpleAction::CommunicatePokemon {
            hand_pokemon: card.clone(),
        })
        .collect();

    if !possible_swaps.is_empty() {
        state.move_generation_stack.push((player, possible_swaps));
    }
}

fn elemental_switch_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;
    if state.maybe_get_active(player).is_none() {
        return;
    }
    let allowed_types = [EnergyType::Fire, EnergyType::Water, EnergyType::Lightning];
    let mut possible_transfers = Vec::new();

    for (from_idx, pokemon) in state.enumerate_bench_pokemon(player) {
        for &energy in &pokemon.attached_energy {
            if allowed_types.contains(&energy) {
                let move_action = SimpleAction::MoveEnergy {
                    from_in_play_idx: from_idx,
                    to_in_play_idx: 0,
                    energy,
                };
                if !possible_transfers.contains(&move_action) {
                    possible_transfers.push(move_action);
                }
            }
        }
    }

    if !possible_transfers.is_empty() {
        state
            .move_generation_stack
            .push((player, possible_transfers));
    }
}

/// Queue the decision for user to select which Supporter from opponent's hand to shuffle
fn silver_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;
    let opponent = (player + 1) % 2;
    let possible_shuffles: Vec<SimpleAction> = state.hands[opponent]
        .iter()
        .filter(|card| card.is_support())
        .map(|card| SimpleAction::ShuffleOpponentSupporter {
            supporter_card: card.clone(),
        })
        .collect();

    if !possible_shuffles.is_empty() {
        state
            .move_generation_stack
            .push((player, possible_shuffles));
    }
}

/// Queue the decision for user to select which Ultra Beast to attach energies to
fn lusamine_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;
    let num_energies_to_attach = min(2, state.discard_energies[player].len());

    let possible_attachments: Vec<SimpleAction> = state
        .enumerate_in_play_pokemon(player)
        .filter(|(_, pokemon)| is_ultra_beast(&pokemon.get_name()))
        .map(|(idx, _)| SimpleAction::AttachFromDiscard {
            in_play_idx: idx,
            num_random_energies: num_energies_to_attach,
        })
        .collect();

    if !possible_attachments.is_empty() {
        state
            .move_generation_stack
            .push((player, possible_attachments));
    }
}

fn lyra_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let possible_activations = state
        .enumerate_bench_pokemon(action.actor)
        .map(|(idx, _)| SimpleAction::Activate { in_play_idx: idx })
        .collect();
    state
        .move_generation_stack
        .push((action.actor, possible_activations))
}

fn eevee_bag_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let choices = vec![
        SimpleAction::ApplyEeveeBagDamageBoost,
        SimpleAction::HealAllEeveeEvolutions,
    ];
    state.move_generation_stack.push((action.actor, choices));
}

fn flame_patch_effect(_: &mut StdRng, state: &mut State, action: &Action) {
    let player = action.actor;

    // Find and remove a Fire energy from discard pile
    if let Some(fire_idx) = state.discard_energies[player]
        .iter()
        .position(|energy| *energy == EnergyType::Fire)
    {
        state.discard_energies[player].remove(fire_idx);

        // Attach it to the active Pokemon
        if let Some(active_pokemon) = state.in_play_pokemon[player][0].as_mut() {
            active_pokemon.attached_energy.push(EnergyType::Fire);
        }
    }
}

fn copycat_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    // Shuffle your hand into your deck. Draw a card for each card in your opponent's hand.
    let player = action.actor;
    let opponent = (player + 1) % 2;

    // Count opponent's hand size before shuffling
    let opponent_hand_size = state.hands[opponent].len();

    debug!(
        "Copycat: Shuffling hand into deck and drawing {} cards (opponent's hand size)",
        opponent_hand_size
    );

    // Shuffle player's hand into their deck
    state.decks[player].cards.append(&mut state.hands[player]);
    state.decks[player].shuffle(false, rng);

    // Draw cards equal to opponent's hand size
    for _ in 0..opponent_hand_size {
        state.maybe_draw_card(player);
    }
}

fn iono_effect(rng: &mut StdRng, state: &mut State, action: &Action) {
    // Each player shuffles the cards in their hand into their deck, then draws that many cards.
    let player = action.actor;
    let opponent = (player + 1) % 2;

    // Count each player's hand size before shuffling
    let player_hand_size = state.hands[player].len();
    let opponent_hand_size = state.hands[opponent].len();

    debug!(
        "Iono: Player {} shuffling {} cards, opponent shuffling {} cards",
        player, player_hand_size, opponent_hand_size
    );

    // Shuffle player's hand into their deck
    state.decks[player].cards.append(&mut state.hands[player]);
    state.decks[player].shuffle(false, rng);

    // Shuffle opponent's hand into their deck
    state.decks[opponent]
        .cards
        .append(&mut state.hands[opponent]);
    state.decks[opponent].shuffle(false, rng);

    // Each player draws the same number of cards they had
    for _ in 0..player_hand_size {
        state.maybe_draw_card(player);
    }
    for _ in 0..opponent_hand_size {
        state.maybe_draw_card(opponent);
    }
}

pub fn may_effect(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    // Put 2 random Pokémon from your deck into your hand.
    // For each Pokémon you put into your hand in this way, choose a Pokémon to shuffle from your hand into your deck.
    let deck_pokemon: Vec<Card> = state.iter_deck_pokemon(acting_player).cloned().collect();
    let num_pokemon = deck_pokemon.len();
    if num_pokemon == 0 {
        // No Pokemon in deck, just shuffle
        return doutcome(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    // For drawing 2 Pokemon, we need to generate all possible pairs
    // Each outcome draws 2 different Pokemon (or fewer if not enough in deck)
    let num_to_draw = min(2, num_pokemon);
    if num_to_draw == 1 {
        // Only 1 Pokemon in deck - simple case
        let probabilities = vec![1.0];
        let mut outcomes: Mutations = vec![];
        outcomes.push(Box::new(move |_rng, state, action| {
            let pokemon = state
                .iter_deck_pokemon(action.actor)
                .next()
                .cloned()
                .expect("Pokemon should be in deck");
            state.transfer_card_from_deck_to_hand(action.actor, &pokemon);
            // Queue shuffling that Pokemon back into deck
            state.move_generation_stack.push((
                action.actor,
                vec![SimpleAction::ShufflePokemonIntoDeck {
                    hand_pokemon: vec![pokemon],
                }],
            ));
        }));
        return (probabilities, outcomes);
    }

    // Drawing 2 Pokemon - generate all possible unordered combinations
    let draw_combinations = generate_combinations(&deck_pokemon, num_to_draw);
    let num_outcomes = draw_combinations.len();
    let probabilities = vec![1.0 / (num_outcomes as f64); num_outcomes];
    let mut outcomes: Mutations = vec![];
    for combo in draw_combinations {
        outcomes.push(Box::new(move |_rng, state, action| {
            // Transfer each Pokemon from the combination to hand
            for pokemon in &combo {
                state.transfer_card_from_deck_to_hand(action.actor, pokemon);
            }

            // Generate all possible 2-combinations of Pokemon in hand to shuffle back
            let hand_pokemon: Vec<Card> = state.iter_hand_pokemon(action.actor).cloned().collect();
            let combinations = generate_combinations(&hand_pokemon, num_to_draw);
            let shuffle_choices: Vec<SimpleAction> = combinations
                .into_iter()
                .map(|combo| SimpleAction::ShufflePokemonIntoDeck {
                    hand_pokemon: combo,
                })
                .collect();
            state
                .move_generation_stack
                .push((action.actor, shuffle_choices));
        }));
    }

    (probabilities, outcomes)
}

fn lisia_effect(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    // Put 2 random Basic Pokémon with 50 HP or less from your deck into your hand.
    pokemon_search_outcomes_with_filter_multiple(acting_player, state, 2, |card| {
        if let Card::Pokemon(pokemon_card) = card {
            pokemon_card.stage == 0 && pokemon_card.hp <= 50
        } else {
            false
        }
    })
}

fn celestic_town_elder_effect(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    // Put 1 random Basic Pokémon from your discard pile into your hand.
    let basic_pokemon: Vec<Card> = state.discard_piles[acting_player]
        .iter()
        .filter(|card| card.is_basic())
        .cloned()
        .collect();

    if basic_pokemon.is_empty() {
        // No basic Pokemon in discard, nothing to do
        return doutcome(|_, _, _| {});
    }

    // Create one outcome for each possible basic Pokemon that could be selected
    let num_outcomes = basic_pokemon.len();
    let probabilities = vec![1.0 / (num_outcomes as f64); num_outcomes];
    let mut outcomes: Mutations = vec![];

    for pokemon in basic_pokemon {
        outcomes.push(Box::new(move |_, state, action| {
            // Find and remove this specific Pokemon from discard pile
            if let Some(idx) = state.discard_piles[action.actor]
                .iter()
                .position(|card| card == &pokemon)
            {
                state.discard_piles[action.actor].remove(idx);
                state.hands[action.actor].push(pokemon.clone());
            }
        }));
    }

    (probabilities, outcomes)
}

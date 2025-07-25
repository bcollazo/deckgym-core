use std::collections::HashMap;
use uuid::Uuid;

use deckgym::{
    actions::Action,
    players::PlayerCode,
    simulate::initialize_logger,
    simulation_event_handler::{
        CompositeSimulationEventHandler, SimulationEventHandler, StatsCollector,
    },
    Simulation, State,
};

/// Run with `cargo run --example sylveon_deck_draw_speed`
fn main() {
    let num_simulations = 1000;
    let deck_a_path = "example_decks/solgaleo_sylveon.txt";
    let deck_b_path = "example_decks/mewtwoex.txt";
    let player_codes = vec![PlayerCode::R, PlayerCode::R];
    initialize_logger(1);
    println!("This will count how fast the user cant draw cards from the deck.");

    let stats_collector = Box::new(StatsCollector::new());
    let deckout_collector = Box::new(DeckOutCollector::new());
    let first_turn_seen_collector = Box::new(FirstTurnSeenCollector::new());
    let mut composite_handler = CompositeSimulationEventHandler::new();
    composite_handler.add_handler(stats_collector);
    composite_handler.add_handler(deckout_collector);
    composite_handler.add_handler(first_turn_seen_collector);
    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        None,
        composite_handler,
    )
    .expect("Failed to create simulation");
    simulation.run();
}

/// Simulation event handler that counts how fast players run out of cards in their decks.
/// It generates a CSV file with (game_id, turn, actor, last_known_deck_size) tuples for each game.
pub struct DeckOutCollector {
    deck_sizes: HashMap<(Uuid, u8, usize), usize>, // (game_id, turn, actor) -> deck_size
}

impl DeckOutCollector {
    pub fn new() -> Self {
        Self {
            deck_sizes: HashMap::new(),
        }
    }
}

impl SimulationEventHandler for DeckOutCollector {
    fn on_action(
        &mut self,
        game_id: Uuid,
        _state_before_action: &State,
        actor: usize,
        _playable_actions: &[Action],
        _action: &Action,
        state_after_action: &State,
    ) {
        let turn = state_after_action.turn_count;
        let deck_size = state_after_action.decks[actor].cards.len();

        // Since this is called multiple times per turn, writing idempotently
        // we'll store the last known deck size for each game-turn-actor combination.
        self.deck_sizes.insert((game_id, turn, actor), deck_size);
    }

    fn on_simulation_end(&mut self) {
        println!("Deck sizes collected for all games. Writing to CSV...");

        // Write the deck sizes to a CSV file
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("deck_sizes.csv");
        let mut wtr = csv::Writer::from_path(&absolute_path).expect("Failed to create CSV writer");

        // Convert to Vec and sort by (game_id, turn, actor)
        let mut sorted_data: Vec<_> = self.deck_sizes.iter().collect();
        sorted_data.sort_by_key(|((game_id, turn, actor), _)| (*game_id, *turn, *actor));

        // Write header
        wtr.write_record(&["game_id", "turn", "actor", "deck_size"])
            .expect("Failed to write header to CSV");
        for ((game_id, turn, actor), deck_size) in sorted_data {
            wtr.write_record(&[
                game_id.to_string(),
                turn.to_string(),
                actor.to_string(),
                deck_size.to_string(),
            ])
            .expect("Failed to write record to CSV");
        }
        wtr.flush().expect("Failed to flush CSV writer");
        println!("Deck sizes written to {}", absolute_path.display());
    }
}

/// Simulation event handler that tracks the first turn a card is in play.
pub struct FirstTurnSeenCollector {
    /// (game_id, actor, card_name) -> min_turn
    first_turn_seen: HashMap<(Uuid, usize, String), u8>,
}

impl FirstTurnSeenCollector {
    pub fn new() -> Self {
        Self {
            first_turn_seen: HashMap::new(),
        }
    }
}

impl SimulationEventHandler for FirstTurnSeenCollector {
    fn on_action(
        &mut self,
        game_id: Uuid,
        _state_before_action: &State,
        actor: usize,
        _playable_actions: &[Action],
        _action: &Action,
        state_after_action: &State,
    ) {
        let turn = state_after_action.turn_count;
        for card in state_after_action.in_play_pokemon[actor].iter() {
            if let Some(card) = card {
                let card_name = card.get_name();
                let key = (game_id, actor, card_name.clone());
                if !self.first_turn_seen.contains_key(&key) {
                    println!(
                        "Setting first turn seen for {:?} in game {}: turn {}",
                        card_name.clone(),
                        game_id,
                        turn
                    );
                    self.first_turn_seen.insert(key, turn);
                }
            }
        }
    }

    fn on_simulation_end(&mut self) {
        println!("First turn seen collected for all games. Writing to CSV...");

        // Write the first turn seen to a CSV file
        let absolute_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("first_turn_seen.csv");
        let mut wtr = csv::Writer::from_path(&absolute_path).expect("Failed to create CSV writer");

        // Convert to Vec and sort by (game_id, actor, card_name)
        let mut sorted_data: Vec<_> = self.first_turn_seen.iter().collect();
        sorted_data
            .sort_by_key(|((game_id, actor, card_name), _)| (*game_id, *actor, card_name.clone()));

        // Write header
        wtr.write_record(&["game_id", "actor", "card_name", "min_turn"])
            .expect("Failed to write header to CSV");
        for ((game_id, actor, card_name), min_turn) in &sorted_data {
            wtr.write_record(&[
                game_id.to_string(),
                actor.to_string(),
                card_name.clone(),
                min_turn.to_string(),
            ])
            .expect("Failed to write record to CSV");
        }
        wtr.flush().expect("Failed to flush CSV writer");
        println!("First turn seen written to {}", absolute_path.display());
    }
}

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use deckgym::{
    actions::Action,
    players::PlayerCode,
    simulate::initialize_logger,
    simulation_event_handler::{SimulationEventHandler, StatsCollector},
    Simulation, State,
};

/// Run with `cargo run --example sylveon_deck_draw_speed --features csv-export`
fn main() {
    let num_simulations = 1_000_000;
    let deck_a_path = "example_decks/solgaleo_shiinotic.txt";
    let deck_b_path = "example_decks/solgaleo_sylveon.txt";
    let player_codes = vec![PlayerCode::ER, PlayerCode::ER];
    initialize_logger(1);
    println!("This will count how fast the user cant draw cards from the deck.");

    let mut simulation = Simulation::new(
        deck_a_path,
        deck_b_path,
        player_codes,
        num_simulations,
        None,
        false, // parallel
        None,  // num_threads
    )
    .expect("Failed to create simulation")
    .register::<StatsCollector>()
    .register::<FirstTurnSeenCollector>();

    simulation.run();
}

/// Simulation event handler that tracks the first turn a card is in play.
pub struct FirstTurnSeenCollector {
    /// (game_id, actor, card_name) -> min_turn
    first_turn_seen: HashMap<(Uuid, usize, String), u8>,
    game_ids: HashSet<Uuid>,
}

impl Default for FirstTurnSeenCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl FirstTurnSeenCollector {
    pub fn new() -> Self {
        Self {
            first_turn_seen: HashMap::new(),
            game_ids: HashSet::new(),
        }
    }
}

impl SimulationEventHandler for FirstTurnSeenCollector {
    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(other) = (other as &dyn std::any::Any).downcast_ref::<Self>() {
            for (key, &turn) in &other.first_turn_seen {
                self.first_turn_seen
                    .entry(key.clone())
                    .and_modify(|existing| *existing = (*existing).min(turn))
                    .or_insert(turn);
            }
            self.game_ids.extend(&other.game_ids);
        }
    }

    fn on_action(
        &mut self,
        game_id: Uuid,
        state_before_action: &State,
        actor: usize,
        _playable_actions: &[Action],
        _action: &Action,
    ) {
        let turn = state_before_action.turn_count;
        for card in state_before_action.in_play_pokemon[actor].iter().flatten() {
            let card_name = card.get_name();
            let key = (game_id, actor, card_name.clone());
            self.first_turn_seen.entry(key).or_insert(turn);
        }
    }

    // Every 100 games, give a progress update
    fn on_game_end(
        &mut self,
        _game_id: Uuid,
        _state: State,
        _result: Option<deckgym::state::GameOutcome>,
    ) {
        if self.game_ids.len().is_multiple_of(1000) {
            println!(
                "Processed {} games so far. First turn seen: {}",
                self.game_ids.len(),
                self.first_turn_seen.len()
            );
        }
        // Store the game ID to track progress
        self.game_ids.insert(_game_id);
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
        wtr.write_record(["game_id", "actor", "card_name", "min_turn"])
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

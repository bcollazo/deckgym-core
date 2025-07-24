use std::collections::HashMap;
use uuid::Uuid;

use deckgym::{
    actions::Action,
    players::PlayerCode,
    simulation_event_handler::{CompositeSimulationEventHandler, SimulationEventHandler},
    Simulation, State,
};

/// Run with `cargo run --example sylveon_deck_draw_speed`
fn main() {
    let num_simulations = 1000;
    let deck_a_path = "example_decks/solgaleo_sylveon.txt";
    let deck_b_path = "example_decks/mewtwoex.txt";
    let player_codes = vec![PlayerCode::E, PlayerCode::E];
    println!("This will count how fast the user cant draw cards from the deck.");

    let deckout_collector = Box::new(DeckOutCollector::new());
    let mut composite_handler = CompositeSimulationEventHandler::new();
    composite_handler.add_handler(deckout_collector);
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

/// It'd be nice to have the state passed in on_action, so that we can access the current player and their deck.
/// we probably have to keep track of (turn, actor, deck_size) tuples to know when a player runs out of cards.
/// Then with that, we can get a turn-where-player-ran-out-of-cards per game.
/// This will generate a CSV file with (game_id, turn, actor, deck_size) tuples for each game.
impl SimulationEventHandler for DeckOutCollector {
    fn on_action(
        &mut self,
        game_id: Uuid,
        _state_before_action: &State,
        actor: usize,
        _playable_actions: &Vec<Action>,
        _action: &Action,
        state_after_action: &State,
    ) {
        let turn = state_after_action.turn_count;
        let deck_size = state_after_action.decks[actor].cards.len();

        // Since this is called multiple times per turn, writing idempotently
        // we'll store the last known deck size for each actor and turn.
        self.deck_sizes.insert((game_id, turn, actor), deck_size);
    }

    fn on_game_end(
        &mut self,
        _game_id: Uuid,
        _state: State,
        _result: Option<deckgym::state::GameOutcome>,
    ) {
        println!("Game ended");
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

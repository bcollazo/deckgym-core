/// Zeraora Thunderclap Flash: Basics count experiment
///
/// Produces the data for bar charts showing:
///   - P(at least 1 Zeraora triggers)
///   - P(both Zeraoras trigger)
/// as a function of number of basic Pokemon (2-8).
///
/// Run with: cargo run --example zeraora_basics --release
use std::any;
use std::fmt::Debug;

use rand::{rngs::StdRng, seq::SliceRandom};
use uuid::Uuid;

use deckgym::{
    actions::{Action, SimpleAction},
    deck::Deck,
    players::Player,
    simulation_event_handler::SimulationEventHandler,
    Simulation, State,
};

#[derive(Default, Clone)]
pub struct ZeraoraTracker {
    pub games_0: u32,
    pub games_1: u32,
    pub games_2: u32,
}

impl ZeraoraTracker {
    fn total(&self) -> u32 {
        self.games_0 + self.games_1 + self.games_2
    }
    fn pct_at_least_1(&self) -> f64 {
        (self.games_1 + self.games_2) as f64 / self.total() as f64 * 100.0
    }
    fn pct_2(&self) -> f64 {
        self.games_2 as f64 / self.total() as f64 * 100.0
    }
}

impl SimulationEventHandler for ZeraoraTracker {
    fn merge(&mut self, other: &dyn SimulationEventHandler) {
        if let Some(o) = (other as &dyn any::Any).downcast_ref::<Self>() {
            self.games_0 += o.games_0;
            self.games_1 += o.games_1;
            self.games_2 += o.games_2;
        }
    }

    fn on_action(
        &mut self,
        _game_id: Uuid,
        state: &State,
        actor: usize,
        _playable_actions: &[Action],
        action: &Action,
    ) {
        // Count Zeraoras in play at end of turn 1 for player 0
        if state.turn_count == 1 && actor == 0 && matches!(action.action, SimpleAction::EndTurn) {
            let n = state.in_play_pokemon[0]
                .iter()
                .flatten()
                .filter(|p| p.get_name() == "Zeraora")
                .count();
            match n {
                0 => self.games_0 += 1,
                1 => self.games_1 += 1,
                _ => self.games_2 += 1,
            }
        }
    }
}

pub struct ZeraoraFirstPlayer {
    deck: Deck,
}

impl Debug for ZeraoraFirstPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZeraoraFirstPlayer")
    }
}

impl Player for ZeraoraFirstPlayer {
    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }

    fn decision_fn(
        &mut self,
        rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        if state.turn_count <= 2 {
            // 1. Place Zeraora if we have one
            for a in possible_actions {
                if let SimpleAction::Place(card, _) = &a.action {
                    if card.get_name() == "Zeraora" {
                        return a.clone();
                    }
                }
            }
            // 2. Play Poké Ball to dig for one
            for a in possible_actions {
                if let SimpleAction::Play { trainer_card } = &a.action {
                    if trainer_card.name.contains("Poké Ball") {
                        return a.clone();
                    }
                }
            }
            // 3. Play Professor's Research
            for a in possible_actions {
                if let SimpleAction::Play { trainer_card } = &a.action {
                    if trainer_card.name.contains("Professor's Research") {
                        return a.clone();
                    }
                }
            }
            // 4. Place any other basic
            for a in possible_actions {
                if let SimpleAction::Place(_, _) = &a.action {
                    return a.clone();
                }
            }
        }
        possible_actions
            .choose(rng)
            .expect("must have action")
            .clone()
    }
}

fn run_variant(deck_path: &str, opponent_path: &str, num_games: u32) -> ZeraoraTracker {
    let deck_a =
        Deck::from_file(deck_path).unwrap_or_else(|e| panic!("Failed to load {deck_path}: {e}"));
    let deck_b = Deck::from_file(opponent_path)
        .unwrap_or_else(|e| panic!("Failed to load {opponent_path}: {e}"));

    let da = deck_a.clone();
    let db = deck_b.clone();

    let mut sim = Simulation::new_with_player_factory(
        deck_a,
        deck_b,
        move |_, _| {
            vec![
                Box::new(ZeraoraFirstPlayer { deck: da.clone() }) as Box<dyn Player>,
                Box::new(ZeraoraFirstPlayer { deck: db.clone() }) as Box<dyn Player>,
            ]
        },
        num_games,
        None,
        true,
        None,
    )
    .expect("simulation init")
    .register::<ZeraoraTracker>();

    sim.run();
    sim.get_event_handler::<ZeraoraTracker>()
        .expect("tracker")
        .clone()
}

fn main() {
    let opponent = "example_decks/venusaur-exeggutor.txt";
    let n = 100_000u32;
    let dir = "example_decks/zeraora_basics";

    println!("Zeraora Basics Experiment (games per variant: {n})");
    println!();

    let mut results_1: Vec<f64> = Vec::new();
    let mut results_2: Vec<f64> = Vec::new();

    for basics in 2..=8 {
        let path = format!("{dir}/{basics}_basics.txt");
        let t = run_variant(&path, opponent, n);
        let pct1 = t.pct_at_least_1();
        let pct2 = t.pct_2();
        results_1.push(pct1);
        results_2.push(pct2);
        println!(
            "Basics={basics}: P(>=1 trigger)={:.1}%  P(2 triggers)={:.1}%",
            pct1, pct2
        );
    }

    println!();
    println!("=== Chart Data ===");
    println!();
    println!("P(at least 1 Zeraora triggers):");
    let y1: Vec<String> = results_1.iter().map(|v| format!("{v:.1}")).collect();
    println!("y: [{}]", y1.join(", "));

    println!();
    println!("P(both Zeraoras trigger):");
    let y2: Vec<String> = results_2.iter().map(|v| format!("{v:.1}")).collect();
    println!("y: [{}]", y2.join(", "));
}

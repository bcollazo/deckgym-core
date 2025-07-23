use colored::Colorize;
use log::{debug, info, trace};
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    actions::{apply_action, Action},
    generate_possible_actions,
    players::Player,
    simulation_event_handler::{CompositeSimulationEventHandler, SimulationEventHandler},
    state::GameOutcome,
    types::EnergyType,
    State,
};

// It has a lifetime to allow it to borrow the event handler mutably for the duration of the game
pub struct Game<'a> {
    seed: u64,
    rng: StdRng,
    players: Vec<Box<dyn Player>>,

    state: State,

    debug: bool,
    event_handler: Option<&'a mut CompositeSimulationEventHandler>,
}

impl<'a> Game<'a> {
    pub fn from_state(state: State, players: Vec<Box<dyn Player>>, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        Game {
            seed,
            rng,
            players,
            state,
            debug: false,
            event_handler: None,
        }
    }

    pub fn new(players: Vec<Box<dyn Player>>, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let deck_a = players[0].get_deck();
        let deck_b = players[1].get_deck();
        let state = State::initialize(&deck_a, &deck_b, &mut rng);
        Game {
            seed,
            rng,
            players,
            state,
            debug: true,
            event_handler: None,
        }
    }

    pub fn new_with_event_handlers(
        players: Vec<Box<dyn Player>>,
        seed: u64,
        event_handler: &'a mut CompositeSimulationEventHandler,
    ) -> Self {
        let mut game = Game::new(players, seed);
        game.event_handler = Some(event_handler);
        game
    }

    // Returns None if the game times out
    pub fn play(&mut self) -> Option<GameOutcome> {
        if self.debug {
            info!("Playing game with seed: {}", self.seed);
        }
        while !self.state.is_game_over() {
            self.play_tick();
        }
        self.state.winner
    }

    pub fn play_tick(&mut self) -> Action {
        let (actor, actions) = generate_possible_actions(&self.state);

        let player = &self.players[actor];
        let color = self.get_color(actor);
        self.print_turn_header(actor, player.as_ref(), &color);
        let action = if actions.len() == 1 {
            debug!("Only one possible action, selecting it.");
            actions[0].clone()
        } else {
            let player = self.players[actor].as_mut();
            trace!(
                "Possible Actions: {:?}",
                actions.iter().map(|x| x.action.clone()).collect::<Vec<_>>()
            );
            player.decision_fn(&mut self.rng, &self.state, &actions)
        };
        if let Some(handler) = &mut self.event_handler {
            handler.on_action(actor, &actions, &action);
        }

        let player = &self.players[actor];
        self.print_action(&action, actor, player.as_ref(), &color);
        self.apply_action(&action);
        self.print_state();
        action
    }

    pub fn get_state_clone(&self) -> State {
        self.state.clone()
    }

    // TODO: Maybe make these only available for testing?
    pub fn apply_action(&mut self, action: &Action) {
        apply_action(&mut self.rng, &mut self.state, action);
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn print_turn_header(&self, actor: usize, player: &dyn Player, color: &str) {
        if self.debug {
            debug!(
                "{}{}",
                format!("===== {}|{:?}|", self.state.turn_count, self.state.points).color(color),
                format!("{actor}:{player:?}").color(color),
            );
        }
    }

    fn print_action(&self, action: &Action, _: usize, player: &dyn Player, color: &str) {
        if self.debug {
            info!(
                "{} chose {}",
                format!("{}:{:?}", self.state.turn_count, player).color(color),
                format!("{:?}", action.action).bold()
            );
        }
    }

    fn print_state(&self) {
        if self.debug {
            trace!("{}", self.state.debug_string());
        }
    }

    /// see https://github.com/colored-rs/colored?tab=readme-ov-file#colors
    fn get_color(&self, actor: usize) -> String {
        let energy = self.state.decks[actor].energy_types[0];
        let color = match energy {
            EnergyType::Colorless => todo!(),
            EnergyType::Fighting => "red",
            EnergyType::Fire => "red",
            EnergyType::Grass => "green",
            EnergyType::Lightning => "yellow",
            EnergyType::Psychic => "magenta",
            EnergyType::Water => "blue",
            EnergyType::Darkness => "bright_black",
            EnergyType::Metal => "bright_black",
            EnergyType::Dragon => todo!(),
        };
        color.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        players::{AttachAttackPlayer, EndTurnPlayer, Player},
        state::GameOutcome,
        test_helpers::load_test_decks,
        Game,
    };

    #[test]
    fn test_poison() {
        let (deck_a, deck_b) = load_test_decks();
        let player_a = Box::new(AttachAttackPlayer { deck: deck_a });
        let player_b = Box::new(EndTurnPlayer { deck: deck_b });
        let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
        let mut game = Game::new(players, 3);

        // Play initial setup phase
        while game.get_state_clone().turn_count == 0 {
            game.play_tick();
        }

        // Manually poison the opponent's Koffing
        let mut state = game.get_state_clone();
        state.in_play_pokemon[1][0].as_mut().unwrap().poisoned = true;
        game.set_state(state);

        // The game starts with AA playing. After each turn 10 damage should be subtracted.
        // So ending 1 Koffing should have 60HP, 2 => 50HP, 3 => 40HP, 4 => 30HP, 5 => 20HP
        while game.get_state_clone().turn_count == 1 {
            game.play_tick();
        }
        // Koffing should have 60 HP starting turn 2
        assert_eq!(game.get_state_clone().get_remaining_hp(1, 0), 60);
        while game.get_state_clone().turn_count == 2 {
            game.play_tick();
        }
        // Koffing should have 50 HP starting turn 3
        assert_eq!(game.get_state_clone().get_remaining_hp(1, 0), 50);
        while game.get_state_clone().turn_count == 3 {
            game.play_tick();
        }

        // Now play the rest. AA should win b.c. ET has no bench pokemon
        let winner = game.play();
        assert_eq!(game.get_state_clone().turn_count, 5);
        assert_eq!(winner, Some(GameOutcome::Win(0)));
    }

    #[test]
    fn test_ko_by_posion() {
        let (deck_a, deck_b) = load_test_decks();
        let player_a = Box::new(EndTurnPlayer { deck: deck_a });
        let player_b = Box::new(AttachAttackPlayer { deck: deck_b });
        let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
        let mut game = Game::new(players, 4); // EndTurnPlayer starts

        // Turn 1, EE ends. Turn 2, AA attaches and attacks. Exeggcute should have 30 HP.
        // Turn 3, ET ends. We artificially poision, so that after playing out turn 4
        // (AA attacks) Exeggcute has 10 HP and KO from poison.
        while game.state.turn_count < 4 {
            game.play_tick();
        }
        assert_eq!(game.get_state_clone().get_remaining_hp(0, 0), 30);

        // Artificially poison Exeggcute
        let mut state = game.get_state_clone();
        state.in_play_pokemon[0][0].as_mut().unwrap().poisoned = true;
        game.set_state(state);

        // Turn 45, AA attacks. After ending, AA should win since no bench.
        while game.state.turn_count == 4 {
            game.play_tick();
        }
        assert_eq!(game.get_state_clone().points[0], 0);
        assert_eq!(game.get_state_clone().points[1], 1);
        game.play();
        assert_eq!(game.get_state_clone().turn_count, 5);
    }

    // TODO: Look for a game that has bench, and pokemon can die from attack + poison
    //   to launche the complicated sequence of Poison K.O. then user having
    //   to select one pokemon to promote to active.

    // TODO: Multiple bench KO
}

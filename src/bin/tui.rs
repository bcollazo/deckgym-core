use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use deckgym::{
    players::{fill_code_array, parse_player_code, PlayerCode},
    tui::{ui, App},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the first deck file
    deck_a: String,

    /// Path to the second deck file
    deck_b: String,

    /// Players' strategies as a comma-separated list
    #[arg(long, value_delimiter = ',', value_parser = parse_player_code)]
    players: Option<Vec<PlayerCode>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();
    let player_codes = fill_code_array(cli.players);

    // Setup panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        // Call the original panic hook
        original_hook(panic_info);
    }));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(&cli.deck_a, &cli.deck_b, player_codes)?;
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    // Numeric keys for action selection (1-9)
                    KeyCode::Char(c @ '1'..='9') => {
                        let index = (c as usize) - ('1' as usize);
                        app.handle_action_selection(index);
                    }
                    // Replay mode controls
                    KeyCode::Down => app.next_state(),
                    KeyCode::Up => app.prev_state(),
                    KeyCode::Char('w') => app.scroll_page_up(),
                    KeyCode::Char('s') => app.scroll_page_down(),
                    KeyCode::Left => app.scroll_player_hand_left(),
                    KeyCode::Right => app.scroll_player_hand_right(),
                    KeyCode::Char('c') => app.toggle_lock_actions_center(),
                    KeyCode::Char('A') => app.scroll_opponent_hand_left(),
                    KeyCode::Char('D') => app.scroll_opponent_hand_right(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Tick the game (advances game in interactive mode)
            app.tick_game();

            // Check if game is over
            if app.is_game_over() {
                return Ok(());
            }

            last_tick = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_ui_renders_without_panic() {
        // Create App using example decks
        let app = App::new(
            "example_decks/venusaur-exeggutor.txt",
            "example_decks/weezing-arbok.txt",
            vec![PlayerCode::R, PlayerCode::R],
        )
        .expect("Failed to create app");

        // Create a test backend
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

        // This should not panic
        terminal.draw(|f| ui(f, &app)).expect("Failed to render UI");
    }
}

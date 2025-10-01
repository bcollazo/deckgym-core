use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use deckgym::{
    types::*, 
    State, 
    Game,
    test_helpers::load_test_decks,
    players::{Player, RandomPlayer}
};
use rand::{thread_rng, Rng};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct App {
    state: State,
}

impl App {
    fn new() -> App {
        // Initialize a real Game like in integration tests
        let (deck_a, deck_b) = load_test_decks(); // venusaur-exeggutor vs weezing-arbok
        
        // Create random players
        let player_a = Box::new(RandomPlayer { deck: deck_a });
        let player_b = Box::new(RandomPlayer { deck: deck_b });
        let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
        
        // Initialize game with a fixed seed for reproducible results
        let mut rng = thread_rng();
        let seed = rng.gen::<u64>();
        let mut game = Game::new(players, seed);
        
        // Play some ticks to get an interesting game state
        let mut ticks_played = 0;
        while ticks_played < 20 && game.get_state_clone().winner.is_none() {
            game.play_tick();
            ticks_played += 1;
        }
        
        // Extract the final state
        let state = game.get_state_clone();
        
        App { state }
    }

    fn increment_counter(&mut self) {
        self.state.turn_count += 1;
    }

    fn decrement_counter(&mut self) {
        if self.state.turn_count > 0 {
            self.state.turn_count -= 1;
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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
        println!("{:?}", err)
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
                    KeyCode::Up => app.increment_counter(),
                    KeyCode::Down => app.decrement_counter(),
                    KeyCode::Char('+') => app.increment_counter(),
                    KeyCode::Char('-') => app.decrement_counter(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn render_hand_card<'a>(card: &'a Card, index: usize) -> (Vec<Line<'a>>, Style) {
    let name = card.get_name();
    let truncated_name = if name.len() > 18 {
        format!("{}...", &name[..15])
    } else {
        name
    };
    
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{}", index + 1), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(truncated_name, Style::default().fg(Color::Cyan))
        ]),
        Line::from(""),
        Line::from(""),
        Line::from("")
    ];
    
    (lines, Style::default().fg(Color::Cyan))
}

fn render_pokemon_card<'a>(
    pokemon: &'a Option<PlayedCard>,
    _title: &str,
    player_color: Color,
) -> (Vec<Line<'a>>, Style) {
    match pokemon {
        Some(played_card) => {
            let name = played_card.card.get_name();
            let hp_text = format!("{}/{}", played_card.remaining_hp, played_card.total_hp);
            let energy_count = played_card.attached_energy.len();
            let energy_text = format!("⚡{}", energy_count);
            
            let mut status_effects = Vec::new();
            if played_card.poisoned { status_effects.push("🟣PSN"); }
            if played_card.paralyzed { status_effects.push("🟡PAR"); }
            if played_card.asleep { status_effects.push("💤SLP"); }
            
            let status_line = if !status_effects.is_empty() {
                status_effects.join(" ")
            } else {
                "".to_string()
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled(name, Style::default().fg(player_color).add_modifier(Modifier::BOLD))
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("HP: ", Style::default().fg(Color::White)),
                    Span::styled(hp_text, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                ]),
                Line::from(vec![
                    Span::styled("Energy: ", Style::default().fg(Color::White)),
                    Span::styled(energy_text, Style::default().fg(Color::Yellow))
                ]),
                if !status_line.is_empty() {
                    Line::from(vec![
                        Span::styled(status_line, Style::default().fg(Color::Magenta))
                    ])
                } else {
                    Line::from("")
                },
                Line::from("")
            ];
            
            (lines, Style::default().fg(player_color))
        }
        None => {
            let lines = vec![
                Line::from(""),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Empty", Style::default().fg(Color::DarkGray))
                ]),
                Line::from(""),
                Line::from(""),
                Line::from("")
            ];
            (lines, Style::default().fg(Color::DarkGray))
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title block
    let title = Paragraph::new("DeckGym TUI - Pokémon TCG Pocket Battle Mat")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("DeckGym"));
    f.render_widget(title, chunks[0]);

    // Battle mat and hand area
    let main_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),     // Battle mat area
                Constraint::Length(8),  // Hand area
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Battle mat area - more compact for space efficiency
    let battle_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(8),  // Opponent bench - compact but readable
                Constraint::Length(8),  // Opponent active
                Constraint::Length(8),  // Player active  
                Constraint::Length(8),  // Player bench
            ]
            .as_ref(),
        )
        .split(main_area[0]);

    // Opponent bench (top row) - centered layout
    let opponent_bench_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(20), // Bench 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 2  
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 3
            Constraint::Min(0),     // Right padding
        ])
        .split(battle_area[0]);

    // Render opponent bench slots (using indices 1, 3, 5 to account for spacing)
    let bench_indices = [1, 3, 5]; // Skip spacing slots
    for (bench_pos, &chunk_idx) in bench_indices.iter().enumerate() {
        let pokemon = &app.state.in_play_pokemon[0][bench_pos + 1]; // bench positions 1, 2, 3
        let (lines, style) = render_pokemon_card(pokemon, &format!("Opp Bench {}", bench_pos + 1), Color::Red);
        
        let pokemon_block = Paragraph::new(lines)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center).title(format!("Bench {}", bench_pos + 1)));
        f.render_widget(pokemon_block, opponent_bench_chunks[chunk_idx]);
    }

    // Opponent active (center) - match bench alignment
    let opponent_active_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding (same as bench)
            Constraint::Length(20), // Bench 1 position (invisible)
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Active (matches middle bench size)
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 3 position (invisible)
            Constraint::Min(0),     // Right padding (same as bench)
        ])
        .split(battle_area[1]);

    let opponent_active = &app.state.in_play_pokemon[0][0];
    let (lines, style) = render_pokemon_card(opponent_active, "Opponent Active", Color::Red);
    let opponent_active_block = Paragraph::new(lines)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center).title("Active"));
    f.render_widget(opponent_active_block, opponent_active_area[3]); // Use middle position (index 3)

    // Player active (center) - match bench alignment
    let player_active_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding (same as bench)
            Constraint::Length(20), // Bench 1 position (invisible)
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Active (matches middle bench size)
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 3 position (invisible)
            Constraint::Min(0),     // Right padding (same as bench)
        ])
        .split(battle_area[2]);

    let player_active = &app.state.in_play_pokemon[1][0];
    let (lines, style) = render_pokemon_card(player_active, "Your Active", Color::Green);
    let player_active_block = Paragraph::new(lines)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center).title("Active"));
    f.render_widget(player_active_block, player_active_area[3]); // Use middle position (index 3)

    // Player bench (bottom row) - centered layout
    let player_bench_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(20), // Bench 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(20), // Bench 3
            Constraint::Min(0),     // Right padding
        ])
        .split(battle_area[3]);

    // Render player bench slots (using indices 1, 3, 5 to account for spacing)
    let bench_indices = [1, 3, 5]; // Skip spacing slots
    for (bench_pos, &chunk_idx) in bench_indices.iter().enumerate() {
        let pokemon = &app.state.in_play_pokemon[1][bench_pos + 1]; // bench positions 1, 2, 3
        let (lines, style) = render_pokemon_card(pokemon, &format!("Your Bench {}", bench_pos + 1), Color::Green);
        
        let pokemon_block = Paragraph::new(lines)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center).title(format!("Bench {}", bench_pos + 1)));
        f.render_widget(pokemon_block, player_bench_chunks[chunk_idx]);
    }

    // Hand display area
    let hand_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(15), // Card 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(15), // Card 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(15), // Card 3
            Constraint::Length(1),  // Spacing
            Constraint::Length(15), // Card 4
            Constraint::Length(1),  // Spacing
            Constraint::Length(15), // Card 5
            Constraint::Min(0),     // Right padding
        ])
        .split(main_area[1]);

    // Render up to 5 cards from player's hand
    let player_hand = &app.state.hands[1]; // Player 1's hand (your hand)
    let cards_to_show = std::cmp::min(5, player_hand.len());
    
    for i in 0..cards_to_show {
        let card = &player_hand[i];
        let (lines, style) = render_hand_card(card, i);
        
        let hand_card_block = Paragraph::new(lines)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center).title("Hand"));
        
        // Render to positions 1, 3, 5, 7, 9 (skipping spacing)
        let chunk_index = 1 + (i * 2);
        f.render_widget(hand_card_block, hand_chunks[chunk_index]);
    }

    // Footer with controls and counter
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[2]);

    let footer = Paragraph::new("Press ESC or 'q' to quit | Use arrow keys or +/- to change turn counter")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(footer, footer_chunks[0]);

    let counter_text = format!("Turn: {}", app.state.turn_count);
    let counter_block = Paragraph::new(counter_text)
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Game State"));
    f.render_widget(counter_block, footer_chunks[1]);
}
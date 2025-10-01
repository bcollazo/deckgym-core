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
    players::{Player, RandomPlayer},
    generate_possible_actions,
};
use rand::{thread_rng, Rng};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct App {
    game: Game<'static>,
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

        App { game }
    }

    fn get_state(&self) -> State {
        self.game.get_state_clone()
    }

    fn play_tick(&mut self) {
        self.game.play_tick();
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
                    KeyCode::Right | KeyCode::Char(' ') => app.play_tick(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn energy_type_to_color(energy_type: EnergyType) -> Color {
    match energy_type {
        EnergyType::Grass => Color::Green,
        EnergyType::Fire => Color::Red,
        EnergyType::Water => Color::Blue,
        EnergyType::Lightning => Color::Yellow,
        EnergyType::Psychic => Color::Magenta,
        EnergyType::Fighting => Color::Red,
        EnergyType::Darkness => Color::DarkGray,
        EnergyType::Metal => Color::White,
        EnergyType::Dragon => Color::Cyan,
        EnergyType::Colorless => Color::Gray,
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
) -> (Vec<Line<'a>>, Style, Color, bool) {
    match pokemon {
        Some(played_card) => {
            let name = played_card.card.get_name();
            let hp_text = format!("{}/{}", played_card.remaining_hp, played_card.total_hp);
            let energy_count = played_card.attached_energy.len();
            let energy_icons = "⚡".repeat(energy_count);

            let mut status_effects = Vec::new();
            if played_card.poisoned { status_effects.push("🟣PSN"); }
            if played_card.paralyzed { status_effects.push("🟡PAR"); }
            if played_card.asleep { status_effects.push("💤SLP"); }

            let status_line = if !status_effects.is_empty() {
                status_effects.join(" ")
            } else {
                "".to_string()
            };

            // Get attack names and energy type (only if it's a Pokemon card)
            let (attack_names, card_type_color): (Vec<String>, Color) = match &played_card.card {
                Card::Pokemon(pokemon_card) => {
                    let attacks = pokemon_card.attacks.iter()
                        .map(|a| {
                            let truncated = if a.title.len() > 20 {
                                format!("{}...", &a.title[..17])
                            } else {
                                a.title.clone()
                            };
                            truncated
                        })
                        .collect();
                    let color = energy_type_to_color(pokemon_card.energy_type);
                    (attacks, color)
                },
                _ => (vec![], Color::Gray)
            };

            // Create first line with name on left and HP on right
            let name_hp_line = Line::from(vec![
                Span::styled(name, Style::default().fg(player_color).add_modifier(Modifier::BOLD)),
                Span::styled(" ".repeat(1), Style::default()), // spacing
                Span::styled(hp_text, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            ]);

            let mut lines = vec![
                name_hp_line,
                Line::from(""),
            ];

            // Add attack names
            for attack_name in attack_names {
                lines.push(Line::from(vec![
                    Span::styled(attack_name, Style::default().fg(Color::White))
                ]));
            }

            // Add status effects if any
            if !status_line.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(status_line, Style::default().fg(Color::Magenta))
                ]));
            }

            // Pad to maintain consistent height (total 6 lines of content)
            while lines.len() < 5 {
                lines.push(Line::from(""));
            }

            // Add energy icons at the bottom
            lines.push(Line::from(vec![
                Span::styled(energy_icons, Style::default().fg(Color::Yellow))
            ]));

            (lines, Style::default().fg(player_color), card_type_color, false)
        }
        None => {
            let lines = vec![
                Line::from(""),
                Line::from(""),
                Line::from(""),
                Line::from(""),
                Line::from(""),
                Line::from("")
            ];
            (lines, Style::default().fg(Color::White), Color::White, true)
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let state = app.get_state();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title block with game status
    let title_text = format!(
        "DeckGym TUI - Turn: {} | P1 Points: {} | P2 Points: {}",
        state.turn_count, state.points[0], state.points[1]
    );
    let title = Paragraph::new(title_text)
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
            Constraint::Length(24), // Bench 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 3
            Constraint::Min(0),     // Right padding
        ])
        .split(battle_area[0]);

    // Render opponent bench slots (using indices 1, 3, 5 to account for spacing)
    let bench_indices = [1, 3, 5]; // Skip spacing slots
    for (bench_pos, &chunk_idx) in bench_indices.iter().enumerate() {
        let pokemon = &state.in_play_pokemon[0][bench_pos + 1]; // bench positions 1, 2, 3
        let (lines, style, border_color, is_empty) = render_pokemon_card(pokemon, &format!("Opp Bench {}", bench_pos + 1), Color::Red);

        let mut block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)).title_alignment(Alignment::Center).title(format!("Bench {}", bench_pos + 1));
        if is_empty {
            block = block.border_type(BorderType::Rounded);
        }

        let pokemon_block = Paragraph::new(lines)
            .style(style)
            .block(block);
        f.render_widget(pokemon_block, opponent_bench_chunks[chunk_idx]);
    }

    // Opponent active (center) - match bench alignment
    let opponent_active_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding (same as bench)
            Constraint::Length(24), // Bench 1 position (invisible)
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Active (matches middle bench size)
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 3 position (invisible)
            Constraint::Min(0),     // Right padding (same as bench)
        ])
        .split(battle_area[1]);

    let opponent_active = &state.in_play_pokemon[0][0];
    let (lines, style, border_color, is_empty) = render_pokemon_card(opponent_active, "Opponent Active", Color::Red);

    let mut block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)).title_alignment(Alignment::Center).title("Active");
    if is_empty {
        block = block.border_type(BorderType::Rounded);
    }

    let opponent_active_block = Paragraph::new(lines)
        .style(style)
        .block(block);
    f.render_widget(opponent_active_block, opponent_active_area[3]); // Use middle position (index 3)

    // Player active (center) - match bench alignment
    let player_active_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding (same as bench)
            Constraint::Length(24), // Bench 1 position (invisible)
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Active (matches middle bench size)
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 3 position (invisible)
            Constraint::Min(0),     // Right padding (same as bench)
        ])
        .split(battle_area[2]);

    let player_active = &state.in_play_pokemon[1][0];
    let (lines, style, border_color, is_empty) = render_pokemon_card(player_active, "Your Active", Color::Green);

    let mut block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)).title_alignment(Alignment::Center).title("Active");
    if is_empty {
        block = block.border_type(BorderType::Rounded);
    }

    let player_active_block = Paragraph::new(lines)
        .style(style)
        .block(block);
    f.render_widget(player_active_block, player_active_area[3]); // Use middle position (index 3)

    // Player bench (bottom row) - centered layout
    let player_bench_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(24), // Bench 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(24), // Bench 3
            Constraint::Min(0),     // Right padding
        ])
        .split(battle_area[3]);

    // Render player bench slots (using indices 1, 3, 5 to account for spacing)
    let bench_indices = [1, 3, 5]; // Skip spacing slots
    for (bench_pos, &chunk_idx) in bench_indices.iter().enumerate() {
        let pokemon = &state.in_play_pokemon[1][bench_pos + 1]; // bench positions 1, 2, 3
        let (lines, style, border_color, is_empty) = render_pokemon_card(pokemon, &format!("Your Bench {}", bench_pos + 1), Color::Green);

        let mut block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)).title_alignment(Alignment::Center).title(format!("Bench {}", bench_pos + 1));
        if is_empty {
            block = block.border_type(BorderType::Rounded);
        }

        let pokemon_block = Paragraph::new(lines)
            .style(style)
            .block(block);
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
    let player_hand = &state.hands[1]; // Player 1's hand (your hand)
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

    // Footer with possible actions
    let (actor, actions) = generate_possible_actions(&state);
    let action_strings: Vec<String> = actions.iter()
        .take(10) // Limit to first 10 actions
        .map(|a| format!("{:?}", a.action))
        .collect();

    let actions_text = if action_strings.is_empty() {
        "No actions available".to_string()
    } else {
        action_strings.join(" | ")
    };

    let footer_text = format!(
        "Controls: ESC/q=quit, Space/Right=next tick | Current Player: P{}\nPossible Actions: {}",
        actor + 1,
        actions_text
    );

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Actions"));
    f.render_widget(footer, chunks[2]);
}
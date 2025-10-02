use crate::generate_possible_actions;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;
use super::render::{render_hand_card, render_pokemon_card};

pub fn ui(f: &mut Frame, app: &App) {
    let state = app.get_state();

    // Main layout: left (battle log), center (game)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25), // Battle log area
            Constraint::Percentage(75), // Game area
        ])
        .split(f.area());

    // Center: game area with header, battle mat, hand areas, and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Length(5), // Opponent's hand
                Constraint::Min(0),    // Battle mat
                Constraint::Length(5), // Player's hand
                Constraint::Length(8), // Footer
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    // Title block with game status
    let title_text = format!(
        "DeckGym TUI - State: {}/{} | Turn: {} | P1 Points: {} | P2 Points: {}",
        app.current_state_index + 1,
        app.states.len(),
        state.turn_count,
        state.points[0],
        state.points[1]
    );
    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("DeckGym"));
    f.render_widget(title, chunks[0]);

    // Opponent's hand (opponent is player 0)
    let opponent_hand = &state.hands[0];
    let opponent_hand_total = opponent_hand.len();

    let opponent_hand_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(18), // Card 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 3
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 4
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 5
            Constraint::Min(0),     // Right padding
        ])
        .split(chunks[1]);

    // Render up to 5 cards from opponent's hand (as hidden cards) with scroll offset
    let opponent_start = app.opponent_hand_scroll;
    let opponent_end = std::cmp::min(opponent_start + 5, opponent_hand_total);
    let opponent_cards_to_show = opponent_end - opponent_start;

    for i in 0..opponent_cards_to_show {
        let card_index = opponent_start + i;

        // Add arrows to indicate more cards
        let left_arrow = if card_index == opponent_start && opponent_start > 0 {
            "←"
        } else {
            " "
        };
        let right_arrow = if card_index == opponent_end - 1 && opponent_end < opponent_hand_total {
            "→"
        } else {
            " "
        };

        let lines = vec![
            Line::from(vec![Span::styled(
                format!("{left_arrow} 🂠 {right_arrow}"),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                "????",
                Style::default().fg(Color::DarkGray),
            )]),
            Line::from(""),
        ];

        let title = format!("#{}", card_index + 1);
        let opponent_card_block = Paragraph::new(lines)
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center)
                    .title(title),
            );

        // Render to positions 1, 3, 5, 7, 9 (skipping spacing)
        let chunk_index = 1 + (i * 2);
        f.render_widget(opponent_card_block, opponent_hand_chunks[chunk_index]);
    }

    // Battle mat area - more compact for space efficiency
    let battle_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(8), // Opponent bench - compact but readable
                Constraint::Length(8), // Opponent active
                Constraint::Length(8), // Player active
                Constraint::Length(8), // Player bench
            ]
            .as_ref(),
        )
        .split(chunks[2]);

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
        let (lines, style, border_color, is_empty) =
            render_pokemon_card(pokemon, &format!("Opp Bench {}", bench_pos + 1), Color::Red);

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title_alignment(Alignment::Center)
            .title(format!("Bench {}", bench_pos + 1));
        if is_empty {
            block = block.border_type(BorderType::Rounded);
        }

        let pokemon_block = Paragraph::new(lines).style(style).block(block);
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
    let (lines, style, border_color, is_empty) =
        render_pokemon_card(opponent_active, "Opponent Active", Color::Red);

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title_alignment(Alignment::Center)
        .title("Active");
    if is_empty {
        block = block.border_type(BorderType::Rounded);
    }

    let opponent_active_block = Paragraph::new(lines).style(style).block(block);
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
    let (lines, style, border_color, is_empty) =
        render_pokemon_card(player_active, "Your Active", Color::Green);

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title_alignment(Alignment::Center)
        .title("Active");
    if is_empty {
        block = block.border_type(BorderType::Rounded);
    }

    let player_active_block = Paragraph::new(lines).style(style).block(block);
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
        let (lines, style, border_color, is_empty) = render_pokemon_card(
            pokemon,
            &format!("Your Bench {}", bench_pos + 1),
            Color::Green,
        );

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title_alignment(Alignment::Center)
            .title(format!("Bench {}", bench_pos + 1));
        if is_empty {
            block = block.border_type(BorderType::Rounded);
        }

        let pokemon_block = Paragraph::new(lines).style(style).block(block);
        f.render_widget(pokemon_block, player_bench_chunks[chunk_idx]);
    }

    // Hand display area
    let player_hand = &state.hands[1]; // Player 1's hand (your hand)
    let player_hand_total = player_hand.len();

    let hand_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Left padding
            Constraint::Length(18), // Card 1
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 2
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 3
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 4
            Constraint::Length(1),  // Spacing
            Constraint::Length(18), // Card 5
            Constraint::Min(0),     // Right padding
        ])
        .split(chunks[3]);

    // Render up to 5 cards from player's hand with scroll offset
    let player_start = app.player_hand_scroll;
    let player_end = std::cmp::min(player_start + 5, player_hand_total);
    let cards_to_show = player_end - player_start;

    for i in 0..cards_to_show {
        let card_index = player_start + i;
        let card = &player_hand[card_index];

        // Add arrows to indicate more cards
        let left_arrow = if card_index == player_start && player_start > 0 {
            "←"
        } else {
            ""
        };
        let right_arrow = if card_index == player_end - 1 && player_end < player_hand_total {
            "→"
        } else {
            ""
        };

        let (mut lines, style) = render_hand_card(card, card_index);

        // Add arrows to the card display
        if !left_arrow.is_empty() || !right_arrow.is_empty() {
            lines.insert(
                0,
                Line::from(vec![
                    Span::styled(format!("{left_arrow} "), Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!(" {right_arrow}"),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
            );
        }

        let hand_card_block = Paragraph::new(lines).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center)
                .title("Hand"),
        );

        // Render to positions 1, 3, 5, 7, 9 (skipping spacing)
        let chunk_index = 1 + (i * 2);
        f.render_widget(hand_card_block, hand_chunks[chunk_index]);
    }

    // Footer with possible actions
    let (actor, actions) = generate_possible_actions(state);
    let action_strings: Vec<String> = actions
        .iter()
        .take(10) // Limit to first 10 actions
        .map(|a| format!("{:?}", a.action))
        .collect();

    let actions_text = if action_strings.is_empty() {
        "No actions available".to_string()
    } else {
        action_strings.join(" | ")
    };

    let footer_text = format!(
        "Controls: ESC/q=quit, Up/Down=navigate states, Space=next, PgUp/PgDn=scroll log, a/d=scroll player hand, A/D=scroll opp hand\nCurrent Player: P{} | Possible Actions: {}",
        actor + 1,
        actions_text
    );

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Actions"));
    f.render_widget(footer, chunks[4]);

    // Left side: Battle log panel with actions
    let mut log_lines = Vec::new();

    // Add initial turn header
    if !app.states.is_empty() {
        let initial_turn = app.states[0].turn_count;
        let header = if initial_turn == 0 {
            "━━━ Setup Phase ━━━".to_string()
        } else {
            format!("━━━ Turn {initial_turn} ━━━")
        };
        log_lines.push(Line::from(vec![Span::styled(
            header,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));
    }

    for (i, action) in app.actions.iter().enumerate() {
        let player_num = action.actor;
        let player_color = if player_num == 0 {
            Color::Red
        } else {
            Color::Green
        };

        // Add cursor indicator before this action if we're between state i and i+1
        if i == app.current_state_index && i < app.actions.len() {
            log_lines.push(Line::from(vec![Span::styled(
                ">>> CURRENT <<<",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
        }

        // Add the action line
        log_lines.push(Line::from(vec![
            Span::styled(
                format!("P{}: ", player_num + 1),
                Style::default()
                    .fg(player_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}", action.action),
                Style::default().fg(Color::White),
            ),
        ]));

        // Check if turn changed after this action
        if i + 1 < app.states.len() {
            let current_turn = app.states[i].turn_count;
            let next_turn = app.states[i + 1].turn_count;

            if next_turn != current_turn {
                log_lines.push(Line::from(""));
                let header = if next_turn == 0 {
                    "━━━ Setup Phase ━━━".to_string()
                } else {
                    format!("━━━ Turn {next_turn} ━━━")
                };
                log_lines.push(Line::from(vec![Span::styled(
                    header,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
            }
        }
    }

    // If we're at the initial state and there are no actions yet
    if app.current_state_index == 0 && app.actions.is_empty() {
        log_lines.push(Line::from(vec![Span::styled(
            ">>> CURRENT <<<",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]));
        log_lines.push(Line::from("Game Start"));
    }

    let battle_log = Paragraph::new(log_lines)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Battle Log"))
        .scroll((app.scroll_offset, 0));
    f.render_widget(battle_log, main_chunks[0]);
}

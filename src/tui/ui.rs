use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::{App, AppMode};
use super::render::{render_discarded_energy_line, render_hand_card, render_pokemon_card};

pub fn ui(f: &mut Frame, app: &App) {
    let state = app.get_state();
    let is_interactive = matches!(&app.mode, super::app::AppMode::Interactive { .. });

    // Main layout: left (battle log), center (game)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25), // Battle log area
            Constraint::Percentage(75), // Game area
        ])
        .split(f.area());

    // Center: game area with battle mat, hand areas, and footer (no separate header)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Opponent's hand (reduced height)
                Constraint::Min(0),    // Battle mat
                Constraint::Length(5), // Player's hand
                Constraint::Length(6), // Footer (includes header info, reduced size)
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

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
        .split(chunks[0]);

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

        let lines = vec![Line::from(vec![Span::styled(
            format!("{left_arrow} ? {right_arrow}"),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )])];

        let title = format!("#{}", card_index + 1);
        let opponent_card_block = Paragraph::new(lines)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
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
        .split(chunks[1]);

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
        .split(chunks[2]);

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
                    Span::styled(
                        format!("{left_arrow} "),
                        Style::default().fg(Color::LightYellow),
                    ),
                    Span::styled(
                        format!(" {right_arrow}"),
                        Style::default().fg(Color::LightYellow),
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

    // Footer with game status and possible actions
    let actor = app.get_current_actor();
    let actions = app.get_possible_actions();

    // Build discarded energy display
    let p1_discard_line = render_discarded_energy_line(&state.discard_energies[0]);
    let p2_discard_line = render_discarded_energy_line(&state.discard_energies[1]);

    // Build header line with game status
    let header_line = if is_interactive {
        format!(
            "DeckGym [INTERACTIVE] | Turn: {} | P1: {} pts | P2: {} pts",
            state.turn_count, state.points[0], state.points[1]
        )
    } else {
        format!(
            "DeckGym [REPLAY] State: {}/{} | Turn: {} | P1: {} pts | P2: {} pts",
            app.get_current_state_index() + 1,
            app.get_states_len(),
            state.turn_count,
            state.points[0],
            state.points[1]
        )
    };

    let footer_lines = if is_interactive {
        // Interactive mode footer
        let current_actor = app.get_current_actor();
        let is_human_turn = current_actor == 1;

        let mut lines = vec![
            Line::from(vec![Span::styled(
                "P1 Discard: ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )])
            .spans
            .into_iter()
            .chain(p1_discard_line.spans.into_iter())
            .collect::<Vec<_>>()
            .into(),
            Line::from(vec![Span::styled(
                "P2 Discard: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )])
            .spans
            .into_iter()
            .chain(p2_discard_line.spans.into_iter())
            .collect::<Vec<_>>()
            .into(),
        ];

        if is_human_turn {
            // Show numbered actions for human player
            let action_strings: Vec<String> = actions
                .iter()
                .take(9) // Limit to first 9 actions (1-9 keys)
                .enumerate()
                .map(|(i, a)| format!("{}. {:?}", i + 1, a.action))
                .collect();

            let actions_text = if action_strings.is_empty() {
                "No actions available".to_string()
            } else {
                action_strings.join(" | ")
            };

            lines.push(Line::from("Controls: ESC/q=quit, 1-9=select action, W/S=scroll log, Left/Right=scroll player hand, A/D=scroll opp hand"));
            lines.push(Line::from(vec![Span::styled(
                "YOUR TURN - Select Action:",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(actions_text));
        } else {
            // AI turn - show waiting message
            lines.push(Line::from("Controls: ESC/q=quit, W/S=scroll log, Left/Right=scroll player hand, A/D=scroll opp hand"));
            lines.push(Line::from(vec![Span::styled(
                "AI TURN - Waiting for opponent...",
                Style::default().fg(Color::Yellow),
            )]));
        }
        lines
    } else {
        // Replay mode footer
        let action_strings: Vec<String> = actions
            .iter()
            .take(10)
            .map(|a| format!("{:?}", a.action))
            .collect();

        let actions_text = if action_strings.is_empty() {
            "No actions available".to_string()
        } else {
            action_strings.join(" | ")
        };

        vec![
            Line::from(vec![
                Span::styled("P1 Discard: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ]).spans.into_iter().chain(p1_discard_line.spans.into_iter()).collect::<Vec<_>>().into(),
            Line::from(vec![
                Span::styled("P2 Discard: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]).spans.into_iter().chain(p2_discard_line.spans.into_iter()).collect::<Vec<_>>().into(),
            Line::from("Controls: ESC/q=quit, Up/Down=navigate states, W/S=scroll log, Left/Right=scroll player hand, A/D=scroll opp hand, C=toggle center"),
            Line::from(format!("Current Player: P{}", actor + 1)),
            Line::from(format!("Possible Actions: {}", actions_text)),
        ]
    };

    let footer = Paragraph::new(footer_lines)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title(header_line));
    f.render_widget(footer, chunks[3]);

    // Left side: Battle log panel with actions
    let mut log_lines = Vec::new();
    let actions = app.get_actions();

    // Track where the "CURRENT" marker is placed in the log_lines vector
    let mut current_marker_line: Option<usize> = None;

    if is_interactive {
        // Interactive mode - live battle log
        let turn_history = app.get_turn_history().unwrap_or_default();
        let mut current_turn: u8 = 0;

        // Initial header
        log_lines.push(Line::from(vec![Span::styled(
            "━━━ Setup Phase ━━━",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        for (i, action) in actions.iter().enumerate() {
            let player_num = action.actor;
            let player_color = if player_num == 0 {
                Color::Red
            } else {
                Color::Green
            };

            // Check if turn changed before this action
            if i < turn_history.len() {
                let action_turn = turn_history[i];
                if action_turn != current_turn {
                    current_turn = action_turn;
                    log_lines.push(Line::from(""));
                    let header = if current_turn == 0 {
                        "━━━ Setup Phase ━━━".to_string()
                    } else {
                        format!("━━━ Turn {} ━━━", current_turn)
                    };
                    log_lines.push(Line::from(vec![Span::styled(
                        header,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )]));
                }
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
        }

        // Show current turn header at the end if it's different
        if state.turn_count != current_turn {
            log_lines.push(Line::from(""));
            let header = if state.turn_count == 0 {
                "━━━ Setup Phase ━━━".to_string()
            } else {
                format!("━━━ Turn {} ━━━", state.turn_count)
            };
            log_lines.push(Line::from(vec![Span::styled(
                header,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
        }
    } else {
        // Replay mode - show full history with turn headers
        if let AppMode::Replay {
            states,
            current_index,
            ..
        } = &app.mode
        {
            let mut current_turn: u8 = if !states.is_empty() {
                states[0].turn_count
            } else {
                0
            };

            // Add initial turn header
            let header = if current_turn == 0 {
                "━━━ Setup Phase ━━━".to_string()
            } else {
                format!("━━━ Turn {current_turn} ━━━")
            };
            log_lines.push(Line::from(vec![Span::styled(
                header,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));

            for (i, action) in actions.iter().enumerate() {
                let player_num = action.actor;
                let player_color = if player_num == 0 {
                    Color::Red
                } else {
                    Color::Green
                };

                // Add cursor indicator before this action if we're between state i and i+1
                if i == *current_index && i < actions.len() {
                    current_marker_line = Some(log_lines.len());
                    log_lines.push(Line::from(vec![Span::styled(
                        ">>> CURRENT <<<",
                        Style::default()
                            .fg(Color::LightYellow)
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
                if i + 1 < states.len() {
                    let next_turn = states[i + 1].turn_count;

                    if next_turn != current_turn && i + 1 < actions.len() {
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

                    current_turn = next_turn;
                }
            }

            // If we're at the initial state and there are no actions yet, or at the final state
            if current_marker_line.is_none() && *current_index == actions.len() {
                current_marker_line = Some(log_lines.len());
                log_lines.push(Line::from(vec![Span::styled(
                    ">>> CURRENT <<<",
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )]));

                if actions.is_empty() {
                    log_lines.push(Line::from("Game Start"));
                }
            }
        }
    }

    // Center the battle log view around the current marker when in
    // Replay mode and the lock_actions_center flag enabled by the user,
    let mut battle_log_scroll = app.scroll_offset;

    if let AppMode::Replay { .. } = &app.mode {
        if app.lock_actions_center {
            if let Some(marker_idx) = current_marker_line {
                // Visible lines inside the block - account for borders (2 lines)
                let area_height = main_chunks[0].height as usize;
                let visible = area_height.saturating_sub(2).max(1);
                let total_lines = log_lines.len();

                // Desired top line so marker is centered
                let desired_top = if marker_idx > visible / 2 {
                    marker_idx - visible / 2
                } else {
                    0
                };

                let max_top = total_lines.saturating_sub(visible);
                let top = std::cmp::min(desired_top, max_top);
                battle_log_scroll = top as u16;
            }
        }
    }

    let battle_log = Paragraph::new(log_lines)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Battle Log"))
        .scroll((battle_log_scroll, 0));
    f.render_widget(battle_log, main_chunks[0]);
}

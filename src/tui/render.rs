use crate::{models::*, tool_ids::ToolId};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

pub(crate) fn energy_type_to_color(energy_type: EnergyType) -> Color {
    match energy_type {
        EnergyType::Grass => Color::Green,
        EnergyType::Fire => Color::Red,
        EnergyType::Water => Color::Blue,
        EnergyType::Lightning => Color::LightYellow,
        EnergyType::Psychic => Color::Magenta,
        EnergyType::Fighting => Color::Red,
        EnergyType::Darkness => Color::DarkGray,
        EnergyType::Metal => Color::White,
        EnergyType::Dragon => Color::Cyan,
        EnergyType::Colorless => Color::Gray,
    }
}

pub(crate) fn energy_type_to_symbol(energy_type: EnergyType) -> &'static str {
    match energy_type {
        EnergyType::Grass => "●",
        EnergyType::Fire => "●",
        EnergyType::Water => "●",
        EnergyType::Lightning => "●",
        EnergyType::Psychic => "●",
        EnergyType::Fighting => "●",
        EnergyType::Darkness => "●",
        EnergyType::Metal => "●",
        EnergyType::Dragon => "●",
        EnergyType::Colorless => "●",
    }
}

pub(crate) fn tool_id_to_name(tool_id: ToolId) -> &'static str {
    match tool_id {
        ToolId::A2147GiantCape => "Giant Cape",
        ToolId::A2148RockyHelmet => "Rocky Helmet",
        ToolId::A3146PoisonBarb => "Poison Barb",
        ToolId::A3147LeafCape => "Leaf Cape",
        ToolId::A3a065ElectricalCord
        | ToolId::A4b318ElectricalCord
        | ToolId::A4b319ElectricalCord => "Electrical Cord",
        ToolId::A4a067InflatableBoat => "Inflatable Boat",
        ToolId::B1219HeavyHelmet => "Heavy Helmet",
        ToolId::B1218SitrusBerry => "Sitrus Berry",
    }
}

pub(crate) fn render_hand_card<'a>(card: &'a Card, index: usize) -> (Vec<Line<'a>>, Style) {
    let name = card.get_name();
    const MAX_WIDTH: usize = 16; // Max characters per line

    // Split the name into words and wrap them
    let mut lines = vec![Line::from(vec![Span::styled(
        format!("#{}", index + 1),
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD),
    )])];

    let words: Vec<&str> = name.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
        // If adding this word would exceed max width, start a new line
        if !current_line.is_empty() && current_line.len() + 1 + word.len() > MAX_WIDTH {
            lines.push(Line::from(vec![Span::styled(
                current_line.clone(),
                Style::default().fg(Color::LightBlue),
            )]));
            current_line.clear();
        }

        // If the word itself is too long, truncate it
        if word.len() > MAX_WIDTH {
            if !current_line.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    current_line.clone(),
                    Style::default().fg(Color::LightBlue),
                )]));
                current_line.clear();
            }
            lines.push(Line::from(vec![Span::styled(
                format!("{}...", &word[..MAX_WIDTH - 3]),
                Style::default().fg(Color::LightBlue),
            )]));
            continue;
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    // Add the last line if there's content
    if !current_line.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            current_line,
            Style::default().fg(Color::LightBlue),
        )]));
    }

    // Add an empty line at the end for spacing
    lines.push(Line::from(""));

    (lines, Style::default().fg(Color::LightBlue))
}

pub(crate) fn count_discarded_energy_by_type(
    discard_energies: &[EnergyType],
) -> HashMap<EnergyType, usize> {
    let mut counts = HashMap::new();
    for energy in discard_energies {
        *counts.entry(*energy).or_insert(0) += 1;
    }
    counts
}

pub(crate) fn render_discarded_energy_line(discard_energies: &'_ [EnergyType]) -> Line<'_> {
    let energy_counts = count_discarded_energy_by_type(discard_energies);

    if energy_counts.is_empty() {
        return Line::from(vec![Span::styled("None", Style::default().fg(Color::Gray))]);
    }

    let mut spans = Vec::new();
    let energy_types = [
        EnergyType::Grass,
        EnergyType::Fire,
        EnergyType::Water,
        EnergyType::Lightning,
        EnergyType::Psychic,
        EnergyType::Fighting,
        EnergyType::Darkness,
        EnergyType::Metal,
        EnergyType::Dragon,
        EnergyType::Colorless,
    ];

    let mut first = true;
    for energy_type in energy_types {
        if let Some(&count) = energy_counts.get(&energy_type) {
            if !first {
                spans.push(Span::styled(" ", Style::default()));
            }
            first = false;

            spans.push(Span::styled(
                energy_type_to_symbol(energy_type),
                Style::default().fg(energy_type_to_color(energy_type)),
            ));
            spans.push(Span::styled(
                format!("×{}", count),
                Style::default().fg(Color::White),
            ));
        }
    }

    Line::from(spans)
}

pub(crate) fn render_pokemon_card<'a>(
    pokemon: &'a Option<PlayedCard>,
    _title: &str,
    player_color: Color,
) -> (Vec<Line<'a>>, Style, Color, bool) {
    match pokemon {
        Some(played_card) => {
            let name = played_card.card.get_name();
            let hp_text = format!("{}/{}", played_card.remaining_hp, played_card.total_hp);

            let mut status_effects = Vec::new();
            if played_card.poisoned {
                status_effects.push("🟣PSN");
            }
            if played_card.paralyzed {
                status_effects.push("🟡PAR");
            }
            if played_card.asleep {
                status_effects.push("💤SLP");
            }

            let status_line = if !status_effects.is_empty() {
                status_effects.join(" ")
            } else {
                "".to_string()
            };

            // Get attack names and energy type (only if it's a Pokemon card)
            let (attack_names, card_type_color): (Vec<String>, Color) = match &played_card.card {
                Card::Pokemon(pokemon_card) => {
                    let attacks = pokemon_card
                        .attacks
                        .iter()
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
                }
                _ => (vec![], Color::Gray),
            };

            // Create first line with name on left and HP on right
            let name_hp_line = Line::from(vec![
                Span::styled(
                    name,
                    Style::default()
                        .fg(player_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ".to_string(), Style::default()), // spacing
                Span::styled(
                    hp_text,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]);

            let mut lines = vec![name_hp_line];

            // Add tool name if attached
            if let Some(tool_id) = played_card.attached_tool {
                let tool_name = tool_id_to_name(tool_id);
                lines.push(Line::from(vec![Span::styled(
                    format!("🔧{}", tool_name),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                )]));
            } else {
                lines.push(Line::from("")); // Empty line if no tool
            }

            // Add attack names
            for attack_name in attack_names {
                lines.push(Line::from(vec![Span::styled(
                    attack_name,
                    Style::default().fg(Color::White),
                )]));
            }

            // Add status effects if any
            if !status_line.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    status_line,
                    Style::default().fg(Color::Magenta),
                )]));
            }

            // Pad to maintain consistent height (total 5 lines of content)
            while lines.len() < 4 {
                lines.push(Line::from(""));
            }

            // Add energy icons at the bottom with colors
            let energy_spans: Vec<Span> = played_card
                .attached_energy
                .iter()
                .map(|&energy_type| {
                    Span::styled(
                        energy_type_to_symbol(energy_type),
                        Style::default().fg(energy_type_to_color(energy_type)),
                    )
                })
                .collect();
            lines.push(Line::from(energy_spans));

            (
                lines,
                Style::default().fg(player_color),
                card_type_color,
                false,
            )
        }
        None => {
            let lines = vec![
                Line::from(""),
                Line::from(""),
                Line::from(""),
                Line::from(""),
                Line::from(""),
            ];
            (lines, Style::default().fg(Color::White), Color::White, true)
        }
    }
}

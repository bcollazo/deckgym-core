use clap::Parser;
use deckgym::card_ids::CardId;
use deckgym::card_validation::{get_implementation_status, ImplementationStatus};
use deckgym::database::get_card_by_enum;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Parser)]
#[command(name = "card_status")]
#[command(about = "Check implementation status of all cards in the database")]
struct Args {
    /// Return only the first incomplete card
    #[arg(long)]
    first_incomplete: bool,

    /// Skip complete cards, show only incomplete ones
    #[arg(long)]
    incomplete_only: bool,
}

struct CardStatusInfo {
    id: String,
    name: String,
    status: ImplementationStatus,
}

// Filtering logic: collect all card statuses
fn collect_card_statuses() -> Vec<CardStatusInfo> {
    CardId::iter()
        .map(|card_id| {
            let card = get_card_by_enum(card_id);
            let status = get_implementation_status(card_id);

            CardStatusInfo {
                id: card.get_id(),
                name: card.get_name(),
                status,
            }
        })
        .collect()
}

// Filtering logic: filter based on flags
fn filter_results(results: Vec<CardStatusInfo>, incomplete_only: bool) -> Vec<CardStatusInfo> {
    if incomplete_only {
        results
            .into_iter()
            .filter(|info| !info.status.is_complete())
            .collect()
    } else {
        results
    }
}

// Presentation logic: render results in human-readable format
fn render_results(results: &[CardStatusInfo]) {
    if results.is_empty() {
        println!("No cards found.");
        return;
    }

    // Calculate column widths
    let max_id_len = results.iter().map(|r| r.id.len()).max().unwrap_or(0);
    let max_name_len = results.iter().map(|r| r.name.len()).max().unwrap_or(0);

    // Print header
    println!(
        "{:width_id$}  {:width_name$}  Status",
        "ID",
        "Name",
        width_id = max_id_len,
        width_name = max_name_len
    );
    println!("{}", "-".repeat(max_id_len + max_name_len + 20));

    // Print each card
    for info in results {
        let status_display = match info.status {
            ImplementationStatus::Complete => "✓ Complete",
            _ => info.status.description(),
        };

        println!(
            "{:width_id$}  {:width_name$}  {}",
            info.id,
            info.name,
            status_display,
            width_id = max_id_len,
            width_name = max_name_len
        );
    }

    // Print summary statistics
    print_summary(results);
}

fn print_summary(results: &[CardStatusInfo]) {
    let total = results.len();
    let complete = results.iter().filter(|r| r.status.is_complete()).count();
    let incomplete = total - complete;

    // Count by status type
    let mut status_counts: HashMap<&str, usize> = HashMap::new();
    for info in results {
        if !info.status.is_complete() {
            let desc = info.status.description();
            *status_counts.entry(desc).or_insert(0) += 1;
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("Summary:");
    println!("  Total cards:      {}", total);
    println!(
        "  Complete:         {} ({:.1}%)",
        complete,
        (complete as f64 / total as f64) * 100.0
    );
    println!(
        "  Incomplete:       {} ({:.1}%)",
        incomplete,
        (incomplete as f64 / total as f64) * 100.0
    );

    if !status_counts.is_empty() {
        println!("\n  Breakdown by issue:");
        let mut sorted_counts: Vec<_> = status_counts.iter().collect();
        sorted_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (desc, count) in sorted_counts {
            println!("    {}: {}", desc, count);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Collect all card statuses
    let all_results = collect_card_statuses();

    // Handle first-incomplete flag
    if args.first_incomplete {
        if let Some(first_incomplete) = all_results.iter().find(|r| !r.status.is_complete()) {
            println!(
                "{} - {} ({})",
                first_incomplete.id,
                first_incomplete.name,
                first_incomplete.status.description()
            );
        } else {
            println!("All cards are complete!");
        }
        return Ok(());
    }

    // Filter results based on flags
    let filtered_results = filter_results(all_results, args.incomplete_only);

    // Render results
    render_results(&filtered_results);

    Ok(())
}

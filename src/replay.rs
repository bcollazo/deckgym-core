//! Replay bundles: a compact, self-contained recording of one game.
//!
//! The engine's [`State`] is built for search, not for drawing: it carries both decks in
//! full at every ply, private bookkeeping for card effects, and enough history for
//! "did your opponent use this attack last turn" style rules. Serializing it once per ply
//! would produce tens of megabytes for a game a viewer watches in thirty seconds.
//!
//! A [`ReplayBundle`] is the projection a visualizer actually needs: card *identifiers*
//! rather than card bodies, one shared dictionary of the card data those identifiers point
//! at, and a per-ply snapshot of what is on the mat. It is deliberately plain JSON with no
//! Rust-specific encoding, so the browser visualizer in `visualizer/` (and anything else
//! that wants to read a game) can consume it without linking the engine.
//!
//! Two producers write the same format:
//! - `cargo run --bin replay` — records a game to a file for static hosting.
//! - the `wasm` feature (see `src/wasm_bindings.rs`) — records one in the browser, so the
//!   visualizer can deal a fresh game without a server round trip.

use std::collections::BTreeMap;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    models::{Card, EnergyType},
    players::{create_players, Player, PlayerCode},
    state::{GameOutcome, PlayedCard},
    Deck, Game, State,
};

/// Bumped when the shape below changes incompatibly, so a visualizer can refuse a bundle
/// it cannot read instead of rendering a confusing half-board.
pub const REPLAY_FORMAT_VERSION: u32 = 1;

/// Safety valve. A game that neither player can close would otherwise record plies until
/// the process runs out of memory; the engine's own `Game::play` has no such bound.
const MAX_PLIES: usize = 20_000;

/// One recorded game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayBundle {
    pub version: u32,
    pub game_id: String,
    pub seed: u64,
    pub players: Vec<PlayerInfo>,
    /// `None` when the recording hit [`MAX_PLIES`] before either player won.
    pub outcome: Option<GameOutcome>,
    /// Every card referenced anywhere in this bundle, keyed by card id ("A1 001").
    pub cards: BTreeMap<String, CardInfo>,
    /// One entry per decision point, each holding the board *before* its action was applied.
    pub plies: Vec<Ply>,
    /// The board after the last ply, i.e. the position the game ended in.
    pub final_state: StateView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    /// Human-readable deck label, e.g. the deck file's stem.
    pub name: String,
    /// The player code that drove this seat, e.g. "R" or "E3".
    pub strategy: String,
    pub energy_types: Vec<EnergyType>,
    pub deck: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ply {
    pub ply: u32,
    pub actor: usize,
    /// The `SimpleAction` variant name, e.g. "Attack" — a stable tag for the visualizer to
    /// branch on without parsing `label`.
    pub kind: String,
    /// The action rendered for a human, from `SimpleAction`'s `Display`.
    pub label: String,
    /// True when this action came off the move-generation stack (a sub-decision forced by a
    /// card effect) rather than being a fresh choice by the player.
    pub is_stack: bool,
    /// How many choices the player had here. 1 means the engine auto-played it.
    pub choices: usize,
    /// The board as it stood before this action was applied.
    pub state: StateView,
}

/// A snapshot of everything a viewer can see on the mat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateView {
    pub turn: u8,
    pub current_player: usize,
    pub points: [u8; 2],
    pub stadium: Option<String>,
    pub winner: Option<GameOutcome>,
    pub sides: Vec<SideView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideView {
    pub hand: Vec<String>,
    pub deck_count: usize,
    pub discard: Vec<String>,
    pub discard_energy: Vec<EnergyType>,
    pub energy_current: Option<EnergyType>,
    pub energy_next: Option<EnergyType>,
    /// Index 0 is the Active slot, 1..=3 the Bench.
    pub in_play: Vec<Option<SlotView>>,
}

/// One occupied slot on the mat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotView {
    pub id: String,
    pub damage: u32,
    /// Total HP *after* tools, stadium and ability bonuses, so `damage/max_hp` is the bar to draw.
    pub max_hp: u32,
    pub remaining_hp: u32,
    pub energy: Vec<EnergyType>,
    pub tool: Option<String>,
    /// Active Special Conditions, lowercase: "poisoned", "burned", "asleep", "paralyzed", "confused".
    pub status: Vec<String>,
    /// The cards underneath this one — its pre-evolutions, oldest first.
    pub behind: Vec<String>,
    pub played_this_turn: bool,
    pub ability_used: bool,
}

/// Card data, deduplicated across the bundle. Pokemon-only and trainer-only fields are both
/// optional so the visualizer can read one shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardInfo {
    pub id: String,
    pub name: String,
    /// "pokemon" or "trainer".
    pub kind: String,
    pub rarity: String,
    pub pack: String,

    // Pokemon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_type: Option<EnergyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolves_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weakness: Option<EnergyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retreat_cost: Option<Vec<EnergyType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<AbilityInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attacks: Vec<AttackInfo>,

    // Trainer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityInfo {
    pub title: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackInfo {
    pub title: String,
    pub cost: Vec<EnergyType>,
    pub damage: u32,
    pub effect: Option<String>,
}

impl From<&Card> for CardInfo {
    fn from(card: &Card) -> Self {
        match card {
            Card::Pokemon(p) => CardInfo {
                id: p.id.clone(),
                name: p.name.clone(),
                kind: "pokemon".to_string(),
                rarity: p.rarity.clone(),
                pack: p.booster_pack.clone(),
                hp: Some(p.hp),
                energy_type: Some(p.energy_type),
                stage: Some(p.stage),
                evolves_from: p.evolves_from.clone(),
                weakness: p.weakness,
                retreat_cost: Some(p.retreat_cost.clone()),
                ability: p.ability.as_ref().map(|a| AbilityInfo {
                    title: a.title.clone(),
                    effect: a.effect.clone(),
                }),
                attacks: p
                    .attacks
                    .iter()
                    .map(|a| AttackInfo {
                        title: a.title.clone(),
                        cost: a.energy_required.clone(),
                        damage: a.fixed_damage,
                        effect: a.effect.clone(),
                    })
                    .collect(),
                trainer_type: None,
                effect: None,
            },
            Card::Trainer(t) => CardInfo {
                id: t.id.clone(),
                name: t.name.clone(),
                kind: "trainer".to_string(),
                rarity: t.rarity.clone(),
                pack: t.booster_pack.clone(),
                hp: None,
                energy_type: None,
                stage: None,
                evolves_from: None,
                weakness: None,
                retreat_cost: None,
                ability: None,
                attacks: vec![],
                trainer_type: Some(format!("{:?}", t.trainer_type)),
                effect: Some(t.effect.clone()),
            },
        }
    }
}

/// Collects card bodies as they are seen, so the bundle's dictionary holds exactly the cards
/// the game touched rather than all ~3,800 in the database.
#[derive(Default)]
struct CardRegistry {
    cards: BTreeMap<String, CardInfo>,
}

impl CardRegistry {
    fn record(&mut self, card: &Card) -> String {
        let id = card.get_id();
        self.cards
            .entry(id.clone())
            .or_insert_with(|| CardInfo::from(card));
        id
    }

    fn record_all(&mut self, cards: &[Card]) -> Vec<String> {
        cards.iter().map(|c| self.record(c)).collect()
    }
}

fn slot_view(registry: &mut CardRegistry, played: &PlayedCard) -> SlotView {
    let mut status = Vec::new();
    if played.is_poisoned() {
        status.push("poisoned".to_string());
    }
    if played.is_burned() {
        status.push("burned".to_string());
    }
    if played.is_asleep() {
        status.push("asleep".to_string());
    }
    if played.is_paralyzed() {
        status.push("paralyzed".to_string());
    }
    if played.is_confused() {
        status.push("confused".to_string());
    }

    SlotView {
        id: registry.record(&played.card),
        damage: played.get_damage_counters(),
        max_hp: played.get_effective_total_hp(),
        remaining_hp: played.get_remaining_hp(),
        energy: played.attached_energy.clone(),
        tool: played.attached_tool.as_ref().map(|t| registry.record(t)),
        status,
        behind: registry.record_all(&played.cards_behind),
        played_this_turn: played.played_this_turn,
        ability_used: played.ability_used,
    }
}

fn state_view(registry: &mut CardRegistry, state: &State) -> StateView {
    let sides = (0..2)
        .map(|player| SideView {
            hand: registry.record_all(&state.hands[player]),
            deck_count: state.decks[player].cards.len(),
            discard: registry.record_all(&state.discard_piles[player]),
            discard_energy: state.discard_energies[player].clone(),
            energy_current: state.energy_zone[player].current,
            energy_next: state.energy_zone[player].next,
            in_play: state.in_play_pokemon[player]
                .iter()
                .map(|slot| slot.as_ref().map(|played| slot_view(registry, played)))
                .collect(),
        })
        .collect();

    StateView {
        turn: state.turn_count,
        current_player: state.current_player,
        points: state.points,
        stadium: state
            .active_stadium
            .as_ref()
            .map(|card| registry.record(card)),
        winner: state.winner,
        sides,
    }
}

/// The `SimpleAction` variant name, taken from its serde encoding so this cannot drift out of
/// sync with the enum: serde writes a unit variant as a bare string and every other variant as
/// a single-key object whose key is the variant name.
fn action_kind(action: &Action) -> String {
    match serde_json::to_value(&action.action) {
        Ok(serde_json::Value::String(name)) => name,
        Ok(serde_json::Value::Object(map)) => map
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string()),
        _ => "Unknown".to_string(),
    }
}

/// Records one game between two decks and returns it as a bundle.
///
/// `seed` of `None` picks a random one, which is then reported in the bundle so the exact game
/// can be replayed later.
pub fn record_game(
    deck_a: Deck,
    deck_b: Deck,
    names: [String; 2],
    player_codes: Vec<PlayerCode>,
    seed: Option<u64>,
) -> ReplayBundle {
    let seed = seed.unwrap_or_else(|| StdRng::from_entropy().gen::<u64>());

    let strategies = [
        format!("{:?}", player_codes[0]),
        format!("{:?}", player_codes[1]),
    ];
    let energy_types = [deck_a.energy_types.clone(), deck_b.energy_types.clone()];
    let deck_lists = [deck_a.cards.clone(), deck_b.cards.clone()];

    let players: Vec<Box<dyn Player>> = create_players(deck_a, deck_b, player_codes);
    let mut game = Game::new(players, seed);

    let mut registry = CardRegistry::default();
    let mut plies: Vec<Ply> = Vec::new();

    let mut state = game.get_state_clone();
    while !game.is_game_over() && plies.len() < MAX_PLIES {
        // Captured before `play_tick` so the ply carries the board the actor was looking at,
        // together with how wide their choice actually was.
        let before = state_view(&mut registry, &state);
        let choices = state.generate_possible_actions().1.len();

        let action = game.play_tick();

        plies.push(Ply {
            ply: plies.len() as u32,
            actor: action.actor,
            kind: action_kind(&action),
            label: action.action.to_string(),
            is_stack: action.is_stack,
            choices,
            state: before,
        });

        state = game.get_state_clone();
    }

    let final_state = state_view(&mut registry, &state);

    let players = (0..2)
        .map(|i| PlayerInfo {
            name: names[i].clone(),
            strategy: strategies[i].clone(),
            energy_types: energy_types[i].clone(),
            deck: registry.record_all(&deck_lists[i]),
        })
        .collect();

    ReplayBundle {
        version: REPLAY_FORMAT_VERSION,
        game_id: uuid::Uuid::new_v4().to_string(),
        seed,
        players,
        outcome: state.winner,
        cards: registry.cards,
        plies,
        final_state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::load_test_decks;

    fn record_test_game(seed: u64) -> ReplayBundle {
        let (deck_a, deck_b) = load_test_decks();
        record_game(
            deck_a,
            deck_b,
            ["A".to_string(), "B".to_string()],
            vec![PlayerCode::R, PlayerCode::R],
            Some(seed),
        )
    }

    #[test]
    fn records_a_finished_game_with_plies_and_an_outcome() {
        let bundle = record_test_game(42);

        assert_eq!(bundle.version, REPLAY_FORMAT_VERSION);
        assert_eq!(bundle.seed, 42);
        assert!(!bundle.plies.is_empty());
        assert!(bundle.outcome.is_some(), "a seeded game should finish");
        assert_eq!(bundle.final_state.winner, bundle.outcome);
        assert_eq!(bundle.plies.len(), MAX_PLIES.min(bundle.plies.len()));
    }

    #[test]
    fn replays_identically_for_the_same_seed() {
        let first = record_test_game(7);
        let second = record_test_game(7);

        let labels = |b: &ReplayBundle| b.plies.iter().map(|p| p.label.clone()).collect::<Vec<_>>();
        assert_eq!(labels(&first), labels(&second));
        assert_eq!(first.outcome, second.outcome);
    }

    #[test]
    fn every_referenced_card_id_resolves_in_the_dictionary() {
        let bundle = record_test_game(11);

        let mut referenced: Vec<String> = Vec::new();
        for ply in &bundle.plies {
            for side in &ply.state.sides {
                referenced.extend(side.hand.iter().cloned());
                referenced.extend(side.discard.iter().cloned());
                for slot in side.in_play.iter().flatten() {
                    referenced.push(slot.id.clone());
                    referenced.extend(slot.behind.iter().cloned());
                    referenced.extend(slot.tool.iter().cloned());
                }
            }
        }
        for player in &bundle.players {
            referenced.extend(player.deck.iter().cloned());
        }

        assert!(!referenced.is_empty());
        for id in referenced {
            assert!(
                bundle.cards.contains_key(&id),
                "card {id} is referenced but missing from the bundle dictionary"
            );
        }
    }

    #[test]
    fn action_kind_is_the_variant_name() {
        let bundle = record_test_game(3);

        // Every game ends turns, and `EndTurn` is a unit variant (serialized as a bare
        // string) while the rest are objects — so this covers both serde encodings.
        assert!(bundle.plies.iter().any(|p| p.kind == "EndTurn"));
        assert!(bundle.plies.iter().any(|p| p.kind != "EndTurn"));
        assert!(bundle.plies.iter().all(|p| !p.kind.is_empty()));
    }

    #[test]
    fn serializes_to_json_that_round_trips() {
        let bundle = record_test_game(5);
        let json = serde_json::to_string(&bundle).expect("bundle should serialize");
        let parsed: ReplayBundle = serde_json::from_str(&json).expect("bundle should round-trip");

        assert_eq!(parsed.plies.len(), bundle.plies.len());
        assert_eq!(parsed.cards.len(), bundle.cards.len());
    }
}

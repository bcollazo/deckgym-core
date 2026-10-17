//! A stable, serializable, display-oriented projection of `State`.
//!
//! `ViewState` decouples the replay viewer and the external-bot protocol from engine internals
//! (many `State`/`PlayedCard` fields are `pub(crate)` and can change shape freely). Cards are
//! referenced by id (`Card::get_id()`); a replay file carries a `cards` table mapping those ids to
//! full `Card` JSON so consumers never need the engine's card database.

use serde::{Deserialize, Serialize};

use crate::{
    models::{Card, EnergyType, StatusCondition},
    state::{EnergyZone, GameOutcome, PlayedCard, State},
};

/// One Pokemon slot in play: the active spot (index 0) or a bench spot (1..=3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotView {
    /// Card id of the Pokemon currently occupying the slot (its current evolution stage).
    pub card: String,
    pub hp: u32,
    pub max_hp: u32,
    pub energy: Vec<EnergyType>,
    /// Card ids of attached Pokemon Tools.
    pub tools: Vec<String>,
    pub status: Vec<StatusCondition>,
    pub played_this_turn: bool,
}

impl SlotView {
    fn from_played_card(card: &PlayedCard) -> Self {
        let mut status = Vec::new();
        if card.is_poisoned() {
            status.push(StatusCondition::Poisoned);
        }
        if card.is_paralyzed() {
            status.push(StatusCondition::Paralyzed);
        }
        if card.is_asleep() {
            status.push(StatusCondition::Asleep);
        }
        if card.is_burned() {
            status.push(StatusCondition::Burned);
        }
        if card.is_confused() {
            status.push(StatusCondition::Confused);
        }
        SlotView {
            card: card.get_id(),
            hp: card.get_remaining_hp(),
            max_hp: card.get_effective_total_hp(),
            energy: card.attached_energy.clone(),
            tools: card.attached_tools.iter().map(Card::get_id).collect(),
            status,
            played_this_turn: card.played_this_turn,
        }
    }
}

/// A player's board and zones, as displayed by the viewer / seen by a bot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerView {
    /// `None` when redacted: `ViewState::for_player` hides the opponent's hand contents (but not
    /// its size) from a bot, matching what a real player would see across the table.
    pub hand: Option<Vec<String>>,
    pub hand_count: usize,
    pub deck_count: usize,
    pub discard: Vec<String>,
    pub discard_energies: Vec<EnergyType>,
    pub energy_zone: EnergyZone,
    /// 0 = active, 1..=3 = bench.
    pub in_play: [Option<SlotView>; 4],
}

impl PlayerView {
    fn from_state(state: &State, player: usize, redact_hand: bool) -> Self {
        let hand_ids: Vec<String> = state.hands[player].iter().map(Card::get_id).collect();
        let in_play = std::array::from_fn(|i| {
            state.in_play_pokemon[player][i]
                .as_ref()
                .map(SlotView::from_played_card)
        });
        PlayerView {
            hand_count: hand_ids.len(),
            hand: if redact_hand { None } else { Some(hand_ids) },
            deck_count: state.decks[player].cards.len(),
            discard: state.discard_piles[player]
                .iter()
                .map(Card::get_id)
                .collect(),
            discard_energies: state.discard_energies[player].clone(),
            energy_zone: state.energy_zone[player],
            in_play,
        }
    }
}

/// A full snapshot of a game at one decision point. `omniscient` sees both hands (used by the
/// replay recorder); `for_player` redacts the opponent's hand (used for a bot's own view).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewState {
    pub turn: u8,
    pub current_player: usize,
    pub points: [u8; 2],
    pub winner: Option<GameOutcome>,
    pub players: [PlayerView; 2],
    /// Card id of the active stadium, if any.
    pub stadium: Option<String>,
}

impl ViewState {
    /// A full, un-redacted snapshot: both hands visible. Used for replays, which are meant to be
    /// inspected after the fact with full information.
    pub fn omniscient(state: &State) -> Self {
        Self::build(state, None)
    }

    /// The snapshot as `player` sees it: their own hand is visible, the opponent's is redacted to
    /// just a count. Used for the state sent to an external bot.
    pub fn for_player(state: &State, player: usize) -> Self {
        Self::build(state, Some(player))
    }

    fn build(state: &State, viewer: Option<usize>) -> Self {
        let players = std::array::from_fn(|i| {
            let redact = matches!(viewer, Some(p) if p != i);
            PlayerView::from_state(state, i, redact)
        });
        ViewState {
            turn: state.turn_count,
            current_player: state.current_player,
            points: state.points,
            winner: state.winner,
            players,
            stadium: state.active_stadium.as_ref().map(Card::get_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::get_test_game_with_board;

    #[test]
    fn omniscient_shows_both_hands() {
        let game = get_test_game_with_board(vec![], vec![]);
        let state = game.get_state_clone();
        let view = ViewState::omniscient(&state);
        assert!(view.players[0].hand.is_some());
        assert!(view.players[1].hand.is_some());
    }

    #[test]
    fn for_player_redacts_only_opponent_hand() {
        let game = get_test_game_with_board(vec![], vec![]);
        let state = game.get_state_clone();

        let view0 = ViewState::for_player(&state, 0);
        assert!(view0.players[0].hand.is_some());
        assert!(view0.players[1].hand.is_none());
        // Count is still visible even when redacted.
        assert_eq!(view0.players[1].hand_count, state.hands[1].len());

        let view1 = ViewState::for_player(&state, 1);
        assert!(view1.players[0].hand.is_none());
        assert!(view1.players[1].hand.is_some());
    }
}

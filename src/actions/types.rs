use crate::{
    models::{Card, EnergyType, TrainerCard},
    tool_ids::ToolId,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Main structure for following Game Tree design. Using "nesting" with a
/// SimpleAction to share common fields here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub actor: usize,
    pub action: SimpleAction,
    pub is_stack: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleAction {
    DrawCard {
        amount: u8,
    },
    Play {
        trainer_card: TrainerCard,
    },

    // Card because of the fossil Trainer Cards...
    // usize is bench 1-based index, with 0 meaning Active pokemon, 1..4 meaning Bench
    Place(Card, usize),
    Evolve(Card, usize),
    UseAbility {
        in_play_idx: usize,
    },

    // Its given it is with the active pokemon, to the other active.
    // usize is the index of the attack in the pokemon's attacks
    Attack(usize),
    // usize is in_play_pokemon index to retreat to. Can't Retreat(0)
    Retreat(usize),
    EndTurn,

    // Atomic actions as part of different effects.
    Attach {
        attachments: Vec<(u32, EnergyType, usize)>, // (amount, energy_type, in_play_idx)
        is_turn_energy: bool, // true if this is the energy from the zone that can be once per turn
    },
    MoveEnergy {
        from_in_play_idx: usize,
        to_in_play_idx: usize,
        energy: EnergyType,
    },
    AttachTool {
        in_play_idx: usize,
        tool_id: ToolId,
    },
    Heal {
        in_play_idx: usize,
        amount: u32,
        cure_status: bool,
    },
    ApplyDamage {
        target_player: usize,
        targets: Vec<(u32, usize)>, // Vec of (damage, in_play_idx)
        is_from_active_attack: bool,
    },
    /// Switch the in_play_idx pokemon with the active pokemon.
    Activate {
        in_play_idx: usize,
    },
    // Custom Mechanics:
    /// Pokemon Communication: swap a specific Pokemon from hand with a random Pokemon from deck
    CommunicatePokemon {
        hand_pokemon: Card,
    },
    /// Silver: shuffle a specific Supporter from opponent's hand into their deck
    ShuffleOpponentSupporter {
        supporter_card: Card,
    },
    /// Lusamine: attach energies from discard to a Pokemon
    AttachFromDiscard {
        in_play_idx: usize,
        num_random_energies: usize,
    },
    Noop, // No operation, used to have the user say "no" to a question
}

impl fmt::Display for SimpleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleAction::DrawCard { amount } => write!(f, "DrawCard({amount})"),
            SimpleAction::Play { trainer_card } => write!(f, "Play({trainer_card:?})"),
            SimpleAction::Place(card, index) => write!(f, "Place({card}, {index})"),
            SimpleAction::Evolve(card, index) => write!(f, "Evolve({card}, {index})"),
            SimpleAction::UseAbility { in_play_idx } => write!(f, "UseAbility({in_play_idx})"),
            SimpleAction::Attack(index) => write!(f, "Attack({index})"),
            SimpleAction::Retreat(index) => write!(f, "Retreat({index})"),
            SimpleAction::EndTurn => write!(f, "EndTurn"),
            SimpleAction::Attach {
                attachments,
                is_turn_energy,
            } => {
                let attachments_str = attachments
                    .iter()
                    .map(|(amount, energy_type, in_play_idx)| {
                        format!("({amount}, {energy_type:?}, {in_play_idx})")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Attach({attachments_str:?}, {is_turn_energy})")
            }
            SimpleAction::MoveEnergy {
                from_in_play_idx,
                to_in_play_idx,
                energy,
            } => {
                write!(
                    f,
                    "MoveEnergy(from:{from_in_play_idx}, to:{to_in_play_idx}, {energy:?})"
                )
            }
            SimpleAction::AttachTool {
                in_play_idx,
                tool_id,
            } => {
                write!(f, "AttachTool({in_play_idx}, {tool_id:?})")
            }
            SimpleAction::Heal {
                in_play_idx,
                amount,
                cure_status,
            } => write!(f, "Heal({in_play_idx}, {amount}, cure:{cure_status})"),
            SimpleAction::ApplyDamage {
                target_player,
                targets,
                is_from_active_attack,
            } => {
                let targets_str = targets
                    .iter()
                    .map(|(damage, in_play_idx)| format!("({damage}, {in_play_idx})"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "ApplyDamage(player:{target_player}, targets:[{targets_str}], from_active:{is_from_active_attack})"
                )
            }
            SimpleAction::Activate { in_play_idx } => write!(f, "Activate({in_play_idx})"),
            SimpleAction::CommunicatePokemon { hand_pokemon } => {
                write!(f, "CommunicatePokemon({hand_pokemon})")
            }
            SimpleAction::ShuffleOpponentSupporter { supporter_card } => {
                write!(f, "ShuffleOpponentSupporter({supporter_card})")
            }
            SimpleAction::AttachFromDiscard {
                in_play_idx,
                num_random_energies,
            } => {
                write!(f, "AttachFromDiscard({in_play_idx}, {num_random_energies})")
            }
            SimpleAction::Noop => write!(f, "Noop"),
        }
    }
}

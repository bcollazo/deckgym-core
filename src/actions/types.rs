use crate::models::{Card, EnergyType, TrainerCard};
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
    Evolve {
        evolution: Card,
        in_play_idx: usize,
        from_deck: bool,
    },
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
        energy_type: EnergyType,
        amount: u32,
    },
    AttachTool {
        in_play_idx: usize,
        tool_card: Card,
    },
    Heal {
        in_play_idx: usize,
        amount: u32,
        cure_status: bool,
    },
    HealAndDiscardEnergy {
        in_play_idx: usize,
        heal_amount: u32,
        discard_energies: Vec<EnergyType>,
    },
    MoveAllDamage {
        from: usize,
        to: usize,
    },
    ApplyDamage {
        attacking_ref: (usize, usize), // (attacking_player, attacking_pokemon_idx)
        targets: Vec<(u32, usize, usize)>, // Vec of (damage, target_player, in_play_idx)
        is_from_active_attack: bool,
    },
    /// Switch the in_play_idx pokemon with the active pokemon.
    Activate {
        player: usize,
        in_play_idx: usize,
    },
    // Custom Mechanics:
    /// Pokemon Communication: swap a specific Pokemon from hand with a random Pokemon from deck
    CommunicatePokemon {
        hand_pokemon: Card,
    },
    /// May: shuffle specific Pokemon from hand into your deck (no replacement)
    ShufflePokemonIntoDeck {
        hand_pokemon: Vec<Card>,
    },
    /// Silver: shuffle a specific Supporter from opponent's hand into their deck
    ShuffleOpponentSupporter {
        supporter_card: Card,
    },
    /// Mega Absol Ex: discard a specific Supporter from opponent's hand
    DiscardOpponentSupporter {
        supporter_card: Card,
    },
    /// Sableye's Dirty Throw: discard a specific card from own hand
    DiscardOwnCard {
        card: Card,
    },
    /// Discard multiple specific cards from own hand
    DiscardOwnCards {
        cards: Vec<Card>,
    },
    /// Lusamine: attach energies from discard to a Pokemon
    AttachFromDiscard {
        in_play_idx: usize,
        num_random_energies: usize,
    },
    /// Eevee Bag Option 1: Apply damage boost for Eevee evolutions this turn
    ApplyEeveeBagDamageBoost,
    /// Eevee Bag Option 2: Heal all Eevee evolutions
    HealAllEeveeEvolutions,
    /// Discard a Fossil from play (Fossils can be discarded at any time during your turn)
    DiscardFossil {
        in_play_idx: usize,
    },
    /// Return a Pokemon in play to your hand (e.g., Ilima).
    ReturnPokemonToHand {
        in_play_idx: usize,
    },
    Noop, // No operation, used to have the user say "no" to a question
}

impl fmt::Display for SimpleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleAction::DrawCard { amount } => write!(f, "DrawCard({amount})"),
            SimpleAction::Play { trainer_card } => write!(f, "Play({trainer_card:?})"),
            SimpleAction::Place(card, index) => write!(f, "Place({card}, {index})"),
            SimpleAction::Evolve {
                evolution,
                in_play_idx,
                from_deck,
            } => {
                write!(
                    f,
                    "Evolve({evolution}, {in_play_idx}, from_deck: {from_deck})"
                )
            }
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
                energy_type,
                amount,
            } => {
                write!(
                    f,
                    "MoveEnergy(from:{from_in_play_idx}, to:{to_in_play_idx}, {amount}x {energy_type:?})"
                )
            }
            SimpleAction::AttachTool {
                in_play_idx,
                tool_card,
            } => {
                write!(f, "AttachTool({in_play_idx}, {})", tool_card.get_name())
            }
            SimpleAction::Heal {
                in_play_idx,
                amount,
                cure_status,
            } => write!(f, "Heal({in_play_idx}, {amount}, cure:{cure_status})"),
            SimpleAction::HealAndDiscardEnergy {
                in_play_idx,
                heal_amount,
                discard_energies,
            } => write!(
                f,
                "HealAndDiscardEnergy({in_play_idx}, {heal_amount}, {discard_energies:?})"
            ),
            SimpleAction::MoveAllDamage { from, to } => {
                write!(f, "MoveAllDamage(from:{from}, to:{to})")
            }
            SimpleAction::ApplyDamage {
                attacking_ref,
                targets,
                is_from_active_attack,
            } => {
                let targets_str = targets
                    .iter()
                    .map(|(damage, target_player, in_play_idx)| {
                        format!("({damage}, {target_player}, {in_play_idx})")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "ApplyDamage(attacker:{:?}, targets:[{}], from_active:{})",
                    attacking_ref, targets_str, is_from_active_attack
                )
            }
            SimpleAction::Activate {
                player,
                in_play_idx,
            } => write!(f, "Activate({player}, {in_play_idx})"),
            SimpleAction::CommunicatePokemon { hand_pokemon } => {
                write!(f, "CommunicatePokemon({hand_pokemon})")
            }
            SimpleAction::ShufflePokemonIntoDeck { hand_pokemon } => {
                write!(f, "ShufflePokemonIntoDeck({:?})", hand_pokemon)
            }
            SimpleAction::ShuffleOpponentSupporter { supporter_card } => {
                write!(f, "ShuffleOpponentSupporter({supporter_card})")
            }
            SimpleAction::DiscardOpponentSupporter { supporter_card } => {
                write!(f, "DiscardOpponentSupporter({supporter_card})")
            }
            SimpleAction::DiscardOwnCard { card } => {
                write!(f, "DiscardOwnCard({card})")
            }
            SimpleAction::DiscardOwnCards { cards } => {
                write!(f, "DiscardOwnCards({:?})", cards)
            }
            SimpleAction::AttachFromDiscard {
                in_play_idx,
                num_random_energies,
            } => {
                write!(f, "AttachFromDiscard({in_play_idx}, {num_random_energies})")
            }
            SimpleAction::ApplyEeveeBagDamageBoost => {
                write!(f, "ApplyEeveeBagDamageBoost")
            }
            SimpleAction::HealAllEeveeEvolutions => {
                write!(f, "HealAllEeveeEvolutions")
            }
            SimpleAction::DiscardFossil { in_play_idx } => {
                write!(f, "DiscardFossil({in_play_idx})")
            }
            SimpleAction::ReturnPokemonToHand { in_play_idx } => {
                write!(f, "ReturnPokemonToHand({in_play_idx})")
            }
            SimpleAction::Noop => write!(f, "Noop"),
        }
    }
}

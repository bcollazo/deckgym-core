use crate::models::{Attack, Card, EnergyType, StatusCondition, TrainerCard};
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

    // Use the carried Attack definition as the current attack of the active Pokemon.
    // Carrying the whole Attack (instead of an index) lets a single codepath serve the
    // active's own attacks, copied attacks (e.g. Mew ex's Genome Hacking), and attacks
    // granted from previous evolutions (e.g. Celebi's Time Recall).
    Attack(Attack),
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
    ScheduleDelayedSpotDamage {
        target_player: usize,
        target_in_play_idx: usize,
        amount: u32,
    },
    /// Armaldo's Abyssal Drop: schedule an outright Knock Out of whatever occupies the chosen
    /// spot at the end of the target player's next turn.
    ScheduleDelayedSpotKnockOut {
        target_player: usize,
        target_in_play_idx: usize,
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
    /// Maintenance: shuffle specific cards from hand into your deck, then draw a card
    ShuffleOwnCardsIntoDeck {
        cards: Vec<Card>,
    },
    /// Kid's Room: switch a specific card from hand with a random Pokemon Tool card from deck
    SwitchHandCardForRandomTool {
        hand_card: Card,
    },
    /// Silver: shuffle a specific Supporter from opponent's hand into their deck
    ShuffleOpponentSupporter {
        supporter_card: Card,
    },
    /// Mega Absol Ex: discard a specific Supporter from opponent's hand
    DiscardOpponentSupporter {
        supporter_card: Card,
    },
    /// Delcatty's Search for Friends: put a specific card from your own discard pile into your
    /// hand. One action per eligible card, so the player picks which one.
    PutDiscardCardInHand {
        card: Card,
    },
    /// Discard multiple specific cards from own hand
    DiscardOwnCards {
        cards: Vec<Card>,
    },
    /// Slowking's Litter: discard a chosen set of cards from your own hand, then deal `damage` to
    /// one target. Like `DiscardOwnBenchedThenDamage`, the two halves are one action so the
    /// damage — which depends on how many cards were discarded — is applied exactly once and goes
    /// through the damage modifiers a single time.
    DiscardOwnCardsThenDamage {
        cards: Vec<Card>,
        damage: u32,
        target_player: usize,
        target_in_play_idx: usize,
    },
    /// Team Rocket's Boss: put a chosen subset of Basic Pokémon found in the opponent's hand
    /// onto the opponent's Bench
    BenchOpponentPokemonFromHand {
        cards: Vec<Card>,
    },
    /// Lusamine: attach energies from discard to a Pokemon
    AttachFromDiscard {
        in_play_idx: usize,
        num_random_energies: usize,
    },
    /// Volkner: attach a fixed number of a specific energy type from discard to a Pokemon
    AttachTypedFromDiscard {
        in_play_idx: usize,
        energy_type: EnergyType,
        count: usize,
    },
    /// Professor Sada: attach 3 specific different-typed energies from discard to Ancient Pokémon
    SadaAttach {
        assignments: Vec<(EnergyType, usize)>, // (energy_type, in_play_idx) × 3
    },
    /// Eevee Bag Option 1: Apply damage boost for Eevee evolutions this turn
    ApplyEeveeBagDamageBoost,
    /// Eevee Bag Option 2: Heal all Eevee evolutions
    HealAllEeveeEvolutions,
    /// Discard a Fossil from play (Fossils can be discarded at any time during your turn)
    DiscardFossil {
        in_play_idx: usize,
    },
    /// Vespiquen ex's Chase Order: discard 1 of your own Benched Pokémon, then deal the attack's
    /// boosted damage to the opponent's Active Pokémon. The two halves are one action so that the
    /// boosted damage is applied once — splitting it would apply Weakness (and other damage
    /// modifiers) to each half.
    DiscardOwnBenchedThenDamage {
        in_play_idx: usize,
        damage: u32,
    },
    /// Gyarados's Wild Swing: discard any number (possibly zero) of your own Benched Pokémon, then
    /// deal the attack's resulting boosted damage to the opponent's Active Pokémon. Like
    /// `DiscardOwnBenchedThenDamage` this is one action so the boosted damage is applied once.
    DiscardOwnBenchedGroupThenDamage {
        in_play_indices: Vec<usize>,
        damage: u32,
    },
    /// Delcatty's Energy Blender: move a single Energy between two of your own Pokémon, then offer
    /// the same choice again (with `remaining_moves` one lower) plus the option to stop.
    ///
    /// "You may move any amount of Energy from your Pokémon in play to your other Pokémon in any
    /// way you like" is played out one Energy at a time rather than enumerated as whole
    /// redistributions: the number of redistributions is exponential in the Energy on the board
    /// (thousands of actions in an ordinary mid-game position), while one-at-a-time keeps the
    /// branching factor to a few dozen and reaches exactly the same set of final boards.
    /// `remaining_moves` starts at the player's total attached Energy — enough for any
    /// redistribution, since no Energy ever needs to move twice — and guarantees termination.
    MoveEnergyAndReoffer {
        from_in_play_idx: usize,
        to_in_play_idx: usize,
        energy_type: EnergyType,
        remaining_moves: usize,
    },
    /// Eldegoss's Float Up / Accelgor's Deck and Cover: shuffle one of your own Pokémon in play,
    /// and everything attached to it (evolution cards, Tool), back into your deck. Attached Energy
    /// goes to the discarded-Energy pile. Distinct from `ShuffleInPlayPokemonIntoDeck`, which
    /// leaves the Tool and Energy behind.
    ShuffleSelfAndAttachmentsIntoDeck {
        in_play_idx: usize,
    },
    /// Use an activated stadium effect (once per turn per player)
    UseStadium,
    /// Return a Pokemon in play to your hand (e.g., Ilima).
    ReturnPokemonToHand {
        in_play_idx: usize,
    },
    /// Shuffle a Pokemon from play into its owner's deck (e.g., Professor Turo).
    ShuffleInPlayPokemonIntoDeck {
        in_play_idx: usize,
    },
    /// Field Blower: discard the tool attached to a specific Pokémon (any player).
    DiscardToolFromPokemon {
        player: usize,
        in_play_idx: usize,
    },
    /// Field Blower: discard the active stadium.
    DiscardActiveStadium,
    /// Crawdaunt's Unruly Claw: discard a random Energy from the opponent's Active Pokémon
    DiscardRandomOpponentActiveEnergy,
    /// Psychic (Supporter): move a random Energy from one of the opponent's Benched Pokémon to
    /// the opponent's Active Pokémon.
    MoveRandomOpponentEnergyToActive {
        from_in_play_idx: usize,
    },
    /// Apply a chosen Special Condition to the opponent's Active Pokémon (e.g. Dustox's Select Powder).
    ApplyStatusToOpponentActive {
        condition: StatusCondition,
    },
    /// Team Rocket's Raticate ex's Thieving Incisors: move the last-attached Energy from the
    /// opponent's Active Pokémon to `to_in_play_idx` on the actor's own board.
    MoveOpponentActiveEnergyToSelf {
        to_in_play_idx: usize,
    },
    /// Team Rocket's Weezing ex's Boiler Smog: apply ALL of these Special Conditions at once to
    /// the opponent's Active Pokémon (e.g. Poisoned and Burned together).
    ApplyStatusesToOpponentActive {
        conditions: Vec<StatusCondition>,
    },
    /// Whitney: heal a Pokémon and cure only the listed Special Conditions (unlike
    /// `Heal { cure_status: true }`, which cures every condition).
    HealAndCureConditions {
        in_play_idx: usize,
        amount: u32,
        conditions: Vec<StatusCondition>,
    },
    /// Acerola: move `amount` damage from one of the actor's own Pokémon onto the opponent's
    /// Active Pokémon.
    MoveDamageToOpponentActive {
        from_in_play_idx: usize,
        amount: u32,
    },
    /// Pokémon Flute: put a Basic Pokémon from the opponent's discard pile onto their Bench.
    BenchOpponentPokemonFromDiscard {
        card: Card,
    },
    /// Rotom Dex: "Then, you may shuffle your deck." The decline branch is `Noop`.
    ShuffleOwnDeck,
    /// Dark Pendant: the actor reveals a random card from their own hand and shuffles it into
    /// their deck. Queued as the single option on the move-generation stack (like Bouncy Body) so
    /// that it resolves with the shared RNG available; it is not a real choice.
    ShuffleRandomOwnHandCardIntoDeck,
    /// Regice's Reflect Energy / Swanna's Feathery Cyclone: move Energy from the actor's Active
    /// Pokémon to one of their Benched Pokémon. `amount: Some(n)` moves n Energy; `None` moves
    /// every Energy attached.
    MoveActiveEnergyToBench {
        to_in_play_idx: usize,
        amount: Option<u32>,
    },
    /// Sandy Shocks's Pull In and Pound / Team Rocket's Hypno's Entrap: switch the opponent's
    /// Benched Pokémon at `in_play_idx` into their Active Spot, then deal `damage` to the new
    /// Active Pokémon. The two halves are one action so the damage always lands on the Pokémon
    /// that was just dragged up.
    SwitchOpponentBenchedThenDamage {
        in_play_idx: usize,
        damage: u32,
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
            SimpleAction::Attack(attack) => write!(f, "Attack({})", attack.title),
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
            SimpleAction::ScheduleDelayedSpotKnockOut {
                target_player,
                target_in_play_idx,
            } => write!(
                f,
                "ScheduleDelayedSpotKnockOut(target:{target_player}:{target_in_play_idx})"
            ),
            SimpleAction::ScheduleDelayedSpotDamage {
                target_player,
                target_in_play_idx,
                amount,
            } => write!(
                f,
                "ScheduleDelayedSpotDamage(target:{target_player}:{target_in_play_idx}, amount:{amount})"
            ),
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
            SimpleAction::ShuffleOwnCardsIntoDeck { cards } => {
                write!(f, "ShuffleOwnCardsIntoDeck({:?})", cards)
            }
            SimpleAction::SwitchHandCardForRandomTool { hand_card } => {
                write!(f, "SwitchHandCardForRandomTool({hand_card})")
            }
            SimpleAction::ShuffleOpponentSupporter { supporter_card } => {
                write!(f, "ShuffleOpponentSupporter({supporter_card})")
            }
            SimpleAction::DiscardOpponentSupporter { supporter_card } => {
                write!(f, "DiscardOpponentSupporter({supporter_card})")
            }
            SimpleAction::PutDiscardCardInHand { card } => {
                write!(f, "PutDiscardCardInHand({card})")
            }
            SimpleAction::DiscardOwnCardsThenDamage {
                cards,
                damage,
                target_player,
                target_in_play_idx,
            } => write!(
                f,
                "DiscardOwnCardsThenDamage({cards:?}, {damage} to {target_player}:{target_in_play_idx})"
            ),
            SimpleAction::DiscardOwnCards { cards } => {
                write!(f, "DiscardOwnCards({:?})", cards)
            }
            SimpleAction::BenchOpponentPokemonFromHand { cards } => {
                write!(f, "BenchOpponentPokemonFromHand({:?})", cards)
            }
            SimpleAction::AttachFromDiscard {
                in_play_idx,
                num_random_energies,
            } => {
                write!(f, "AttachFromDiscard({in_play_idx}, {num_random_energies})")
            }
            SimpleAction::AttachTypedFromDiscard {
                in_play_idx,
                energy_type,
                count,
            } => {
                write!(f, "AttachTypedFromDiscard({in_play_idx}, {energy_type:?}, {count})")
            }
            SimpleAction::SadaAttach { assignments } => {
                let s = assignments
                    .iter()
                    .map(|(e, idx)| format!("{e:?}→{idx}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "SadaAttach([{s}])")
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
            SimpleAction::DiscardOwnBenchedThenDamage {
                in_play_idx,
                damage,
            } => {
                write!(f, "DiscardOwnBenchedThenDamage({in_play_idx}, {damage})")
            }
            SimpleAction::DiscardOwnBenchedGroupThenDamage {
                in_play_indices,
                damage,
            } => {
                write!(
                    f,
                    "DiscardOwnBenchedGroupThenDamage({in_play_indices:?}, {damage})"
                )
            }
            SimpleAction::ShuffleSelfAndAttachmentsIntoDeck { in_play_idx } => {
                write!(f, "ShuffleSelfAndAttachmentsIntoDeck({in_play_idx})")
            }
            SimpleAction::MoveEnergyAndReoffer {
                from_in_play_idx,
                to_in_play_idx,
                energy_type,
                remaining_moves,
            } => {
                write!(
                    f,
                    "MoveEnergyAndReoffer({from_in_play_idx} -> {to_in_play_idx}, {energy_type:?}, {remaining_moves} left)"
                )
            }
            SimpleAction::ReturnPokemonToHand { in_play_idx } => {
                write!(f, "ReturnPokemonToHand({in_play_idx})")
            }
            SimpleAction::ShuffleInPlayPokemonIntoDeck { in_play_idx } => {
                write!(f, "ShuffleInPlayPokemonIntoDeck({in_play_idx})")
            }
            SimpleAction::DiscardToolFromPokemon { player, in_play_idx } => {
                write!(f, "DiscardToolFromPokemon({player}, {in_play_idx})")
            }
            SimpleAction::DiscardActiveStadium => write!(f, "DiscardActiveStadium"),
            SimpleAction::DiscardRandomOpponentActiveEnergy => {
                write!(f, "DiscardRandomOpponentActiveEnergy")
            }
            SimpleAction::MoveRandomOpponentEnergyToActive { from_in_play_idx } => {
                write!(f, "MoveRandomOpponentEnergyToActive({from_in_play_idx})")
            }
            SimpleAction::UseStadium => write!(f, "UseStadium"),
            SimpleAction::ApplyStatusToOpponentActive { condition } => {
                write!(f, "ApplyStatusToOpponentActive({condition:?})")
            }
            SimpleAction::ApplyStatusesToOpponentActive { conditions } => {
                write!(f, "ApplyStatusesToOpponentActive({conditions:?})")
            }
            SimpleAction::MoveOpponentActiveEnergyToSelf { to_in_play_idx } => {
                write!(f, "MoveOpponentActiveEnergyToSelf({to_in_play_idx})")
            }
            SimpleAction::HealAndCureConditions {
                in_play_idx,
                amount,
                conditions,
            } => {
                write!(
                    f,
                    "HealAndCureConditions({in_play_idx}, {amount}, {conditions:?})"
                )
            }
            SimpleAction::MoveDamageToOpponentActive {
                from_in_play_idx,
                amount,
            } => {
                write!(f, "MoveDamageToOpponentActive({from_in_play_idx}, {amount})")
            }
            SimpleAction::BenchOpponentPokemonFromDiscard { card } => {
                write!(f, "BenchOpponentPokemonFromDiscard({card})")
            }
            SimpleAction::ShuffleOwnDeck => write!(f, "ShuffleOwnDeck"),
            SimpleAction::ShuffleRandomOwnHandCardIntoDeck => {
                write!(f, "ShuffleRandomOwnHandCardIntoDeck")
            }
            SimpleAction::MoveActiveEnergyToBench {
                to_in_play_idx,
                amount,
            } => {
                write!(f, "MoveActiveEnergyToBench(to:{to_in_play_idx}, {amount:?})")
            }
            SimpleAction::SwitchOpponentBenchedThenDamage {
                in_play_idx,
                damage,
            } => {
                write!(f, "SwitchOpponentBenchedThenDamage({in_play_idx}, {damage})")
            }
            SimpleAction::Noop => write!(f, "Noop"),
        }
    }
}

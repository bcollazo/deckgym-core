use core::fmt;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    card_ids::CardId,
    effects::CardEffect,
    models::{Attack, Card, EnergyType, StatusCondition, TrainerType},
    tool_ids::ToolId,
    AbilityId, State,
};

/// This represents a card in the mat. Has a pointer to the card
/// description, but captures the extra variable properties while in mat.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayedCard {
    pub card: Card,
    pub remaining_hp: u32,
    pub total_hp: u32,
    pub attached_energy: Vec<EnergyType>,
    pub attached_tool: Option<ToolId>,
    pub played_this_turn: bool,
    pub ability_used: bool,
    pub poisoned: bool,
    pub paralyzed: bool,
    pub asleep: bool,
    pub burned: bool,
    pub confused: bool,
    pub cards_behind: Vec<Card>,

    /// Effects that should be cleared if moved to the bench (by retreat or similar).
    /// The second value is the number of turns left for the effect.
    effects: Vec<(CardEffect, u8)>,
}
impl PlayedCard {
    pub fn new(
        card: Card,
        remaining_hp: u32,
        total_hp: u32,
        attached_energy: Vec<EnergyType>,
        played_this_turn: bool,
        cards_behind: Vec<Card>,
    ) -> Self {
        PlayedCard {
            card,
            remaining_hp,
            total_hp,
            attached_energy,
            played_this_turn,
            cards_behind,

            attached_tool: None,
            ability_used: false,
            poisoned: false,
            paralyzed: false,
            asleep: false,
            burned: false,
            confused: false,
            effects: vec![],
        }
    }

    pub fn get_id(&self) -> String {
        match &self.card {
            Card::Pokemon(pokemon_card) => pokemon_card.id.clone(),
            Card::Trainer(trainer_card) => trainer_card.id.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        match &self.card {
            Card::Pokemon(pokemon_card) => pokemon_card.name.clone(),
            Card::Trainer(trainer_card) => trainer_card.name.clone(),
        }
    }

    /// Returns true if this card is a Fossil trainer card
    pub(crate) fn is_fossil(&self) -> bool {
        match &self.card {
            Card::Trainer(trainer_card) => trainer_card.trainer_card_type == TrainerType::Fossil,
            _ => false,
        }
    }

    pub(crate) fn get_attacks(&self) -> &Vec<Attack> {
        match &self.card {
            Card::Pokemon(pokemon_card) => &pokemon_card.attacks,
            _ => panic!("Unsupported playable card type"),
        }
    }

    pub(crate) fn heal(&mut self, amount: u32) {
        self.remaining_hp = (self.remaining_hp + amount).min(self.get_effective_total_hp());
    }

    pub(crate) fn attach_energy(&mut self, energy: &EnergyType, amount: u8) {
        self.attached_energy
            .extend(std::iter::repeat_n(*energy, amount as usize));
    }

    pub(crate) fn apply_damage(&mut self, damage: u32) {
        self.remaining_hp = self.remaining_hp.saturating_sub(damage);
    }

    // Option because if playing an item card... (?)
    pub(crate) fn get_energy_type(&self) -> Option<EnergyType> {
        match &self.card {
            Card::Pokemon(pokemon_card) => Some(pokemon_card.energy_type),
            _ => None,
        }
    }

    /// Check if this Pokemon evolved from a specific Pokemon name
    pub(crate) fn evolved_from(&self, base_name: &str) -> bool {
        if let Card::Pokemon(pokemon_card) = &self.card {
            if let Some(evolves_from) = &pokemon_card.evolves_from {
                return evolves_from == base_name;
            }
        }
        false
    }

    pub(crate) fn is_damaged(&self) -> bool {
        self.remaining_hp < self.get_effective_total_hp()
    }

    /// Returns effective total HP considering abilities like Reuniclus Infinite Increase
    pub(crate) fn get_effective_total_hp(&self) -> u32 {
        let mut effective_hp = self.total_hp;

        // Reuniclus Infinite Increase: +30 HP for each Psychic Energy attached
        if let Some(ability_id) = AbilityId::from_pokemon_id(&self.get_id()[..]) {
            if ability_id == AbilityId::B1a034ReuniclusInfiniteIncrease {
                let psychic_count = self
                    .attached_energy
                    .iter()
                    .filter(|e| **e == EnergyType::Psychic)
                    .count() as u32;
                effective_hp += psychic_count * 30;
            }
        }

        effective_hp
    }

    pub(crate) fn has_status_condition(&self) -> bool {
        self.poisoned || self.paralyzed || self.asleep || self.confused
    }

    pub(crate) fn has_tool_attached(&self) -> bool {
        self.attached_tool.is_some()
    }

    /// Duration means:
    ///   - 0: only during this turn
    ///   - 1: during opponent's next turn
    ///   - 2: on your next turn
    pub fn add_effect(&mut self, effect: CardEffect, duration: u8) {
        self.effects.push((effect, duration));
    }

    pub(crate) fn get_active_effects(&self) -> Vec<CardEffect> {
        self.effects
            .iter()
            .map(|(effect, _)| effect.clone())
            .collect()
    }

    pub(crate) fn clear_status_and_effects(&mut self) {
        self.poisoned = false;
        self.paralyzed = false;
        self.asleep = false;
        self.burned = false;
        self.confused = false;
        self.effects.clear();
    }

    pub(crate) fn cure_status_conditions(&mut self) {
        self.poisoned = false;
        self.paralyzed = false;
        self.asleep = false;
        self.burned = false;
        self.confused = false;
    }

    /// Apply a status condition to this Pokémon, respecting Arceus ex immunity
    pub(crate) fn apply_status_condition(&mut self, status: StatusCondition) {
        // Arceus Ex avoids status effects
        let string_id = self.get_id();
        let arceus_ids = [
            CardId::A2a071ArceusEx,
            CardId::A2a086ArceusEx,
            CardId::A2a095ArceusEx,
            CardId::A2a096ArceusEx,
        ];
        let card_id = CardId::from_card_id(&string_id).unwrap();
        if arceus_ids.contains(&card_id) {
            debug!("Arceus Ex avoids status effect");
            return;
        }

        match status {
            StatusCondition::Asleep => self.asleep = true,
            StatusCondition::Paralyzed => self.paralyzed = true,
            StatusCondition::Poisoned => self.poisoned = true,
            StatusCondition::Burned => self.burned = true,
            StatusCondition::Confused => self.confused = true,
        }
    }

    pub(crate) fn end_turn_maintenance(&mut self) {
        // Remove all the ones that are 0, and subtract 1 from the rest
        self.effects.retain_mut(|(_, duration)| {
            if *duration > 0 {
                *duration -= 1;
                true
            } else {
                false
            }
        });

        // Reset played_this_turn and ability_used
        self.played_this_turn = false;
        self.ability_used = false;
    }

    /// Returns effective attached energy considering Serperior's Jungle Totem ability.
    /// If Jungle Totem is active for Grass Pokemon, Grass energy counts double.
    pub(crate) fn get_effective_attached_energy(
        &self,
        state: &State,
        player: usize,
    ) -> Vec<EnergyType> {
        let double_grass = self.has_double_grass(state, player);
        if double_grass {
            let mut doubled = Vec::new();
            for energy in &self.attached_energy {
                doubled.push(*energy);
                if *energy == EnergyType::Grass {
                    doubled.push(EnergyType::Grass); // Add another Grass energy
                }
            }
            doubled
        } else {
            self.attached_energy.to_vec()
        }
    }

    pub(crate) fn has_double_grass(&self, state: &State, player: usize) -> bool {
        let pokemon_type = self.card.get_type();
        let jungle_totem_active = has_serperior_jungle_totem(state, player);
        jungle_totem_active && pokemon_type == Some(EnergyType::Grass)
    }
}

impl fmt::Debug for PlayedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}({}hp,{:?})",
                self.get_name(),
                self.remaining_hp,
                self.attached_energy
            )
        } else {
            write!(
                f,
                "{}({}hp,{})",
                self.get_name(),
                self.remaining_hp,
                self.attached_energy.len()
            )
        }
    }
}

pub fn has_serperior_jungle_totem(state: &crate::state::State, player: usize) -> bool {
    state.enumerate_in_play_pokemon(player).any(|(_, pokemon)| {
        AbilityId::from_pokemon_id(&pokemon.get_id()[..])
            .map(|id| id == AbilityId::A1a006SerperiorJungleTotem)
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        card_ids::CardId, database::get_card_by_enum, hooks::to_playable_card,
        models::has_serperior_jungle_totem, state::State,
    };

    #[test]
    fn test_has_serperior_jungle_totem_with_serperior() {
        // Arrange: Create a state with Serperior on the bench
        let mut state = State::default();
        let serperior_card = get_card_by_enum(CardId::A1a006Serperior);
        let played_serperior = to_playable_card(&serperior_card, false);

        // Place Serperior in bench slot 1
        state.in_play_pokemon[0][1] = Some(played_serperior);

        // Act & Assert
        assert!(
            has_serperior_jungle_totem(&state, 0),
            "Should detect Serperior's Jungle Totem ability when Serperior is in play"
        );
    }

    #[test]
    fn test_has_serperior_jungle_totem_without_serperior() {
        // Arrange: Create a state without Serperior
        let mut state = State::default();
        let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
        let played_bulbasaur = to_playable_card(&bulbasaur_card, false);

        // Place Bulbasaur in active slot
        state.in_play_pokemon[0][0] = Some(played_bulbasaur);

        // Act & Assert
        assert!(
            !has_serperior_jungle_totem(&state, 0),
            "Should not detect Jungle Totem ability when Serperior is not in play"
        );
    }

    #[test]
    fn test_has_serperior_jungle_totem_wrong_player() {
        // Arrange: Create a state with Serperior for player 0
        let mut state = State::default();
        let serperior_card = get_card_by_enum(CardId::A1a006Serperior);
        let played_serperior = to_playable_card(&serperior_card, false);

        // Place Serperior in player 0's bench
        state.in_play_pokemon[0][1] = Some(played_serperior);

        // Act & Assert: Check for player 1
        assert!(
            !has_serperior_jungle_totem(&state, 1),
            "Should not detect Jungle Totem ability for opponent player"
        );
    }
}

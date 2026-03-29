use crate::{
    actions::{
        abilities::AbilityMechanic, ability_mechanic_from_effect, get_ability_mechanic,
        handle_damage_only, handle_knockouts,
    },
    effects::TurnEffect,
    models::{EnergyType, StatusCondition},
    State,
};

impl State {
    pub(crate) fn attach_energy_from_zone(
        &mut self,
        actor: usize,
        in_play_idx: usize,
        energy: EnergyType,
        amount: u32,
        is_turn_energy: bool,
    ) -> bool {
        if !self.can_attach_energy_from_zone(in_play_idx) {
            return false;
        }
        let attached =
            self.attach_energy_internal(actor, in_play_idx, energy, amount, true, is_turn_energy);
        if attached && is_turn_energy {
            self.current_energy = None;
        }
        attached
    }

    /// Attaches energies from the discard pile to a Pokemon in play.
    /// Removes the specified energies from discard_energies and attaches them to the Pokemon.
    pub(crate) fn attach_energy_from_discard(
        &mut self,
        player: usize,
        in_play_idx: usize,
        energies: &[EnergyType],
    ) {
        // Remove energies from discard pile
        for energy in energies {
            let pos = self.discard_energies[player]
                .iter()
                .position(|e| e == energy)
                .expect("Energy should be in discard pile");
            self.discard_energies[player].remove(pos);
        }

        // Attach energies to Pokemon
        for energy in energies {
            self.attach_energy_internal(player, in_play_idx, *energy, 1, false, false);
        }
    }

    pub(crate) fn can_attach_energy_from_zone(&self, in_play_idx: usize) -> bool {
        if in_play_idx != 0 {
            return true;
        }
        let blocked = self
            .get_current_turn_effects()
            .iter()
            .any(|x| matches!(x, TurnEffect::NoEnergyFromZoneToActive));
        !blocked
    }

    fn attach_energy_internal(
        &mut self,
        actor: usize,
        in_play_idx: usize,
        energy: EnergyType,
        amount: u32,
        from_zone: bool,
        is_turn_energy: bool,
    ) -> bool {
        if amount == 0 {
            return false;
        }
        let pokemon = self.in_play_pokemon[actor][in_play_idx]
            .as_mut()
            .expect("Pokemon should be there if attaching energy to it");
        pokemon
            .attached_energy
            .extend(std::iter::repeat_n(energy, amount as usize));
        for _ in 0..amount {
            self.on_attach_energy(actor, in_play_idx, energy, from_zone, is_turn_energy);
        }
        handle_knockouts(self, (0, 0), false);
        true
    }

    fn on_attach_energy(
        &mut self,
        actor: usize,
        in_play_idx: usize,
        energy_type: EnergyType,
        from_zone: bool,
        is_turn_energy: bool,
    ) {
        let mechanic = {
            let pokemon = self.in_play_pokemon[actor][in_play_idx]
                .as_ref()
                .expect("Pokemon should be there if attaching energy to it");
            get_ability_mechanic(&pokemon.card).cloned()
        };

        if from_zone {
            let opponent = (actor + 1) % 2;
            if let Some(opponent_active) = self.in_play_pokemon[opponent][0].as_ref() {
                let has_electromagnetic_wall = opponent_active
                    .card
                    .get_ability()
                    .and_then(|ability| ability_mechanic_from_effect(&ability.effect))
                    .is_some_and(|mechanic| *mechanic == AbilityMechanic::ElectromagneticWall);
                if has_electromagnetic_wall {
                    handle_damage_only(
                        self,
                        (opponent, 0),
                        &[(20, actor, in_play_idx)],
                        false,
                        None,
                    );
                }
            }
        }

        // Check for Darkrai ex's Nightmare Aura ability
        if let Some(mechanic) = mechanic {
            if matches!(
                mechanic,
                AbilityMechanic::DamageOpponentActiveOnZoneAttachToSelf {
                    energy_type: EnergyType::Darkness,
                    amount: 20,
                    only_turn_energy: true,
                }
            ) && energy_type == EnergyType::Darkness
                && is_turn_energy
            {
                // Deal 20 damage to opponent's active Pokémon
                let opponent = (actor + 1) % 2;
                handle_damage_only(
                    self,
                    (actor, in_play_idx),
                    &[(20, opponent, 0)],
                    false,
                    None,
                );
            }

            // Check for Komala's Comatose ability
            if matches!(
                mechanic,
                AbilityMechanic::SleepOnZoneAttachToSelfWhileActive
            ) && in_play_idx == 0
                && from_zone
            {
                // As long as this Pokémon is in the Active Spot, whenever you attach an Energy from your Energy Zone to it, it is now Asleep.
                self.apply_status_condition(actor, 0, StatusCondition::Asleep);
            }

            // Check for Cresselia ex's Lunar Plumage ability
            if matches!(
                mechanic,
                AbilityMechanic::HealSelfOnZoneAttach {
                    energy_type: EnergyType::Psychic,
                    amount: 20,
                }
            ) && energy_type == EnergyType::Psychic
                && from_zone
            {
                // Whenever you attach a Psychic Energy from your Energy Zone to this Pokémon, heal 20 damage from this Pokémon.
                let pokemon = self.in_play_pokemon[actor][in_play_idx]
                    .as_mut()
                    .expect("Pokemon should be there if attaching energy to it");
                pokemon.heal(20);
            }
        }
    }
}

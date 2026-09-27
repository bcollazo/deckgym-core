use core::fmt;
use serde::{Deserialize, Serialize};

use super::State;
use crate::{
    actions::{
        abilities::AbilityMechanic, card_effect_from_ability_mechanic, get_ability_mechanic,
    },
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    hooks::is_ancient_pokemon,
    models::{Attack, Card, EnergyType, StatusCondition, TrainerType, BASIC_STAGE},
    tools::has_tool,
};

/// This represents a card in the mat. Has a pointer to the card
/// description, but captures the extra variable properties while in mat.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayedCard {
    pub card: Card,
    damage_counters: u32,
    base_hp: u32,
    stadium_hp_bonus: u32,
    /// Whether Serperior's Jungle Totem ability is active for this Pokemon's owner
    /// (i.e. any of their Pokemon in play has it) and this Pokemon is [G] type, so its
    /// attached [G] Energy is doubled for effects like Serperior's own Regal Bloom.
    /// Kept in sync via `State::refresh_double_grass_bonus_for_player` whenever the
    /// board composition for this Pokemon's owner changes.
    double_grass_active: bool,
    /// Extra HP granted by an ability on this Pokémon's owner's board (Lilligant's Toughness
    /// Aroma). Kept in sync by `State::refresh_ability_board_bonuses`, like `stadium_hp_bonus`.
    #[serde(default)]
    ability_hp_bonus: u32,
    /// Whether a Heal Block ability (Claydol) is in play for either player, in which case
    /// `heal` is a no-op. Kept in sync by `State::refresh_ability_board_bonuses`.
    #[serde(default)]
    heal_blocked: bool,
    pub attached_energy: Vec<EnergyType>,
    pub attached_tool: Option<Card>,
    pub played_this_turn: bool,
    pub moved_to_active_this_turn: bool,
    pub ability_used: bool,
    poisoned: bool,
    /// Checkup damage this Pokemon takes from its current Poison, when an attack replaced the usual
    /// amount ("Do 20 damage to this Pokemon instead of the usual amount for this Special
    /// Condition." — Toxicroak's Toxic, Toxapex's Severe Poison). Cleared whenever the Poison is
    /// cleared or re-applied normally, so a later ordinary Poison is back to 10.
    #[serde(default)]
    poison_damage_override: Option<u32>,
    paralyzed: bool,
    asleep: bool,
    burned: bool,
    confused: bool,
    pub cards_behind: Vec<Card>,
    pub prevent_first_attack_damage_used: bool,
    pub has_attacked_since_play: bool,
    /// Set when a "when this Pokémon is Knocked Out, flip a coin; if heads your opponent can't
    /// get any points for it" ability (Dusknoir's Fade into Darkness, Glimmora's Shattering
    /// Crystal) came up heads for the Knock Out about to be resolved. Read (and then irrelevant,
    /// since the card leaves play) by `handle_knockouts`.
    #[serde(default)]
    pub(crate) knockout_points_denied: bool,
    /// Whether this Pokémon has been damaged by an attack while in the Active Spot during the
    /// current turn, and during the previous turn (e.g. for Wobbuffet's Reply Strongly). Rolled
    /// over by `end_turn_maintenance`, and carried with the card if it retreats.
    #[serde(default)]
    damaged_by_attack_while_active_this_turn: bool,
    #[serde(default)]
    damaged_by_attack_while_active_last_turn: bool,

    /// Effects that should be cleared if moved to the bench (by retreat or similar).
    /// The second value is the number of turns left for the effect.
    effects: Vec<(CardEffect, u8)>,
}
impl PlayedCard {
    pub fn new(
        card: Card,
        damage_counters: u32,
        base_hp: u32,
        attached_energy: Vec<EnergyType>,
        played_this_turn: bool,
        cards_behind: Vec<Card>,
    ) -> Self {
        PlayedCard {
            card,
            damage_counters,
            base_hp,
            stadium_hp_bonus: 0,
            double_grass_active: false,
            ability_hp_bonus: 0,
            heal_blocked: false,
            attached_energy,
            played_this_turn,
            moved_to_active_this_turn: false,
            cards_behind,

            attached_tool: None,
            ability_used: false,
            poisoned: false,
            poison_damage_override: None,
            paralyzed: false,
            asleep: false,
            burned: false,
            confused: false,
            effects: vec![],
            prevent_first_attack_damage_used: false,
            has_attacked_since_play: false,
            knockout_points_denied: false,
            damaged_by_attack_while_active_this_turn: false,
            damaged_by_attack_while_active_last_turn: false,
        }
    }

    /// Create a fresh PlayedCard from a Card at full HP with no energy, tools, or status.
    pub fn from_card(card: &Card) -> Self {
        let base_hp = match card {
            Card::Pokemon(pokemon_card) => pokemon_card.hp,
            Card::Trainer(trainer_card) => {
                if trainer_card.trainer_card_type == TrainerType::Fossil {
                    40
                } else {
                    panic!(
                        "Cannot create PlayedCard from non-Fossil Trainer: {:?}",
                        trainer_card
                    );
                }
            }
        };
        Self::new(card.clone(), 0, base_hp, vec![], false, vec![])
    }

    /// Create a fresh PlayedCard from a CardId at full HP with no energy, tools, or status.
    pub fn from_id(card_id: CardId) -> Self {
        let card = get_card_by_enum(card_id);
        Self::from_card(&card)
    }

    pub fn with_energy(mut self, energy: Vec<EnergyType>) -> Self {
        self.attached_energy = energy;
        self
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage_counters = self.damage_counters.saturating_add(damage);
        self
    }

    pub fn with_remaining_hp(mut self, remaining_hp: u32) -> Self {
        self.set_remaining_hp(remaining_hp);
        self
    }

    /// Set the remaining HP to an exact value (e.g. Ursaluna's Guts leaves it at 10).
    pub(crate) fn set_remaining_hp(&mut self, remaining_hp: u32) {
        let effective_hp = self.get_effective_total_hp();
        let clamped_remaining = remaining_hp.min(effective_hp);
        self.damage_counters = effective_hp.saturating_sub(clamped_remaining);
    }

    pub fn with_tool(mut self, tool: Card) -> Self {
        self.attached_tool = Some(tool);
        self
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
        // Claydol's Heal Block: "Pokémon (both yours and your opponent's) can't be healed."
        if self.heal_blocked {
            return;
        }
        self.damage_counters = self.damage_counters.saturating_sub(amount);
    }

    pub(crate) fn apply_damage(&mut self, damage: u32) {
        self.damage_counters = self.damage_counters.saturating_add(damage);
    }

    /// Every Energy type this Pokémon counts as **while in play**: the printed type(s) of its
    /// card (`Card::get_types`) plus any type granted by its Ability
    /// (`AbilityMechanic::GrantedTypes`, i.e. Urshifu's Double Type). The list is deduplicated
    /// and keeps the printed type first.
    ///
    /// The lookup goes through `ability_mechanic()`, so a Pokémon that has lost its Abilities
    /// (Budew's Prickly Powder, Alolan Muk-style suppression) falls back to its printed type.
    ///
    /// # Which checks use the type *set* and which use the printed type
    ///
    /// Rule of thumb: a Pokémon **in play** is every one of its types at once, so any rule that
    /// asks "is this a [X] Pokémon" is satisfied if *any* of its types is [X]. Cards that are
    /// **not** in play (deck, hand, discard) have no Ability active, so they keep the printed
    /// type. Bonuses granted this way are still applied **once**, never once per matching type.
    ///
    /// | Site | Rule chosen |
    /// |---|---|
    /// | Weakness (`hooks::core::get_weakness_application`) | Defender's Weakness vs the **attacker's type set**: weak to either type → the usual flat +20 (or Bounded Field ×2). Never doubled when more than one type is involved — Weakness names a single type, so at most one can match. |
    /// | Typed damage auras (`IncreaseDamageForTypeInPlay`, `IncreaseDamageForTwoTypesInPlay`) | Attacker's type set. `…TwoTypes…` still adds its bonus once even if both listed types match. |
    /// | `ReduceDamageFromAttacksByAttackerType` (Thick Fat) | Attacker's type set intersects the listed types. |
    /// | `TurnEffect::IncreasedDamageForType{,AgainstEx}` | Attacker's type set. |
    /// | `TurnEffect::ReducedDamageForType` | Defender's type set. |
    /// | Arena of Antiquity ([F] attacker), Peculiar Plaza ([P] retreat) | In-play Pokémon's type set. |
    /// | Typed retreat reductions (`ReduceRetreatCostOfYourActiveTypedFromBench`, Inflatable Boat) | Active's type set; each source still applies once. |
    /// | Typed Tools on their holder (Leaf Cape, Steel Apron, Metal Core Barrier, Dark Pendant, Deceptive Needle, Electrical Cord) | Holder's type set — the Tool is attached to a Pokémon in play. |
    /// | Typed HP auras (`IncreaseHpOfYourTypedPokemon`) | Recipient's type set. |
    /// | Jungle Totem ([G] Energy doubling) | Holder's type set. |
    /// | Typed Energy attach / move abilities and Trainers (Vaporeon, Lunala ex, Baxcalibur, Electric Generator, Misty, …) | Target's type set. |
    /// | Typed heals and typed selection of Pokémon **in play** (Erika, Diantha, Ilima, Parasol Lady, Wallace, Quick Growth, `HealTypedPokemonOnEvolve`, …) | Target's type set. |
    /// | `num_in_play_of_type` and every "count/choose your [X] Pokémon" attack | Type set of each in-play Pokémon; a dual-type Pokémon is still counted once per type asked about. |
    /// | "If your opponent's Active is a [X] Pokémon" (`ExtraDamageIfDefenderType`, `…TypeIn`) | Defender's type set; the bonus applies once. |
    /// | Victory Star (attacker must be [R]) | Attacker's type set. |
    /// | KO bookkeeping for revenge attacks (`record_knocked_out_by_opponent_attack`) | Records **all** of the KO'd Pokémon's types, so a [D]-restricted revenge attack sees a KO'd [F]/[D] Pokémon. |
    /// | Deck / hand / discard-pile searches and mills (`pokemon_search_outcomes_by_type`, Fishing Net, Fisher, Fragrant Forest, `milled_card_matches`, the deck side of Wallace and Quick Growth) | **Printed** type — the card is not in play, so its Ability is not active. |
    /// | Deck building (`Deck::energy_types`), TUI colours, `card_enum_generator` | **Printed** type. |
    pub fn get_energy_types(&self) -> Vec<EnergyType> {
        let mut types = self.card.get_types();
        if let Some(AbilityMechanic::GrantedTypes { energy_types }) = self.ability_mechanic() {
            for granted in energy_types {
                if !types.contains(granted) {
                    types.push(*granted);
                }
            }
        }
        types
    }

    /// Whether this Pokémon counts as `energy_type` while in play — the membership test over
    /// `get_energy_types`, without the allocation. This is what almost every "is this a [X]
    /// Pokémon" rule should call.
    pub fn is_type(&self, energy_type: EnergyType) -> bool {
        if self.card.is_type(energy_type) {
            return true;
        }
        matches!(
            self.ability_mechanic(),
            Some(AbilityMechanic::GrantedTypes { energy_types }) if energy_types.contains(&energy_type)
        )
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
        self.damage_counters > 0
    }

    /// Keeps `double_grass_active` in sync with whether Jungle Totem is active for this
    /// Pokemon's owner. Called by `State::refresh_double_grass_bonus_for_player` whenever
    /// the owner's board composition changes.
    pub(crate) fn refresh_double_grass_active(&mut self, jungle_totem_active_for_owner: bool) {
        self.double_grass_active = jungle_totem_active_for_owner && self.is_type(EnergyType::Grass);
    }

    /// Keeps the ability-derived board bonuses in sync. Called by
    /// `State::refresh_ability_board_bonuses` whenever either player's board changes.
    pub(crate) fn refresh_ability_board_bonuses(
        &mut self,
        typed_hp_bonuses: &[(EnergyType, u32)],
        heal_blocked: bool,
    ) {
        // One bonus per source ability, even if this Pokemon counts as several of the boosted
        // types at once.
        self.ability_hp_bonus = typed_hp_bonuses
            .iter()
            .filter(|(energy_type, _)| self.is_type(*energy_type))
            .map(|(_, amount)| *amount)
            .sum();
        self.heal_blocked = heal_blocked;
    }

    pub(crate) fn refresh_starting_plains_bonus(&mut self, starting_plains_active: bool) {
        let is_basic_pokemon = matches!(
            &self.card,
            Card::Pokemon(pokemon_card) if pokemon_card.stage == BASIC_STAGE
        );
        self.stadium_hp_bonus = if starting_plains_active && is_basic_pokemon {
            20
        } else {
            0
        };
    }

    pub fn get_remaining_hp(&self) -> u32 {
        self.get_effective_total_hp()
            .saturating_sub(self.damage_counters)
    }

    pub(crate) fn is_knocked_out(&self) -> bool {
        self.damage_counters >= self.get_effective_total_hp()
    }

    pub(crate) fn get_damage_counters(&self) -> u32 {
        self.damage_counters
    }

    /// Returns effective total HP considering abilities like Reuniclus Infinite Increase
    pub(crate) fn get_effective_total_hp(&self) -> u32 {
        let mut effective_hp = self.base_hp;

        // Tool bonuses. Type/stage-specific caps only apply to matching Pokémon (the tools are
        // attachable to anything, but their HP bonus is gated by the holder).
        if has_tool(self, CardId::A2147GiantCape) {
            effective_hp += 20;
        } else if has_tool(self, CardId::A3147LeafCape) && self.is_type(EnergyType::Grass) {
            // Leaf Cape: "The [G] Pokémon this card is attached to gets +30 HP."
            effective_hp += 30;
        } else if has_tool(self, CardId::B3b065ElegantCape)
            && matches!(&self.card, Card::Pokemon(p) if p.stage == 1)
        {
            // Elegant Cape: "The Stage 1 Pokémon this card is attached to gets +30 HP."
            effective_hp += 30;
        } else if has_tool(self, CardId::B3a069AncientBoosterEnergyCapsule)
            && is_ancient_pokemon(&self.get_name())
        {
            effective_hp += 40;
        }

        effective_hp += self.stadium_hp_bonus;
        effective_hp += self.ability_hp_bonus;

        // E.g. Reuniclus Infinite Increase, Serperior Regal Bloom: +HP for each Energy of a type attached
        if let Some(AbilityMechanic::IncreaseHpPerAttachedEnergy {
            energy_type,
            amount,
        }) = self.ability_mechanic()
        {
            let mut matching_count = self
                .attached_energy
                .iter()
                .filter(|e| *e == energy_type)
                .count() as u32;
            // Serperior's Jungle Totem: each [G] Energy attached to your [G] Pokemon provides
            // 2 [G] Energy, so it doubles the count feeding this Pokemon's own HP-per-energy ability.
            if *energy_type == EnergyType::Grass && self.double_grass_active {
                matching_count *= 2;
            }
            effective_hp += matching_count * amount;
        }

        effective_hp
    }

    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Replaces the Checkup damage of this Pokemon's current Poison. Set right after the Poison
    /// lands; `set_status_raw(Poisoned)` and every cure path reset it.
    pub(crate) fn set_poison_damage_override(&mut self, amount: u32) {
        self.poison_damage_override = Some(amount);
    }

    /// The Checkup damage this Pokemon's Poison deals before any board-wide bonus, i.e. the usual
    /// 10 unless an attack overrode it.
    pub(crate) fn poison_base_damage(&self) -> u32 {
        self.poison_damage_override.unwrap_or(10)
    }

    pub fn is_paralyzed(&self) -> bool {
        self.paralyzed
    }

    pub fn is_asleep(&self) -> bool {
        self.asleep
    }

    pub fn is_burned(&self) -> bool {
        self.burned
    }

    pub fn is_confused(&self) -> bool {
        self.confused
    }

    pub(crate) fn has_status_condition(&self) -> bool {
        self.poisoned || self.paralyzed || self.asleep || self.burned || self.confused
    }

    /// How many Special Conditions currently affect this Pokémon (e.g. for Team Rocket's Magmar's
    /// Derisive Roasting, which scales with that count).
    pub fn count_status_conditions(&self) -> usize {
        [
            self.poisoned,
            self.paralyzed,
            self.asleep,
            self.burned,
            self.confused,
        ]
        .iter()
        .filter(|flag| **flag)
        .count()
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

    /// Records that this Pokémon was damaged by an attack while in the Active Spot this turn.
    pub(crate) fn mark_damaged_by_attack_while_active(&mut self) {
        self.damaged_by_attack_while_active_this_turn = true;
    }

    /// Whether this Pokémon was damaged by an attack during the previous turn while it was in
    /// the Active Spot (e.g. for Wobbuffet's Reply Strongly).
    pub fn was_damaged_by_attack_while_active_last_turn(&self) -> bool {
        self.damaged_by_attack_while_active_last_turn
    }

    /// Whether this Pokémon currently carries `effect`, regardless of its remaining duration.
    pub fn has_effect(&self, effect: &CardEffect) -> bool {
        self.effects.iter().any(|(stored, _)| stored == effect)
    }

    pub(crate) fn get_active_effects(&self) -> Vec<CardEffect> {
        self.effects
            .iter()
            .map(|(effect, _)| effect.clone())
            .collect()
    }

    /// All effects currently on this Pokémon: the stored (turn-duration) effects from
    /// `add_effect`, plus effects *derived* from its passive ability (the "abilities-as-effects"
    /// model — see `card_effect_from_ability_mechanic`). Damage code should query this instead of
    /// separately scanning for defensive abilities, so a passive like Cloyster's Shell Armor and a
    /// stored effect like Carracosta's Blocking Shell are handled through one list. Derived effects
    /// are present exactly while the ability-holder is in play (no turn duration).
    pub(crate) fn get_effective_card_effects(&self) -> Vec<CardEffect> {
        let mut effects = self.get_active_effects();
        if let Some(mechanic) = self.ability_mechanic() {
            if let Some(derived) = card_effect_from_ability_mechanic(mechanic) {
                effects.push(derived);
            }
        }
        effects
    }

    /// Whether this Pokémon has been stripped of its Abilities (Budew's Prickly Powder).
    fn abilities_disabled(&self) -> bool {
        self.effects
            .iter()
            .any(|(effect, _)| matches!(effect, CardEffect::AbilitiesDisabled))
    }

    /// This Pokémon's Ability as it applies *in play*, i.e. `None` while its Abilities are
    /// disabled. Every ability lookup that has a `PlayedCard` in hand should go through this
    /// rather than reaching into `self.card` directly, so "loses all Abilities" is honoured
    /// uniformly.
    pub(crate) fn ability(&self) -> Option<crate::models::Ability> {
        if self.abilities_disabled() {
            return None;
        }
        self.card.get_ability()
    }

    /// The `AbilityMechanic` this Pokémon contributes in play, or `None` while its Abilities are
    /// disabled. The in-play counterpart of `get_ability_mechanic(&card)`.
    pub(crate) fn ability_mechanic(&self) -> Option<&'static AbilityMechanic> {
        if self.abilities_disabled() {
            return None;
        }
        get_ability_mechanic(&self.card)
    }

    /// Whether this Pokémon contributes `mechanic` in play. The in-play counterpart of
    /// `has_ability_mechanic(&card, mechanic)`.
    pub(crate) fn has_ability(&self, mechanic: &AbilityMechanic) -> bool {
        self.ability_mechanic() == Some(mechanic)
    }

    pub(crate) fn get_effects(&self) -> &Vec<(CardEffect, u8)> {
        &self.effects
    }

    pub(crate) fn clear_status_and_effects(&mut self) {
        self.poisoned = false;
        self.poison_damage_override = None;
        self.paralyzed = false;
        self.asleep = false;
        self.burned = false;
        self.confused = false;
        self.effects.clear();
    }

    pub(crate) fn cure_status_conditions(&mut self) {
        self.poisoned = false;
        self.poison_damage_override = None;
        self.paralyzed = false;
        self.asleep = false;
        self.burned = false;
        self.confused = false;
    }

    pub(crate) fn clear_status_condition(&mut self, status: StatusCondition) {
        match status {
            StatusCondition::Poisoned => {
                self.poisoned = false;
                self.poison_damage_override = None;
            }
            StatusCondition::Paralyzed => self.paralyzed = false,
            StatusCondition::Asleep => self.asleep = false,
            StatusCondition::Burned => self.burned = false,
            StatusCondition::Confused => self.confused = false,
        }
    }

    /// Raw status setter — does NOT check immunity. Use `State::apply_status_condition` instead.
    pub(crate) fn set_status_raw(&mut self, status: StatusCondition) {
        match status {
            StatusCondition::Asleep => self.asleep = true,
            StatusCondition::Paralyzed => self.paralyzed = true,
            StatusCondition::Poisoned => {
                self.poisoned = true;
                // An ordinary Poison replaces any earlier custom-damage Poison.
                self.poison_damage_override = None;
            }
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

        // A points-denial coin flip that didn't end up mattering (the Knock Out was prevented)
        // must not carry over to a later Knock Out.
        self.knockout_points_denied = false;
        // Roll the "damaged by an attack while Active" flag over to the previous turn.
        self.damaged_by_attack_while_active_last_turn =
            std::mem::take(&mut self.damaged_by_attack_while_active_this_turn);

        // Reset played_this_turn, moved_to_active_this_turn, and ability_used
        self.played_this_turn = false;
        self.moved_to_active_this_turn = false;
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
        let jungle_totem_active = has_serperior_jungle_totem(state, player);
        jungle_totem_active && self.is_type(EnergyType::Grass)
    }
}

impl fmt::Debug for PlayedCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}({}hp,{:?})",
                self.get_name(),
                self.get_remaining_hp(),
                self.attached_energy
            )
        } else {
            write!(
                f,
                "{}({}hp,{})",
                self.get_name(),
                self.get_remaining_hp(),
                self.attached_energy.len()
            )
        }
    }
}

pub fn has_serperior_jungle_totem(state: &State, player: usize) -> bool {
    state
        .enumerate_in_play_pokemon(player)
        .any(|(_, pokemon)| pokemon.has_ability(&AbilityMechanic::DoubleGrassEnergy))
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

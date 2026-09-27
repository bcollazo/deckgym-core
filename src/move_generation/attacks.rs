use crate::{
    actions::{abilities::AbilityMechanic, Mechanic, SimpleAction, EFFECT_MECHANIC_MAP},
    card_ids::CardId,
    effects::CardEffect,
    hooks::{contains_energy, get_effective_attack_cost},
    models::{Attack, PlayedCard},
    tools::has_tool,
    State,
};

pub(crate) fn generate_attack_actions(state: &State) -> Vec<SimpleAction> {
    let current_player = state.current_player;
    let mut actions = Vec::new();
    if let Some(active_pokemon) = &state.in_play_pokemon[current_player][0] {
        // Fossil cards cannot attack
        if active_pokemon.is_fossil() {
            return actions;
        }

        // Check if the active Pokémon has the CannotAttack effect
        let active_effects = active_pokemon.get_active_effects();
        let cannot_attack = active_effects
            .iter()
            .any(|effect| matches!(effect, CardEffect::CannotAttack));
        if cannot_attack {
            return actions;
        }

        // Regigigas's Seal of Antiquity: "If you don't have Regirock, Regice, and Registeel on
        // your Bench, this Pokémon can't attack."
        if let Some(AbilityMechanic::CannotAttackUnlessNamedOnBench { names }) =
            active_pokemon.ability_mechanic()
        {
            let all_on_bench = names.iter().all(|name| {
                state
                    .enumerate_bench_pokemon(current_player)
                    .any(|(_, pokemon)| pokemon.get_name() == *name)
            });
            if !all_on_bench {
                return actions;
            }
        }

        let restricted_attack_names: Vec<String> = active_effects
            .iter()
            .filter_map(|effect| match effect {
                CardEffect::CannotUseAttack(attack_name) => Some(attack_name.clone()),
                _ => None,
            })
            .collect();

        // The active Pokémon's own attacks, plus any granted by Celebi's Time Recall or by a
        // Memory Light attached to the active Pokémon itself.
        let mut available_attacks: Vec<Attack> = active_pokemon.get_attacks().clone();
        if time_recall_active(state, current_player)
            || has_tool(active_pokemon, CardId::A4a068MemoryLight)
        {
            available_attacks.extend(previous_evolution_attacks(active_pokemon));
        }

        let mut offered: Vec<Attack> = Vec::new();
        for attack in available_attacks {
            // Avoid offering an identical attack twice (e.g. an attack kept unchanged across an
            // evolution). Dedup on the whole Attack, not just the title: a previous evolution can
            // share an attack's name while differing in cost/damage/effect (e.g. Swirlix's and
            // Slurpuff's "Sweets Relay"), and those are genuinely distinct, usable attacks.
            if offered.contains(&attack) {
                continue;
            }
            if restricted_attack_names.contains(&attack.title) {
                continue;
            }
            if !attack_precondition_met(state, current_player, &attack) {
                continue;
            }
            let modified_cost = get_effective_attack_cost(&attack, state, current_player);
            if contains_energy(active_pokemon, &modified_cost, state, current_player) {
                offered.push(attack.clone());
                actions.push(SimpleAction::Attack(attack));
            }
        }
    }
    actions
}

/// Whether an attack whose effect text carries a "You can use this attack only if ..." clause has
/// that condition met. Attacks without such a clause are always usable (subject to the ordinary
/// Energy cost and effect restrictions checked by the caller).
fn attack_precondition_met(state: &State, player: usize, attack: &Attack) -> bool {
    let Some(effect) = attack.effect.as_deref() else {
        return true;
    };
    match EFFECT_MECHANIC_MAP.get(effect) {
        // Mesprit's Supreme Blast: "You can use this attack only if you have Uxie and Azelf on
        // your Bench."
        Some(Mechanic::RequireBenchedNamesThenDiscardAllEnergy {
            required_bench_names,
        }) => required_bench_names.iter().all(|name| {
            state
                .enumerate_bench_pokemon(player)
                .any(|(_, pokemon)| pokemon.get_name() == *name)
        }),
        _ => true,
    }
}

/// Celebi's Time Recall: while a Pokémon with the ability is in play, each of your evolved
/// Pokémon can use any attack from its previous Evolutions.
fn time_recall_active(state: &State, player: usize) -> bool {
    state
        .enumerate_in_play_pokemon(player)
        .any(|(_, pokemon)| pokemon.has_ability(&AbilityMechanic::TimeRecall))
}

/// The attacks of a Pokémon's previous Evolutions, i.e. the under-cards recorded on it when it
/// evolved (`cards_behind`). Granted by Celebi's Time Recall (board-wide) and by Memory Light
/// ("The Pokémon this card is attached to can use any attack from its previous Evolutions.").
/// We only need the active Pokémon's, since only the active Pokémon can attack.
fn previous_evolution_attacks(active_pokemon: &PlayedCard) -> Vec<Attack> {
    active_pokemon
        .cards_behind
        .iter()
        .flat_map(|card| card.get_attacks())
        .collect()
}

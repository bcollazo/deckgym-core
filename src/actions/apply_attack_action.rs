use std::collections::HashMap;

use log::trace;
use rand::{rngs::StdRng, Rng};

use crate::{
    actions::{
        abilities::AbilityMechanic,
        apply_action_helpers::{handle_damage, handle_damage_only, handle_knockouts},
        apply_evolve,
        attack_helpers::{
            collect_in_play_indices_by_type, energy_any_way_choices, generate_distributions,
        },
        attacks::{BenchSide, CopyAttackSource, Mechanic},
        effect_ability_mechanic_map::ability_mechanic_from_effect,
        effect_mechanic_map::EFFECT_MECHANIC_MAP,
        Action,
    },
    combinatorics::generate_combinations,
    effects::{CardEffect, TurnEffect},
    hooks::{can_evolve_into, contains_energy, get_attack_cost, get_retreat_cost, get_stage},
    models::{Attack, Card, EnergyType, StatusCondition, TrainerType},
    State,
};

use super::{
    apply_action_helpers::{Mutation, Mutations},
    mutations::{
        active_damage_doutcome, active_damage_effect_doutcome, active_damage_effect_mutation,
        active_damage_mutation, build_status_effect, damage_effect_doutcome,
    },
    outcomes::{CoinSeq, Outcomes},
    shared_mutations::{
        pokemon_search_outcomes, pokemon_search_outcomes_by_type, search_and_bench_basic,
        search_and_bench_by_name, supporter_search_outcomes,
    },
    SimpleAction,
};

// This is a reducer of all actions relating to attacks.
pub(crate) fn forecast_attack(acting_player: usize, state: &State, index: usize) -> Outcomes {
    let active = state.get_active(acting_player);
    let attack = active.card.get_attacks()[index].clone();
    trace!("Forecasting attack: {active:?} {attack:?}");

    let base_outcomes = forecast_attack_inner(state, &active.card, &attack, index);

    apply_attack_common_modifiers(acting_player, state, &attack, base_outcomes)
}

pub(crate) fn forecast_copied_attack(
    acting_player: usize,
    state: &State,
    source_player: usize,
    source_in_play_idx: usize,
    attack_index: usize,
    require_attacker_energy_match: bool,
) -> Outcomes {
    let source = state.in_play_pokemon[source_player][source_in_play_idx]
        .as_ref()
        .expect("Copied-attack source Pokemon should exist");
    let attack = source
        .card
        .get_attacks()
        .get(attack_index)
        .cloned()
        .unwrap_or_else(|| {
            panic!(
                "Copied attack index {} should exist for source {}:{}",
                attack_index, source_player, source_in_play_idx
            )
        });

    if require_attacker_energy_match {
        let active = state.get_active(acting_player);
        let modified_cost = get_attack_cost(&attack.energy_required, state, acting_player);
        if !contains_energy(active, &modified_cost, state, acting_player) {
            return Outcomes::single_fn(|_, _, _| {});
        }
    }

    let base_outcomes = forecast_attack_inner(state, &source.card, &attack, attack_index);

    apply_copied_attack_modifiers(acting_player, state, &attack, base_outcomes)
}

fn apply_attack_common_modifiers(
    acting_player: usize,
    state: &State,
    attack: &Attack,
    base_outcomes: Outcomes,
) -> Outcomes {
    let active = state.get_active(acting_player);
    let has_block_effect = active
        .get_active_effects()
        .iter()
        .any(|effect| matches!(effect, CardEffect::CoinFlipToBlockAttack));

    let mut outcomes = base_outcomes;

    // Handle confusion: 50% chance the attack fails (coin flip)
    if active.is_confused() {
        outcomes = apply_confusion_coin_flip(outcomes);
    }

    // Handle CoinFlipToBlockAttack: 50% chance attack is blocked
    if has_block_effect {
        outcomes = apply_block_attack_coin_flip(outcomes);
    }

    apply_defender_damage_prevention_if_needed(acting_player, state, attack, outcomes)
}

fn apply_copied_attack_modifiers(
    acting_player: usize,
    state: &State,
    attack: &Attack,
    base_outcomes: Outcomes,
) -> Outcomes {
    apply_defender_damage_prevention_if_needed(acting_player, state, attack, base_outcomes)
}

fn apply_defender_damage_prevention_if_needed(
    acting_player: usize,
    state: &State,
    attack: &Attack,
    outcomes: Outcomes,
) -> Outcomes {
    // Check if DEFENDING Pokemon has CoinFlipToPreventDamage ability (only if attack does damage)
    if attack.fixed_damage > 0 {
        let opponent = (acting_player + 1) % 2;
        let defender = state.get_active(opponent);
        let defender_has_coin_flip_prevent = defender
            .card
            .get_ability()
            .and_then(|a| ability_mechanic_from_effect(&a.effect))
            .map(|m| matches!(m, AbilityMechanic::CoinFlipToPreventDamage))
            .unwrap_or(false);

        if defender_has_coin_flip_prevent {
            return apply_defender_coin_flip_prevent_damage(outcomes);
        }
    }

    outcomes
}

fn forecast_attack_inner(state: &State, card: &Card, attack: &Attack, _index: usize) -> Outcomes {
    let Some(effect_text) = &attack.effect else {
        return active_damage_doutcome(attack.fixed_damage);
    };
    let mechanic = EFFECT_MECHANIC_MAP.get(&effect_text[..]);
    let Some(mechanic) = mechanic else {
        panic!(
            "No implementation found for attack effect: {:?} on attack {:?} of Pokemon {:?}",
            effect_text, attack, card
        );
    };
    forecast_effect_attack_by_mechanic(state, attack, mechanic)
}

/// Applies confusion coin flip: 50% chance the attack fails (does nothing)
fn apply_confusion_coin_flip(base_outcomes: Outcomes) -> Outcomes {
    prepend_coin_gate(base_outcomes, Box::new(|_, _, _| {}))
}

/// Applies CoinFlipToBlockAttack effect: 50% chance the attack is blocked (tails)
fn apply_block_attack_coin_flip(base_outcomes: Outcomes) -> Outcomes {
    prepend_coin_gate(base_outcomes, Box::new(|_, _, _| {}))
}

fn prepend_coin_gate(base_outcomes: Outcomes, gate_mutation: Mutation) -> Outcomes {
    let (base_probs, base_mutations) = base_outcomes.into_branches();
    Outcomes::from_parts(
        std::iter::once(0.5)
            .chain(base_probs.into_iter().map(|p| p * 0.5))
            .collect(),
        std::iter::once(gate_mutation)
            .chain(base_mutations)
            .collect(),
    )
}

/// Applies defender's CoinFlipToPreventDamage ability: 50% chance damage is prevented (heads)
fn apply_defender_coin_flip_prevent_damage(base_outcomes: Outcomes) -> Outcomes {
    prepend_coin_gate(
        base_outcomes,
        Box::new(|_, _, _| {
            // TODO: This makes it so that the attack effect doesn't happen,
            // but in the game it does happen, its just the damage that is reduced to 0.
        }),
    )
}

// Handles attacks that have effects.
fn forecast_effect_attack_by_mechanic(
    state: &State,
    attack: &Attack,
    mechanic: &Mechanic,
) -> Outcomes {
    match mechanic {
        Mechanic::CelebiExPowerfulBloom => celebi_powerful_bloom(state),
        Mechanic::CoinFlipPerSpecificEnergyType {
            energy_type,
            include_fixed_damage,
            damage_per_heads,
        } => coin_flip_per_specific_energy_type(
            state,
            attack.fixed_damage,
            *energy_type,
            *include_fixed_damage,
            *damage_per_heads,
        ),
        Mechanic::SelfHeal { amount } => self_heal_attack(*amount, attack),
        Mechanic::HealOneYourPokemon { amount } => heal_one_your_pokemon_attack(*amount),
        Mechanic::CoinFlipSelfHeal { amount } => {
            coin_flip_self_heal_attack(attack.fixed_damage, *amount)
        }
        Mechanic::SelfChargeActive { energies } => {
            self_charge_active_from_energies(attack.fixed_damage, energies.clone())
        }
        Mechanic::CoinFlipSelfChargeActive { energies } => {
            coin_flip_self_charge_active(attack.fixed_damage, energies.clone())
        }
        Mechanic::ChargeYourTypeAnyWay { energy_type, count } => {
            charge_energy_any_way_to_type(attack.fixed_damage, *energy_type, *count)
        }
        Mechanic::ManaphyOceanicGift => manaphy_oceanic(),
        Mechanic::PalkiaExDimensionalStorm => palkia_dimensional_storm(state),
        Mechanic::MegaKangaskhanExDoublePunchingFamily => {
            mega_kangaskhan_ex_double_punching_family(attack)
        }
        Mechanic::MoltresExInfernoDance => moltres_inferno_dance(),
        Mechanic::MagikarpWaterfallEvolution => waterfall_evolution(state),
        Mechanic::MoveAllEnergyTypeToBench { energy_type } => {
            move_all_energy_type_to_bench(state, attack, *energy_type)
        }
        Mechanic::MoveFixedEnergyTypeToBench {
            energy_type,
            amount,
        } => move_fixed_energy_type_to_bench(state, attack, *energy_type, *amount),
        Mechanic::ChargeBench {
            energies,
            target_benched_type,
        } => energy_bench_attack(energies.clone(), *target_benched_type, state, attack),
        Mechanic::VaporeonHyperWhirlpool => vaporeon_hyper_whirlpool(state, attack.fixed_damage),
        Mechanic::SearchToHandByEnergy { energy_type } => {
            pokemon_search_outcomes_by_type(state, false, *energy_type)
        }
        Mechanic::SearchRandomPokemonToHand => {
            pokemon_search_outcomes(state.current_player, state, false)
        }
        Mechanic::SearchToHandSupporterCard => {
            supporter_search_outcomes(state.current_player, state)
        }
        Mechanic::SearchToBenchByName { name } => search_and_bench_by_name(state, name.clone()),
        Mechanic::SearchToBenchBasic => search_and_bench_basic(state),
        Mechanic::InflictStatusConditions {
            conditions,
            target_opponent,
        } => {
            if *target_opponent {
                damage_multiple_status_attack(conditions.clone(), attack)
            } else {
                damage_and_self_multiple_status_attack(attack.fixed_damage, conditions.clone())
            }
        }
        Mechanic::ChanceStatusAttack { condition } => {
            damage_chance_status_attack(attack.fixed_damage, *condition)
        }
        Mechanic::DamageAllOpponentPokemon { damage } => {
            damage_all_opponent_pokemon(state, *damage)
        }
        Mechanic::DiscardEnergyFromOpponentActive => {
            damage_and_discard_energy(attack.fixed_damage, 1)
        }
        Mechanic::CoinFlipDiscardEnergyFromOpponentActive => mawile_crunch(),
        Mechanic::DiscardOpponentActiveToolsBeforeDamage => {
            discard_opponent_active_tools_before_damage(attack.fixed_damage, attack.title.clone())
        }
        Mechanic::ExtraDamageIfEx { extra_damage } => {
            extra_damage_if_opponent_is_ex(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::ExtraDamageIfOpponentHasSpecialCondition { extra_damage } => unseen_claw_attack(
            state.current_player,
            state,
            *extra_damage,
            attack.fixed_damage,
        ),
        Mechanic::ExtraDamageIfSupportPlayedThisTurn { extra_damage } => {
            brave_buddies_attack(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::SelfDamage { amount } => self_damage_attack(attack.fixed_damage, *amount),
        Mechanic::CoinFlipExtraDamage { extra_damage } => {
            coinflip_extra_damage_attack(attack.fixed_damage, *extra_damage)
        }
        Mechanic::CoinFlipExtraDamageOrSelfDamage {
            extra_damage,
            self_damage,
        } => extra_or_self_damage_attack(attack.fixed_damage, *extra_damage, *self_damage),
        Mechanic::CoinFlipSelfDamage { self_damage } => {
            coinflip_self_damage_attack(attack.fixed_damage, *self_damage)
        }
        Mechanic::ExtraDamageForEachHeads {
            include_fixed_damage,
            damage_per_head,
            num_coins,
        } => damage_for_each_heads_attack(
            *include_fixed_damage,
            *damage_per_head,
            *num_coins,
            attack,
        ),
        Mechanic::DiscardSelfEnergyPerHeadsExtraDamage {
            num_coins,
            energy_type,
            damage_per_discarded_energy,
        } => discard_self_energy_per_heads_extra_damage_attack(
            attack.fixed_damage,
            *num_coins,
            *energy_type,
            *damage_per_discarded_energy,
        ),
        Mechanic::CoinFlipNoEffect => coinflip_no_effect(attack.fixed_damage),
        Mechanic::SelfDiscardEnergy { energies } => {
            self_energy_discard_attack(attack.fixed_damage, energies.clone())
        }
        Mechanic::SelfDiscardEnergyAndInflictStatus {
            energies,
            conditions,
        } => self_discard_energy_and_inflict_status(
            attack.fixed_damage,
            energies.clone(),
            conditions.clone(),
        ),
        Mechanic::SelfDiscardEnergyAndCardEffect {
            energies,
            effect,
            duration,
        } => self_discard_energy_and_card_effect(
            attack.fixed_damage,
            energies.clone(),
            effect.clone(),
            *duration,
        ),
        Mechanic::ExtraDamageIfExtraEnergy {
            required_extra_energy,
            extra_damage,
        } => extra_energy_attack(state, attack, required_extra_energy.clone(), *extra_damage),
        Mechanic::ExtraDamageIfTypeEnergyInPlay {
            energy_type,
            minimum_count,
            extra_damage,
        } => extra_damage_if_type_energy_in_play_attack(
            state,
            attack.fixed_damage,
            *energy_type,
            *minimum_count,
            *extra_damage,
        ),
        Mechanic::ExtraDamageIfStadiumInPlay { extra_damage } => {
            extra_damage_if_stadium_in_play(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::ExtraDamageIfBothHeads { extra_damage } => {
            both_heads_bonus_damage_attack(attack.fixed_damage, *extra_damage)
        }
        Mechanic::DirectDamage { damage, bench_only } => direct_damage(*damage, *bench_only),
        Mechanic::DamageAndTurnEffect { effect, duration } => {
            damage_and_turn_effect_attack(attack.fixed_damage, effect.clone(), *duration)
        }
        Mechanic::DamageAndCardEffect {
            opponent,
            effect,
            duration,
            coin_flip,
        } => damage_and_card_effect_attack(
            attack.fixed_damage,
            *opponent,
            effect.clone(),
            *duration,
            *coin_flip,
        ),
        Mechanic::CoinFlipNoDamageOrDamageAndCardEffect {
            opponent,
            effect,
            duration,
        } => coin_flip_no_damage_or_damage_and_card_effect_attack(
            attack.fixed_damage,
            *opponent,
            effect.clone(),
            *duration,
        ),
        Mechanic::DrawCard { amount } => draw_and_damage_outcome(attack.fixed_damage, *amount),
        Mechanic::SelfDiscardAllEnergy => damage_and_discard_all_energy(attack.fixed_damage),
        Mechanic::SelfDiscardAllTypeEnergy { energy_type } => {
            discard_all_energy_of_type_attack(attack.fixed_damage, *energy_type)
        }
        Mechanic::SelfDiscardAllTypeEnergyAndDamageAnyOpponentPokemon {
            energy_type,
            damage,
        } => discard_all_energy_of_type_then_damage_any_opponent_pokemon(*energy_type, *damage),
        Mechanic::SelfDiscardRandomEnergy => damage_and_discard_random_energy(attack.fixed_damage),
        Mechanic::AlsoBenchDamage {
            opponent,
            damage,
            must_have_energy,
        } => also_bench_damage(
            state,
            *opponent,
            attack.fixed_damage,
            *damage,
            *must_have_energy,
        ),
        Mechanic::AlsoChoiceBenchDamage { opponent, damage } => {
            also_choice_bench_damage(state, *opponent, attack.fixed_damage, *damage)
        }
        Mechanic::ExtraDamageIfHurt {
            extra_damage,
            opponent,
        } => extra_damage_if_hurt(state, attack.fixed_damage, *extra_damage, *opponent),
        Mechanic::ExtraDamageIfUndamaged { extra_damage } => {
            extra_damage_if_undamaged(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::DamageEqualToSelfDamage => damage_equal_to_self_damage(state),
        Mechanic::ExtraDamageEqualToSelfDamage => {
            extra_damage_equal_to_self_damage(state, attack.fixed_damage)
        }
        Mechanic::BenchCountDamage {
            include_fixed_damage,
            damage_per,
            energy_type,
            bench_side,
        } => bench_count_damage_attack(
            state,
            attack.fixed_damage,
            *include_fixed_damage,
            *damage_per,
            *energy_type,
            bench_side,
        ),
        Mechanic::EvolutionBenchCountDamage {
            include_fixed_damage,
            damage_per,
        } => evolution_bench_count_damage_attack(
            state,
            attack.fixed_damage,
            *include_fixed_damage,
            *damage_per,
        ),
        Mechanic::ExtraDamagePerEnergy {
            opponent,
            damage_per_energy,
        } => extra_damage_per_energy(state, attack.fixed_damage, *opponent, *damage_per_energy),
        Mechanic::ExtraDamagePerRetreatCost { damage_per_energy } => {
            extra_damage_per_retreat_cost(state, attack.fixed_damage, *damage_per_energy)
        }
        Mechanic::DamagePerEnergyAll {
            opponent,
            damage_per_energy,
        } => damage_per_energy_all(state, *opponent, *damage_per_energy),
        Mechanic::DiscardHandCards { count } => {
            discard_hand_cards_required_attack(state, attack.fixed_damage, *count)
        }
        Mechanic::ExtraDamagePerSpecificEnergy {
            energy_type,
            damage_per_energy,
        } => extra_damage_per_specific_energy(
            state,
            attack.fixed_damage,
            *energy_type,
            *damage_per_energy,
        ),
        Mechanic::ExtraDamageIfToolAttached { extra_damage } => {
            extra_damage_if_tool_attached(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::DiscardRandomGlobalEnergy { count } => {
            discard_random_global_energy_attack(attack.fixed_damage, *count, state)
        }
        Mechanic::RandomDamageToOpponentPokemonPerSelfEnergy {
            energy_type,
            damage_per_hit,
        } => {
            random_damage_to_opponent_pokemon_per_self_energy(state, *energy_type, *damage_per_hit)
        }
        Mechanic::RandomSpreadDamage {
            times,
            damage_per_hit,
            include_own_bench,
        } => random_spread_damage(state, *times, *damage_per_hit, *include_own_bench),
        Mechanic::ExtraDamageIfKnockedOutLastTurn { extra_damage } => {
            extra_damage_if_knocked_out_last_turn_attack(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::ExtraDamageIfMovedFromBench { extra_damage } => {
            extra_damage_if_moved_from_bench_attack(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::ExtraDamageIfEvolvedThisTurn { extra_damage } => {
            extra_damage_if_evolved_this_turn_attack(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::RecoilIfKo { self_damage } => {
            recoil_if_ko_attack(attack.fixed_damage, *self_damage)
        }
        Mechanic::ShuffleOpponentActiveIntoDeck => shuffle_opponent_active_into_deck(),
        Mechanic::KnockBackOpponentActive => knock_back_attack(attack.fixed_damage),
        Mechanic::FlipUntilTailsDamage { damage_per_heads } => {
            flip_until_tails_attack(*damage_per_heads)
        }
        Mechanic::DirectDamageIfDamaged { damage } => direct_damage_if_damaged(*damage),
        Mechanic::AttachEnergyToBenchedBasic { energy_type } => {
            attach_energy_to_benched_basic(state.current_player, *energy_type)
        }
        Mechanic::DamageAndDiscardOpponentDeck {
            damage,
            discard_count,
        } => damage_and_discard_opponent_deck(*damage, *discard_count),
        Mechanic::MegaAmpharosExLightningLancer => mega_ampharos_lightning_lancer(),
        Mechanic::OminousClaw => ominous_claw_attack(state.current_player, attack.fixed_damage),
        Mechanic::DarknessClaw => darkness_claw_attack(state.current_player, attack.fixed_damage),
        Mechanic::BlockBasicAttack => block_basic_attack(attack.fixed_damage),
        Mechanic::SwitchSelfWithBench => switch_self_with_bench(state, attack.fixed_damage),
        Mechanic::SelfHealIfStadiumInPlay { amount } => {
            self_heal_if_stadium_in_play(state, attack.fixed_damage, *amount)
        }
        Mechanic::InflictStatusIfStadiumInPlay { status } => {
            inflict_status_if_stadium_in_play(state, attack.fixed_damage, *status)
        }
        Mechanic::ConditionalBenchDamage {
            required_extra_energy,
            bench_damage,
            num_bench_targets,
            opponent,
        } => conditional_bench_damage_attack(
            state,
            attack,
            required_extra_energy.clone(),
            *bench_damage,
            *num_bench_targets,
            *opponent,
        ),
        Mechanic::ExtraDamageForEachHeadsWithStatus {
            include_fixed_damage,
            damage_per_head,
            num_coins,
            status,
        } => damage_for_each_heads_with_status_attack(
            *include_fixed_damage,
            *damage_per_head,
            *num_coins,
            attack,
            *status,
        ),
        Mechanic::DamageAndMultipleCardEffects {
            opponent,
            effects,
            duration,
        } => damage_and_multiple_card_effects_attack(
            attack.fixed_damage,
            *opponent,
            effects.clone(),
            *duration,
        ),
        Mechanic::DamageReducedBySelfDamage => damage_reduced_by_self_damage_attack(state, attack),
        Mechanic::ExtraDamagePerTrainerInOpponentDeck { damage_per_trainer } => {
            extra_damage_per_trainer_in_opponent_deck_attack(
                state,
                attack.fixed_damage,
                *damage_per_trainer,
            )
        }
        Mechanic::ExtraDamagePerSupporterInDiscard {
            damage_per_supporter,
        } => extra_damage_per_supporter_in_discard_attack(
            state,
            attack.fixed_damage,
            *damage_per_supporter,
        ),
        Mechanic::ExtraDamagePerOwnPoint { damage_per_point } => {
            extra_damage_per_own_point_attack(state, attack.fixed_damage, *damage_per_point)
        }
        Mechanic::ExtraDamageIfCardInDiscard {
            card_name,
            extra_damage,
        } => extra_damage_if_card_in_discard_attack(
            state,
            attack.fixed_damage,
            card_name.clone(),
            *extra_damage,
        ),
        Mechanic::CoinFlipToBlockAttackNextTurn => {
            coin_flip_to_block_attack_next_turn(attack.fixed_damage)
        }
        Mechanic::DelayedSpotDamage { amount } => delayed_spot_damage(*amount),
        Mechanic::CopyAttack {
            source,
            require_attacker_energy_match,
        } => copy_attack(state, source, *require_attacker_energy_match),
        Mechanic::SelfAsleepAndHeal { amount } => {
            self_asleep_and_heal_attack(*amount, attack.fixed_damage)
        }
        Mechanic::FlipCoinsBenchDamagePerHead {
            num_coins,
            bench_damage_per_head,
        } => flip_coins_bench_damage_per_head(
            state,
            attack.fixed_damage,
            *num_coins,
            *bench_damage_per_head,
        ),
        Mechanic::ExtraDamageIfSelfHpAtMost {
            threshold,
            extra_damage,
        } => extra_damage_if_self_hp_at_most(state, attack.fixed_damage, *threshold, *extra_damage),
        Mechanic::ExtraDamageIfOpponentHpMoreThanSelf { extra_damage } => {
            extra_damage_if_opponent_hp_more_than_self(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::CoinFlipShuffleRandomOpponentHandCardIntoDeck => {
            coin_flip_shuffle_random_opponent_hand_card_into_deck()
        }
        Mechanic::ExtraDamageIfCombinedActiveEnergyAtLeast {
            threshold,
            extra_damage,
        } => extra_damage_if_combined_active_energy_at_least(
            state,
            attack.fixed_damage,
            *threshold,
            *extra_damage,
        ),
        Mechanic::CoinFlipChargeBench {
            energies,
            target_benched_type,
        } => coin_flip_charge_bench(
            state,
            attack.fixed_damage,
            energies.clone(),
            *target_benched_type,
        ),
        Mechanic::CoinFlipAlsoChoiceBenchDamage { opponent, damage } => {
            coin_flip_also_choice_bench_damage(state, *opponent, attack.fixed_damage, *damage)
        }
        Mechanic::ExtraDamageIfDefenderPoisoned { extra_damage } => {
            extra_damage_if_defender_poisoned(state, attack.fixed_damage, *extra_damage)
        }
    }
}

fn copy_attack(
    _state: &State,
    source: &CopyAttackSource,
    require_attacker_energy_match: bool,
) -> Outcomes {
    let source = source.clone();
    active_damage_effect_doutcome(0, move |_, state, action| {
        let choices =
            copied_attack_choices(state, action.actor, &source, require_attacker_energy_match);
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn copied_attack_choices(
    state: &State,
    acting_player: usize,
    source: &CopyAttackSource,
    require_attacker_energy_match: bool,
) -> Vec<SimpleAction> {
    let opponent = (acting_player + 1) % 2;
    match source {
        CopyAttackSource::OpponentActive => copied_attack_choices_from_slots(
            state,
            opponent,
            std::iter::once(0),
            require_attacker_energy_match,
        ),
        CopyAttackSource::OpponentInPlay => copied_attack_choices_from_slots(
            state,
            opponent,
            state
                .enumerate_in_play_pokemon(opponent)
                .map(|(idx, _)| idx),
            require_attacker_energy_match,
        ),
        CopyAttackSource::OwnBenchNonEx => copied_attack_choices_from_slots(
            state,
            acting_player,
            state
                .enumerate_bench_pokemon(acting_player)
                .filter(|(_, pokemon)| !pokemon.card.is_ex())
                .map(|(idx, _)| idx),
            require_attacker_energy_match,
        ),
    }
}

fn copied_attack_choices_from_slots<I>(
    state: &State,
    source_player: usize,
    source_slots: I,
    require_attacker_energy_match: bool,
) -> Vec<SimpleAction>
where
    I: IntoIterator<Item = usize>,
{
    let mut choices = Vec::new();
    for source_in_play_idx in source_slots {
        let Some(source) = state.in_play_pokemon[source_player][source_in_play_idx].as_ref() else {
            continue;
        };
        for (attack_index, attack) in source.card.get_attacks().iter().enumerate() {
            if is_copy_attack(attack) {
                continue;
            }
            choices.push(SimpleAction::UseCopiedAttack {
                source_player,
                source_in_play_idx,
                attack_index,
                require_attacker_energy_match,
            });
        }
    }
    choices
}

fn is_copy_attack(attack: &Attack) -> bool {
    attack
        .effect
        .as_deref()
        .and_then(|effect_text| EFFECT_MECHANIC_MAP.get(effect_text))
        .is_some_and(|mechanic| matches!(mechanic, Mechanic::CopyAttack { .. }))
}

fn recoil_if_ko_attack(damage: u32, self_damage: u32) -> Outcomes {
    Outcomes::single(Box::new(move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let attacking_ref = (action.actor, 0);

        // Resolve attack damage and any immediate counter-damage before knockout handling.
        handle_damage_only(state, attacking_ref, &[(damage, opponent, 0)], true, None);

        // Check knockout status before any discard/promotion resolution happens.
        let opponent_ko = state.in_play_pokemon[opponent][0]
            .as_ref()
            .is_some_and(|p| p.is_knocked_out());

        // If the attack knocked out the opponent, Head Smash deals recoil to the attacker.
        if opponent_ko {
            let attacker = state.in_play_pokemon[action.actor][0]
                .as_mut()
                .expect("Attacker should still be present before knockout resolution");
            attacker.apply_damage(self_damage);
        }

        handle_knockouts(state, attacking_ref, true);
    }))
}

fn coinflip_no_effect(fixed_damage: u32) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_mutation(fixed_damage),
        active_damage_mutation(0),
    )
}

fn coinflip_extra_damage_attack(base_damage: u32, extra_damage: u32) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_mutation(base_damage + extra_damage),
        active_damage_mutation(base_damage),
    )
}

/// Used for attacks that deal damage and damage themselves only on tails.
fn coinflip_self_damage_attack(base_damage: u32, self_damage: u32) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_mutation(base_damage),
        active_damage_effect_mutation(base_damage, move |_, state, action| {
            let active = state.get_active_mut(action.actor);
            active.apply_damage(self_damage);
        }),
    )
}

fn discard_self_energy_per_heads_extra_damage_attack(
    base_damage: u32,
    num_coins: usize,
    energy_type: EnergyType,
    damage_per_discarded_energy: u32,
) -> Outcomes {
    Outcomes::binomial_by_heads(num_coins, move |heads| {
        Box::new(move |_, state, action| {
            let requested = vec![energy_type; heads];
            let actual = {
                let active = state.get_active(action.actor);
                available_requested_energy_to_discard(active, &requested)
            };
            let damage = base_damage + (actual.len() as u32 * damage_per_discarded_energy);
            let opponent = (action.actor + 1) % 2;
            let attacking_ref = (action.actor, 0);

            if !actual.is_empty() {
                state.discard_from_active(action.actor, &actual);
            }

            handle_damage_only(state, attacking_ref, &[(damage, opponent, 0)], true, None);
            handle_knockouts(state, attacking_ref, true);
        })
    })
}

fn both_heads_bonus_damage_attack(base_damage: u32, extra_damage: u32) -> Outcomes {
    Outcomes::from_coin_branches(vec![
        (
            0.25,
            active_damage_mutation(base_damage + extra_damage),
            vec![CoinSeq(vec![true, true])],
        ),
        (
            0.75,
            active_damage_mutation(base_damage),
            vec![
                CoinSeq(vec![true, false]),
                CoinSeq(vec![false, true]),
                CoinSeq(vec![false, false]),
            ],
        ),
    ])
    .expect("ExtraDamageIfBothHeads branches should be valid")
}

fn celebi_powerful_bloom(state: &State) -> Outcomes {
    let active_pokemon = state.get_active(state.current_player);
    let total_energy = active_pokemon.attached_energy.len();

    if total_energy == 0 {
        // No energy attached, no coins to flip
        return Outcomes::single(active_damage_mutation(0));
    }

    Outcomes::binomial_by_heads(total_energy, move |heads| {
        active_damage_mutation((heads as u32) * 50)
    })
}

fn coin_flip_per_specific_energy_type(
    state: &State,
    base_damage: u32,
    energy_type: EnergyType,
    include_fixed_damage: bool,
    damage_per_heads: u32,
) -> Outcomes {
    let active_pokemon = state.get_active(state.current_player);
    let energy_count = active_pokemon
        .attached_energy
        .iter()
        .filter(|&&e| e == energy_type)
        .count();

    let base_damage = if include_fixed_damage { base_damage } else { 0 };

    Outcomes::binomial_by_heads(energy_count, move |heads| {
        active_damage_mutation(base_damage + (heads as u32) * damage_per_heads)
    })
}

fn mega_kangaskhan_ex_double_punching_family(attack: &Attack) -> Outcomes {
    active_damage_effect_doutcome(attack.fixed_damage, |_, state, action| {
        // Force Handle K.O., to maybe .insert(0 promotions to the move_generation_stack
        let attacking_ref = (action.actor, 0);
        let is_from_active_attack = true;
        handle_knockouts(state, attacking_ref, is_from_active_attack);

        // .insert(0 damage to purposely do after the K.O. promotions
        let opponent = (action.actor + 1) % 2;
        let targets = vec![(40, opponent, 0)];
        state.move_generation_stack.insert(
            0,
            (
                action.actor,
                vec![SimpleAction::ApplyDamage {
                    attacking_ref,
                    targets,
                    is_from_active_attack: true,
                }],
            ),
        )
    })
}

/// For Magikarp's Waterfall Evolution: Put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it.
fn waterfall_evolution(state: &State) -> Outcomes {
    let active_pokemon = state.get_active(state.current_player);

    // Find all cards in deck that can evolve from the active Pokemon
    let evolution_cards: Vec<Card> = state.decks[state.current_player]
        .cards
        .iter()
        .filter(|card| can_evolve_into(card, active_pokemon))
        .cloned()
        .collect();
    if evolution_cards.is_empty() {
        // No evolution cards in deck, just shuffle
        return Outcomes::single_fn(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    // Generate outcomes for each possible evolution card
    let num_evolution_cards = evolution_cards.len();
    let probabilities = vec![1.0 / (num_evolution_cards as f64); num_evolution_cards];
    let mut outcomes: Mutations = vec![];
    for evolution_card in evolution_cards {
        outcomes.push(Box::new(move |rng, state, action| {
            // Evolve the active Pokemon (position 0) using the centralized logic
            apply_evolve(action.actor, state, &evolution_card, 0, true);

            // Shuffle the deck
            state.decks[action.actor].shuffle(false, rng);
        }));
    }

    Outcomes::from_parts(probabilities, outcomes)
}

/// For Manaphy's Oceanic attack: Choose 2 benched Pokémon and attach Water Energy to each
fn manaphy_oceanic() -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let benched_pokemon: Vec<usize> = state
            .enumerate_bench_pokemon(action.actor)
            .map(|(idx, _)| idx)
            .collect();

        let mut choices = Vec::new();
        if benched_pokemon.len() == 1 {
            // Only 1 benched Pokémon, can only choose that one
            choices.push(SimpleAction::Attach {
                attachments: vec![(1, EnergyType::Water, benched_pokemon[0])],
                is_turn_energy: false,
            });
        } else if benched_pokemon.len() >= 2 {
            // 2 or more benched Pokémon: must choose exactly 2
            // Generate all combinations of choosing 2 benched Pokémon
            for i in 0..benched_pokemon.len() {
                for j in (i + 1)..benched_pokemon.len() {
                    choices.push(SimpleAction::Attach {
                        attachments: vec![
                            (1, EnergyType::Water, benched_pokemon[i]),
                            (1, EnergyType::Water, benched_pokemon[j]),
                        ],
                        is_turn_energy: false,
                    });
                }
            }
        }
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn palkia_dimensional_storm(state: &State) -> Outcomes {
    // This attack does 150 damage to Active, and 20 to every bench pokemon
    // it then also discards 3 energies. This is deterministic
    let targets: Vec<(u32, usize)> = state
        .enumerate_bench_pokemon((state.current_player + 1) % 2)
        .map(|(idx, _)| (20, idx))
        .chain(std::iter::once((150, 0))) // Add active Pokémon directly
        .collect();
    damage_effect_doutcome(targets, |_, state, action| {
        discard_requested_energy_from_active_best_effort(
            state,
            action.actor,
            &[EnergyType::Water; 3],
        );
    })
}

fn moltres_inferno_dance() -> Outcomes {
    Outcomes::binomial_by_heads(3, move |heads| {
        active_damage_effect_mutation(0, move |_, state, action| {
            if heads == 0 {
                return;
            }

            // First collect all eligible fire pokemon in bench
            let mut fire_bench_idx = Vec::new();
            for (in_play_idx, pokemon) in state.enumerate_bench_pokemon(action.actor) {
                if pokemon.get_energy_type() == Some(EnergyType::Fire) {
                    fire_bench_idx.push(in_play_idx);
                }
            }

            if fire_bench_idx.is_empty() {
                return;
            }

            let all_choices = generate_energy_distributions(&fire_bench_idx, heads);
            if !all_choices.is_empty() {
                state
                    .move_generation_stack
                    .push((action.actor, all_choices));
            }
        })
    })
}

fn charge_energy_any_way_to_type(damage: u32, energy_type: EnergyType, count: usize) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let target_indices = collect_in_play_indices_by_type(state, action.actor, energy_type);
        let choices = energy_any_way_choices(&target_indices, energy_type, count);
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn move_all_energy_type_to_bench(
    state: &State,
    attack: &Attack,
    energy_type: EnergyType,
) -> Outcomes {
    // Count how many of the specified energy type the active Pokemon has
    let active = state.get_active(state.current_player);
    let energy_count = active
        .attached_energy
        .iter()
        .filter(|&&e| e == energy_type)
        .count();

    if energy_count == 0 {
        // No energy of this type, just do damage
        return active_damage_doutcome(attack.fixed_damage);
    }

    // Generate move actions for each benched Pokemon
    let bench_pokemon: Vec<usize> = state
        .enumerate_bench_pokemon(state.current_player)
        .map(|(idx, _)| idx)
        .collect();

    if bench_pokemon.is_empty() {
        // No bench Pokemon, can't move energy, just do damage
        return active_damage_doutcome(attack.fixed_damage);
    }

    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        // Collect bench Pokemon
        let bench_pokemon: Vec<usize> = state
            .enumerate_bench_pokemon(action.actor)
            .map(|(idx, _)| idx)
            .collect();

        if bench_pokemon.is_empty() {
            return; // No bench Pokemon
        }

        // Count how many energies of this type are on the active Pokemon
        let active = &state.in_play_pokemon[action.actor][0]
            .as_ref()
            .expect("Active should be there");
        let energy_count = active
            .attached_energy
            .iter()
            .filter(|&&e| e == energy_type)
            .count() as u32;

        if energy_count > 0 {
            // Create one bulk MoveEnergy action per bench Pokemon
            let choices: Vec<SimpleAction> = bench_pokemon
                .iter()
                .map(|&to_idx| SimpleAction::MoveEnergy {
                    from_in_play_idx: 0,
                    to_in_play_idx: to_idx,
                    energy_type,
                    amount: energy_count,
                })
                .collect();
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn move_fixed_energy_type_to_bench(
    state: &State,
    attack: &Attack,
    energy_type: EnergyType,
    amount: u32,
) -> Outcomes {
    let active = state.get_active(state.current_player);
    let energy_count = active
        .attached_energy
        .iter()
        .filter(|&&e| e == energy_type)
        .count() as u32;

    if energy_count < amount
        || state
            .enumerate_bench_pokemon(state.current_player)
            .next()
            .is_none()
    {
        return active_damage_doutcome(attack.fixed_damage);
    }

    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        let active = state.in_play_pokemon[action.actor][0]
            .as_ref()
            .expect("Active should be there");
        let energy_count = active
            .attached_energy
            .iter()
            .filter(|&&e| e == energy_type)
            .count() as u32;

        if energy_count < amount {
            return;
        }

        let choices: Vec<SimpleAction> = state
            .enumerate_bench_pokemon(action.actor)
            .map(|(to_idx, _)| SimpleAction::MoveEnergy {
                from_in_play_idx: 0,
                to_in_play_idx: to_idx,
                energy_type,
                amount,
            })
            .collect();

        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn generate_energy_distributions(fire_bench_idx: &[usize], heads: usize) -> Vec<SimpleAction> {
    let mut all_choices = Vec::new();

    // Generate all possible ways to distribute the energy
    let mut distributions = Vec::new();
    generate_distributions(
        fire_bench_idx,
        heads,
        0,
        &mut vec![0; fire_bench_idx.len()],
        &mut distributions,
    );

    // Convert each distribution into an Attach action
    for dist in distributions {
        let mut attachments = Vec::new();
        for (i, &pokemon_idx) in fire_bench_idx.iter().enumerate() {
            if dist[i] > 0 {
                attachments.push((dist[i] as u32, EnergyType::Fire, pokemon_idx));
            }
        }
        all_choices.push(SimpleAction::Attach {
            attachments,
            is_turn_energy: false,
        });
    }

    all_choices
}

fn damage_for_each_heads_attack(
    include_fixed_damage: bool,
    damage_per_head: u32,
    num_coins: usize,
    attack: &Attack,
) -> Outcomes {
    let fixed_damage = if include_fixed_damage {
        attack.fixed_damage
    } else {
        0
    };
    Outcomes::binomial_by_heads(num_coins, move |heads_count| {
        active_damage_mutation(fixed_damage + damage_per_head * heads_count as u32)
    })
}

/// Deal damage and attach energy to a pokemon of choice in the bench.
pub(crate) fn energy_bench_attack(
    energies: Vec<EnergyType>,
    target_benched_type: Option<EnergyType>,
    state: &State,
    attack: &Attack,
) -> Outcomes {
    let choices = state
        .enumerate_bench_pokemon(state.current_player)
        .filter(|(_, played_card)| {
            target_benched_type.is_none() || played_card.get_energy_type() == target_benched_type
        })
        .map(|(in_play_idx, _)| SimpleAction::Attach {
            attachments: energies
                .iter()
                .map(|&energy| (1, energy, in_play_idx))
                .collect(),
            is_turn_energy: false,
        })
        .collect::<Vec<_>>();
    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        if choices.is_empty() {
            return; // do nothing, since we use common_attack_mutation, turn should end, and no damage applied.
        }
        state
            .move_generation_stack
            .push((action.actor, choices.clone()));
    })
}

/// Used for attacks that on heads deal extra damage, on tails deal self damage.
fn extra_or_self_damage_attack(base_damage: u32, extra_damage: u32, self_damage: u32) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_mutation(base_damage + extra_damage),
        active_damage_effect_mutation(base_damage, move |_, state, action| {
            let active = state.get_active_mut(action.actor);
            active.apply_damage(self_damage);
        }),
    )
}

fn damage_chance_status_attack(damage: u32, status: StatusCondition) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_effect_mutation(damage, build_status_effect(status)),
        active_damage_mutation(damage),
    )
}

/// For attacks that do damage based on benched Pokemon count (new Mechanic-based approach).
fn bench_count_damage_attack(
    state: &State,
    base_damage: u32,
    include_base_damage: bool,
    damage_per: u32,
    energy_type: Option<EnergyType>,
    bench_side: &BenchSide,
) -> Outcomes {
    let current_player = state.current_player;
    let opponent = (current_player + 1) % 2;

    let players = match bench_side {
        BenchSide::YourBench => vec![current_player],
        BenchSide::OpponentBench => vec![opponent],
        BenchSide::BothBenches => vec![current_player, opponent],
    };

    let bench_count = players
        .iter()
        .flat_map(|&player| state.enumerate_bench_pokemon(player))
        .filter(|(_, pokemon)| {
            energy_type.is_none_or(|energy| pokemon.get_energy_type() == Some(energy))
        })
        .count() as u32;

    let total_damage = if include_base_damage {
        base_damage + damage_per * bench_count
    } else {
        damage_per * bench_count
    };
    active_damage_doutcome(total_damage)
}

fn evolution_bench_count_damage_attack(
    state: &State,
    base_damage: u32,
    include_base_damage: bool,
    damage_per: u32,
) -> Outcomes {
    let current_player = state.current_player;
    let evolution_count = state
        .enumerate_bench_pokemon(current_player)
        .filter(|(_, pokemon)| {
            if let Card::Pokemon(pokemon_card) = &pokemon.card {
                pokemon_card.stage > 0
            } else {
                false
            }
        })
        .count() as u32;

    let total_damage = if include_base_damage {
        base_damage + damage_per * evolution_count
    } else {
        damage_per * evolution_count
    };
    active_damage_doutcome(total_damage)
}

fn also_choice_bench_damage(
    state: &State,
    opponent: bool,
    active_damage: u32,
    bench_damage: u32,
) -> Outcomes {
    let opponent_player = (state.current_player + 1) % 2;
    let bench_target = if opponent {
        opponent_player
    } else {
        state.current_player
    };
    let choices: Vec<_> = state
        .enumerate_bench_pokemon(bench_target)
        .map(|(in_play_idx, _)| {
            let targets = vec![
                (active_damage, opponent_player, 0),
                (bench_damage, bench_target, in_play_idx),
            ];
            SimpleAction::ApplyDamage {
                attacking_ref: (state.current_player, 0),
                targets,
                is_from_active_attack: true,
            }
        })
        .collect();
    Outcomes::single(Box::new(
        move |_: &mut StdRng, state: &mut State, action: &Action| {
            if !choices.is_empty() {
                state.move_generation_stack.push((action.actor, choices));
            }
        },
    ))
}

fn self_charge_active_from_energies(damage: u32, energies: Vec<EnergyType>) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        for energy in &energies {
            if state.in_play_pokemon[action.actor][0].is_none() {
                continue; // probably K.O.d from Jolteon Ex in first loop
            }

            state.attach_energy_from_zone(action.actor, 0, *energy, 1, false);
        }
    })
}

fn coin_flip_self_charge_active(damage: u32, energies: Vec<EnergyType>) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_effect_mutation(damage, move |_, state, action| {
            for energy in &energies {
                if state.in_play_pokemon[action.actor][0].is_none() {
                    continue;
                }

                state.attach_energy_from_zone(action.actor, 0, *energy, 1, false);
            }
        }),
        active_damage_mutation(damage),
    )
}

/// Used for attacks that can go directly to bench.
/// It will queue (via move_generation_stack) for the user to choose a pokemon to damage.
fn direct_damage(damage: u32, bench_only: bool) -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        if bench_only {
            for (in_play_idx, _) in state.enumerate_bench_pokemon(opponent) {
                choices.push(SimpleAction::ApplyDamage {
                    attacking_ref: (action.actor, 0),
                    targets: vec![(damage, opponent, in_play_idx)],
                    is_from_active_attack: true,
                });
            }
        } else {
            for (in_play_idx, _) in state.enumerate_in_play_pokemon(opponent) {
                choices.push(SimpleAction::ApplyDamage {
                    attacking_ref: (action.actor, 0),
                    targets: vec![(damage, opponent, in_play_idx)],
                    is_from_active_attack: true,
                });
            }
        }
        if choices.is_empty() {
            return; // do nothing, since we use common_attack_mutation, turn should end, and no damage applied.
        }
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn delayed_spot_damage(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_in_play_pokemon(opponent) {
            choices.push(SimpleAction::ScheduleDelayedSpotDamage {
                target_player: opponent,
                target_in_play_idx: in_play_idx,
                amount: damage,
            });
        }
        if choices.is_empty() {
            return;
        }
        state.move_generation_stack.push((action.actor, choices));
    })
}

/// For attacks that can target opponent's Pokémon that have damage on them.
/// e.g. Decidueye ex's Pierce the Pain
fn direct_damage_if_damaged(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, pokemon) in state.enumerate_in_play_pokemon(opponent) {
            // Only add as a target if the Pokémon has damage (remaining_hp < total_hp)
            if pokemon.is_damaged() {
                choices.push(SimpleAction::ApplyDamage {
                    attacking_ref: (action.actor, 0),
                    targets: vec![(damage, opponent, in_play_idx)],
                    is_from_active_attack: true,
                });
            }
        }
        if choices.is_empty() {
            return; // No valid targets - no damage applied
        }
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn discard_all_energy_of_type_then_damage_any_opponent_pokemon(
    energy_type: EnergyType,
    damage: u32,
) -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, action| {
        // Count and discard all matching energy from the attacking Pokémon.
        let active = state.get_active(action.actor);
        let matching_count = active
            .attached_energy
            .iter()
            .filter(|e| **e == energy_type)
            .count();
        let to_discard = vec![energy_type; matching_count];
        state.discard_from_active(action.actor, &to_discard);

        // Create choices for which opponent's Pokémon to damage
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_in_play_pokemon(opponent) {
            choices.push(SimpleAction::ApplyDamage {
                attacking_ref: (action.actor, 0),
                targets: vec![(damage, opponent, in_play_idx)],
                is_from_active_attack: true,
            });
        }
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn available_requested_energy_to_discard(
    active: &crate::models::PlayedCard,
    requested: &[EnergyType],
) -> Vec<EnergyType> {
    let mut remaining = active.attached_energy.clone();
    let mut actual = Vec::new();
    for energy in requested {
        if let Some(pos) = remaining.iter().position(|e| *e == *energy) {
            remaining.swap_remove(pos);
            actual.push(*energy);
        }
    }
    actual
}

fn discard_requested_energy_from_active_best_effort(
    state: &mut State,
    actor: usize,
    requested: &[EnergyType],
) {
    let actual = {
        let active = state.get_active(actor);
        available_requested_energy_to_discard(active, requested)
    };
    if !actual.is_empty() {
        state.discard_from_active(actor, &actual);
    }
}

/// Discard energy from the active (attacking) Pokémon.
fn self_energy_discard_attack(fixed_damage: u32, to_discard: Vec<EnergyType>) -> Outcomes {
    active_damage_effect_doutcome(fixed_damage, move |_, state, action| {
        discard_requested_energy_from_active_best_effort(state, action.actor, &to_discard);
    })
}

fn self_discard_energy_and_inflict_status(
    fixed_damage: u32,
    to_discard: Vec<EnergyType>,
    conditions: Vec<StatusCondition>,
) -> Outcomes {
    active_damage_effect_doutcome(fixed_damage, move |_, state, action| {
        discard_requested_energy_from_active_best_effort(state, action.actor, &to_discard);

        let opponent = (action.actor + 1) % 2;
        for condition in &conditions {
            state.apply_status_condition(opponent, 0, *condition);
        }
    })
}

fn self_discard_energy_and_card_effect(
    fixed_damage: u32,
    to_discard: Vec<EnergyType>,
    effect: CardEffect,
    duration: u8,
) -> Outcomes {
    active_damage_effect_doutcome(fixed_damage, move |_, state, action| {
        discard_requested_energy_from_active_best_effort(state, action.actor, &to_discard);
        state
            .get_active_mut(action.actor)
            .add_effect(effect.clone(), duration);
    })
}

/// For attacks that deal damage and discard random energy from opponent's active Pokémon
fn damage_and_discard_energy(damage: u32, discard_count: usize) -> Outcomes {
    active_damage_effect_doutcome(damage, move |rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut to_discard = Vec::new();
        let mut remaining = state.get_active(opponent).attached_energy.clone();

        for _ in 0..discard_count {
            if remaining.is_empty() {
                break; // No more energy to discard
            }

            let energy_count = remaining.len();
            let rand_idx = rng.gen_range(0..energy_count);
            to_discard.push(remaining.swap_remove(rand_idx));
        }

        if !to_discard.is_empty() {
            state.discard_from_active(opponent, &to_discard);
        }
    })
}

fn discard_opponent_active_tools_before_damage(damage: u32, attack_name: String) -> Outcomes {
    Outcomes::single(Box::new(move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        if state.in_play_pokemon[opponent][0]
            .as_ref()
            .is_some_and(|pokemon| pokemon.attached_tool.is_some())
        {
            state.discard_tool(opponent, 0);
        }

        let attacking_ref = (action.actor, 0);
        handle_damage_only(
            state,
            attacking_ref,
            &[(damage, opponent, 0)],
            true,
            Some(attack_name.as_str()),
        );
        handle_knockouts(state, attacking_ref, true);
    }))
}

/// For attacks that deal damage and discard cards from the top of opponent's deck
fn damage_and_discard_opponent_deck(damage: u32, discard_count: usize) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;

        for _ in 0..discard_count {
            if let Some(card) = state.decks[opponent].draw() {
                state.discard_piles[opponent].push(card);
            } else {
                break; // No more cards to discard
            }
        }
    })
}

fn vaporeon_hyper_whirlpool(_state: &State, damage: u32) -> Outcomes {
    // Flip coins until tails - capped at 5 heads for practicality
    Outcomes::geometric_until_tails(5, move |energies_to_remove| {
        active_damage_effect_mutation(damage, move |_, state, action| {
            let opponent = (action.actor + 1) % 2;
            let mut to_discard = Vec::new();
            let mut remaining = state.get_active(opponent).attached_energy.clone();

            // Collect energies to discard
            for _ in 0..energies_to_remove {
                if remaining.is_empty() {
                    break; // No more energy to discard
                }
                // NOTE: Using last energy instead of random selection to avoid expanding the game tree.
                // This is a simplification - the card text says "random Energy" but we always
                // remove the last one for performance reasons.
                to_discard.push(remaining.pop().expect("already checked non-empty"));
            }

            // Discard collected energies properly (moves to discard pile)
            if !to_discard.is_empty() {
                state.discard_from_active(opponent, &to_discard);
            }
        })
    })
}

/// For attacks that deal damage to opponent and also damage themselves
fn self_damage_attack(damage: u32, self_damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.apply_damage(self_damage);
    })
}

/// For attacks that deal damage and apply multiple status effects to opponent (e.g. Mega Venusaur Critical Bloom)
fn damage_multiple_status_attack(statuses: Vec<StatusCondition>, attack: &Attack) -> Outcomes {
    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        for status in &statuses {
            state.apply_status_condition(opponent, 0, *status);
        }
    })
}

/// For attacks that deal damage to opponent and apply multiple status effects to the attacker (e.g. Snorlax Collapse)
fn damage_and_self_multiple_status_attack(damage: u32, statuses: Vec<StatusCondition>) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        for status in &statuses {
            state.apply_status_condition(action.actor, 0, *status);
        }
    })
}

/// Draw cards and deal damage in the same attack.
fn draw_and_damage_outcome(damage: u32, amount: u8) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::DrawCard { amount }]));
    })
}

fn heal_one_your_pokemon_attack(amount: u32) -> Outcomes {
    Outcomes::single_fn(move |_rng, state, action| {
        let choices = state
            .enumerate_in_play_pokemon(action.actor)
            .filter(|(_, pokemon)| pokemon.is_damaged())
            .map(|(in_play_idx, _)| SimpleAction::Heal {
                in_play_idx,
                amount,
                cure_status: false,
            })
            .collect::<Vec<_>>();
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

fn coin_flip_self_heal_attack(damage: u32, heal: u32) -> Outcomes {
    Outcomes::binary_coin(
        active_damage_effect_mutation(damage, move |_, state, action| {
            state.get_active_mut(action.actor).heal(heal);
        }),
        active_damage_mutation(damage),
    )
}

/// Generic attack that deals bonus damage if the Pokémon has enough energy of a specific type attached.
/// Used by attacks like Hydro Pump, Hydro Bazooka, and Blazing Beatdown.
fn extra_energy_attack(
    state: &State,
    attack: &Attack,
    required_extra_energy: Vec<EnergyType>,
    extra_damage: u32,
) -> Outcomes {
    let pokemon = state.in_play_pokemon[state.current_player][0]
        .as_ref()
        .expect("Active Pokemon should be there if attacking");

    // Use the contains_energy hook to consider
    let cost_with_extra_energy = attack
        .energy_required
        .iter()
        .cloned()
        .chain(required_extra_energy.iter().cloned())
        .collect::<Vec<EnergyType>>();
    if contains_energy(
        pokemon,
        &cost_with_extra_energy,
        state,
        state.current_player,
    ) {
        active_damage_doutcome(attack.fixed_damage + extra_damage)
    } else {
        active_damage_doutcome(attack.fixed_damage)
    }
}

/// For attacks that flip a coin until tails, dealing damage for each heads.
/// Uses geometric distribution truncated at a reasonable number to avoid infinite outcomes.
fn flip_until_tails_attack(damage_per_heads: u32) -> Outcomes {
    // Truncate at 8 heads to keep the probability space manageable.
    Outcomes::geometric_until_tails(8, move |heads| {
        active_damage_mutation((heads as u32) * damage_per_heads)
    })
}

fn self_heal_attack(heal: u32, attack: &Attack) -> Outcomes {
    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.heal(heal);
    })
}

fn self_heal_if_stadium_in_play(state: &State, damage: u32, heal: u32) -> Outcomes {
    if state.active_stadium.is_some() {
        active_damage_effect_doutcome(damage, move |_, state, action| {
            state.get_active_mut(action.actor).heal(heal);
        })
    } else {
        active_damage_doutcome(damage)
    }
}

fn inflict_status_if_stadium_in_play(
    state: &State,
    damage: u32,
    status: StatusCondition,
) -> Outcomes {
    if state.active_stadium.is_some() {
        active_damage_effect_doutcome(damage, move |_, state, action| {
            let opponent = (action.actor + 1) % 2;
            state.apply_status_condition(opponent, 0, status);
        })
    } else {
        active_damage_doutcome(damage)
    }
}

/// For attacks that put this Pokémon to sleep and heal it (e.g. Slowpoke's Rest).
fn self_asleep_and_heal_attack(heal: u32, damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        state.apply_status_condition(action.actor, 0, StatusCondition::Asleep);
        state.get_active_mut(action.actor).heal(heal);
    })
}

/// For attacks that flip coins and deal damage per head to each of the opponent's Benched Pokémon.
/// (e.g. Mega Slowbro ex's Laundry-Go-Round)
fn flip_coins_bench_damage_per_head(
    state: &State,
    fixed_damage: u32,
    num_coins: usize,
    bench_damage_per_head: u32,
) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    let bench_indices: Vec<usize> = state
        .enumerate_bench_pokemon(opponent)
        .map(|(idx, _)| idx)
        .collect();
    Outcomes::binomial_by_heads(num_coins, move |heads| {
        let bench_dmg = heads as u32 * bench_damage_per_head;
        let bench_indices = bench_indices.clone();
        Box::new(move |_: &mut StdRng, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            let attacking_ref = (action.actor, 0);
            let mut targets = vec![(fixed_damage, opponent, 0usize)];
            if bench_dmg > 0 {
                for &idx in &bench_indices {
                    targets.push((bench_dmg, opponent, idx));
                }
            }
            handle_damage(state, attacking_ref, &targets, true, None);
        })
    })
}

fn damage_and_turn_effect_attack(damage: u32, effect: TurnEffect, effect_duration: u8) -> Outcomes {
    let effect_clone = effect.clone();
    active_damage_effect_doutcome(damage, move |_, state, _| {
        state.add_turn_effect(effect_clone.clone(), effect_duration);
    })
}

fn damage_and_card_effect_attack(
    damage: u32,
    opponent: bool,
    effect: CardEffect,
    effect_duration: u8,
    coin_flip: bool,
) -> Outcomes {
    let effect_on_target = move |_: &mut StdRng, state: &mut State, action: &Action| {
        let player = if opponent {
            (action.actor + 1) % 2
        } else {
            action.actor
        };
        state
            .get_active_mut(player)
            .add_effect(effect.clone(), effect_duration);
    };

    if coin_flip {
        Outcomes::binary_coin(
            active_damage_effect_mutation(damage, effect_on_target),
            active_damage_mutation(damage),
        )
    } else {
        active_damage_effect_doutcome(damage, effect_on_target)
    }
}

fn coin_flip_no_damage_or_damage_and_card_effect_attack(
    damage: u32,
    opponent: bool,
    effect: CardEffect,
    effect_duration: u8,
) -> Outcomes {
    let effect_on_target = move |_: &mut StdRng, state: &mut State, action: &Action| {
        let player = if opponent {
            (action.actor + 1) % 2
        } else {
            action.actor
        };
        state
            .get_active_mut(player)
            .add_effect(effect.clone(), effect_duration);
    };

    Outcomes::binary_coin(
        active_damage_effect_mutation(damage, effect_on_target),
        active_damage_mutation(0),
    )
}

/// Discard all energy from this Pokemon
fn damage_and_discard_all_energy(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.attached_energy.clear(); // Discard all energy
    })
}

fn damage_and_discard_random_energy(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |rng, state, action| {
        let active = state.get_active(action.actor);
        if !active.attached_energy.is_empty() {
            let idx = rng.gen_range(0..active.attached_energy.len());
            let energy = active.attached_energy[idx];
            state.discard_from_active(action.actor, &[energy]);
        }
    })
}

/// For attacks that discard all energy of a specific type after dealing damage.
fn discard_all_energy_of_type_attack(damage: u32, energy_type: EnergyType) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        // Collect all energy of the specified type from the active Pokémon
        let to_discard: Vec<EnergyType> = state
            .get_active(action.actor)
            .attached_energy
            .iter()
            .filter(|&&e| e == energy_type)
            .copied()
            .collect();

        // Use the state method to properly discard energies
        state.discard_from_active(action.actor, &to_discard);
    })
}

fn discard_random_global_energy_attack(
    fixed_damage: u32,
    count: usize,
    _state: &State,
) -> Outcomes {
    active_damage_effect_doutcome(fixed_damage, move |rng, state, _action| {
        for _ in 0..count {
            let mut pokemon_with_energy: Vec<(usize, usize, usize)> = Vec::new();

            // Collect all Pokémon in play (yours and opponent's) that have energy attached
            // Store (player_idx, in_play_idx, energy_count) for weighted selection
            for player_idx in 0..2 {
                for (in_play_idx, pokemon) in state.enumerate_in_play_pokemon(player_idx) {
                    let energy_count = pokemon.attached_energy.len();
                    if energy_count > 0 {
                        pokemon_with_energy.push((player_idx, in_play_idx, energy_count));
                    }
                }
            }

            if pokemon_with_energy.is_empty() {
                return; // No Pokémon with energy to discard from
            }

            // Weight selection by energy count: a Pokemon with 9 energies should be
            // hit 9x more often than one with 1 energy
            let total_energy: usize = pokemon_with_energy.iter().map(|(_, _, e)| e).sum();
            let mut roll = rng.gen_range(0..total_energy);
            let mut selected_player_idx = 0;
            let mut selected_in_play_idx = 0;
            for (player_idx, in_play_idx, energy_count) in &pokemon_with_energy {
                if roll < *energy_count {
                    selected_player_idx = *player_idx;
                    selected_in_play_idx = *in_play_idx;
                    break;
                }
                roll -= energy_count;
            }

            let pokemon = state.in_play_pokemon[selected_player_idx][selected_in_play_idx]
                .as_mut()
                .expect("Pokemon should be there");

            // Discard one random energy from the selected Pokémon
            let energy_count = pokemon.attached_energy.len();
            if energy_count > 0 {
                let rand_idx = rng.gen_range(0..energy_count);
                pokemon.attached_energy.remove(rand_idx);
            }
        }
    })
}

fn also_bench_damage(
    state: &State,
    opponent: bool,
    active_damage: u32,
    bench_damage: u32,
    must_have_energy: bool,
) -> Outcomes {
    let player = if opponent {
        (state.current_player + 1) % 2
    } else {
        state.current_player
    };
    let mut targets: Vec<(u32, usize)> = state
        .enumerate_bench_pokemon(player)
        .filter(|(_, pokemon)| {
            if must_have_energy {
                !pokemon.attached_energy.is_empty()
            } else {
                true
            }
        })
        .map(|(idx, _)| (bench_damage, idx))
        .collect();
    targets.push((active_damage, 0)); // Active Pokémon is always index 0
    damage_effect_doutcome(targets, |_, _, _| {})
}

/// Deals the same damage to all of opponent's Pokémon (active and bench) - like Spiritomb/Clawitzer
fn damage_all_opponent_pokemon(state: &State, damage: u32) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    // Collect all opponent's Pokémon (active at index 0, plus bench)
    let targets: Vec<(u32, usize)> = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(idx, _)| (damage, idx))
        .collect();
    damage_effect_doutcome(targets, |_, _, _| {})
}

fn extra_damage_if_self_hp_at_most(
    state: &State,
    base: u32,
    threshold: u32,
    extra: u32,
) -> Outcomes {
    let attacker = state.get_active(state.current_player);
    if attacker.get_remaining_hp() <= threshold {
        active_damage_doutcome(base + extra)
    } else {
        active_damage_doutcome(base)
    }
}

fn extra_damage_if_opponent_hp_more_than_self(state: &State, base: u32, extra: u32) -> Outcomes {
    let attacker = state.get_active(state.current_player);
    let opponent = state.get_active((state.current_player + 1) % 2);
    if opponent.get_remaining_hp() > attacker.get_remaining_hp() {
        active_damage_doutcome(base + extra)
    } else {
        active_damage_doutcome(base)
    }
}

fn extra_damage_if_hurt(state: &State, base: u32, extra: u32, opponent: bool) -> Outcomes {
    let target = if opponent {
        (state.current_player + 1) % 2
    } else {
        state.current_player
    };
    let target_active = state.get_active(target);
    if target_active.is_damaged() {
        active_damage_doutcome(base + extra)
    } else {
        active_damage_doutcome(base)
    }
}

fn extra_damage_if_undamaged(state: &State, base: u32, extra: u32) -> Outcomes {
    let attacker = state.get_active(state.current_player);
    if attacker.is_damaged() {
        active_damage_doutcome(base)
    } else {
        active_damage_doutcome(base + extra)
    }
}

fn damage_equal_to_self_damage(state: &State) -> Outcomes {
    let attacker = state.get_active(state.current_player);
    let damage = attacker.get_damage_counters();
    active_damage_doutcome(damage)
}

fn extra_damage_equal_to_self_damage(state: &State, base_damage: u32) -> Outcomes {
    let attacker = state.get_active(state.current_player);
    let self_damage = attacker.get_damage_counters();
    active_damage_doutcome(base_damage + self_damage)
}

fn extra_damage_per_energy(
    state: &State,
    base_damage: u32,
    opponent: bool,
    damage_per_energy: u32,
) -> Outcomes {
    let target = if opponent {
        (state.current_player + 1) % 2
    } else {
        state.current_player
    };
    let target_active = state.get_active(target);
    let damage = base_damage
        + (target_active
            .get_effective_attached_energy(state, target)
            .len() as u32)
            * damage_per_energy;
    active_damage_doutcome(damage)
}

fn extra_damage_per_retreat_cost(
    state: &State,
    base_damage: u32,
    damage_per_energy: u32,
) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    let retreat_cost = get_retreat_cost(state, opponent_active);
    let damage = base_damage + (retreat_cost.len() as u32) * damage_per_energy;
    active_damage_doutcome(damage)
}

fn damage_per_energy_all(state: &State, opponent: bool, damage_per_energy: u32) -> Outcomes {
    let target = if opponent {
        (state.current_player + 1) % 2
    } else {
        state.current_player
    };
    let total_energy: u32 = state.in_play_pokemon[target]
        .iter()
        .flatten()
        .map(|pokemon| pokemon.get_effective_attached_energy(state, target).len() as u32)
        .sum();
    let damage = total_energy * damage_per_energy;
    active_damage_doutcome(damage)
}

/// Damage per specific energy type attached to self (e.g., Genesect's Metal Blast)
fn extra_damage_per_specific_energy(
    state: &State,
    base_damage: u32,
    energy_type: EnergyType,
    damage_per_energy: u32,
) -> Outcomes {
    let active = state.get_active(state.current_player);
    let matching_energy_count = active
        .attached_energy
        .iter()
        .filter(|e| **e == energy_type)
        .count() as u32;
    let damage = base_damage + matching_energy_count * damage_per_energy;
    active_damage_doutcome(damage)
}

fn extra_damage_if_type_energy_in_play_attack(
    state: &State,
    base_damage: u32,
    energy_type: EnergyType,
    minimum_count: usize,
    extra_damage: u32,
) -> Outcomes {
    let total_in_play_type_energy: usize = state
        .enumerate_in_play_pokemon(state.current_player)
        .map(|(_, pokemon)| {
            pokemon
                .attached_energy
                .iter()
                .filter(|energy| **energy == energy_type)
                .count()
        })
        .sum();

    if total_in_play_type_energy >= minimum_count {
        active_damage_doutcome(base_damage + extra_damage)
    } else {
        active_damage_doutcome(base_damage)
    }
}

fn extra_damage_if_stadium_in_play(state: &State, base_damage: u32, extra_damage: u32) -> Outcomes {
    if state.active_stadium.is_some() {
        active_damage_doutcome(base_damage + extra_damage)
    } else {
        active_damage_doutcome(base_damage)
    }
}

fn extra_damage_if_opponent_is_ex(state: &State, base_damage: u32, extra_damage: u32) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    let damage = if opponent_active.card.is_ex() {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn extra_damage_if_tool_attached(state: &State, base_damage: u32, extra_damage: u32) -> Outcomes {
    let active = state.get_active(state.current_player);
    let damage = if active.has_tool_attached() {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn extra_damage_if_knocked_out_last_turn_attack(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> Outcomes {
    let damage = if state.knocked_out_by_opponent_attack_last_turn {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn extra_damage_if_moved_from_bench_attack(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> Outcomes {
    let moved = state.in_play_pokemon[state.current_player][0]
        .as_ref()
        .map(|p| p.moved_to_active_this_turn)
        .unwrap_or(false);
    let damage = if moved {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn extra_damage_if_evolved_this_turn_attack(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> Outcomes {
    let evolved = state.in_play_pokemon[state.current_player][0]
        .as_ref()
        .map(|p| p.played_this_turn)
        .unwrap_or(false);
    let damage = if evolved {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn knock_back_attack(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_bench_pokemon(opponent) {
            choices.push(SimpleAction::Activate {
                player: opponent,
                in_play_idx,
            });
        }
        if choices.is_empty() {
            return; // No benched pokemon to knock back
        }
        state.move_generation_stack.push((opponent, choices));
    })
}

/// For Mawile's Crunch attack: deals 20 damage, flip a coin, if heads discard a random energy from opponent's active
fn mawile_crunch() -> Outcomes {
    Outcomes::binary_coin(
        active_damage_effect_mutation(20, move |rng, state, action| {
            // Heads: damage + discard random energy
            let opponent = (action.actor + 1) % 2;
            let active = state.get_active_mut(opponent);

            if !active.attached_energy.is_empty() {
                let energy_count = active.attached_energy.len();
                let rand_idx = rng.gen_range(0..energy_count);
                active.attached_energy.remove(rand_idx);
            }
        }),
        active_damage_mutation(20), // Tails: just damage
    )
}

/// For baby pokémon attacks: Attach an energy from Energy Zone to a benched Basic pokémon
fn attach_energy_to_benched_basic(acting_player: usize, energy_type: EnergyType) -> Outcomes {
    active_damage_effect_doutcome(0, move |_, state, _| {
        let possible_moves = state
            .enumerate_bench_pokemon(acting_player)
            .filter(|(_, pokemon)| get_stage(pokemon) == 0)
            .map(|(in_play_idx, _)| SimpleAction::Attach {
                attachments: vec![(1, energy_type, in_play_idx)],
                is_turn_energy: false,
            })
            .collect::<Vec<_>>();
        if !possible_moves.is_empty() {
            state
                .move_generation_stack
                .push((acting_player, possible_moves));
        }
    })
}

/// For Silvally's Brave Buddies attack: 50 damage, or 100 damage if a Supporter was played this turn
fn brave_buddies_attack(state: &State, fixed_damage: u32, extra_damage: u32) -> Outcomes {
    if state.has_played_support {
        active_damage_doutcome(fixed_damage + extra_damage)
    } else {
        active_damage_doutcome(fixed_damage)
    }
}

/// For Absol's Unseen Claw (A3 112): Deals 20 damage, +60 if opponent's Active has a Special Condition
fn unseen_claw_attack(
    acting_player: usize,
    state: &State,
    extra_damage: u32,
    fixed_damage: u32,
) -> Outcomes {
    let opponent = (acting_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    let damage = if opponent_active.has_status_condition() {
        fixed_damage + extra_damage
    } else {
        fixed_damage
    };
    active_damage_doutcome(damage)
}

/// For Absol's Ominous Claw (B1 150): Deals 50 damage, flip coin, if heads discard a Supporter from opponent's hand
fn ominous_claw_attack(acting_player: usize, fixed_damage: u32) -> Outcomes {
    // 50% chance for heads (discard supporter), 50% for tails (just damage)
    Outcomes::binary_coin(
        active_damage_effect_mutation(fixed_damage, move |_, state, _action| {
            let opponent = (acting_player + 1) % 2;
            let possible_discards: Vec<SimpleAction> = state
                .iter_hand_supporters(opponent)
                .map(|card| SimpleAction::DiscardOpponentSupporter {
                    supporter_card: card.clone(),
                })
                .collect();

            if !possible_discards.is_empty() {
                state
                    .move_generation_stack
                    .push((acting_player, possible_discards));
            }
        }),
        // Tails: just damage
        active_damage_mutation(fixed_damage),
    )
}

/// For Mega Absol ex's Darkness Claw: Deals 80 damage and lets player discard a Supporter from opponent's hand
fn darkness_claw_attack(acting_player: usize, fixed_damage: u32) -> Outcomes {
    active_damage_effect_doutcome(fixed_damage, move |_, state, _action| {
        let opponent = (acting_player + 1) % 2;
        let possible_discards: Vec<SimpleAction> = state
            .iter_hand_supporters(opponent)
            .map(|card| SimpleAction::DiscardOpponentSupporter {
                supporter_card: card.clone(),
            })
            .collect();

        if !possible_discards.is_empty() {
            state
                .move_generation_stack
                .push((acting_player, possible_discards));
        }
    })
}

/// For Sableye's Dirty Throw (B1 101): Discard a card from hand to deal 70 damage. If can't discard, attack does nothing.
fn discard_hand_cards_required_attack(state: &State, fixed_damage: u32, count: usize) -> Outcomes {
    let acting_player = state.current_player;
    if state.hands[acting_player].len() < count {
        return active_damage_doutcome(0);
    }

    active_damage_effect_doutcome(fixed_damage, move |_, state, action| {
        let hand_cards: Vec<Card> = state.hands[action.actor].to_vec();
        let choices: Vec<SimpleAction> = generate_combinations(&hand_cards, count)
            .into_iter()
            .map(|combo| SimpleAction::DiscardOwnCards { cards: combo })
            .collect();

        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

/// For Umbreon's Dark Binding: If the Defending Pokémon is a Basic Pokémon, it can't attack during your opponent's next turn.
fn block_basic_attack(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let opponent_active = state.get_active_mut(opponent);

        // Check if the defending Pokemon is a Basic Pokemon (stage 0)
        if opponent_active.card.is_basic() {
            opponent_active.add_effect(CardEffect::CannotAttack, 1);
        }
    })
}

/// For Aerodactyl's Primal Wingbeat: Flip a coin. If heads, opponent shuffles their Active Pokémon into their deck.
fn shuffle_opponent_active_into_deck() -> Outcomes {
    Outcomes::binary_coin(
        // Heads: shuffle opponent's active into deck
        active_damage_effect_mutation(0, move |rng, state, action| {
            let opponent = (action.actor + 1) % 2;

            // Get the active Pokemon
            let active_pokemon = state.in_play_pokemon[opponent][0]
                .take()
                .expect("Active Pokemon should be there");

            // Put the card (and evolution chain) back into deck
            let mut cards_to_shuffle = active_pokemon.cards_behind.clone();
            cards_to_shuffle.push(active_pokemon.card.clone());

            // Add cards to deck
            state.decks[opponent].cards.extend(cards_to_shuffle);

            // Put energies back into discard pile
            state.discard_energies[opponent].extend(active_pokemon.attached_energy.iter().cloned());

            // Shuffle the deck
            state.decks[opponent].shuffle(false, rng);

            // Trigger promotion from bench (or declare winner if no bench)
            state.trigger_promotion_or_declare_winner(opponent);
        }),
        // Tails: just do nothing
        active_damage_mutation(0),
    )
}

fn coin_flip_shuffle_random_opponent_hand_card_into_deck() -> Outcomes {
    Outcomes::binary_coin(
        // Heads: shuffle a random card from opponent's hand into their deck
        active_damage_effect_mutation(0, move |rng, state, action| {
            let opponent = (action.actor + 1) % 2;
            if state.hands[opponent].is_empty() {
                return;
            }
            let idx = rng.gen_range(0..state.hands[opponent].len());
            let card = state.hands[opponent].remove(idx);
            state.decks[opponent].cards.push(card);
            state.decks[opponent].shuffle(false, rng);
        }),
        // Tails: do nothing
        active_damage_mutation(0),
    )
}

/// Teal Mask Ogerpon ex – Energized Leaves:
/// If total energy on both Active Pokémon ≥ threshold, deal extra_damage more.
fn extra_damage_if_combined_active_energy_at_least(
    state: &State,
    base_damage: u32,
    threshold: usize,
    extra_damage: u32,
) -> Outcomes {
    let current_player = state.current_player;
    let opponent = (current_player + 1) % 2;
    let combined = state.get_active(current_player).attached_energy.len()
        + state.get_active(opponent).attached_energy.len();
    let total = if combined >= threshold {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(total)
}

/// Hearthflame Mask Ogerpon – Hearthflame Dance:
/// Flip a coin. If heads, take `energies` from your Energy Zone and attach to 1 Benched Pokémon.
fn coin_flip_charge_bench(
    state: &State,
    base_damage: u32,
    energies: Vec<EnergyType>,
    target_benched_type: Option<EnergyType>,
) -> Outcomes {
    let choices = state
        .enumerate_bench_pokemon(state.current_player)
        .filter(|(_, played_card)| {
            target_benched_type.is_none() || played_card.get_energy_type() == target_benched_type
        })
        .map(|(in_play_idx, _)| SimpleAction::Attach {
            attachments: energies
                .iter()
                .map(|&energy| (1, energy, in_play_idx))
                .collect(),
            is_turn_energy: false,
        })
        .collect::<Vec<_>>();
    Outcomes::binary_coin(
        active_damage_effect_mutation(base_damage, move |_, state, action| {
            if !choices.is_empty() {
                state
                    .move_generation_stack
                    .push((action.actor, choices.clone()));
            }
        }),
        active_damage_mutation(base_damage),
    )
}

/// Wellspring Mask Ogerpon – Wellspring Dance:
/// Flip a coin. If heads, this attack also does `bench_damage` to 1 of the chosen side's bench.
fn coin_flip_also_choice_bench_damage(
    state: &State,
    opponent: bool,
    active_damage: u32,
    bench_damage: u32,
) -> Outcomes {
    let opponent_player = (state.current_player + 1) % 2;
    let bench_target = if opponent {
        opponent_player
    } else {
        state.current_player
    };
    // Build choices that bundle active + bench damage atomically (avoids stale slot issues).
    let choices: Vec<_> = state
        .enumerate_bench_pokemon(bench_target)
        .map(|(in_play_idx, _)| SimpleAction::ApplyDamage {
            attacking_ref: (state.current_player, 0),
            targets: vec![
                (active_damage, opponent_player, 0),
                (bench_damage, bench_target, in_play_idx),
            ],
            is_from_active_attack: true,
        })
        .collect();

    if choices.is_empty() {
        // No bench targets: coin flip has no effect; always deal active damage.
        return active_damage_doutcome(active_damage);
    }

    // Heads: defer all damage via ApplyDamage choice (atomic KO resolution).
    // Tails: deal active damage directly (bench untouched).
    Outcomes::binary_coin(
        Box::new(move |_: &mut StdRng, state: &mut State, action: &Action| {
            state
                .move_generation_stack
                .push((action.actor, choices.clone()));
        }),
        active_damage_mutation(active_damage),
    )
}

fn extra_damage_if_defender_poisoned(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    let damage = if state.get_active(opponent).is_poisoned() {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn mega_ampharos_lightning_lancer() -> Outcomes {
    // 1 of your opponent's Benched Pokémon is chosen at random 3 times.
    // For each time a Pokémon was chosen, also do 20 damage to it.
    Outcomes::single_fn(|rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        let targets: Vec<(u32, usize, usize)> =
            generate_random_spread_indices(rng, state, opponent, true, 3)
                .into_iter()
                .map(|idx| (20, opponent, idx))
                .chain(std::iter::once((100, opponent, 0))) // Add active Pokémon directly
                .collect();

        let attacking_ref = (action.actor, 0);
        handle_damage(state, attacking_ref, &targets, true, None);
    })
}

fn random_damage_to_opponent_pokemon_per_self_energy(
    state: &State,
    energy_type: EnergyType,
    damage_per_hit: u32,
) -> Outcomes {
    let energy_count = state
        .get_active(state.current_player)
        .attached_energy
        .iter()
        .filter(|&&e| e == energy_type)
        .count();

    if energy_count == 0 {
        return active_damage_doutcome(0);
    }

    Outcomes::single(Box::new(move |rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        let targets: Vec<(u32, usize, usize)> =
            generate_random_spread_indices(rng, state, opponent, false, energy_count)
                .into_iter()
                .map(|idx| (damage_per_hit, opponent, idx))
                .collect();

        let attacking_ref = (action.actor, 0);
        handle_damage(state, attacking_ref, &targets, true, None);
    }))
}

fn generate_random_spread_indices(
    rng: &mut StdRng,
    state: &State,
    player: usize,
    bench_only: bool,
    count: usize,
) -> Vec<usize> {
    let mut targets = vec![];
    for _ in 0..count {
        let possible_indices: Vec<usize> = if bench_only {
            state
                .enumerate_bench_pokemon(player)
                .map(|(idx, _)| idx)
                .collect()
        } else {
            state
                .enumerate_in_play_pokemon(player)
                .map(|(idx, _)| idx)
                .collect()
        };
        if possible_indices.is_empty() {
            continue;
        }

        let rand_idx = rng.gen_range(0..possible_indices.len());
        targets.push(possible_indices[rand_idx]);
    }
    targets
}

/// Damage distribution: Vec of (player, in_play_idx, total_damage)
type DamageDistribution = Vec<(usize, usize, u32)>;
/// Enumerated outcome: (probability, damage_distribution)
type EnumeratedOutcome = (f64, DamageDistribution);

/// Generates forecastable outcomes for random multi-target damage attacks.
/// Given a list of possible targets, enumerates all possible targeting combinations
/// and groups them by damage distribution with correct probabilities.
///
/// Returns a Vec of (probability, damage_distribution) where damage_distribution
/// is a sorted Vec of (player, in_play_idx, total_damage).
pub(crate) fn enumerate_random_damage_outcomes(
    possible_targets: &[(usize, usize)],
    times: usize,
    damage_per_hit: u32,
) -> Vec<EnumeratedOutcome> {
    let n = possible_targets.len();
    if n == 0 {
        return vec![];
    }

    let total_sequences = n.pow(times as u32);
    let prob_per_sequence = 1.0 / total_sequences as f64;

    let mut outcome_groups: HashMap<Vec<(usize, usize, u32)>, f64> = HashMap::new();

    for seq_idx in 0..total_sequences {
        let mut damage_map: HashMap<(usize, usize), u32> = HashMap::new();
        let mut remaining = seq_idx;
        for _ in 0..times {
            let target_idx = remaining % n;
            remaining /= n;
            let target = possible_targets[target_idx];
            *damage_map.entry(target).or_insert(0) += damage_per_hit;
        }

        let mut key: Vec<(usize, usize, u32)> = damage_map
            .into_iter()
            .map(|((p, i), d)| (p, i, d))
            .collect();
        key.sort();

        *outcome_groups.entry(key).or_insert(0.0) += prob_per_sequence;
    }

    outcome_groups
        .into_iter()
        .map(|(dist, prob)| (prob, dist))
        .collect()
}

/// Converts enumerated damage outcomes into Outcomes struct for forecasting.
fn random_damage_outcomes_to_outcomes(outcomes: Vec<EnumeratedOutcome>) -> Outcomes {
    if outcomes.is_empty() {
        return Outcomes::single_fn(|_, _, _| {});
    }

    let mut probabilities = Vec::with_capacity(outcomes.len());
    let mut mutations: Mutations = Vec::with_capacity(outcomes.len());

    for (prob, damage_dist) in outcomes {
        probabilities.push(prob);
        mutations.push(Box::new(
            move |_: &mut StdRng, state: &mut State, action: &Action| {
                let attacking_ref = (action.actor, 0);
                let targets: Vec<(u32, usize, usize)> = damage_dist
                    .iter()
                    .map(|&(player, idx, damage)| (damage, player, idx))
                    .collect();
                handle_damage(state, attacking_ref, &targets, true, None);
            },
        ));
    }

    Outcomes::from_parts(probabilities, mutations)
}

/// Random spread damage attack (e.g., Draco Meteor, Spurt Fire).
/// Always targets all opponent Pokemon. Optionally includes own bench.
fn random_spread_damage(
    state: &State,
    times: usize,
    damage_per_hit: u32,
    include_own_bench: bool,
) -> Outcomes {
    let actor = state.current_player;
    let opponent = (actor + 1) % 2;

    // Always include all opponent Pokemon
    let mut possible_targets: Vec<(usize, usize)> = state
        .enumerate_in_play_pokemon(opponent)
        .map(|(idx, _)| (opponent, idx))
        .collect();

    // Optionally add own bench (never own active - that's the attacker)
    if include_own_bench {
        for (idx, _) in state.enumerate_bench_pokemon(actor) {
            possible_targets.push((actor, idx));
        }
    }

    let outcomes = enumerate_random_damage_outcomes(&possible_targets, times, damage_per_hit);
    random_damage_outcomes_to_outcomes(outcomes)
}

fn switch_self_with_bench(state: &State, damage: u32) -> Outcomes {
    let choices: Vec<_> = state
        .enumerate_bench_pokemon(state.current_player)
        .map(|(in_play_idx, _)| SimpleAction::Activate {
            player: state.current_player,
            in_play_idx,
        })
        .collect();

    Outcomes::single(Box::new(
        move |_: &mut StdRng, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            let attacking_ref = (action.actor, 0);

            // Deal damage to opponent's active
            handle_damage(state, attacking_ref, &[(damage, opponent, 0)], true, None);

            // Push choices for switching if there are benched Pokemon and pokemon
            // is still alive (after possible counterdamage)
            if !choices.is_empty() && state.maybe_get_active(action.actor).is_some() {
                state.move_generation_stack.push((action.actor, choices));
            }
        },
    ))
}

/// Mega Steelix ex - Adamantine Rolling: Deals damage and applies multiple card effects
fn damage_and_multiple_card_effects_attack(
    damage: u32,
    opponent: bool,
    effects: Vec<CardEffect>,
    effect_duration: u8,
) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let player = if opponent {
            (action.actor + 1) % 2
        } else {
            action.actor
        };
        let target_pokemon = state.get_active_mut(player);
        for effect in effects.iter() {
            target_pokemon.add_effect(effect.clone(), effect_duration);
        }
    })
}

/// Mega Lopunny ex - Rapid Smashers: Flips coins for damage and always inflicts status
fn damage_for_each_heads_with_status_attack(
    include_fixed_damage: bool,
    damage_per_head: u32,
    num_coins: usize,
    attack: &Attack,
    status: StatusCondition,
) -> Outcomes {
    Outcomes::binomial_by_heads(num_coins, move |heads| {
        let damage = if include_fixed_damage {
            attack.fixed_damage + (heads as u32 * damage_per_head)
        } else {
            heads as u32 * damage_per_head
        };
        active_damage_effect_mutation(damage, build_status_effect(status))
    })
}

/// Mega Blastoise ex - Triple Bombardment: Conditional bench damage based on extra energy
fn conditional_bench_damage_attack(
    state: &State,
    attack: &Attack,
    required_extra_energy: Vec<EnergyType>,
    bench_damage: u32,
    num_bench_targets: usize,
    opponent: bool,
) -> Outcomes {
    let pokemon = state.get_active(state.current_player);
    let cost_with_extra_energy = attack
        .energy_required
        .iter()
        .cloned()
        .chain(required_extra_energy.iter().cloned())
        .collect::<Vec<EnergyType>>();

    let has_extra_energy = contains_energy(
        pokemon,
        &cost_with_extra_energy,
        state,
        state.current_player,
    );

    if has_extra_energy {
        let opponent_player = (state.current_player + 1) % 2;
        let bench_target = if opponent {
            opponent_player
        } else {
            state.current_player
        };
        let benched: Vec<usize> = state
            .enumerate_bench_pokemon(bench_target)
            .map(|(idx, _)| idx)
            .collect();

        // Only create choices with bench damage if there are enough bench targets
        // Otherwise, just apply active damage without creating choices
        if benched.len() >= num_bench_targets {
            let choices: Vec<_> = if num_bench_targets == 1 {
                benched
                    .iter()
                    .map(|&bench_idx| {
                        let targets = vec![
                            (attack.fixed_damage, opponent_player, 0),
                            (bench_damage, bench_target, bench_idx),
                        ];
                        SimpleAction::ApplyDamage {
                            attacking_ref: (state.current_player, 0),
                            targets,
                            is_from_active_attack: true,
                        }
                    })
                    .collect()
            } else if num_bench_targets == 2 {
                let mut choices = Vec::new();
                for i in 0..benched.len() {
                    for j in (i + 1)..benched.len() {
                        let targets = vec![
                            (attack.fixed_damage, opponent_player, 0),
                            (bench_damage, bench_target, benched[i]),
                            (bench_damage, bench_target, benched[j]),
                        ];
                        choices.push(SimpleAction::ApplyDamage {
                            attacking_ref: (state.current_player, 0),
                            targets,
                            is_from_active_attack: true,
                        });
                    }
                }
                choices
            } else {
                vec![]
            };

            Outcomes::single(Box::new(
                move |_: &mut StdRng, state: &mut State, action: &Action| {
                    if !choices.is_empty() {
                        state.move_generation_stack.push((action.actor, choices));
                    }
                },
            ))
        } else {
            // Not enough bench targets, just apply damage to active without creating choices
            active_damage_doutcome(attack.fixed_damage)
        }
    } else {
        active_damage_doutcome(attack.fixed_damage)
    }
}

/// Xerneas - Geoburst: Damage reduced by self damage
fn damage_reduced_by_self_damage_attack(state: &State, attack: &Attack) -> Outcomes {
    let active = state.get_active(state.current_player);
    let damage_taken = active.get_damage_counters();
    let actual_damage = attack.fixed_damage.saturating_sub(damage_taken);
    active_damage_doutcome(actual_damage)
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::{
        actions::{Action, SimpleAction},
        card_ids::CardId,
        models::{EnergyType, PlayedCard},
        State,
    };

    use super::extra_or_self_damage_attack;

    #[test]
    fn test_extra_or_self_damage_attack_double_ko_promotes() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();

        state.current_player = 0;
        state.turn_count = 3;

        // Attacker (Electabuzz) at 20 HP, with bench for promotion
        state.in_play_pokemon[0][0] = Some(
            PlayedCard::from_id(CardId::A1101Electabuzz)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])
                .with_remaining_hp(20),
        );
        state.in_play_pokemon[0][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

        // Opponent active at 40 HP so base damage KOs, with bench for promotion
        state.in_play_pokemon[1][0] =
            Some(PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(40));
        state.in_play_pokemon[1][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        let (_probs, mut muts) = extra_or_self_damage_attack(40, 40, 20).into_branches();
        // Tails outcome: base damage + self damage
        let mutation = muts.remove(1);
        mutation(&mut rng, &mut state, &action);

        // Both actives should be knocked out
        assert!(state.in_play_pokemon[0][0].is_none());
        assert!(state.in_play_pokemon[1][0].is_none());

        let mut has_promo_0 = false;
        let mut has_promo_1 = false;
        for (player, actions) in state.move_generation_stack.iter() {
            if actions
                .iter()
                .any(|a| matches!(a, SimpleAction::Activate { .. }))
            {
                if *player == 0 {
                    has_promo_0 = true;
                } else if *player == 1 {
                    has_promo_1 = true;
                }
            }
        }

        assert!(has_promo_0, "Expected promotion for player 0");
        assert!(has_promo_1, "Expected promotion for player 1");
    }

    #[test]
    fn test_extra_or_self_damage_attack_self_ko_promotes_attacker() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();

        state.current_player = 0;
        state.turn_count = 3;

        // Attacker (Electabuzz) at 20 HP, with bench for promotion
        state.in_play_pokemon[0][0] = Some(
            PlayedCard::from_id(CardId::A1101Electabuzz)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])
                .with_remaining_hp(20),
        );
        state.in_play_pokemon[0][1] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

        // Opponent active survives base damage
        state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::A1001Bulbasaur));

        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        let (_probs, mut muts) = extra_or_self_damage_attack(40, 40, 20).into_branches();
        // Tails outcome: base damage + self damage
        let mutation = muts.remove(1);
        mutation(&mut rng, &mut state, &action);

        // Attacker active should be knocked out, opponent active should remain
        assert!(state.in_play_pokemon[0][0].is_none());
        assert!(state.in_play_pokemon[1][0].is_some());

        let has_promo_0 = state.move_generation_stack.iter().any(|(player, actions)| {
            *player == 0
                && actions
                    .iter()
                    .any(|a| matches!(a, SimpleAction::Activate { .. }))
        });
        let has_promo_1 = state.move_generation_stack.iter().any(|(player, actions)| {
            *player == 1
                && actions
                    .iter()
                    .any(|a| matches!(a, SimpleAction::Activate { .. }))
        });

        assert!(has_promo_0, "Expected promotion for player 0");
        assert!(!has_promo_1, "Did not expect promotion for player 1");
    }
}

/// Porygon-Z - Cyberjack: Extra damage per trainer in opponent deck
fn extra_damage_per_trainer_in_opponent_deck_attack(
    state: &State,
    base_damage: u32,
    damage_per_trainer: u32,
) -> Outcomes {
    let opponent = (state.current_player + 1) % 2;
    let trainer_count = state.decks[opponent]
        .cards
        .iter()
        .filter(|card| matches!(card, crate::models::Card::Trainer(_)))
        .count() as u32;
    let total_damage = base_damage + (trainer_count * damage_per_trainer);
    active_damage_doutcome(total_damage)
}

/// Chandelure - Past Friends: Extra damage per Supporter in your discard pile.
fn extra_damage_per_supporter_in_discard_attack(
    state: &State,
    base_damage: u32,
    damage_per_supporter: u32,
) -> Outcomes {
    let supporter_count = state.discard_piles[state.current_player]
        .iter()
        .filter(|card| {
            matches!(
                card,
                Card::Trainer(trainer) if trainer.trainer_card_type == TrainerType::Supporter
            )
        })
        .count() as u32;
    let total_damage = base_damage + (supporter_count * damage_per_supporter);
    active_damage_doutcome(total_damage)
}

/// Mega Manectric ex - Lightning Accelerator: Extra damage per point you have gotten
fn extra_damage_per_own_point_attack(
    state: &State,
    base_damage: u32,
    damage_per_point: u32,
) -> Outcomes {
    let points = state.points[state.current_player] as u32;
    let total_damage = base_damage + (points * damage_per_point);
    active_damage_doutcome(total_damage)
}

/// Sunflora - Quick-Grow Beam: Extra damage if specific card in discard
fn extra_damage_if_card_in_discard_attack(
    state: &State,
    base_damage: u32,
    card_name: String,
    extra_damage: u32,
) -> Outcomes {
    let has_card_in_discard = state.discard_piles[state.current_player]
        .iter()
        .any(|card| {
            if let crate::models::Card::Trainer(trainer) = card {
                trainer.name == card_name
            } else {
                false
            }
        });
    let total_damage = if has_card_in_discard {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(total_damage)
}

/// Magnezone - Mirror Shot: Coin flip to block opponent attack next turn
fn coin_flip_to_block_attack_next_turn(damage: u32) -> Outcomes {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        state
            .get_active_mut(opponent)
            .add_effect(CardEffect::CoinFlipToBlockAttack, 1);
    })
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::{
        actions::Action, card_ids::CardId, database::get_card_by_enum, hooks::to_playable_card,
    };

    use super::*;

    #[test]
    fn test_arceus_does_90_damage() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();
        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        let receiver = get_card_by_enum(CardId::A1003Venusaur); // 160 hp
        state.in_play_pokemon[1][0] = Some(to_playable_card(&receiver, false));
        let attacker = get_card_by_enum(CardId::A2a071ArceusEx);
        state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker, false));
        let some_base_pokemon = get_card_by_enum(CardId::A1001Bulbasaur);
        state.in_play_pokemon[0][1] = Some(to_playable_card(&some_base_pokemon, false));

        let (_, mut lazy_mutations) =
            bench_count_damage_attack(&state, 70, true, 20, None, &BenchSide::YourBench)
                .into_branches();
        lazy_mutations.remove(0)(&mut rng, &mut state, &action);

        assert_eq!(state.get_active(1).get_remaining_hp(), 70);
    }

    #[test]
    fn test_generate_energy_distributions() {
        // 1 pokemon, 1 head
        let fire_pokemon = vec![1];
        let choices = generate_energy_distributions(&fire_pokemon, 1);
        assert_eq!(choices.len(), 1);
        if let SimpleAction::Attach { attachments, .. } = &choices[0] {
            assert_eq!(attachments, &[(1, EnergyType::Fire, 1)]);
        } else {
            panic!("Expected SimpleAction::Attach");
        }

        // 1 pokemon, 2 heads
        let choices = generate_energy_distributions(&fire_pokemon, 2);
        assert_eq!(choices.len(), 1);
        if let SimpleAction::Attach { attachments, .. } = &choices[0] {
            assert_eq!(attachments, &[(2, EnergyType::Fire, 1)]);
        } else {
            panic!("Expected SimpleAction::Attach");
        }

        // 2 pokemon, 2 heads
        let fire_pokemon = vec![1, 2];
        let choices = generate_energy_distributions(&fire_pokemon, 2);
        assert_eq!(choices.len(), 3);
        let expected_distributions = [
            vec![(2, EnergyType::Fire, 2)],
            vec![(1, EnergyType::Fire, 1), (1, EnergyType::Fire, 2)],
            vec![(2, EnergyType::Fire, 1)],
        ];
        for (i, choice) in choices.iter().enumerate() {
            if let SimpleAction::Attach { attachments, .. } = choice {
                assert_eq!(attachments, &expected_distributions[i]);
            } else {
                panic!("Expected SimpleAction::Attach");
            }
        }

        // 2 pokemon, 3 heads
        let choices = generate_energy_distributions(&fire_pokemon, 3);
        assert_eq!(choices.len(), 4);
        let expected_distributions = [
            vec![(3, EnergyType::Fire, 2)],
            vec![(1, EnergyType::Fire, 1), (2, EnergyType::Fire, 2)],
            vec![(2, EnergyType::Fire, 1), (1, EnergyType::Fire, 2)],
            vec![(3, EnergyType::Fire, 1)],
        ];
        for (i, choice) in choices.iter().enumerate() {
            if let SimpleAction::Attach { attachments, .. } = choice {
                assert_eq!(attachments, &expected_distributions[i]);
            } else {
                panic!("Expected SimpleAction::Attach");
            }
        }

        // 3 pokemon, 2 heads
        let fire_pokemon = vec![1, 2, 3];
        let choices = generate_energy_distributions(&fire_pokemon, 2);
        assert_eq!(choices.len(), 6);
        let expected_distributions = [
            vec![(2, EnergyType::Fire, 3)],
            vec![(1, EnergyType::Fire, 2), (1, EnergyType::Fire, 3)],
            vec![(2, EnergyType::Fire, 2)],
            vec![(1, EnergyType::Fire, 1), (1, EnergyType::Fire, 3)],
            vec![(1, EnergyType::Fire, 1), (1, EnergyType::Fire, 2)],
            vec![(2, EnergyType::Fire, 1)],
        ];
        for (i, choice) in choices.iter().enumerate() {
            if let SimpleAction::Attach { attachments, .. } = choice {
                assert_eq!(attachments, &expected_distributions[i]);
            } else {
                panic!("Expected SimpleAction::Attach");
            }
        }
    }

    #[test]
    fn test_flip_until_tails_probabilities() {
        // Test that flip_until_tails_attack generates correct probabilities
        let (probabilities, _mutations) = flip_until_tails_attack(20).into_branches();

        // Check that we have 9 outcomes (0 to 8 heads)
        assert_eq!(probabilities.len(), 9);

        // Check first few probabilities match geometric distribution
        // P(0 heads) = 0.5, P(1 heads) = 0.25, P(2 heads) = 0.125, etc.
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.25).abs() < 0.001);
        assert!((probabilities[2] - 0.125).abs() < 0.001);

        // Check probabilities sum to approximately 1
        let sum: f64 = probabilities.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_fixed_coin_probabilistic_attack() {
        // Test Jolteon Pin Missile (4 coins, 40 damage each)
        let (probabilities, _mutations) =
            Outcomes::binomial_by_heads(4, |heads| active_damage_mutation((heads as u32) * 40))
                .into_branches();

        // Check we have 5 outcomes (0 to 4 heads)
        assert_eq!(probabilities.len(), 5);

        // Check that probabilities match expected binomial distribution for 4 coins
        assert!((probabilities[0] - 0.0625).abs() < 0.001); // 0 heads
        assert!((probabilities[1] - 0.25).abs() < 0.001); // 1 heads
        assert!((probabilities[2] - 0.375).abs() < 0.001); // 2 heads
        assert!((probabilities[3] - 0.25).abs() < 0.001); // 3 heads
        assert!((probabilities[4] - 0.0625).abs() < 0.001); // 4 heads
    }

    #[test]
    fn test_celebi_powerful_bloom_probabilities() {
        // Test with 2 energy attached (2 coins)
        let mut state = State::default();

        // Set up a Pokemon in the active position
        let celebi = get_card_by_enum(CardId::A1a003CelebiEx);
        state.in_play_pokemon[0][0] = Some(to_playable_card(&celebi, false));

        state.attach_energy_from_zone(0, 0, EnergyType::Grass, 1, false);
        state.attach_energy_from_zone(0, 0, EnergyType::Fire, 1, false);

        let (probabilities, _mutations) = celebi_powerful_bloom(&state).into_branches();

        // Should have 3 outcomes (0, 1, 2 heads)
        assert_eq!(probabilities.len(), 3);

        // Check probabilities for 2 coins: 0.25, 0.5, 0.25
        assert!((probabilities[0] - 0.25).abs() < 0.001); // 0 heads: C(2,0) / 4 = 1/4
        assert!((probabilities[1] - 0.5).abs() < 0.001); // 1 heads: C(2,1) / 4 = 2/4
        assert!((probabilities[2] - 0.25).abs() < 0.001); // 2 heads: C(2,2) / 4 = 1/4

        // Test with no energy attached
        let mut state_no_energy = State::default();
        state_no_energy.in_play_pokemon[0][0] = Some(to_playable_card(&celebi, false));
        let (probabilities_no_energy, _) = celebi_powerful_bloom(&state_no_energy).into_branches();

        // Should have 1 outcome (0 damage)
        assert_eq!(probabilities_no_energy.len(), 1);
        assert!((probabilities_no_energy[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(Outcomes::binomial_coefficient(0, 0), 1);
        assert_eq!(Outcomes::binomial_coefficient(1, 0), 1);
        assert_eq!(Outcomes::binomial_coefficient(1, 1), 1);
        assert_eq!(Outcomes::binomial_coefficient(2, 0), 1);
        assert_eq!(Outcomes::binomial_coefficient(2, 1), 2);
        assert_eq!(Outcomes::binomial_coefficient(2, 2), 1);
        assert_eq!(Outcomes::binomial_coefficient(4, 2), 6);
        assert_eq!(Outcomes::binomial_coefficient(5, 3), 10);
        assert_eq!(Outcomes::binomial_coefficient(6, 2), 15);
    }

    #[test]
    fn test_single_coin_attacks() {
        // Test Ponyta Stomp (1 coin, 0 or 30 damage)
        let (probabilities, _mutations) =
            Outcomes::binary_coin(active_damage_mutation(30), active_damage_mutation(0))
                .into_branches();
        assert_eq!(probabilities.len(), 2);
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.5).abs() < 0.001);

        // Test Rapidash Rising Lunge (1 coin, 0 or 60 damage)
        let (probabilities, _mutations) =
            Outcomes::binary_coin(active_damage_mutation(60), active_damage_mutation(0))
                .into_branches();
        assert_eq!(probabilities.len(), 2);
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.5).abs() < 0.001);

        // Test Mankey Focus Fist (1 coin, 0 or 50 damage)
        let (probabilities, _mutations) =
            Outcomes::binary_coin(active_damage_mutation(50), active_damage_mutation(0))
                .into_branches();
        assert_eq!(probabilities.len(), 2);
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_guzzlord_grindcore_does_not_respect_oricorio_safeguard() {
        // Test that Guzzlord ex's Grindcore attack does damage to Oricorio
        // despite Oricorio's Safeguard ability (which should prevent damage from ex Pokemon)
        // The first mutation (0 heads, immediate tails) should still do 30 damage
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();
        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        // Set up Oricorio (with Safeguard ability) as the opponent's active
        let oricorio = get_card_by_enum(CardId::A3066Oricorio); // 70 HP, Safeguard ability
        state.in_play_pokemon[1][0] = Some(to_playable_card(&oricorio, false));

        // Set up Guzzlord ex as the attacker
        let guzzlord = get_card_by_enum(CardId::A3a043GuzzlordEx); // 170 HP ex Pokemon
        state.in_play_pokemon[0][0] = Some(to_playable_card(&guzzlord, false));

        let attack = state.get_active(0).get_attacks()[0].clone();
        let effect = attack
            .effect
            .as_ref()
            .expect("Guzzlord ex attack should have effect text");
        let mechanic = EFFECT_MECHANIC_MAP
            .get(effect.as_str())
            .expect("Guzzlord ex effect should be mapped");
        let (_probabilities, mut mutations) =
            forecast_effect_attack_by_mechanic(&state, &attack, mechanic).into_branches();

        // Apply the first outcome mutation and ensure Oricorio's Safeguard still blocks ex damage.
        mutations.remove(0)(&mut rng, &mut state, &action);

        // Verify Oricorio did NOT take damage
        assert_eq!(state.get_active(1).get_remaining_hp(), 70);
    }

    #[test]
    fn test_extra_damage_if_type_energy_in_play_attack() {
        let mut rng = StdRng::seed_from_u64(0);
        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        let attacker = get_card_by_enum(CardId::B2a042BelliboltEx);
        let bench_lightning = get_card_by_enum(CardId::A2058Shinx);
        let receiver = get_card_by_enum(CardId::A1003Venusaur); // 160 HP

        let mut below_threshold = State::default();
        below_threshold.in_play_pokemon[0][0] = Some(to_playable_card(&attacker, false));
        below_threshold.in_play_pokemon[0][1] = Some(to_playable_card(&bench_lightning, false));
        below_threshold.in_play_pokemon[1][0] = Some(to_playable_card(&receiver, false));
        below_threshold.attach_energy_from_zone(0, 0, EnergyType::Lightning, 2, false);
        below_threshold.attach_energy_from_zone(0, 1, EnergyType::Lightning, 1, false);

        let (_, mut below_mutations) = extra_damage_if_type_energy_in_play_attack(
            &below_threshold,
            70,
            EnergyType::Lightning,
            4,
            70,
        )
        .into_branches();
        below_mutations.remove(0)(&mut rng, &mut below_threshold, &action);
        assert_eq!(below_threshold.get_active(1).get_remaining_hp(), 90);

        let mut at_threshold = State::default();
        at_threshold.in_play_pokemon[0][0] = Some(to_playable_card(&attacker, false));
        at_threshold.in_play_pokemon[0][1] = Some(to_playable_card(&bench_lightning, false));
        at_threshold.in_play_pokemon[1][0] = Some(to_playable_card(&receiver, false));
        at_threshold.attach_energy_from_zone(0, 0, EnergyType::Lightning, 2, false);
        at_threshold.attach_energy_from_zone(0, 1, EnergyType::Lightning, 2, false);

        let (_, mut threshold_mutations) = extra_damage_if_type_energy_in_play_attack(
            &at_threshold,
            70,
            EnergyType::Lightning,
            4,
            70,
        )
        .into_branches();
        threshold_mutations.remove(0)(&mut rng, &mut at_threshold, &action);
        assert_eq!(at_threshold.get_active(1).get_remaining_hp(), 20);
    }

    #[test]
    fn test_vaporeon_hyper_whirlpool_discards_without_duplicate_energy_panic() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut state = State::default();
        let action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };

        let attacker = get_card_by_enum(CardId::A1080Vaporeon);
        state.in_play_pokemon[0][0] = Some(to_playable_card(&attacker, false));

        let defender = get_card_by_enum(CardId::A1001Bulbasaur);
        state.in_play_pokemon[1][0] = Some(
            to_playable_card(&defender, false)
                .with_energy(vec![EnergyType::Water, EnergyType::Lightning]),
        );

        let (_probs, mut mutations) = vaporeon_hyper_whirlpool(&state, 60).into_branches();
        mutations.remove(2)(&mut rng, &mut state, &action);

        assert_eq!(state.get_active(1).attached_energy.len(), 0);
    }

    mod random_damage_outcomes_tests {
        use super::super::enumerate_random_damage_outcomes;

        #[test]
        fn test_one_target_three_hits_single_outcome() {
            // With 1 target and 3 hits, there's only 1 possible outcome: all hits go to that target
            let targets = vec![(1, 0)]; // opponent's active
            let outcomes = enumerate_random_damage_outcomes(&targets, 3, 50);

            assert_eq!(outcomes.len(), 1);
            let (prob, damage_dist) = &outcomes[0];
            assert!((prob - 1.0).abs() < 1e-9);
            assert_eq!(damage_dist, &vec![(1, 0, 150)]); // 3 * 50 = 150 damage
        }

        #[test]
        fn test_two_targets_three_hits_outcomes() {
            // With 2 targets (A, B) and 3 hits, there are 4 unique damage distributions:
            // - All 3 to A: (150, 0) - 1 way (AAA)
            // - 2 to A, 1 to B: (100, 50) - 3 ways (AAB, ABA, BAA)
            // - 1 to A, 2 to B: (50, 100) - 3 ways (ABB, BAB, BBA)
            // - All 3 to B: (0, 150) - 1 way (BBB)
            // Total: 8 sequences (2^3)
            let targets = vec![(1, 0), (1, 1)]; // opponent's active and bench
            let outcomes = enumerate_random_damage_outcomes(&targets, 3, 50);

            assert_eq!(outcomes.len(), 4);

            // Sort outcomes by damage distribution for easier comparison
            let mut sorted_outcomes: Vec<_> =
                outcomes.iter().map(|(p, d)| (*p, d.clone())).collect();
            sorted_outcomes.sort_by(|a, b| a.1.cmp(&b.1));

            // Check probabilities: 1/8, 3/8, 3/8, 1/8
            let prob_sum: f64 = sorted_outcomes.iter().map(|(p, _)| p).sum();
            assert!((prob_sum - 1.0).abs() < 1e-9);

            // Verify the 4 distributions exist with correct probabilities
            // Distribution with all damage to first target
            let all_to_first = sorted_outcomes
                .iter()
                .find(|(_, d)| d == &vec![(1, 0, 150)]);
            assert!(all_to_first.is_some());
            assert!((all_to_first.unwrap().0 - 0.125).abs() < 1e-9); // 1/8

            // Distribution with 2 to first, 1 to second
            let two_one = sorted_outcomes
                .iter()
                .find(|(_, d)| d == &vec![(1, 0, 100), (1, 1, 50)]);
            assert!(two_one.is_some());
            assert!((two_one.unwrap().0 - 0.375).abs() < 1e-9); // 3/8
        }

        #[test]
        fn test_three_targets_single_hit() {
            // With 3 targets and 1 hit, there are 3 outcomes, each with probability 1/3
            let targets = vec![(0, 1), (1, 0), (1, 1)]; // own bench, opponent active, opponent bench
            let outcomes = enumerate_random_damage_outcomes(&targets, 1, 100);

            assert_eq!(outcomes.len(), 3);

            for (prob, damage_dist) in &outcomes {
                assert!((prob - 1.0 / 3.0).abs() < 1e-9);
                assert_eq!(damage_dist.len(), 1);
                assert_eq!(damage_dist[0].2, 100);
            }
        }

        #[test]
        fn test_empty_targets() {
            let targets: Vec<(usize, usize)> = vec![];
            let outcomes = enumerate_random_damage_outcomes(&targets, 3, 50);
            assert!(outcomes.is_empty());
        }

        #[test]
        fn test_probability_sum_always_one() {
            // Test various configurations to ensure probabilities always sum to 1
            for num_targets in 1..=5 {
                for times in 1..=4 {
                    let targets: Vec<(usize, usize)> = (0..num_targets).map(|i| (0, i)).collect();
                    let outcomes = enumerate_random_damage_outcomes(&targets, times, 10);

                    let prob_sum: f64 = outcomes.iter().map(|(p, _)| p).sum();
                    assert!(
                        (prob_sum - 1.0).abs() < 1e-9,
                        "Probability sum {} != 1.0 for {} targets, {} times",
                        prob_sum,
                        num_targets,
                        times
                    );
                }
            }
        }
    }
}

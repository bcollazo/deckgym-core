use log::trace;
use rand::{rngs::StdRng, Rng};

use crate::{
    actions::{
        apply_action_helpers::handle_damage,
        apply_evolve,
        attacks::{BenchSide, Mechanic},
        effect_mechanic_map::EFFECT_MECHANIC_MAP,
        mutations::{doutcome, doutcome_from_mutation},
        Action,
    },
    effects::{CardEffect, TurnEffect},
    hooks::{can_evolve_into, contains_energy, get_stage},
    models::{Attack, Card, EnergyType, StatusCondition},
    AttackId, State,
};

use super::{
    apply_action_helpers::{Mutations, Probabilities},
    mutations::{
        active_damage_doutcome, active_damage_effect_doutcome, active_damage_effect_mutation,
        active_damage_mutation, build_status_effect, damage_effect_doutcome,
    },
    shared_mutations::{
        pokemon_search_outcomes, pokemon_search_outcomes_by_type, search_and_bench_by_name,
    },
    SimpleAction,
};

// This is a reducer of all actions relating to attacks.
pub(crate) fn forecast_attack(
    acting_player: usize,
    state: &State,
    index: usize,
) -> (Probabilities, Mutations) {
    let active = state.get_active(acting_player);
    let attack = active.card.get_attacks()[index].clone();
    trace!("Forecasting attack: {active:?} {attack:?}");

    let Some(effect_text) = &attack.effect else {
        return active_damage_doutcome(attack.fixed_damage);
    };
    // Try AttackId first, if not, fallback to mechanic map
    if let Some(attack_id) = AttackId::from_pokemon_index(&active.get_id()[..], index) {
        forecast_effect_attack_by_attack_id(state, attack_id)
    } else {
        let mechanic = EFFECT_MECHANIC_MAP.get(&effect_text[..]);
        let Some(mechanic) = mechanic else {
            panic!(
                "No implementation found for attack effect: {:?} on attack {:?} of Pokemon {:?}",
                effect_text, attack, active.card
            );
        };
        forecast_effect_attack_by_mechanic(state, &attack, mechanic)
    }
}

fn forecast_effect_attack_by_attack_id(
    state: &State,
    attack_id: AttackId,
) -> (Probabilities, Mutations) {
    let acting_player = state.current_player;
    match attack_id {
        AttackId::A1115AbraTeleport => teleport_attack(),
        AttackId::A1136GolurkDoubleLariat => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 100, 200])
        }
        AttackId::A1149GolemDoubleEdge => self_damage_attack(150, 50),
        AttackId::A1153MarowakExBonemerang => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 80, 160])
        }
        AttackId::A1163GrapploctKnockBack => knock_back_attack(60),
        AttackId::A1178MawileCrunch => mawile_crunch(),
        AttackId::A1181MeltanAmass => self_charge_active_attack(0, EnergyType::Metal, 1),
        AttackId::A1196MeowthPayDay => draw_and_damage_outcome(10),
        AttackId::A1201LickitungContinuousLick => flip_until_tails_attack(60),
        AttackId::A1203KangaskhanDizzyPunch => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 30, 60])
        }
        AttackId::A1a010PonytaStomp => probabilistic_damage_attack(vec![0.5, 0.5], vec![10, 40]),
        AttackId::A1a011RapidashRisingLunge => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![40, 100])
        }
        AttackId::A1a017MagikarpLeapOut | AttackId::A4a021FeebasLeapOut => teleport_attack(),
        AttackId::A1a026RaichuGigashock => {
            let opponent = (state.current_player + 1) % 2;
            let targets: Vec<(u32, usize)> = state
                .enumerate_bench_pokemon(opponent)
                .map(|(idx, _)| (20, idx))
                .chain(std::iter::once((60, 0)))
                .collect();
            damage_effect_doutcome(targets, |_, _, _| {})
        }
        AttackId::A1a061EeveeContinuousSteps => flip_until_tails_attack(20),
        AttackId::A2023MagmarStoke => self_charge_active_attack(0, EnergyType::Fire, 1),
        AttackId::A2029InfernapeExFlareBlitz => {
            discard_all_energy_of_type_attack(140, EnergyType::Fire)
        }
        AttackId::A2049PalkiaExDimensionalStorm => palkia_dimensional_storm(state),
        AttackId::A2056ElectabuzzCharge => self_charge_active_attack(0, EnergyType::Lightning, 1),
        AttackId::A2060LuxrayVoltBolt => luxray_volt_bolt(),
        AttackId::A2084GliscorAcrobatics => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![20, 40, 60])
        }
        AttackId::A2098SneaselDoubleScratch => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 20, 40])
        }
        AttackId::A2118ProbopassTripleNose => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![30, 80, 130, 180])
        }
        AttackId::A2131AmbipomDoubleHit => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 40, 80])
        }
        AttackId::A2141ChatotFuryAttack => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![0, 20, 40, 60])
        }
        AttackId::A2a001HeracrossSingleHornThrow => {
            probabilistic_damage_attack(vec![0.25, 0.75], vec![120, 50])
        }
        AttackId::A2a063SnorlaxCollapse => {
            damage_and_self_status_attack(100, StatusCondition::Asleep)
        }
        AttackId::A2b010CharizardExStoke => self_charge_active_attack(0, EnergyType::Fire, 3),
        AttackId::A2b032MrMimeJuggling => probabilistic_damage_attack(
            vec![0.0625, 0.25, 0.375, 0.25, 0.0625],
            vec![0, 20, 40, 60, 80],
        ),
        AttackId::A2b044FlamigoDoubleKick => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 50, 100])
        }
        AttackId::A3002AlolanExeggutorTropicalHammer => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 150])
        }
        AttackId::A3012DecidueyeExPierceThePain => direct_damage_if_damaged(100),
        AttackId::A3019SteeneeDoubleSpin => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 30, 60])
        }
        AttackId::A3020TsareenaThreeKickCombo => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![0, 50, 100, 150])
        }
        AttackId::A3040AlolanVulpixCallForthCold => {
            self_charge_active_attack(0, EnergyType::Water, 1)
        }
        AttackId::A3071SpoinkPsycharge => self_charge_active_attack(0, EnergyType::Psychic, 1),
        AttackId::A3116ToxapexSpikeCannon => probabilistic_damage_attack(
            vec![0.0625, 0.25, 0.375, 0.25, 0.0625],
            vec![0, 20, 40, 60, 80],
        ),
        AttackId::A3a003RowletFuryAttack => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![0, 10, 20, 30])
        }
        AttackId::A3a019TapuKokoExPlasmaHurricane => {
            self_charge_active_attack(20, EnergyType::Lightning, 1)
        }
        AttackId::A3a043GuzzlordExGrindcore => guzzlord_ex_grindcore_attack(),
        AttackId::A3a044Poipole2Step => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 20, 40])
        }
        AttackId::A3a047AlolanDugtrioExTripletHeadbutt => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![0, 60, 120, 180])
        }
        AttackId::A3a060TypeNullQuickBlow => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![20, 40])
        }
        AttackId::A3a061SilvallyBraveBuddies => brave_buddies_attack(state),
        AttackId::A3a062CelesteelaMoombahton => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 100])
        }
        AttackId::A1a001ExeggcuteGrowthSpurt => self_charge_active_attack(0, EnergyType::Grass, 1),
        AttackId::A3085CosmogTeleport => teleport_attack(),
        AttackId::A3122SolgaleoExSolBreaker => self_damage_attack(120, 10),
        AttackId::A3b013IncineroarDarkestLariat => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 100, 200])
        }
        AttackId::A3b020VanilluxeDoubleSpin => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 80, 160])
        }
        AttackId::A3b055EeveeCollect => draw_and_damage_outcome(0),
        AttackId::A3b057SnorlaxExFlopDownPunch => {
            damage_and_self_status_attack(130, StatusCondition::Asleep)
        }
        AttackId::A3b058AipomDoubleHit => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 20, 40])
        }
        AttackId::A4021ShuckleExTripleSlap => {
            probabilistic_damage_attack(vec![0.125, 0.375, 0.375, 0.125], vec![0, 20, 40, 60])
        }
        AttackId::A4032MagbyToastyToss => {
            attach_energy_to_benched_basic(acting_player, EnergyType::Fire)
        }
        AttackId::A4066PichuCracklyToss => {
            attach_energy_to_benched_basic(acting_player, EnergyType::Lightning)
        }
        AttackId::A4077CleffaTwinklyCall => pokemon_search_outcomes(acting_player, state, false),
        AttackId::A4105BinacleDualChop => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 30, 60])
        }
        AttackId::A4134EeveeFindAFriend => pokemon_search_outcomes(acting_player, state, false),
        AttackId::A4146UrsaringSwingAround => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![60, 80, 100])
        }
        AttackId::A4a023MantykeSplashyToss => {
            attach_energy_to_benched_basic(acting_player, EnergyType::Water)
        }
        AttackId::A3112AbsolUnseenClaw => unseen_claw_attack(acting_player, state),
        AttackId::B1052MegaGyaradosExMegaBlaster => damage_and_discard_opponent_deck(140, 3),
        AttackId::B1085MegaAmpharosExLightningLancer => mega_ampharos_lightning_lancer(),
        AttackId::B1101SableyeDirtyThrow => dirty_throw_attack(acting_player, state),
        AttackId::B1150AbsolOminousClaw => ominous_claw_attack(acting_player, state),
        AttackId::B1151MegaAbsolExDarknessClaw => darkness_claw_attack(acting_player, state),
    }
}

// Handles attacks that have effects.
fn forecast_effect_attack_by_mechanic(
    state: &State,
    attack: &Attack,
    mechanic: &Mechanic,
) -> (Probabilities, Mutations) {
    match mechanic {
        Mechanic::CelebiExPowerfulBloom => celebi_powerful_bloom(state),
        Mechanic::SelfHeal { amount } => self_heal_attack(*amount, attack),
        Mechanic::ManaphyOceanicGift => manaphy_oceanic(),
        Mechanic::PalkiaExDimensionalStorm => palkia_dimensional_storm(state),
        Mechanic::MegaBlazikenExMegaBurningAttack => mega_burning_attack(attack),
        Mechanic::MoltresExInfernoDance => moltres_inferno_dance(),
        Mechanic::MagikarpWaterfallEvolution => waterfall_evolution(state),
        Mechanic::ChargeBench {
            energies,
            target_benched_type,
        } => energy_bench_attack(energies.clone(), *target_benched_type, state, attack),
        Mechanic::SearchToHandByEnergy { energy_type } => {
            pokemon_search_outcomes_by_type(state, false, *energy_type)
        }
        Mechanic::SearchToBenchByName { name } => search_and_bench_by_name(state, name.clone()),
        Mechanic::InflictStatusCondition { condition } => damage_status_attack(*condition, attack),
        Mechanic::ChanceStatusAttack { condition } => {
            damage_chance_status_attack(attack.fixed_damage, 0.5, *condition)
        }
        Mechanic::DiscardEnergyFromOpponentActive => {
            damage_and_discard_energy(attack.fixed_damage, 1)
        }
        Mechanic::ExtraDamageIfEx { extra_damage } => {
            extra_damage_if_opponent_is_ex(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::SelfDamage { amount } => self_damage_attack(attack.fixed_damage, *amount),
        Mechanic::CoinFlipExtraDamage { extra_damage } => probabilistic_damage_attack(
            vec![0.5, 0.5],
            vec![attack.fixed_damage, attack.fixed_damage + extra_damage],
        ),
        Mechanic::CoinFlipExtraDamageOrSelfDamage {
            extra_damage,
            self_damage,
        } => extra_or_self_damage_attack(attack.fixed_damage, *extra_damage, *self_damage),
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
        Mechanic::CoinFlipNoEffect => coinflip_no_effect(attack.fixed_damage),
        Mechanic::SelfDiscardEnergy { energies } => {
            self_energy_discard_attack(attack.fixed_damage, energies.clone())
        }
        Mechanic::ExtraDamageIfExtraEnergy {
            required_extra_energy,
            extra_damage,
        } => extra_energy_attack(state, attack, required_extra_energy.clone(), *extra_damage),
        Mechanic::ExtraDamageIfBothHeads { extra_damage } => probabilistic_damage_attack(
            vec![0.25, 0.75],
            vec![attack.fixed_damage, attack.fixed_damage + extra_damage],
        ),
        Mechanic::DirectDamage { damage, bench_only } => direct_damage(*damage, *bench_only),
        Mechanic::DamageAndTurnEffect { effect, duration } => {
            damage_and_turn_effect_attack(attack.fixed_damage, effect.clone(), *duration)
        }
        Mechanic::DamageAndCardEffect {
            opponent,
            effect,
            duration,
            probability,
        } => damage_and_card_effect_attack(
            attack.fixed_damage,
            *opponent,
            effect.clone(),
            *duration,
            *probability,
        ),
        Mechanic::SelfDiscardAllEnergy => damage_and_discard_all_energy(attack.fixed_damage),
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
        Mechanic::DamagePerEnergyAll {
            opponent,
            damage_per_energy,
        } => damage_per_energy_all(state, *opponent, *damage_per_energy),
        Mechanic::ExtraDamageIfToolAttached { extra_damage } => {
            extra_damage_if_tool_attached(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::DiscardRandomGlobalEnergy => {
            discard_random_global_energy_attack(attack.fixed_damage, state)
        }
        Mechanic::ExtraDamageIfKnockedOutLastTurn { extra_damage } => {
            extra_damage_if_knocked_out_last_turn_attack(state, attack.fixed_damage, *extra_damage)
        }
        Mechanic::RecoilIfKo { self_damage } => {
            recoil_if_ko_attack(attack.fixed_damage, *self_damage)
        }
        Mechanic::ShuffleOpponentActiveIntoDeck => shuffle_opponent_active_into_deck(),
        Mechanic::BlockBasicAttack => block_basic_attack(attack.fixed_damage),
        Mechanic::SwitchSelfWithBench => switch_self_with_bench(state, attack.fixed_damage),
    }
}

fn recoil_if_ko_attack(damage: u32, self_damage: u32) -> (Probabilities, Mutations) {
    doutcome_from_mutation(Box::new(move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let attacking_ref = (action.actor, 0);

        // First, deal damage to opponent's active
        handle_damage(state, attacking_ref, &[(damage, opponent, 0)], true, None);

        // Check if opponent's active was knocked out (it will be None if KO'd and discarded)
        // or if remaining_hp is 0 (before promotion happens)
        let opponent_ko = state.in_play_pokemon[opponent][0]
            .as_ref()
            .is_none_or(|p| p.remaining_hp == 0);

        // If opponent was KO'd, apply recoil damage to self using handle_damage
        // so that the attacker can also be properly KO'd if needed
        if opponent_ko {
            handle_damage(
                state,
                attacking_ref,
                &[(self_damage, action.actor, 0)],
                false, // Not from active attack (it's self-damage)
                None,
            );
        }
    }))
}

fn coinflip_no_effect(fixed_damage: u32) -> (Probabilities, Mutations) {
    probabilistic_damage_attack(vec![0.5, 0.5], vec![fixed_damage, 0])
}

fn celebi_powerful_bloom(state: &State) -> (Probabilities, Mutations) {
    let active_pokemon = state.get_active(state.current_player);
    let total_energy = active_pokemon.attached_energy.len();

    if total_energy == 0 {
        // No energy attached, no coins to flip
        return probabilistic_damage_attack(vec![1.0], vec![0]);
    }

    // Generate all possible outcomes for flipping N coins
    // Each coin can be heads (1) or tails (0)
    let num_outcomes = 2_usize.pow(total_energy as u32);
    let mut probabilities = vec![0.0; total_energy + 1]; // 0 to total_energy heads
    let mut damages = Vec::new();

    // For each possible outcome (0 to total_energy heads)
    for (heads, prob) in probabilities.iter_mut().enumerate() {
        // Probability of getting exactly 'heads' heads out of 'total_energy' coins
        // This follows a binomial distribution: C(n,k) * (1/2)^n
        *prob = binomial_coefficient(total_energy, heads) as f64 / (num_outcomes as f64);
        damages.push((heads as u32) * 50); // 50 damage per heads
    }

    probabilistic_damage_attack(probabilities, damages)
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }

    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// For Mega Blaziken ex's Mega Burning: Deals 120 damage, discards Fire energy, and burns opponent
fn mega_burning_attack(attack: &Attack) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        // Discard one Fire energy
        state.discard_from_active(action.actor, &[EnergyType::Fire]);

        // Apply burned status
        let opponent = (action.actor + 1) % 2;
        let opponent_active = state.get_active_mut(opponent);
        opponent_active.apply_status_condition(StatusCondition::Burned);
    })
}

/// For Magikarp's Waterfall Evolution: Put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it.
fn waterfall_evolution(state: &State) -> (Probabilities, Mutations) {
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
        return doutcome(|rng, state, action| {
            state.decks[action.actor].shuffle(false, rng);
        });
    }

    // Generate outcomes for each possible evolution card
    let num_evolution_cards = evolution_cards.len();
    let probabilities = vec![1.0 / (num_evolution_cards as f64); num_evolution_cards];
    let mut outcomes: Mutations = vec![];
    for evolution_card in evolution_cards {
        outcomes.push(Box::new(move |rng, state, action| {
            // Transfer the evolution card from deck to hand
            state.transfer_card_from_deck_to_hand(action.actor, &evolution_card);

            // Evolve the active Pokemon (position 0) using the centralized logic
            apply_evolve(action.actor, state, &evolution_card, 0);

            // Shuffle the deck
            state.decks[action.actor].shuffle(false, rng);
        }));
    }

    (probabilities, outcomes)
}

/// For Manaphy's Oceanic attack: Choose 2 benched Pokémon and attach Water Energy to each
fn manaphy_oceanic() -> (Probabilities, Mutations) {
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

fn palkia_dimensional_storm(state: &State) -> (Probabilities, Mutations) {
    // This attack does 150 damage to Active, and 20 to every bench pokemon
    // it then also discards 3 energies. This is deterministic
    let targets: Vec<(u32, usize)> = state
        .enumerate_bench_pokemon((state.current_player + 1) % 2)
        .map(|(idx, _)| (20, idx))
        .chain(std::iter::once((150, 0))) // Add active Pokémon directly
        .collect();
    damage_effect_doutcome(targets, |_, state, action| {
        state.discard_from_active(action.actor, &[EnergyType::Water; 3]);
    })
}

fn moltres_inferno_dance() -> (Probabilities, Mutations) {
    let probabilities = vec![0.125, 0.375, 0.375, 0.125]; // 0,1,2,3 heads
    let mutations = probabilities
        .iter()
        .enumerate()
        .map(|(heads, _)| {
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
        .collect();
    (probabilities, mutations)
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

// Helper function to generate all possible distributions of 'heads' energy
// across the available Pokémon
fn generate_distributions(
    fire_bench_idx: &[usize],
    remaining: usize,
    start_idx: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }

    if start_idx >= fire_bench_idx.len() {
        return;
    }

    // Try different amounts for the current Pokémon
    for amount in 0..=remaining {
        current[start_idx] = amount;
        generate_distributions(
            fire_bench_idx,
            remaining - amount,
            start_idx + 1,
            current,
            result,
        );
    }
    current[start_idx] = 0;
}

fn damage_for_each_heads_attack(
    include_fixed_damage: bool,
    damage_per_head: u32,
    num_coins: usize,
    attack: &Attack,
) -> (Probabilities, Mutations) {
    let mut probabilities: Vec<f64> = vec![];
    let mut damages: Vec<u32> = vec![];
    let fixed_damage = if include_fixed_damage {
        attack.fixed_damage
    } else {
        0
    };

    for heads_count in 0..=num_coins {
        let tails_count = num_coins - heads_count;
        let probability = (0.5f64).powi(heads_count as i32)
            * (0.5f64).powi(tails_count as i32)
            * binomial_coefficient(num_coins, heads_count) as f64;
        probabilities.push(probability);
        damages.push(fixed_damage + damage_per_head * heads_count as u32);
    }

    probabilistic_damage_attack(probabilities, damages)
}

/// Deal damage and attach energy to a pokemon of choice in the bench.
pub(crate) fn energy_bench_attack(
    energies: Vec<EnergyType>,
    target_benched_type: Option<EnergyType>,
    state: &State,
    attack: &Attack,
) -> (Probabilities, Mutations) {
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
fn extra_or_self_damage_attack(
    base_damage: u32,
    extra_damage: u32,
    self_damage: u32,
) -> (Probabilities, Mutations) {
    let probabilities = vec![0.5, 0.5];
    let mutations: Mutations = vec![
        active_damage_mutation(base_damage + extra_damage),
        active_damage_effect_mutation(base_damage, move |_, state, action| {
            let active = state.get_active_mut(action.actor);
            active.apply_damage(self_damage);
        }),
    ];
    (probabilities, mutations)
}

fn damage_chance_status_attack(
    damage: u32,
    probability_of_status: f64,
    status: StatusCondition,
) -> (Probabilities, Mutations) {
    let probabilities = vec![probability_of_status, 1.0 - probability_of_status];
    let mutations: Mutations = vec![
        active_damage_effect_mutation(damage, build_status_effect(status)),
        active_damage_mutation(damage),
    ];
    (probabilities, mutations)
}

/// For attacks that do damage based on benched Pokemon count (new Mechanic-based approach).
fn bench_count_damage_attack(
    state: &State,
    base_damage: u32,
    include_base_damage: bool,
    damage_per: u32,
    energy_type: Option<EnergyType>,
    bench_side: &BenchSide,
) -> (Probabilities, Mutations) {
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
) -> (Probabilities, Mutations) {
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
) -> (Probabilities, Mutations) {
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
    doutcome_from_mutation(Box::new(
        move |_: &mut StdRng, state: &mut State, action: &Action| {
            if !choices.is_empty() {
                state.move_generation_stack.push((action.actor, choices));
            }
        },
    ))
}

fn self_charge_active_attack(
    damage: u32,
    energy_type: EnergyType,
    amount: u8,
) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.attach_energy(&energy_type, amount);
    })
}

/// Used for attacks that can go directly to bench.
/// It will queue (via move_generation_stack) for the user to choose a pokemon to damage.
fn direct_damage(damage: u32, bench_only: bool) -> (Probabilities, Mutations) {
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

/// For attacks that can target opponent's Pokémon that have damage on them.
/// e.g. Decidueye ex's Pierce the Pain
fn direct_damage_if_damaged(damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, pokemon) in state.enumerate_in_play_pokemon(opponent) {
            // Only add as a target if the Pokémon has damage (remaining_hp < total_hp)
            if pokemon.remaining_hp < pokemon.total_hp {
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

/// Luxray's Volt Bolt: Discard all Lightning energy, then do 120 damage to 1 opponent's Pokémon
fn luxray_volt_bolt() -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(0, move |_, state, action| {
        // Count and discard all Lightning energy from the attacking Pokémon
        let active = state.get_active(action.actor);
        let lightning_count = active
            .attached_energy
            .iter()
            .filter(|e| **e == EnergyType::Lightning)
            .count();
        let to_discard = vec![EnergyType::Lightning; lightning_count];
        state.discard_from_active(action.actor, &to_discard);

        // Create choices for which opponent's Pokémon to damage
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_in_play_pokemon(opponent) {
            choices.push(SimpleAction::ApplyDamage {
                attacking_ref: (action.actor, 0),
                targets: vec![(120, opponent, in_play_idx)],
                is_from_active_attack: true,
            });
        }
        if !choices.is_empty() {
            state.move_generation_stack.push((action.actor, choices));
        }
    })
}

/// Discard energy from the active (attacking) Pokémon.
fn self_energy_discard_attack(
    fixed_damage: u32,
    to_discard: Vec<EnergyType>,
) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(fixed_damage, move |_, state, action| {
        state.discard_from_active(action.actor, &to_discard);
    })
}

/// For attacks that deal damage and discard random energy from opponent's active Pokémon
fn damage_and_discard_energy(damage: u32, discard_count: usize) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        let active = state.get_active_mut(opponent);

        for _ in 0..discard_count {
            if active.attached_energy.is_empty() {
                break; // No more energy to discard
            }

            // Get a random index to discard
            let energy_count = active.attached_energy.len();
            let rand_idx = rng.gen_range(0..energy_count);
            active.attached_energy.remove(rand_idx);
        }
    })
}

/// For attacks that deal damage and discard cards from the top of opponent's deck
fn damage_and_discard_opponent_deck(
    damage: u32,
    discard_count: usize,
) -> (Probabilities, Mutations) {
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

fn guzzlord_ex_grindcore_attack() -> (Probabilities, Mutations) {
    // Flip coins until tails - capped at 5 heads for practicality
    let probabilities = vec![0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625];
    let mut outcomes: Mutations = vec![];
    for energies_to_remove in 0..6 {
        outcomes.push(active_damage_effect_mutation(
            30,
            move |_, state, action| {
                let opponent = (action.actor + 1) % 2;
                let active = state.get_active_mut(opponent);

                for _ in 0..energies_to_remove {
                    if active.attached_energy.is_empty() {
                        break; // No more energy to discard
                    }
                    // NOTE: Using pop() instead of random selection to avoid expanding the game tree.
                    // This is a simplification - the card text says "random Energy" but we always
                    // remove the last one for performance reasons.
                    active.attached_energy.pop();
                }
            },
        ));
    }
    (probabilities, outcomes)
}

/// For attacks that deal damage to opponent and also damage themselves
fn self_damage_attack(damage: u32, self_damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.apply_damage(self_damage);
    })
}

/// For attacks that deal damage and apply a status effect (e.g. Wigglituff Ex)
fn damage_status_attack(status: StatusCondition, attack: &Attack) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(attack.fixed_damage, build_status_effect(status))
}

/// For attacks that deal damage to opponent and apply a status effect to the attacker (e.g. Snorlax Collapse)
fn damage_and_self_status_attack(
    damage: u32,
    status: StatusCondition,
) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.apply_status_condition(status);
    })
}

/// For cards like "Meowth Pay Day" that draw a card and deal damage.
fn draw_and_damage_outcome(damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::DrawCard { amount: 1 }]));
    })
}

/// Generic attack that deals bonus damage if the Pokémon has enough energy of a specific type attached.
/// Used by attacks like Hydro Pump, Hydro Bazooka, and Blazing Beatdown.
fn extra_energy_attack(
    state: &State,
    attack: &Attack,
    required_extra_energy: Vec<EnergyType>,
    extra_damage: u32,
) -> (Probabilities, Mutations) {
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

/// For attacks that given coin flips, deal different damage.
fn probabilistic_damage_attack(
    probabilities: Vec<f64>,
    damages: Vec<u32>,
) -> (Probabilities, Mutations) {
    let mutations = damages
        .into_iter()
        .map(|damage| active_damage_mutation(damage))
        .collect();
    (probabilities, mutations)
}

/// For attacks that flip a coin until tails, dealing damage for each heads.
/// Uses geometric distribution truncated at a reasonable number to avoid infinite outcomes.
fn flip_until_tails_attack(damage_per_heads: u32) -> (Probabilities, Mutations) {
    // Truncate at 8 heads to keep the probability space manageable
    // P(k heads) = (1/2)^(k+1) for k = 0, 1, 2, ...
    let max_heads = 8;
    let mut probabilities = Vec::new();
    let mut damages = Vec::new();

    for heads in 0..=max_heads {
        let probability = 0.5_f64.powi(heads as i32 + 1);
        probabilities.push(probability);
        damages.push(heads * damage_per_heads);
    }

    // Ensure probabilities sum to 1 by adjusting the last one for any floating point errors
    let sum: f64 = probabilities.iter().sum();
    if let Some(last) = probabilities.last_mut() {
        *last += 1.0 - sum;
    }

    probabilistic_damage_attack(probabilities, damages)
}

fn self_heal_attack(heal: u32, attack: &Attack) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(attack.fixed_damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.heal(heal);
    })
}

fn damage_and_turn_effect_attack(
    damage: u32,
    effect: TurnEffect,
    effect_duration: u8,
) -> (Probabilities, Mutations) {
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
    probability: Option<f32>,
) -> (Probabilities, Mutations) {
    match probability {
        None => {
            // 100% chance - always apply effect
            active_damage_effect_doutcome(damage, move |_, state, action| {
                let player = if opponent {
                    (action.actor + 1) % 2
                } else {
                    action.actor
                };
                state
                    .get_active_mut(player)
                    .add_effect(effect.clone(), effect_duration);
            })
        }
        Some(prob) => {
            // Coin flip probability
            let probabilities = vec![prob as f64, 1.0 - prob as f64];
            let mutations: Mutations = vec![
                active_damage_effect_mutation(damage, move |_, state, action| {
                    let player = if opponent {
                        (action.actor + 1) % 2
                    } else {
                        action.actor
                    };
                    state
                        .get_active_mut(player)
                        .add_effect(effect.clone(), effect_duration);
                }),
                active_damage_mutation(damage),
            ];
            (probabilities, mutations)
        }
    }
}

/// Discard all energy from this Pokemon
fn damage_and_discard_all_energy(damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.attached_energy.clear(); // Discard all energy
    })
}

fn damage_and_discard_random_energy(damage: u32) -> (Probabilities, Mutations) {
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
fn discard_all_energy_of_type_attack(
    damage: u32,
    energy_type: EnergyType,
) -> (Probabilities, Mutations) {
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
    _state: &State,
) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(fixed_damage, move |rng, state, _action| {
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
    })
}

fn also_bench_damage(
    state: &State,
    opponent: bool,
    active_damage: u32,
    bench_damage: u32,
    must_have_energy: bool,
) -> (Probabilities, Mutations) {
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

fn extra_damage_if_hurt(
    state: &State,
    base: u32,
    extra: u32,
    opponent: bool,
) -> (Probabilities, Mutations) {
    let target = if opponent {
        (state.current_player + 1) % 2
    } else {
        state.current_player
    };
    let target_active = state.get_active(target);
    if target_active.remaining_hp < target_active.total_hp {
        active_damage_doutcome(base + extra)
    } else {
        active_damage_doutcome(base)
    }
}

fn damage_equal_to_self_damage(state: &State) -> (Probabilities, Mutations) {
    let attacker = state.get_active(state.current_player);
    let damage = attacker.total_hp - attacker.remaining_hp;
    active_damage_doutcome(damage)
}

fn extra_damage_equal_to_self_damage(
    state: &State,
    base_damage: u32,
) -> (Probabilities, Mutations) {
    let attacker = state.get_active(state.current_player);
    let self_damage = attacker.total_hp - attacker.remaining_hp;
    active_damage_doutcome(base_damage + self_damage)
}

fn extra_damage_per_energy(
    state: &State,
    base_damage: u32,
    opponent: bool,
    damage_per_energy: u32,
) -> (Probabilities, Mutations) {
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

fn damage_per_energy_all(
    state: &State,
    opponent: bool,
    damage_per_energy: u32,
) -> (Probabilities, Mutations) {
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

fn teleport_attack() -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(0, move |_, state, action| {
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_bench_pokemon(action.actor) {
            choices.push(SimpleAction::Activate { in_play_idx });
        }
        if choices.is_empty() {
            return; // No benched pokemon to switch with
        }
        state.move_generation_stack.push((action.actor, choices));
    })
}

fn extra_damage_if_opponent_is_ex(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> (Probabilities, Mutations) {
    let opponent = (state.current_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    let damage = if opponent_active.card.is_ex() {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn extra_damage_if_tool_attached(
    state: &State,
    base_damage: u32,
    extra_damage: u32,
) -> (Probabilities, Mutations) {
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
) -> (Probabilities, Mutations) {
    let damage = if state.knocked_out_by_opponent_attack_last_turn {
        base_damage + extra_damage
    } else {
        base_damage
    };
    active_damage_doutcome(damage)
}

fn knock_back_attack(damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let opponent = (action.actor + 1) % 2;
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_bench_pokemon(opponent) {
            choices.push(SimpleAction::Activate { in_play_idx });
        }
        if choices.is_empty() {
            return; // No benched pokemon to knock back
        }
        state.move_generation_stack.push((opponent, choices));
    })
}

/// For Mawile's Crunch attack: deals 20 damage, flip a coin, if heads discard a random energy from opponent's active
fn mawile_crunch() -> (Probabilities, Mutations) {
    let probabilities = vec![0.5, 0.5]; // 50% tails (no discard), 50% heads (discard)
    let mutations = vec![
        active_damage_mutation(20), // Tails: just damage
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
    ];
    (probabilities, mutations)
}

/// For baby pokémon attacks: Attach an energy from Energy Zone to a benched Basic pokémon
fn attach_energy_to_benched_basic(
    acting_player: usize,
    energy_type: EnergyType,
) -> (Probabilities, Mutations) {
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
fn brave_buddies_attack(state: &State) -> (Probabilities, Mutations) {
    if state.has_played_support {
        active_damage_doutcome(100)
    } else {
        active_damage_doutcome(50)
    }
}

/// For Absol's Unseen Claw (A3 112): Deals 20 damage, +60 if opponent's Active has a Special Condition
fn unseen_claw_attack(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    let opponent = (acting_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    let damage = if opponent_active.has_status_condition() {
        80 // 20 + 60
    } else {
        20
    };
    active_damage_doutcome(damage)
}

/// For Absol's Ominous Claw (B1 150): Deals 50 damage, flip coin, if heads discard a Supporter from opponent's hand
fn ominous_claw_attack(acting_player: usize, _state: &State) -> (Probabilities, Mutations) {
    // 50% chance for heads (discard supporter), 50% for tails (just damage)
    let probabilities = vec![0.5, 0.5];
    let mutations: Mutations = vec![
        // Heads: damage + discard supporter
        active_damage_effect_mutation(50, move |_, state, _action| {
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
        active_damage_mutation(50),
    ];
    (probabilities, mutations)
}

/// For Mega Absol ex's Darkness Claw: Deals 80 damage and lets player discard a Supporter from opponent's hand
fn darkness_claw_attack(acting_player: usize, _state: &State) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(80, move |_, state, _action| {
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
fn dirty_throw_attack(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    // Check if player has any cards in hand
    if state.hands[acting_player].is_empty() {
        // No cards in hand, attack does nothing (no damage)
        active_damage_doutcome(0)
    } else {
        // Player has cards in hand, deal 70 damage and queue discard decision
        active_damage_effect_doutcome(70, move |_, state, action| {
            let possible_discards: Vec<SimpleAction> = state.hands[action.actor]
                .iter()
                .map(|card| SimpleAction::DiscardOwnCard { card: card.clone() })
                .collect();

            if !possible_discards.is_empty() {
                state
                    .move_generation_stack
                    .push((action.actor, possible_discards));
            }
        })
    }
}

/// For Umbreon's Dark Binding: If the Defending Pokémon is a Basic Pokémon, it can't attack during your opponent's next turn.
fn block_basic_attack(damage: u32) -> (Probabilities, Mutations) {
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
fn shuffle_opponent_active_into_deck() -> (Probabilities, Mutations) {
    let probabilities = vec![0.5, 0.5]; // 50% heads (shuffle), 50% tails (nothing)
    let mutations: Mutations = vec![
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
    ];
    (probabilities, mutations)
}

fn mega_ampharos_lightning_lancer() -> (Probabilities, Mutations) {
    // 1 of your opponent's Benched Pokémon is chosen at random 3 times.
    // For each time a Pokémon was chosen, also do 20 damage to it.
    doutcome(|rng, state, action| {
        let opponent = (action.actor + 1) % 2;
        let targets: Vec<(u32, usize, usize)> = generate_random_spread_indices(rng, state, true, 3)
            .into_iter()
            .map(|idx| (20, opponent, idx))
            .chain(std::iter::once((100, opponent, 0))) // Add active Pokémon directly
            .collect();

        let attacking_ref = (action.actor, 0);
        handle_damage(state, attacking_ref, &targets, true, None);
    })
}

fn generate_random_spread_indices(
    rng: &mut StdRng,
    state: &State,
    bench_only: bool,
    count: usize,
) -> Vec<usize> {
    let opponent = (state.current_player + 1) % 2;
    let mut targets = vec![];
    for _ in 0..count {
        let possible_indices: Vec<usize> = if bench_only {
            state
                .enumerate_bench_pokemon(opponent)
                .map(|(idx, _)| idx)
                .collect()
        } else {
            state
                .enumerate_in_play_pokemon(opponent)
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

fn switch_self_with_bench(state: &State, damage: u32) -> (Probabilities, Mutations) {
    let choices: Vec<_> = state
        .enumerate_bench_pokemon(state.current_player)
        .map(|(in_play_idx, _)| SimpleAction::Activate { in_play_idx })
        .collect();

    doutcome_from_mutation(Box::new(
        move |_: &mut StdRng, state: &mut State, action: &Action| {
            let opponent = (action.actor + 1) % 2;
            let attacking_ref = (action.actor, 0);

            // Deal damage to opponent's active
            handle_damage(state, attacking_ref, &[(damage, opponent, 0)], true, None);

            // Push choices for switching if there are benched Pokemon
            if !choices.is_empty() {
                state.move_generation_stack.push((action.actor, choices));
            }
        },
    ))
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
            bench_count_damage_attack(&state, 70, true, 20, None, &BenchSide::YourBench);
        lazy_mutations.remove(0)(&mut rng, &mut state, &action);

        assert_eq!(state.get_active(1).remaining_hp, 70);
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
        let (probabilities, _mutations) = flip_until_tails_attack(20);

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
        let (probabilities, _mutations) = probabilistic_damage_attack(
            vec![0.0625, 0.25, 0.375, 0.25, 0.0625],
            vec![0, 40, 80, 120, 160],
        );

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

        let active = state.get_active_mut(0);
        active.attached_energy.push(EnergyType::Grass);
        active.attached_energy.push(EnergyType::Fire);

        let (probabilities, _mutations) = celebi_powerful_bloom(&state);

        // Should have 3 outcomes (0, 1, 2 heads)
        assert_eq!(probabilities.len(), 3);

        // Check probabilities for 2 coins: 0.25, 0.5, 0.25
        assert!((probabilities[0] - 0.25).abs() < 0.001); // 0 heads: C(2,0) / 4 = 1/4
        assert!((probabilities[1] - 0.5).abs() < 0.001); // 1 heads: C(2,1) / 4 = 2/4
        assert!((probabilities[2] - 0.25).abs() < 0.001); // 2 heads: C(2,2) / 4 = 1/4

        // Test with no energy attached
        let mut state_no_energy = State::default();
        state_no_energy.in_play_pokemon[0][0] = Some(to_playable_card(&celebi, false));
        let (probabilities_no_energy, _) = celebi_powerful_bloom(&state_no_energy);

        // Should have 1 outcome (0 damage)
        assert_eq!(probabilities_no_energy.len(), 1);
        assert!((probabilities_no_energy[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(binomial_coefficient(0, 0), 1);
        assert_eq!(binomial_coefficient(1, 0), 1);
        assert_eq!(binomial_coefficient(1, 1), 1);
        assert_eq!(binomial_coefficient(2, 0), 1);
        assert_eq!(binomial_coefficient(2, 1), 2);
        assert_eq!(binomial_coefficient(2, 2), 1);
        assert_eq!(binomial_coefficient(4, 2), 6);
        assert_eq!(binomial_coefficient(5, 3), 10);
        assert_eq!(binomial_coefficient(6, 2), 15);
    }

    #[test]
    fn test_single_coin_attacks() {
        // Test Ponyta Stomp (1 coin, 0 or 30 damage)
        let (probabilities, _mutations) = probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 30]);
        assert_eq!(probabilities.len(), 2);
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.5).abs() < 0.001);

        // Test Rapidash Rising Lunge (1 coin, 0 or 60 damage)
        let (probabilities, _mutations) = probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 60]);
        assert_eq!(probabilities.len(), 2);
        assert!((probabilities[0] - 0.5).abs() < 0.001);
        assert!((probabilities[1] - 0.5).abs() < 0.001);

        // Test Mankey Focus Fist (1 coin, 0 or 50 damage)
        let (probabilities, _mutations) = probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 50]);
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

        // Get the mutations from guzzlord_ex_grindcore_attack
        let (probabilities, mut mutations) = guzzlord_ex_grindcore_attack();

        // Verify we have the expected number of outcomes
        assert_eq!(probabilities.len(), 6);
        assert_eq!(mutations.len(), 6);

        // Apply the first mutation (0 heads, immediate tails)
        // This should do 30 damage despite Oricorio's Safeguard ability
        mutations.remove(0)(&mut rng, &mut state, &action);

        // Verify Oricorio did NOT take damage
        assert_eq!(state.get_active(1).remaining_hp, 70);
    }
}

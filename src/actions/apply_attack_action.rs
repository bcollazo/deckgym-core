use log::trace;
use rand::Rng;

use crate::{
    attack_ids::AttackId,
    hooks::get_damage_from_attack,
    types::{EnergyType, StatusCondition},
    State,
};

use super::{
    apply_action_helpers::{Mutations, Probabilities},
    mutations::{
        active_damage_doutcome, active_damage_effect_doutcome, active_damage_effect_mutation,
        active_damage_mutation, build_status_effect, damage_effect_doutcome,
        index_active_damage_doutcome,
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
    trace!("Forecasting attack: {:?} {:?}", active, attack);
    if attack.effect.is_none() {
        let damage = get_damage_from_attack(state, acting_player, index, 0);
        active_damage_doutcome(damage)
    } else {
        forecast_effect_attack(acting_player, state, index)
    }
}

fn celebi_powerful_bloom(acting_player: usize, state: &State) -> (Probabilities, Mutations) {
    let active_pokemon = state.get_active(acting_player);
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

/// Handles attacks that have effects.
fn forecast_effect_attack(
    acting_player: usize,
    state: &State,
    index: usize,
) -> (Probabilities, Mutations) {
    let attack_id = {
        let active = state.get_active(acting_player);
        AttackId::from_pokemon_index(&active.get_id()[..], index).unwrap_or_else(|| {
            panic!(
                "Attack not found for Pokemon: {:?} {:?} {:?}",
                active.card,
                active.card.get_attacks(),
                index
            )
        })
    };
    match attack_id {
        AttackId::A1003VenusaurMegaDrain => self_heal_attack(30, index),
        AttackId::A1004VenusaurExGiantBloom => self_heal_attack(30, index),
        AttackId::A1013VileplumeSoothingScent => damage_status_attack(80, StatusCondition::Asleep),
        AttackId::A1017VenomothPoisonPowder => damage_status_attack(30, StatusCondition::Poisoned),
        AttackId::A1022ExeggutorStomp => probabilistic_damage_attack(vec![0.5, 0.5], vec![30, 60]),
        AttackId::A1023ExeggutorExTropicalSwing => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![40, 80])
        }
        AttackId::A1024TangelaAbsorb => self_heal_attack(10, index),
        AttackId::A1026PinsirDoubleHorn => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 50, 100])
        }
        AttackId::A1029PetililBlot => self_heal_attack(10, index),
        AttackId::A1030LilligantLeafSupply => energy_bench_attack(0, 1, EnergyType::Grass),
        AttackId::A1031Skiddo => probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 40]),
        AttackId::A1033CharmanderEmber => self_energy_discard_attack(0, vec![EnergyType::Fire]),
        AttackId::A1035CharizardFireSpin => {
            self_energy_discard_attack(0, vec![EnergyType::Fire, EnergyType::Fire])
        }
        AttackId::A1036CharizardExCrimsonStorm => {
            self_energy_discard_attack(1, vec![EnergyType::Fire, EnergyType::Fire])
        }
        AttackId::A1038NinetalesFlamethrower => {
            self_energy_discard_attack(0, vec![EnergyType::Fire])
        }
        AttackId::A1040ArcanineHeatTackle => self_damage_attack(100, 20),
        AttackId::A1041ArcanineExInfernoOnrush => self_damage_attack(120, 20),
        AttackId::A1045FlareonFlamethrower => self_energy_discard_attack(0, vec![EnergyType::Fire]),
        AttackId::A1046MoltresSkyAttack => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 130])
        }
        AttackId::A1047MoltresExInfernoDance => moltres_inferno_dance(),
        AttackId::A1052CentiskorchFireBlast => {
            self_energy_discard_attack(0, vec![EnergyType::Fire])
        }
        AttackId::A1055BlastoiseHydroPump => hydro_pump_attack(acting_player, state, 80, 5, 60),
        AttackId::A1056BlastoiseExHydroBazooka => {
            hydro_pump_attack(acting_player, state, 100, 5, 60)
        }
        AttackId::A1057PsyduckHeadache => damage_and_turn_effect_attack(0, 1),
        AttackId::A1063TentacruelPoisonTentacles => {
            damage_status_attack(50, StatusCondition::Poisoned)
        }
        AttackId::A1069KinglerKOCrab => {
            probabilistic_damage_attack(vec![0.25, 0.75], vec![160, 80])
        }
        AttackId::A1071SeadraWaterArrow => direct_damage(50, false),
        AttackId::A1073SeakingHornHazard => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![80, 0])
        }
        AttackId::A1078GyaradosHyperBeam => damage_and_discard_energy(100, 1),
        AttackId::A1079LaprasHydroPump => hydro_pump_attack(acting_player, state, 20, 4, 70),
        AttackId::A1080VaporeonBubbleDrain => self_heal_attack(30, 0),
        AttackId::A1083ArticunoIceBeam => {
            damage_chance_status_attack(60, 0.5, StatusCondition::Paralyzed)
        }
        AttackId::A1084ArticunoExBlizzard => articuno_ex_blizzard(state),
        AttackId::A1091BruxishSecondStrike => extra_damage_if_hurt(10, 60, acting_player, state),
        AttackId::A1093FrosmothPowderSnow => damage_status_attack(40, StatusCondition::Asleep),
        AttackId::A1095RaichuThunderbolt => thunderbolt_attack(),
        AttackId::A1096PikachuExCircleCircuit => {
            bench_count_attack(acting_player, state, 0, 30, Some(EnergyType::Lightning))
        }
        AttackId::A1101ElectabuzzThunderPunch => extra_or_self_damage_attack(40, 40, 20),
        AttackId::A1102JolteonPinMissile => probabilistic_damage_attack(
            vec![0.0625, 0.25, 0.375, 0.25, 0.0625],
            vec![0, 40, 80, 120, 160],
        ),
        AttackId::A1103ZapdosRagingThunder => self_benched_damage(30, index),
        AttackId::A1104ZapdosExThunderingHurricane => probabilistic_damage_attack(
            vec![0.0625, 0.25, 0.375, 0.25, 0.0625],
            vec![0, 50, 100, 150, 200],
        ),
        AttackId::A1106ZebstrikaThunderSpear => direct_damage(30, false),
        AttackId::A1128MewtwoPowerBlast => {
            self_energy_discard_attack(index, vec![EnergyType::Psychic])
        }
        AttackId::A1129MewtwoExPsydrive => {
            self_energy_discard_attack(index, vec![EnergyType::Psychic, EnergyType::Psychic])
        }
        AttackId::A1136GolurkDoubleLariat => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 100, 200])
        }
        AttackId::A1149GolemDoubleEdge => self_damage_attack(150, 50),
        AttackId::A1153MarowakExBonemerang => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 80, 160])
        }
        AttackId::A1154HitmonleeStretchKick => direct_damage(30, true),
        AttackId::A1165ArbokCorner => damage_and_turn_effect_attack(index, 1),
        AttackId::A1171NidokingPoisonHorn => damage_status_attack(90, StatusCondition::Poisoned),
        AttackId::A1195WigglytuffSleepySong => damage_status_attack(80, StatusCondition::Asleep),
        AttackId::A1196MeowthPayDay => draw_and_damage_outcome(10),
        AttackId::A1201LickitungContinuousLick => flip_until_tails_attack(60),
        AttackId::A1203KangaskhanDizzyPunch => {
            probabilistic_damage_attack(vec![0.25, 0.5, 0.25], vec![0, 30, 60])
        }
        AttackId::A1a003CelebiExPowerfulBloom => celebi_powerful_bloom(acting_player, state),
        AttackId::A1a010PonytaStomp => probabilistic_damage_attack(vec![0.5, 0.5], vec![10, 40]),
        AttackId::A1a011RapidashRisingLunge => {
            probabilistic_damage_attack(vec![0.5, 0.5], vec![40, 100])
        }
        AttackId::A1a021LumineonAqua => direct_damage(50, true),
        AttackId::A1a026RaichuGigashock => {
            let opponent = (state.current_player + 1) % 2;
            let targets: Vec<(u32, usize)> = state
                .enumerate_bench_pokemon(opponent)
                .map(|(idx, _)| (20, idx))
                .chain(std::iter::once((60, 0)))
                .collect();
            damage_effect_doutcome(targets, |_, _, _| {})
        }
        AttackId::A1a030DedenneThunderShock => {
            damage_chance_status_attack(10, 0.5, StatusCondition::Paralyzed)
        }
        AttackId::A1a041MankeyFocusFist => probabilistic_damage_attack(vec![0.5, 0.5], vec![0, 50]),
        AttackId::A1a061EeveeContinuousSteps => flip_until_tails_attack(20),
        AttackId::A2049PalkiaDimensionalStorm => palkia_dimensional_storm(state),
        AttackId::A2119DialgaExMetallicTurbo => energy_bench_attack(index, 2, EnergyType::Metal),
        AttackId::A2a071ArceusExUltimateForce => {
            bench_count_attack(acting_player, state, 70, 20, None)
        }
        AttackId::A2035PiplupHeal | AttackId::PA034PiplupHeal => self_heal_attack(20, index),
    }
}

fn palkia_dimensional_storm(state: &State) -> (Probabilities, Mutations) {
    // This attack does 150 damage to Active, and 20 to every bench pokemon
    // it then also discards 3 energies. This is deterministic
    let targets: Vec<(u32, usize)> = state
        .enumerate_in_play_pokemon((state.current_player + 1) % 2)
        .map(|(idx, _)| (20, idx))
        .chain(std::iter::once((150, 0))) // Add active Pokémon directly
        .collect();
    damage_effect_doutcome(targets, |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.discard_energy(&EnergyType::Water);
        active.discard_energy(&EnergyType::Water);
        active.discard_energy(&EnergyType::Water);
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

/// Deal damage and attach energy to a pokemon of choice in the bench.
fn energy_bench_attack(
    attack_index: usize,
    amount: u32,
    energy: EnergyType,
) -> (Probabilities, Mutations) {
    index_active_damage_doutcome(attack_index, move |_, state, action| {
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_bench_pokemon(action.actor) {
            choices.push(SimpleAction::Attach {
                attachments: vec![(amount, energy, in_play_idx)],
                is_turn_energy: false,
            });
        }
        if choices.is_empty() {
            return; // do nothing, since we use common_attack_mutation, turn should end, and no damage applied.
        }
        state.move_generation_stack.push((action.actor, choices));
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

/// For attacks that do damage for each pokemon (optionally of a type) in your bench.
///  e.g. "Pikachu Ex Circle Circuit".
fn bench_count_attack(
    acting_player: usize,
    state: &State,
    base_damage: u32,
    damage_per: u32,
    energy: Option<EnergyType>,
) -> (Probabilities, Mutations) {
    let mut bench_count = 0;
    for (_, pokemon) in state.enumerate_bench_pokemon(acting_player) {
        if let Some(energy) = energy {
            if pokemon.get_energy_type() == Some(energy) {
                bench_count += 1;
            }
        } else {
            bench_count += 1;
        }
    }
    active_damage_doutcome(base_damage + damage_per * bench_count)
}

/// Used for attacks that can go directly to one of your own benched Pokémon.
fn self_benched_damage(damage: u32, attack_index: usize) -> (Probabilities, Mutations) {
    index_active_damage_doutcome(attack_index, move |_, state, action| {
        let mut choices = Vec::new();
        for (in_play_idx, _) in state.enumerate_bench_pokemon(action.actor) {
            choices.push(SimpleAction::ApplyDamage {
                targets: vec![(damage, in_play_idx)],
            });
        }
        if choices.is_empty() {
            return;
        }
        state.move_generation_stack.push((action.actor, choices));
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
                    targets: vec![(damage, in_play_idx)],
                });
            }
        } else {
            for (in_play_idx, _) in state.enumerate_in_play_pokemon(opponent) {
                choices.push(SimpleAction::ApplyDamage {
                    targets: vec![(damage, in_play_idx)],
                });
            }
        }
        if choices.is_empty() {
            return; // do nothing, since we use common_attack_mutation, turn should end, and no damage applied.
        }
        state.move_generation_stack.push((action.actor, choices));
    })
}

/// Discard energy from the active (attacking) Pokémon.
fn self_energy_discard_attack(
    attack_index: usize,
    to_discard: Vec<EnergyType>,
) -> (Probabilities, Mutations) {
    index_active_damage_doutcome(attack_index, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        for energy in to_discard.iter() {
            active.discard_energy(energy);
        }
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

/// For attacks that deal damage to opponent and also damage themselves
fn self_damage_attack(damage: u32, self_damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.apply_damage(self_damage);
    })
}

/// For attacks that deal damage and apply a status effect (e.g. Wigglituff Ex)
fn damage_status_attack(damage: u32, status: StatusCondition) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, build_status_effect(status))
}

/// For cards like "Meowth Pay Day" that draw a card and deal damage.
fn draw_and_damage_outcome(damage: u32) -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(damage, move |_, state, action| {
        state
            .move_generation_stack
            .push((action.actor, vec![SimpleAction::DrawCard]));
    })
}

// If this Pokemon has at least 2 extra Water Energy attached, this attack does 60 more damage.
/// For water Pokémon with Hydro Pump attack that deals more damage with extra energy
fn hydro_pump_attack(
    acting_player: usize,
    state: &State,
    base_damage: u32,
    energy_threshold: usize, // Minimum total water energy needed for bonus damage
    bonus_damage: u32,       // Extra damage when threshold is met
) -> (Probabilities, Mutations) {
    let pokemon = state.in_play_pokemon[acting_player][0]
        .as_ref()
        .expect("Active Pokemon should be there if attacking");

    // Count total water energy
    let water_energy_count = pokemon
        .attached_energy
        .iter()
        .filter(|&energy| *energy == EnergyType::Water)
        .count();

    // Check if we meet or exceed the energy threshold
    if water_energy_count >= energy_threshold {
        active_damage_doutcome(base_damage + bonus_damage)
    } else {
        active_damage_doutcome(base_damage)
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

fn self_heal_attack(heal: u32, index: usize) -> (Probabilities, Mutations) {
    index_active_damage_doutcome(index, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.heal(heal);
    })
}

fn damage_and_turn_effect_attack(index: usize, effect_duration: u8) -> (Probabilities, Mutations) {
    index_active_damage_doutcome(index, move |_, state, action| {
        let active = state.get_active(action.actor);
        // TODO: Maybe create an EffectId enum and have a mapping between card,attack_idx to effect?
        state.add_turn_effect(active.card.clone(), effect_duration);
    })
}

/// For Raichu's Thunderbolt attack that deals 140 damage and discards all energy
fn thunderbolt_attack() -> (Probabilities, Mutations) {
    active_damage_effect_doutcome(140, move |_, state, action| {
        let active = state.get_active_mut(action.actor);
        active.attached_energy.clear(); // Discard all energy
    })
}

fn articuno_ex_blizzard(state: &State) -> (Probabilities, Mutations) {
    // Blizzard: 80 to active, 10 to each opponent's benched Pokémon
    let opponent = (state.current_player + 1) % 2;
    let mut targets: Vec<(u32, usize)> = state
        .enumerate_bench_pokemon(opponent)
        .map(|(idx, _)| (10, idx))
        .collect();
    // Active Pokémon is always index 0
    targets.push((80, 0));
    damage_effect_doutcome(targets, |_, _, _| {})
}

fn extra_damage_if_hurt(
    base: u32,
    extra: u32,
    acting_player: usize,
    state: &State,
) -> (Probabilities, Mutations) {
    let opponent = (acting_player + 1) % 2;
    let opponent_active = state.get_active(opponent);
    if opponent_active.remaining_hp < opponent_active.total_hp {
        active_damage_doutcome(base + extra)
    } else {
        active_damage_doutcome(base)
    }
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

        let (_, mut lazy_mutations) = bench_count_attack(0, &state, 70, 20, None);
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

        let (probabilities, _mutations) = celebi_powerful_bloom(0, &state);

        // Should have 3 outcomes (0, 1, 2 heads)
        assert_eq!(probabilities.len(), 3);

        // Check probabilities for 2 coins: 0.25, 0.5, 0.25
        assert!((probabilities[0] - 0.25).abs() < 0.001); // 0 heads: C(2,0) / 4 = 1/4
        assert!((probabilities[1] - 0.5).abs() < 0.001); // 1 heads: C(2,1) / 4 = 2/4
        assert!((probabilities[2] - 0.25).abs() < 0.001); // 2 heads: C(2,2) / 4 = 1/4

        // Test with no energy attached
        let mut state_no_energy = State::default();
        state_no_energy.in_play_pokemon[0][0] = Some(to_playable_card(&celebi, false));
        let (probabilities_no_energy, _) = celebi_powerful_bloom(0, &state_no_energy);

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
}

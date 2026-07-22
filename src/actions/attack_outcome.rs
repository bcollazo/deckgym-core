use std::rc::Rc;

use rand::rngs::StdRng;

use crate::hooks::{modify_damage, DamageModifierContext};
use crate::State;

use super::apply_action_helpers::{
    enumerate_guts_survivor_subsets, guts_flipping_targets, handle_damage_only, handle_knockouts,
    set_guts_survivors_to_10, Mutation, Probabilities,
};
use super::outcomes::{generate_sequences_with_heads, CoinPaths, CoinSeq, Outcomes};
use super::{Action, SimpleAction};

/// A single damage target described as plain data: `(amount, is_opponent_target, in_play_idx)`.
/// `is_opponent_target` indicates whether `in_play_idx` refers to a slot on the attacker's
/// opponent's side (true) or the attacker's own side (false). The amount is the *raw*
/// pre-modifier damage; weakness/Giovanni/etc. are applied at resolution time via `modify_damage`.
pub type DamageTarget = (u32, bool, usize);

/// A reusable (multi-call) effect closure. Effects are stored as `Rc<dyn Fn>` rather than the
/// `FnOnce` `Mutation` so that an `AttackOutcome` can be cloned — which the defender
/// damage-prevention transform needs in order to split one branch into a heads variant
/// (active damage prevented) and a tails variant (full damage), both running the same effect.
type SharedEffect = Rc<dyn Fn(&mut StdRng, &mut State, &Action)>;

/// The structured result of (one branch of) an attack: the damage it deals, carried as data,
/// plus the non-damage effects that run before and/or after damage is applied.
///
/// Keeping damage out of the effect closures lets us (a) prevent only the active Pokémon's
/// damage on a defender's coin flip while still running effects, and (b) compute expected
/// damage by inspection without executing any closures.
#[derive(Clone)]
pub struct AttackOutcome {
    /// Damage targets dealt by this outcome (raw, pre-modifier).
    pub damage: Vec<DamageTarget>,
    /// Effect that runs before damage is applied (e.g. discard the defender's tool so that
    /// damage modifiers see the post-discard board).
    pre_damage_effect: Option<SharedEffect>,
    /// Effect that runs after damage is applied (the common case: status, energy moves, etc.).
    post_damage_effect: Option<SharedEffect>,
    /// Opponent in-play indices of Guts Pokémon that survive this outcome's lethal damage
    /// (their coin came up heads): after damage is applied their remaining HP is set to 10,
    /// before knockouts are resolved. Populated by `split_with_guts_survival`.
    guts_survivors: Vec<usize>,
}

impl AttackOutcome {
    /// An outcome that does nothing (no damage, no effect).
    pub fn noop() -> Self {
        Self {
            damage: vec![],
            pre_damage_effect: None,
            post_damage_effect: None,
            guts_survivors: vec![],
        }
    }

    /// Damage-only outcome.
    pub fn damage(targets: Vec<DamageTarget>) -> Self {
        Self {
            damage: targets,
            pre_damage_effect: None,
            post_damage_effect: None,
            guts_survivors: vec![],
        }
    }

    /// Damage plus a post-damage effect.
    pub fn damage_then_effect<F>(targets: Vec<DamageTarget>, effect: F) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self {
            damage: targets,
            pre_damage_effect: None,
            post_damage_effect: Some(Rc::new(effect)),
            guts_survivors: vec![],
        }
    }

    /// A pre-damage effect followed by damage (effect resolves before damage modifiers).
    pub fn effect_then_damage<F>(effect: F, targets: Vec<DamageTarget>) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self {
            damage: targets,
            pre_damage_effect: Some(Rc::new(effect)),
            post_damage_effect: None,
            guts_survivors: vec![],
        }
    }

    /// An effect-only outcome that deals no Pokémon damage of its own.
    pub fn effect_only<F>(effect: F) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self {
            damage: vec![],
            pre_damage_effect: None,
            post_damage_effect: Some(Rc::new(effect)),
            guts_survivors: vec![],
        }
    }

    /// Resolve `is_opponent_target` into concrete player indices for the acting player.
    fn resolved_targets(&self, actor: usize) -> Vec<(u32, usize, usize)> {
        let opponent = (actor + 1) % 2;
        self.damage
            .iter()
            .map(|(amount, is_opponent, idx)| {
                let target_player = if *is_opponent { opponent } else { actor };
                (*amount, target_player, *idx)
            })
            .collect()
    }

    /// Convert this structured outcome into a `Mutation` that applies it to the state:
    /// pre-effect, then damage (with modifiers/counterattacks), then Guts survival,
    /// then post-effect, then knockouts.
    fn into_mutation(self) -> Mutation {
        Box::new(move |rng, state, action| {
            let attacking_ref = (action.actor, 0);
            let resolved = self.resolved_targets(action.actor);

            if let Some(pre) = &self.pre_damage_effect {
                pre(rng, state, action);
            }

            if !resolved.is_empty() {
                let attack_metadata = attack_metadata_from_action(state, action);
                handle_damage_only(
                    state,
                    attacking_ref,
                    &resolved,
                    true,
                    DamageModifierContext {
                        attack_name: attack_metadata.name.as_deref(),
                        attack_effect: attack_metadata.effect.as_deref(),
                    },
                );
            }

            if !self.guts_survivors.is_empty() {
                debug_assert!(!resolved.is_empty(), "Guts survivors require lethal damage");
                let opponent = (action.actor + 1) % 2;
                let survivors: Vec<(usize, usize)> = self
                    .guts_survivors
                    .iter()
                    .map(|idx| (opponent, *idx))
                    .collect();
                set_guts_survivors_to_10(state, &survivors);
            }

            if let Some(post) = &self.post_damage_effect {
                post(rng, state, action);
            }

            // Only resolve knockouts here when this outcome dealt damage. Pure-effect outcomes
            // rely on the catch-all knockout pass in `wrap_with_common_logic`, matching the
            // historical behavior of effect-only outcomes.
            if !resolved.is_empty() {
                handle_knockouts(state, attacking_ref, true);
            }
        })
    }
}

/// A probability distribution over `AttackOutcome`s, mirroring `Outcomes` but with damage
/// carried as data. Built by the attack mechanic helpers and converted to `Outcomes` at the
/// boundary of `forecast_attack`/`forecast_copied_attack`.
pub struct AttackOutcomes {
    branches: Vec<AttackBranch>,
}

struct AttackBranch {
    probability: f64,
    outcome: AttackOutcome,
    coin_paths: CoinPaths,
}

impl AttackOutcomes {
    pub fn single(outcome: AttackOutcome) -> Self {
        Self {
            branches: vec![AttackBranch {
                probability: 1.0,
                outcome,
                coin_paths: CoinPaths::None,
            }],
        }
    }

    /// A single effect-only outcome (no damage). Analogous to `Outcomes::single_fn`.
    pub fn single_effect<F>(effect: F) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self::single(AttackOutcome::effect_only(effect))
    }

    pub fn from_parts(probabilities: Probabilities, outcomes: Vec<AttackOutcome>) -> Self {
        assert_eq!(
            probabilities.len(),
            outcomes.len(),
            "from_parts length mismatch: probabilities={} outcomes={}",
            probabilities.len(),
            outcomes.len()
        );
        let branches = probabilities
            .into_iter()
            .zip(outcomes)
            .map(|(probability, outcome)| AttackBranch {
                probability,
                outcome,
                coin_paths: CoinPaths::None,
            })
            .collect();
        Self { branches }
    }

    pub fn binary_coin(heads: AttackOutcome, tails: AttackOutcome) -> Self {
        Self {
            branches: vec![
                AttackBranch {
                    probability: 0.5,
                    outcome: heads,
                    coin_paths: CoinPaths::Exact(vec![CoinSeq(vec![true])]),
                },
                AttackBranch {
                    probability: 0.5,
                    outcome: tails,
                    coin_paths: CoinPaths::Exact(vec![CoinSeq(vec![false])]),
                },
            ],
        }
    }

    pub fn from_coin_branches(branches: Vec<(f64, AttackOutcome, Vec<CoinSeq>)>) -> Self {
        let branches = branches
            .into_iter()
            .map(|(probability, outcome, sequences)| AttackBranch {
                probability,
                outcome,
                coin_paths: CoinPaths::Exact(sequences),
            })
            .collect();
        Self { branches }
    }

    pub fn binomial_by_heads(
        flips: usize,
        mut make_outcome: impl FnMut(usize) -> AttackOutcome,
    ) -> Self {
        let denominator = 2_usize.pow(flips as u32) as f64;
        let mut branches: Vec<(f64, AttackOutcome, Vec<CoinSeq>)> = vec![];
        for heads in 0..=flips {
            let probability = Outcomes::binomial_coefficient(flips, heads) as f64 / denominator;
            let sequences = generate_sequences_with_heads(flips, heads)
                .into_iter()
                .map(CoinSeq)
                .collect::<Vec<_>>();
            branches.push((probability, make_outcome(heads), sequences));
        }
        Self::from_coin_branches(branches)
    }

    pub fn geometric_until_tails(
        max_heads: usize,
        mut make_outcome: impl FnMut(usize) -> AttackOutcome,
    ) -> Self {
        let mut branches: Vec<(f64, AttackOutcome, Vec<CoinSeq>)> = vec![];
        for heads in 0..=max_heads {
            let mut sequence = vec![true; heads];
            let probability = if heads < max_heads {
                sequence.push(false);
                0.5_f64.powi((heads + 1) as i32)
            } else {
                0.5_f64.powi(heads as i32)
            };
            branches.push((probability, make_outcome(heads), vec![CoinSeq(sequence)]));
        }
        Self::from_coin_branches(branches)
    }

    /// Adapter for effect-only producers that already return an `Outcomes` (e.g. the shared
    /// search/bench helpers). Each branch's `Mutation` becomes a post-damage effect with no
    /// structured damage, preserving probabilities and coin metadata.
    pub fn from_effect_outcomes(outcomes: Outcomes) -> Self {
        let branches = outcomes
            .into_branches_with_coin_paths()
            .into_iter()
            .map(|(probability, mutation, coin_paths)| {
                // Wrap the FnOnce mutation in an Rc<RefCell<Option<..>>> so it can be stored
                // as a (nominally reusable) SharedEffect. Effect-only outcomes are never
                // duplicated by the prevention transform, so it is only ever invoked once.
                let cell = std::cell::RefCell::new(Some(mutation));
                let effect: SharedEffect = Rc::new(move |rng, state, action| {
                    if let Some(m) = cell.borrow_mut().take() {
                        m(rng, state, action);
                    }
                });
                AttackBranch {
                    probability,
                    outcome: AttackOutcome {
                        damage: vec![],
                        pre_damage_effect: None,
                        post_damage_effect: Some(effect),
                        guts_survivors: vec![],
                    },
                    coin_paths,
                }
            })
            .collect();
        Self { branches }
    }

    /// Prepend a nullifying 0.5 gate (heads = the whole attack does nothing), scaling the base
    /// branches by 0.5. Used for confusion and CoinFlipToBlockAttack. Coin metadata is dropped
    /// (these are not card-effect coins owned by the acting player).
    pub fn prepend_nullifying_coin_gate(self) -> Self {
        let mut branches = vec![AttackBranch {
            probability: 0.5,
            outcome: AttackOutcome::noop(),
            coin_paths: CoinPaths::None,
        }];
        for branch in self.branches {
            branches.push(AttackBranch {
                probability: branch.probability * 0.5,
                outcome: branch.outcome,
                coin_paths: CoinPaths::None,
            });
        }
        Self { branches }
    }

    /// Apply the defender's "if any damage is done to this Pokémon by attacks, flip a coin; if
    /// heads, prevent that damage" ability (e.g. Meowth's Carefree Steps) to each opponent
    /// in-play slot in `prevented_indices`.
    ///
    /// The ability applies independently to each such Pokémon — whether Active or Benched — and
    /// only when it actually takes damage in a given branch. Each branch is therefore split into
    /// `2^k` sub-branches (where `k` is the number of those Pokémon taking damage in that branch),
    /// one per combination of heads/tails, removing the damage to the Pokémon whose coin came up
    /// heads while keeping all other damage and all effects. Coin metadata is dropped (these are
    /// the defender's coins, not the acting player's).
    pub fn split_with_damage_prevention(self, prevented_indices: &[usize]) -> Self {
        let mut branches = vec![];
        for branch in self.branches {
            // Only the protected Pokémon that actually take (>0) opponent damage flip a coin.
            let flipping: Vec<usize> = prevented_indices
                .iter()
                .copied()
                .filter(|target_idx| {
                    branch
                        .outcome
                        .damage
                        .iter()
                        .any(|(amount, is_opponent, idx)| {
                            *is_opponent && idx == target_idx && *amount > 0
                        })
                })
                .collect();

            if flipping.is_empty() {
                branches.push(branch);
                continue;
            }

            let combos = 1usize << flipping.len();
            let sub_probability = branch.probability / combos as f64;
            for mask in 0..combos {
                // The subset of flipping Pokémon whose coin came up heads (damage prevented).
                let prevented_now: Vec<usize> = flipping
                    .iter()
                    .enumerate()
                    .filter(|(bit, _)| (mask >> bit) & 1 == 1)
                    .map(|(_, idx)| *idx)
                    .collect();
                let mut outcome = branch.outcome.clone();
                outcome
                    .damage
                    .retain(|(_, is_opponent, idx)| !(*is_opponent && prevented_now.contains(idx)));
                branches.push(AttackBranch {
                    probability: sub_probability,
                    outcome,
                    coin_paths: CoinPaths::None,
                });
            }
        }
        Self { branches }
    }

    /// Apply the defender's "if this Pokémon would be Knocked Out by damage from an attack,
    /// flip a coin; if heads, it is not Knocked Out and its remaining HP becomes 10" ability
    /// (e.g. Ursaluna's Guts) to each opponent in-play slot in `guts_indices`.
    ///
    /// The ability applies independently to each such Pokémon, and only in branches where the
    /// (modified) damage it takes would knock it out. Each such branch is split into `2^k`
    /// sub-branches, one per combination of heads/tails. On heads the damage still applies in
    /// full — so on-damage triggers like Rocky Helmet's counterattack fire normally — and a
    /// post-damage effect then sets the survivor's remaining HP to exactly 10 before knockouts
    /// are resolved. Coin metadata is dropped (these are the defender's coins, not the acting
    /// player's).
    ///
    /// Knock outs are forecast with the pre-attack board (like `expected_damage_to`), so damage
    /// modifiers changed by a branch's own pre-damage effect are not taken into account.
    pub fn split_with_guts_survival(
        self,
        state: &State,
        acting_player: usize,
        attack_name: Option<&str>,
        attack_effect: Option<&str>,
    ) -> Self {
        let opponent = (acting_player + 1) % 2;
        let mut branches = vec![];
        for branch in self.branches {
            // Only the Guts Pokémon that would be knocked out by this branch's damage flip a
            // coin. Guts applies to the defender here, so only opponent-side damage is
            // considered (unlike the ApplyDamage path, which supports arbitrary targets).
            let resolved: Vec<(u32, usize, usize)> = branch
                .outcome
                .damage
                .iter()
                .filter(|(_, is_opponent, _)| *is_opponent)
                .map(|(amount, _, idx)| (*amount, opponent, *idx))
                .collect();
            let flipping = guts_flipping_targets(
                state,
                (acting_player, 0),
                &resolved,
                true,
                DamageModifierContext {
                    attack_name,
                    attack_effect,
                },
            );

            if flipping.is_empty() {
                branches.push(branch);
                continue;
            }

            let subsets = enumerate_guts_survivor_subsets(&flipping);
            let sub_probability = branch.probability / subsets.len() as f64;
            for survivors in subsets {
                let mut outcome = branch.outcome.clone();
                outcome.guts_survivors = survivors.into_iter().map(|(_, idx)| idx).collect();
                branches.push(AttackBranch {
                    probability: sub_probability,
                    outcome,
                    coin_paths: CoinPaths::None,
                });
            }
        }
        Self { branches }
    }

    /// Expected raw+modified damage dealt to a specific target across all branches, computed
    /// purely by inspecting branch data (no closures are executed, no RNG is consumed).
    ///
    /// `attack_name` should be the title of the attack being forecast (used by attack-name
    /// specific damage modifiers).
    ///
    /// This is a public lookup API for callers (e.g. bots/value functions) that want the expected
    /// damage of a forecast attack; it is not yet wired into the default engine paths.
    #[allow(dead_code)]
    pub fn expected_damage_to(
        &self,
        state: &State,
        attacking_ref: (usize, usize),
        target_player: usize,
        target_idx: usize,
        attack_name: Option<&str>,
        attack_effect: Option<&str>,
    ) -> f64 {
        let actor = attacking_ref.0;
        let opponent = (actor + 1) % 2;
        self.branches
            .iter()
            .map(|branch| {
                let raw: u32 = branch
                    .outcome
                    .damage
                    .iter()
                    .filter(|(_, is_opponent, idx)| {
                        let player = if *is_opponent { opponent } else { actor };
                        player == target_player && *idx == target_idx
                    })
                    .map(|(amount, _, idx)| {
                        modify_damage(
                            state,
                            attacking_ref,
                            (*amount, target_player, *idx),
                            true,
                            DamageModifierContext {
                                attack_name,
                                attack_effect,
                            },
                        )
                    })
                    .sum();
                branch.probability * raw as f64
            })
            .sum()
    }

    /// Convenience: expected modified damage dealt to the opponent's Active Pokémon.
    #[allow(dead_code)]
    pub fn expected_damage_to_opponent_active(
        &self,
        state: &State,
        acting_player: usize,
        attack_name: Option<&str>,
        attack_effect: Option<&str>,
    ) -> f64 {
        let opponent = (acting_player + 1) % 2;
        self.expected_damage_to(
            state,
            (acting_player, 0),
            opponent,
            0,
            attack_name,
            attack_effect,
        )
    }

    /// Lower into `Outcomes` and return its `(probabilities, mutations)` branches. Convenience
    /// used by tests that want to apply a specific branch's mutation directly.
    #[cfg(test)]
    pub fn into_branches(self) -> (Probabilities, super::apply_action_helpers::Mutations) {
        self.into_outcomes().into_branches()
    }

    /// Lower the structured distribution into a generic `Outcomes`, converting each
    /// `AttackOutcome` into a `Mutation`. This is the boundary between attack-specific
    /// forecasting and the shared apply/forecast machinery.
    pub fn into_outcomes(self) -> Outcomes {
        let branches = self
            .branches
            .into_iter()
            .map(|branch| {
                (
                    branch.probability,
                    branch.outcome.into_mutation(),
                    branch.coin_paths,
                )
            })
            .collect::<Vec<_>>();
        Outcomes::from_branches_with_coin_paths(branches)
            .expect("attack outcome branches should form a valid distribution")
    }
}

/// Look up the title of the attack being resolved, for attack-name-specific damage modifiers.
struct AttackMetadata {
    name: Option<String>,
    effect: Option<String>,
}

fn attack_metadata_from_action(_state: &State, action: &Action) -> AttackMetadata {
    match &action.action {
        SimpleAction::Attack(attack) => AttackMetadata {
            name: Some(attack.title.clone()),
            effect: attack.effect.clone(),
        },
        _ => AttackMetadata {
            name: None,
            effect: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card_ids::CardId;
    use crate::models::PlayedCard;

    fn state_with_grimer_vs_meowth() -> State {
        let mut state = State::default();
        state.current_player = 0;
        // Grimer (Darkness) attacking Meowth (weakness Fighting) => no weakness multiplier,
        // so modify_damage returns the raw amount.
        state.in_play_pokemon[0][0] = Some(PlayedCard::from_id(CardId::A1174Grimer));
        state.in_play_pokemon[1][0] = Some(PlayedCard::from_id(CardId::B2124Meowth));
        state
    }

    #[test]
    fn expected_damage_reads_branch_data_without_running_closures() {
        let state = state_with_grimer_vs_meowth();
        let outcomes = AttackOutcomes::single(AttackOutcome::damage(vec![(20, true, 0)]));
        let expected = outcomes.expected_damage_to_opponent_active(&state, 0, None, None);
        assert!(
            (expected - 20.0).abs() < 1e-9,
            "expected 20, got {expected}"
        );
    }

    #[test]
    fn expected_damage_halves_under_active_damage_prevention() {
        let state = state_with_grimer_vs_meowth();
        let outcomes = AttackOutcomes::single(AttackOutcome::damage(vec![(20, true, 0)]))
            .split_with_damage_prevention(&[0]);
        // Heads branch (0.5) prevents the active damage, tails branch (0.5) deals 20.
        let expected = outcomes.expected_damage_to_opponent_active(&state, 0, None, None);
        assert!(
            (expected - 10.0).abs() < 1e-9,
            "expected 10, got {expected}"
        );
    }

    #[test]
    fn binomial_distribution_sums_to_one() {
        let outcomes = AttackOutcomes::binomial_by_heads(3, |heads| {
            AttackOutcome::damage(vec![(heads as u32 * 10, true, 0)])
        });
        let total: f64 = outcomes.branches.iter().map(|b| b.probability).sum();
        assert!((total - 1.0).abs() < 1e-9);
    }
}

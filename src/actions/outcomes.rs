use std::rc::Rc;

use rand::rngs::StdRng;

use super::apply_action_helpers::{AdditionalEffect, Mutation, Mutations, Probabilities};
use super::mutations::build_attack_mutation;
use super::Action;
use crate::State;

pub struct Outcomes {
    branches: Vec<OutcomeBranch>,
}

pub struct OutcomeBranch {
    pub probability: f64,
    pub mutation: Mutation,
    pub coin_paths: CoinPaths,
    pub damage_targets: Vec<DamageTarget>,
    pub additional_effect: AdditionalEffect,
}

/// A single damage target produced by an attack, before any per-target
/// modifications (e.g. weakness, Carefree Steps prevention) are applied.
///
/// `is_opponent_target` indicates whether `in_play_idx` refers to a slot on
/// the attacker's opponent's side (true) or the attacker's own side (false).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageTarget {
    pub amount: u32,
    pub is_opponent_target: bool,
    pub in_play_idx: usize,
}

fn no_additional_effect() -> AdditionalEffect {
    Rc::new(|_, _, _| {})
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoinSeq(pub Vec<bool>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoinPaths {
    None,
    Exact(Vec<CoinSeq>),
}

#[derive(Debug)]
pub enum ForecastBuildError {
    EmptyBranches,
    ProbabilityOutOfRange,
    ProbabilitySumInvalid,
    CoinPathsEmpty,
}

impl Outcomes {
    pub fn single(mutation: Mutation) -> Self {
        Self {
            branches: vec![OutcomeBranch {
                probability: 1.0,
                mutation,
                coin_paths: CoinPaths::None,
                damage_targets: vec![],
                additional_effect: no_additional_effect(),
            }],
        }
    }

    pub fn single_fn<F>(f: F) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self::single(Box::new(f))
    }

    /// Tags every branch with the given damage targets and additional effect,
    /// so that downstream consumers (e.g. Carefree Steps) can reason about the
    /// damage an outcome will produce without inspecting its mutation closure.
    pub fn with_damage_info(
        mut self,
        damage_targets: Vec<DamageTarget>,
        additional_effect: AdditionalEffect,
    ) -> Self {
        for branch in &mut self.branches {
            branch.damage_targets = damage_targets.clone();
            branch.additional_effect = additional_effect.clone();
        }
        self
    }

    pub fn has_damage_targets(&self) -> bool {
        self.branches.iter().any(|b| !b.damage_targets.is_empty())
    }

    /// Splits each branch that has damage targets matching `can_prevent` into
    /// sub-branches covering every combination of "prevented" (heads) vs
    /// "not prevented" (tails) for those targets, each with independent 50/50
    /// probability. Targets with zero damage, or not matching `can_prevent`,
    /// are left untouched. Branches without any matching targets are passed
    /// through unchanged.
    pub fn apply_coin_flip_damage_prevention(
        self,
        can_prevent: impl Fn(&DamageTarget) -> bool,
    ) -> Self {
        let mut branches = Vec::with_capacity(self.branches.len());
        for branch in self.branches {
            let preventable: Vec<usize> = branch
                .damage_targets
                .iter()
                .enumerate()
                .filter(|(_, target)| target.amount > 0 && can_prevent(target))
                .map(|(idx, _)| idx)
                .collect();

            if preventable.is_empty() {
                branches.push(branch);
                continue;
            }

            let sub_probability = branch.probability / (1u32 << preventable.len()) as f64;
            for mask in 0..(1usize << preventable.len()) {
                let mut targets = branch.damage_targets.clone();
                for (bit, &idx) in preventable.iter().enumerate() {
                    if (mask >> bit) & 1 == 1 {
                        targets[idx].amount = 0;
                    }
                }
                let additional_effect = branch.additional_effect.clone();
                let mutation = build_attack_mutation(targets.clone(), additional_effect.clone());
                branches.push(OutcomeBranch {
                    probability: sub_probability,
                    mutation,
                    coin_paths: branch.coin_paths.clone(),
                    damage_targets: targets,
                    additional_effect,
                });
            }
        }
        Self { branches }
    }

    // Useful for constructing outcomes that are not based on coin flips, or when coin flip metadata is not needed.
    pub fn from_parts(probabilities: Probabilities, mutations: Mutations) -> Self {
        assert_eq!(
            probabilities.len(),
            mutations.len(),
            "from_parts length mismatch: probabilities={} mutations={}",
            probabilities.len(),
            mutations.len()
        );
        let built = probabilities
            .into_iter()
            .zip(mutations)
            .map(|(probability, mutation)| OutcomeBranch {
                probability,
                mutation,
                coin_paths: CoinPaths::None,
                damage_targets: vec![],
                additional_effect: no_additional_effect(),
            })
            .collect();
        let outcomes = Self { branches: built };
        outcomes
            .validate()
            .expect("probability/mutation branches should be valid");
        outcomes
    }

    pub fn binary_coin(heads_mutation: Mutation, tails_mutation: Mutation) -> Self {
        Self {
            branches: vec![
                OutcomeBranch {
                    probability: 0.5,
                    mutation: heads_mutation,
                    coin_paths: CoinPaths::Exact(vec![CoinSeq(vec![true])]),
                    damage_targets: vec![],
                    additional_effect: no_additional_effect(),
                },
                OutcomeBranch {
                    probability: 0.5,
                    mutation: tails_mutation,
                    coin_paths: CoinPaths::Exact(vec![CoinSeq(vec![false])]),
                    damage_targets: vec![],
                    additional_effect: no_additional_effect(),
                },
            ],
        }
    }

    pub fn from_coin_branches(
        branches: Vec<(f64, Mutation, Vec<CoinSeq>)>,
    ) -> Result<Self, ForecastBuildError> {
        let built = branches
            .into_iter()
            .map(|(probability, mutation, sequences)| OutcomeBranch {
                probability,
                mutation,
                coin_paths: CoinPaths::Exact(sequences),
                damage_targets: vec![],
                additional_effect: no_additional_effect(),
            })
            .collect();
        let outcomes = Self { branches: built };
        outcomes.validate()?;
        Ok(outcomes)
    }

    pub fn binomial_by_heads(
        flips: usize,
        mut make_mutation: impl FnMut(usize) -> Mutation,
    ) -> Self {
        let denominator = 2_usize.pow(flips as u32) as f64;
        let mut branches: Vec<(f64, Mutation, Vec<CoinSeq>)> = vec![];
        for heads in 0..=flips {
            let probability = Self::binomial_coefficient(flips, heads) as f64 / denominator;
            let sequences = generate_sequences_with_heads(flips, heads)
                .into_iter()
                .map(CoinSeq)
                .collect::<Vec<_>>();
            branches.push((probability, make_mutation(heads), sequences));
        }
        Self::from_coin_branches(branches)
            .expect("binomial_by_heads should always create valid branches")
    }

    pub fn geometric_until_tails(
        max_heads: usize,
        mut make_mutation: impl FnMut(usize) -> Mutation,
    ) -> Self {
        let mut branches: Vec<(f64, Mutation, Vec<CoinSeq>)> = vec![];
        for heads in 0..=max_heads {
            let mut sequence = vec![true; heads];
            let probability = if heads < max_heads {
                sequence.push(false);
                0.5_f64.powi((heads + 1) as i32)
            } else {
                0.5_f64.powi(heads as i32)
            };
            branches.push((probability, make_mutation(heads), vec![CoinSeq(sequence)]));
        }
        Self::from_coin_branches(branches)
            .expect("geometric_until_tails should always create valid branches")
    }

    pub fn into_branches(self) -> (Probabilities, Mutations) {
        let mut probabilities = Vec::with_capacity(self.branches.len());
        let mut mutations = Vec::with_capacity(self.branches.len());
        for branch in self.branches {
            probabilities.push(branch.probability);
            mutations.push(branch.mutation);
        }
        (probabilities, mutations)
    }

    pub fn map_mutations(self, mut f: impl FnMut(Mutation) -> Mutation) -> Self {
        let branches = self
            .branches
            .into_iter()
            .map(|branch| OutcomeBranch {
                probability: branch.probability,
                mutation: f(branch.mutation),
                coin_paths: branch.coin_paths,
                damage_targets: branch.damage_targets,
                additional_effect: branch.additional_effect,
            })
            .collect();
        Self { branches }
    }

    /// Forces the first coin in each coin-path branch to be heads.
    ///
    /// Returns:
    /// - `Ok(forced)` when at least one coin-based branch exists and filtering succeeds.
    ///   Probabilities are reweighted and normalized after removing non-heads-first paths.
    /// - `Err(original_or_empty)` when forcing cannot be meaningfully applied.
    ///
    /// `Err` cases:
    /// 1. No coin metadata is present in the outcomes (`CoinPaths::None` everywhere).
    ///    In this case, there is nothing to force, so callers typically keep the original outcomes.
    /// 2. Coin metadata exists, but forcing first-heads removes all remaining probability mass
    ///    (for example, no sequence starts with heads after filtering).
    ///    This avoids constructing an invalid zero-probability distribution.
    pub fn force_first_heads(self) -> Result<Self, Self> {
        let mut saw_coin = false;
        let mut branches: Vec<OutcomeBranch> = vec![];

        for branch in self.branches {
            match branch.coin_paths {
                CoinPaths::None => branches.push(branch),
                CoinPaths::Exact(seqs) => {
                    saw_coin = true;
                    let total = seqs.len();
                    let kept = seqs
                        .into_iter()
                        .filter(|seq| seq.0.first().copied().unwrap_or(false))
                        .collect::<Vec<_>>();

                    if kept.is_empty() {
                        continue;
                    }

                    let scaled_probability = branch.probability * kept.len() as f64 / total as f64;
                    branches.push(OutcomeBranch {
                        probability: scaled_probability,
                        mutation: branch.mutation,
                        coin_paths: CoinPaths::Exact(kept),
                        damage_targets: branch.damage_targets,
                        additional_effect: branch.additional_effect,
                    });
                }
            }
        }

        if !saw_coin {
            return Err(Self { branches });
        }

        let sum: f64 = branches.iter().map(|b| b.probability).sum();
        if sum <= 0.0 {
            return Err(Self { branches });
        }

        for branch in &mut branches {
            branch.probability /= sum;
        }

        Ok(Self { branches })
    }

    pub(crate) fn binomial_coefficient(n: usize, k: usize) -> usize {
        if k > n {
            return 0;
        }
        if k == 0 || k == n {
            return 1;
        }

        let k = k.min(n - k);
        (0..k).fold(1usize, |acc, i| acc * (n - i) / (i + 1))
    }

    fn validate(&self) -> Result<(), ForecastBuildError> {
        if self.branches.is_empty() {
            return Err(ForecastBuildError::EmptyBranches);
        }
        let mut sum = 0.0_f64;
        for branch in &self.branches {
            if !branch.probability.is_finite() || !(0.0..=1.0).contains(&branch.probability) {
                return Err(ForecastBuildError::ProbabilityOutOfRange);
            }
            sum += branch.probability;
            if let CoinPaths::Exact(seqs) = &branch.coin_paths {
                if seqs.is_empty() {
                    return Err(ForecastBuildError::CoinPathsEmpty);
                }
            }
        }
        if (sum - 1.0).abs() > 1e-9 {
            return Err(ForecastBuildError::ProbabilitySumInvalid);
        }
        Ok(())
    }
}

fn generate_sequences_with_heads(flips: usize, heads: usize) -> Vec<Vec<bool>> {
    if flips == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    let max_mask = 1_usize << flips;
    for mask in 0..max_mask {
        if mask.count_ones() as usize == heads {
            let mut seq = Vec::with_capacity(flips);
            for i in 0..flips {
                seq.push(((mask >> i) & 1) == 1);
            }
            out.push(seq);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::Outcomes;

    #[test]
    fn geometric_until_tails_max_heads_5_probabilities() {
        let outcomes = Outcomes::geometric_until_tails(5, |_| Box::new(|_, _, _| {}));
        let (probabilities, _) = outcomes.into_branches();

        let expected = [0.5, 0.25, 0.125, 0.0625, 0.03125, 0.03125];
        assert_eq!(probabilities.len(), expected.len());
        for (actual, exp) in probabilities.iter().zip(expected.iter()) {
            assert!((actual - exp).abs() < 1e-9);
        }
        assert!((probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn binomial_coefficient_sanity() {
        assert_eq!(Outcomes::binomial_coefficient(0, 0), 1);
        assert_eq!(Outcomes::binomial_coefficient(5, 0), 1);
        assert_eq!(Outcomes::binomial_coefficient(5, 5), 1);
        assert_eq!(Outcomes::binomial_coefficient(5, 2), 10);
        assert_eq!(
            Outcomes::binomial_coefficient(5, 2),
            Outcomes::binomial_coefficient(5, 3)
        );
    }
}

use rand::rngs::StdRng;

use super::apply_action_helpers::{Mutation, Mutations, Probabilities};
use super::Action;
use crate::State;

pub struct Outcomes {
    branches: Vec<OutcomeBranch>,
}

pub struct OutcomeBranch {
    pub probability: f64,
    pub mutation: Mutation,
    pub coin_paths: CoinPaths,
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
            }],
        }
    }

    pub fn single_fn<F>(f: F) -> Self
    where
        F: Fn(&mut StdRng, &mut State, &Action) + 'static,
    {
        Self::single(Box::new(f))
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
                },
                OutcomeBranch {
                    probability: 0.5,
                    mutation: tails_mutation,
                    coin_paths: CoinPaths::Exact(vec![CoinSeq(vec![false])]),
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

        let expected = vec![0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625];
        assert_eq!(probabilities.len(), expected.len());
        for (actual, exp) in probabilities.iter().zip(expected.iter()) {
            assert!((actual - exp).abs() < 1e-12);
        }
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

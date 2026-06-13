use super::apply_action_helpers::{AdditionalEffect, Mutation};
use super::mutations::build_attack_mutation;
use super::outcomes::{CoinPaths, OutcomeBranch, Outcomes};

/// A single damage target produced by an attack, before any per-target
/// modifications (e.g. weakness, Carefree Steps prevention) are applied.
///
/// `is_opponent_target` indicates whether `in_play_idx` refers to a slot on
/// the attacker's opponent's side (true) or the attacker's own side (false).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DamageTarget {
    pub amount: u32,
    pub is_opponent_target: bool,
    pub in_play_idx: usize,
}

/// The payload of an `AttackOutcomeBranch`: either an opaque mutation (for
/// mechanics that don't report damage targets) or an explicit set of damage
/// targets plus an additional effect, from which the mutation can be built
/// once any per-target damage prevention has been applied.
pub(crate) enum AttackPayload {
    Opaque(Mutation),
    Damage {
        targets: Vec<DamageTarget>,
        additional_effect: AdditionalEffect,
    },
}

pub(crate) struct AttackOutcomeBranch {
    pub probability: f64,
    pub coin_paths: CoinPaths,
    pub payload: AttackPayload,
}

/// Dedicated outcome type for the attack-forecast pipeline. Unlike the
/// generic `Outcomes` type, branches can carry explicit `DamageTarget`
/// metadata so that per-target effects (e.g. Carefree Steps' independent
/// coin-flip damage prevention) can be applied without losing track of any
/// additional effects (status conditions, etc.) tied to the same branch.
pub(crate) struct AttackOutcomes {
    branches: Vec<AttackOutcomeBranch>,
}

impl AttackOutcomes {
    /// Wraps a plain `Outcomes` value, treating every branch's mutation as
    /// opaque (no damage-target metadata available).
    pub(crate) fn opaque(outcomes: Outcomes) -> Self {
        let branches = outcomes
            .into_raw_branches()
            .into_iter()
            .map(|b| AttackOutcomeBranch {
                probability: b.probability,
                coin_paths: b.coin_paths,
                payload: AttackPayload::Opaque(b.mutation),
            })
            .collect();
        Self { branches }
    }

    /// Builds a single-branch outcome reporting the given damage targets and
    /// additional effect.
    pub(crate) fn from_damage_targets(
        targets: Vec<DamageTarget>,
        additional_effect: AdditionalEffect,
    ) -> Self {
        Self {
            branches: vec![AttackOutcomeBranch {
                probability: 1.0,
                coin_paths: CoinPaths::None,
                payload: AttackPayload::Damage {
                    targets,
                    additional_effect,
                },
            }],
        }
    }

    pub(crate) fn has_damage_targets(&self) -> bool {
        self.branches.iter().any(|b| match &b.payload {
            AttackPayload::Damage { targets, .. } => !targets.is_empty(),
            AttackPayload::Opaque(_) => false,
        })
    }

    /// Splits each branch that has damage targets matching `can_prevent` into
    /// sub-branches covering every combination of "prevented" (heads) vs
    /// "not prevented" (tails) for those targets, each with independent 50/50
    /// probability. Targets with zero damage, or not matching `can_prevent`,
    /// are left untouched. Opaque branches, and branches without any matching
    /// targets, are passed through unchanged.
    pub(crate) fn apply_coin_flip_damage_prevention(
        self,
        can_prevent: impl Fn(&DamageTarget) -> bool,
    ) -> Self {
        let mut branches = Vec::with_capacity(self.branches.len());
        for branch in self.branches {
            let (targets, additional_effect) = match branch.payload {
                AttackPayload::Opaque(mutation) => {
                    branches.push(AttackOutcomeBranch {
                        probability: branch.probability,
                        coin_paths: branch.coin_paths,
                        payload: AttackPayload::Opaque(mutation),
                    });
                    continue;
                }
                AttackPayload::Damage {
                    targets,
                    additional_effect,
                } => (targets, additional_effect),
            };

            let preventable: Vec<usize> = targets
                .iter()
                .enumerate()
                .filter(|(_, target)| target.amount > 0 && can_prevent(target))
                .map(|(idx, _)| idx)
                .collect();

            if preventable.is_empty() {
                branches.push(AttackOutcomeBranch {
                    probability: branch.probability,
                    coin_paths: branch.coin_paths,
                    payload: AttackPayload::Damage {
                        targets,
                        additional_effect,
                    },
                });
                continue;
            }

            let sub_probability = branch.probability / (1u32 << preventable.len()) as f64;
            for mask in 0..(1usize << preventable.len()) {
                let mut targets = targets.clone();
                for (bit, &idx) in preventable.iter().enumerate() {
                    if (mask >> bit) & 1 == 1 {
                        targets[idx].amount = 0;
                    }
                }
                branches.push(AttackOutcomeBranch {
                    probability: sub_probability,
                    coin_paths: branch.coin_paths.clone(),
                    payload: AttackPayload::Damage {
                        targets,
                        additional_effect: additional_effect.clone(),
                    },
                });
            }
        }
        Self { branches }
    }

    /// Converts back to a plain `Outcomes`, building the mutation for any
    /// `Damage` branches via `build_attack_mutation`.
    pub(crate) fn into_outcomes(self) -> Outcomes {
        let branches = self
            .branches
            .into_iter()
            .map(|branch| {
                let mutation = match branch.payload {
                    AttackPayload::Opaque(mutation) => mutation,
                    AttackPayload::Damage {
                        targets,
                        additional_effect,
                    } => build_attack_mutation(targets, additional_effect),
                };
                OutcomeBranch {
                    probability: branch.probability,
                    mutation,
                    coin_paths: branch.coin_paths,
                }
            })
            .collect();
        Outcomes::from_raw_branches(branches)
    }
}

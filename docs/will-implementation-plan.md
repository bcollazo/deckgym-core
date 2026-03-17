# Will Implementation Plan (Card-Effect Coin Flips Only) with Constructor-Controlled `ForecastOutcomes`

## Summary
Implement `Will` by refactoring forecast outputs to a coin-sequence-aware `ForecastOutcomes` model and applying a one-time “force first heads” transform to the next eligible coin event from **Attack, Ability, or Trainer** effects.
Use associated constructors on `ForecastOutcomes` (no separate builder type) so branch creation is controlled and validated centrally.

## In Scope
- Coin flips originating from:
  - Attack effects
  - Ability effects
  - Trainer effects (Supporter/Item/Tool/Stadium)
- `Will` cards:
  - `CardId::A4156Will`
  - `CardId::A4196Will`
- Full migration of in-scope forecast producers to `ForecastOutcomes` constructors.

## Out of Scope
- System/runtime non-card flips (checkup/status/setup/system tosses).

## Data Model

### `ForecastOutcomes`
- `ForecastOutcomes { branches: Vec<OutcomeBranch> }`

### `OutcomeBranch`
- `probability: f64`
- `mutation: Mutation`
- `coin_paths: CoinPaths`

### `CoinPaths`
- `None` for deterministic branches
- `Exact(Vec<CoinSeq>)` for coin-linked branches

### `CoinSeq`
- `CoinSeq(Vec<bool>)` (`true=heads`, `false=tails`)

## Constructor API (Associated Fns on `ForecastOutcomes`)
1. `deterministic(mutation: Mutation) -> Self`
2. `from_deterministic_branches(branches: Vec<(f64, Mutation)>) -> Result<Self, ForecastBuildError>`
3. `binary_coin(heads_mutation: Mutation, tails_mutation: Mutation) -> Self`
4. `from_coin_branches(branches: Vec<(f64, Mutation, Vec<CoinSeq>)>) -> Result<Self, ForecastBuildError>`
5. `binomial_by_heads(flips: usize, make_mutation: impl FnMut(usize) -> Mutation) -> Self`
6. `geometric_until_tails(max_heads: usize, make_mutation: impl FnMut(usize) -> Mutation) -> Self`
7. `custom_coin_paths(...)` as alias to validated coin-branch constructor (optional naming preference)

## Constructor Invariants (enforced centrally)
1. Branch list non-empty.
2. Every probability is finite and in `[0,1]`.
3. Probability sum is `1.0 ± epsilon`.
4. `CoinPaths::None` only used for deterministic branch constructors.
5. `CoinPaths::Exact` must be non-empty per branch.
6. Sequence lengths must be consistent across all coin branches in one `ForecastOutcomes`.
7. No duplicated sequences across branches.

## Will State and Semantics
- Add to `State`:
  - `pending_forced_first_heads: [bool; 2]`
- Add methods:
  - `set_pending_forced_first_heads(player)`
  - `has_pending_forced_first_heads(player) -> bool`
  - `consume_pending_forced_first_heads(player) -> bool`
- Reset at turn transition (expires end of turn if unused).

## Resolution Algorithm (`apply_action`)
1. Forecast returns `ForecastOutcomes`.
2. Before sampling:
   - If actor has pending `Will`:
     - For coin branches (`CoinPaths::Exact`), keep only sequences where first flip is heads.
     - Recompute each branch probability from surviving sequence mass.
     - Normalize probabilities.
     - Consume pending `Will` exactly once when at least one eligible coin sequence was affected.
3. Sample branch with `WeightedIndex`.
4. Apply selected mutation.

## Ownership Rule
- `Will` applies only to flips associated with the acting player’s card effect.
- Opponent-triggered card-effect flips remain unaffected (for example defender `Meowth` prevention flip).

## File-Level Implementation Steps
1. `src/actions/apply_action_helpers.rs` (or new `src/actions/forecast_outcomes.rs`)
   - Add `ForecastOutcomes`, `OutcomeBranch`, `CoinPaths`, `CoinSeq`, constructor impls, validation.
2. `src/actions/apply_action.rs`
   - Update router and execution path to consume `ForecastOutcomes`.
   - Add Will transform hook before sampling.
3. `src/state/mod.rs`
   - Add Will pending state + helper methods + turn reset behavior.
4. `src/move_generation/move_generation_trainer.rs`
   - Add Will IDs as playable support.
5. `src/actions/apply_trainer_action.rs`
   - Add Will effect branch to set pending forced-heads.
   - Migrate trainer coin producers to constructors.
6. `src/actions/apply_attack_action.rs`
   - Migrate all in-scope attack coin producers to constructors:
     - binary coin
     - binomial fixed-N
     - geometric until tails
     - custom grouped outcomes
7. `src/actions/apply_abilities_action.rs`
   - Migrate ability coin producers to constructors.
8. Remove legacy tuple-style construction once all in-scope producers are migrated.

## Tests

### Existing integration tests (must pass)
- `tests/will_test.rs`

### New required tests
1. Constructor validation tests:
   - invalid probability sum rejects
   - duplicate coin sequence rejects
   - inconsistent sequence length rejects
2. `binomial_by_heads` sequence/probability correctness.
3. `geometric_until_tails` capped-path correctness.
4. Will transform tests:
   - first coin forced heads
   - only once per turn
   - expires unused at end turn
   - does not affect opponent-owned coin flips.

## Acceptance Criteria
1. All Attack/Ability/Trainer coin forecasts are expressed through `ForecastOutcomes` constructors.
2. `Will` behavior matches card text:
   - next eligible coin event only
   - first coin forced heads
   - same turn only
   - ownership-respecting
3. Existing and new tests pass.
4. No remaining in-scope coin pathway bypasses constructor-controlled creation.

## Assumptions
1. Trainer includes Supporter/Item/Tool/Stadium.
2. Coin sequence metadata is attached per branch via `CoinPaths::Exact`.
3. Non-card/system flips are intentionally untouched.

# Testing Guide

Rust integration tests are grouped by behavior:

- `tests/engine.rs` covers game API and engine flow.
- `tests/mechanics.rs` covers reusable rules and effect resolution.
- `tests/cards.rs` covers card-specific scenarios across Pokemon, trainers, tools, and stadiums.
- `tests/regressions.rs` covers cross-cutting bug reproductions and migration checks.

Shared integration fixtures live in `tests/support/`. Add new tests to the domain that best matches the behavior under test:

- engine or state flow -> `tests/engine/`
- generic mechanics -> `tests/mechanics/`
- card-specific logic -> `tests/pokemon/`, `tests/trainers/`, `tests/tools/`, or `tests/stadiums/`
- multi-system regressions -> `tests/regressions/`

Useful commands:

```bash
cargo test --test engine
cargo test --test mechanics
cargo test --test cards
cargo test --test regressions
```

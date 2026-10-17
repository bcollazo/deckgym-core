# Testing Guide

Integration tests are grouped into four test binaries (fewer binaries means faster builds and links):

| Binary | Directory | What goes there |
| --- | --- | --- |
| `engine` | `tests/engine/` | Game API, apply-action flow, decks, `play` / replay / external-bot APIs |
| `mechanics` | `tests/mechanics/`, `tests/rules/` | Reusable mechanics (status, knockouts, retreat, abilities) and rules-text accuracy |
| `cards` | `tests/pokemon/`, `tests/trainers/`, `tests/tools/`, `tests/stadiums/` | Card-specific scenarios |
| `regressions` | `tests/regressions/` | Cross-cutting bug reproductions and data-migration checks |

`tests/perf_watch.rs` is deliberately its own binary: perf-watch is process-wide configuration.

## Adding a test

1. Create `tests/<group>/<name>_test.rs`.
2. Register it in that group's `mod.rs` (`mod <name>_test;`). An unregistered file is never compiled or run.

Prefer Game-level tests built with `deckgym::test_support::get_test_game_with_board`.

## Useful commands

```bash
cargo test --features test-utils --test cards
cargo test --features test-utils --test cards -- ogerpon
cargo test --features test-utils --test engine
cargo test --features test-utils --test mechanics
cargo test --features test-utils --test regressions
```

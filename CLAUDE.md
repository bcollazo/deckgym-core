# Coding Guidelines

- Read all *.md files in the repository (including in hidden folders) before starting any task.
- When writing tests to reproduce gameplay bugs or to test attacks,
abilities or trainer cards, unless its a specific in-memory function,
try to make the tests at the Game class public API level. Similar to `test_genome_hacking_uses_copied_attack_as_mew_ex_attack`. In particular,
try to use helper functions like `get_test_game_with_board` to reduce
boilerplate.
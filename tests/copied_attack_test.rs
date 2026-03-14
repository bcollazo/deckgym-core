use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_genome_hacking_copies_simple_damage_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    let Card::Pokemon(opponent_active) = &state.get_active(1).card else {
        panic!("Expected opponent active to be a Pokemon");
    };
    let expected_damage = opponent_active.attacks[0].fixed_damage;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_attack = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 0,
                    attack_index: 0,
                    require_attacker_energy_match: false,
                }
            )
        })
        .expect("Expected copied attack choice for opponent active's first attack")
        .clone();

    game.apply_action(&copied_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - expected_damage,
        "Genome Hacking should deal the copied attack's damage to the opponent active"
    );
}

#[test]
fn test_genome_hacking_uses_copied_attack_as_mew_ex_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Psychic,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1115Abra)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_teleport = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 0,
                    attack_index: 0,
                    require_attacker_energy_match: false,
                }
            )
        })
        .expect("Expected copied choice for Abra's Teleport")
        .clone();

    game.apply_action(&copied_teleport);

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        actions.iter().any(|action| {
            matches!(
                action.action,
                SimpleAction::Activate {
                    player: 0,
                    in_play_idx: 1,
                }
            )
        }),
        "Copied Teleport should queue a switch choice for Mew ex's controller"
    );
    assert!(
        !actions.iter().any(|action| {
            matches!(
                action.action,
                SimpleAction::Activate {
                    player: 1,
                    ..
                }
            )
        }),
        "Copied Teleport should not create switch choices for the opponent"
    );
}

#[test]
fn test_genome_hacking_only_offers_opponent_active_attacks() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Psychic,
            EnergyType::Psychic,
            EnergyType::Psychic,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1115Abra),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        actions.iter().all(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 0,
                    ..
                }
            )
        }),
        "Genome Hacking should only offer attacks from the opponent's Active Pokemon"
    );
}

#[test]
fn test_copy_anything_can_choose_opponent_bench_attack_and_do_nothing_if_energy_does_not_match() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1205Ditto).with_energy(vec![EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1036CharizardEx),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_bench_attack = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 1,
                    attack_index: 0,
                    require_attacker_energy_match: true,
                }
            )
        })
        .expect("Copy Anything should offer attacks from opponent bench Pokemon")
        .clone();

    game.apply_action(&copied_bench_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before,
        "Copy Anything should do nothing when Ditto does not have the copied attack's required Energy"
    );
}

#[test]
fn test_copy_anything_copies_opponent_bench_attack_when_energy_matches() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1205Ditto)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1115Abra),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_bench_attack = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 1,
                    attack_index: 0,
                    require_attacker_energy_match: true,
                }
            )
        })
        .expect("Copy Anything should offer a matching-energy attack from opponent bench")
        .clone();

    game.apply_action(&copied_bench_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - 40,
        "Copy Anything should resolve the copied opponent bench attack when Ditto has the required Energy"
    );
}

#[test]
fn test_copy_a_friend_uses_own_non_ex_bench_attacks_only() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a055Ditto)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1a032MewEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    assert!(
        actions.iter().any(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 0,
                    source_in_play_idx: 1,
                    attack_index: 0,
                    require_attacker_energy_match: true,
                }
            )
        }),
        "Copy a Friend should offer attacks from your non-ex Benched Pokemon"
    );
    assert!(
        !actions.iter().any(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 0,
                    source_in_play_idx: 2,
                    ..
                }
            )
        }),
        "Copy a Friend should not offer attacks from Benched Pokemon ex"
    );

    let copied_bulbasaur_attack = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 0,
                    source_in_play_idx: 1,
                    attack_index: 0,
                    require_attacker_energy_match: true,
                }
            )
        })
        .expect("Expected copied attack from own non-ex bench")
        .clone();

    game.apply_action(&copied_bulbasaur_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - 40,
        "Copy a Friend should resolve the copied non-ex bench attack"
    );
}

#[test]
fn test_genome_hacking_can_copy_a_copy_attack() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Psychic,
            EnergyType::Water,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1205Ditto),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 3;

    let hp_before = state.get_active(1).get_remaining_hp();
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(1),
        is_stack: false,
    });

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);

    let copied_copy_anything = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 0,
                    attack_index: 0,
                    require_attacker_energy_match: false,
                }
            )
        })
        .expect("Genome Hacking should offer Ditto's Copy Anything")
        .clone();

    game.apply_action(&copied_copy_anything);

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let nested_copied_attack = actions
        .iter()
        .find(|action| {
            matches!(
                action.action,
                SimpleAction::UseCopiedAttack {
                    source_player: 1,
                    source_in_play_idx: 1,
                    attack_index: 0,
                    require_attacker_energy_match: true,
                }
            )
        })
        .expect("Copied Copy Anything should offer nested copy choices")
        .clone();

    game.apply_action(&nested_copied_attack);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        hp_before - 40,
        "Genome Hacking should be able to resolve a nested copied attack through Ditto"
    );
}

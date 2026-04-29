use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition, TrainerCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn drain_stack(game: &mut deckgym::Game<'static>) {
    while !game.get_state_clone().move_generation_stack.is_empty() {
        let (_, actions) = game.get_state_clone().generate_possible_actions();
        game.apply_action(&actions[0]);
    }
}

fn setup_cursola_vs_parasect(seed: u64) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 3;
    state.points = [0, 0];
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a035GalarianCursola),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1015Parasect).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    game.set_state(state);
    game
}

fn parasect_attacks(game: &mut deckgym::Game<'static>) {
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
}

fn assert_promotion_is_queued(state: &deckgym::State) {
    let (actor, actions) = state.generate_possible_actions();
    assert!(actor == 0 || actor == 1);
    assert!(actions
        .iter()
        .all(|action| matches!(action.action, SimpleAction::Activate { .. })));
}

#[test]
fn test_galarian_cursola_perish_body_can_ko_attacker_on_heads() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let parasect = get_card_by_enum(CardId::A1015Parasect);
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..100 {
        let mut game = setup_cursola_vs_parasect(seed);
        parasect_attacks(&mut game);
        let state = game.get_state_clone();

        assert!(state.discard_piles[0].contains(&cursola));
        assert_eq!(
            state.points[1], 1,
            "Seed {seed}: Parasect should score Cursola"
        );

        if state.in_play_pokemon[1][0].is_none() {
            saw_heads = true;
            assert!(state.discard_piles[1].contains(&parasect));
            assert_eq!(
                state.points[0], 1,
                "Seed {seed}: Cursola should score Parasect on heads"
            );
            assert_promotion_is_queued(&state);
        } else {
            saw_tails = true;
            assert_eq!(
                state.get_active(1).get_name(),
                "Parasect",
                "Seed {seed}: Parasect should survive on tails"
            );
            assert_eq!(state.points[0], 0);
        }

        if saw_heads && saw_tails {
            break;
        }
    }

    assert!(
        saw_heads,
        "Expected at least one seed where Perish Body hits heads"
    );
    assert!(
        saw_tails,
        "Expected at least one seed where Perish Body hits tails"
    );
}

#[test]
fn test_galarian_cursola_perish_body_does_not_trigger_from_bench() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A4a035GalarianCursola),
        ],
        vec![PlayedCard::from_id(CardId::A1015Parasect)],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(100, 0, 1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert_eq!(state.points, [0, 1]);
    assert_eq!(state.get_active(1).get_name(), "Parasect");
}

#[test]
fn test_galarian_cursola_perish_body_does_not_trigger_from_non_attack_damage() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let mut game = setup_cursola_vs_parasect(0);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(100, 0, 0)],
            is_from_active_attack: false,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert_eq!(state.points, [0, 1]);
    assert_eq!(state.get_active(1).get_name(), "Parasect");
}

#[test]
fn test_galarian_cursola_perish_body_does_not_trigger_from_poison_checkup() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let mut game = setup_cursola_vs_parasect(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.in_play_pokemon[0][0] =
        Some(PlayedCard::from_id(CardId::A4a035GalarianCursola).with_remaining_hp(10));
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert_eq!(state.points, [0, 1]);
    assert_eq!(state.get_active(1).get_name(), "Parasect");
}

#[test]
fn test_galarian_cursola_perish_body_is_not_forced_by_will_from_previous_turn() {
    let will = trainer_from_id(CardId::A4156Will);
    let will_card = Card::Trainer(will.clone());
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..100 {
        let mut game = setup_cursola_vs_parasect(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 3;
        state.hands[0] = vec![will_card.clone()];
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will.clone(),
            },
            is_stack: false,
        });
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        });
        parasect_attacks(&mut game);

        let state = game.get_state_clone();
        if state.in_play_pokemon[1][0].is_none() {
            saw_heads = true;
        } else {
            saw_tails = true;
        }

        if saw_heads && saw_tails {
            break;
        }
    }

    assert!(
        saw_heads,
        "Will should not prevent Perish Body from hitting heads"
    );
    assert!(saw_tails, "Will should not force Perish Body to heads");
}

#[test]
fn test_galarian_cursola_perish_body_does_not_trigger_from_rocky_helmet() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let rocky_helmet = get_card_by_enum(CardId::A2148RockyHelmet);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a035GalarianCursola)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])
                .with_remaining_hp(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1003Venusaur).with_tool(rocky_helmet),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert_eq!(state.points, [0, 1]);
    assert_eq!(state.get_active(1).get_name(), "Venusaur");
}

#[test]
fn test_galarian_cursola_perish_body_can_be_forced_by_will_with_spike_armor() {
    let will = trainer_from_id(CardId::A4156Will);
    let will_card = Card::Trainer(will.clone());
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let sandslash = get_card_by_enum(CardId::A3039AlolanSandslash);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 3;
    state.hands[0] = vec![will_card];
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a035GalarianCursola)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])
                .with_remaining_hp(60),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A3039AlolanSandslash).with_energy(vec![EnergyType::Water]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    drain_stack(&mut game);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert!(state.discard_piles[1].contains(&sandslash));
    assert_eq!(state.points, [1, 1]);
}

#[test]
fn test_galarian_cursola_perish_body_can_be_forced_by_will_with_cursed_prose() {
    let will = trainer_from_id(CardId::A4156Will);
    let will_card = Card::Trainer(will.clone());
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let mismagius = get_card_by_enum(CardId::A4a033Mismagius);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 3;
    state.hands[0] = vec![will_card];
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a035GalarianCursola),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A4a033Mismagius).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    drain_stack(&mut game);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.discard_piles[0].contains(&cursola));
    assert!(state.discard_piles[1].contains(&mismagius));
    assert_eq!(state.points, [1, 1]);
}

#[test]
fn test_mismagius_cursed_prose_does_not_damage_if_target_moves_to_bench() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 3;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a035GalarianCursola),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A4a033Mismagius).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    drain_stack(&mut game);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Activate {
            player: 0,
            in_play_idx: 1,
        },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(!state.discard_piles[0].contains(&cursola));
    assert_eq!(state.points, [0, 0]);
    assert_eq!(
        state.in_play_pokemon[0][1].as_ref().unwrap().get_name(),
        "Galarian Cursola"
    );
}

#[test]
fn test_galarian_cursola_perish_body_tracks_jumpluff_after_breeze_by_switch() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let jumpluff = get_card_by_enum(CardId::A4a003JumpluffEx);
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 1;
        state.turn_count = 3;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A4a035GalarianCursola).with_remaining_hp(70),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![
                PlayedCard::from_id(CardId::A4a003JumpluffEx).with_energy(vec![EnergyType::Grass]),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Activate {
                player: 1,
                in_play_idx: 1,
            },
            is_stack: true,
        });

        let state = game.get_state_clone();
        assert!(state.discard_piles[0].contains(&cursola));
        assert_eq!(state.points[1], 1);
        assert_eq!(state.get_active(1).get_name(), "Charmander");

        if state.discard_piles[1].contains(&jumpluff) {
            saw_heads = true;
            assert_eq!(state.points[0], 2);
        } else {
            saw_tails = true;
            assert_eq!(
                state.in_play_pokemon[1][1].as_ref().unwrap().get_name(),
                "Jumpluff ex"
            );
            assert_eq!(state.points[0], 0);
        }

        if saw_heads && saw_tails {
            break;
        }
    }

    assert!(
        saw_heads,
        "Expected Perish Body heads against benched Jumpluff ex"
    );
    assert!(
        saw_tails,
        "Expected Perish Body tails against benched Jumpluff ex"
    );
}

#[test]
fn test_galarian_cursola_perish_body_triggers_from_scizor_gale_thrust_extra_damage() {
    let cursola = get_card_by_enum(CardId::A4a035GalarianCursola);
    let scizor = get_card_by_enum(CardId::A4123Scizor);
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        let mut active_scizor = PlayedCard::from_id(CardId::A4123Scizor)
            .with_energy(vec![EnergyType::Metal, EnergyType::Metal]);
        active_scizor.moved_to_active_this_turn = true;
        state.current_player = 1;
        state.turn_count = 3;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A4a035GalarianCursola),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![active_scizor, PlayedCard::from_id(CardId::A1033Charmander)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert!(state.discard_piles[0].contains(&cursola));
        assert_eq!(state.points[1], 1);

        if state.discard_piles[1].contains(&scizor) {
            saw_heads = true;
            assert_eq!(state.points[0], 1);
        } else {
            saw_tails = true;
            assert_eq!(state.get_active(1).get_name(), "Scizor");
            assert_eq!(state.points[0], 0);
        }

        if saw_heads && saw_tails {
            break;
        }
    }

    assert!(saw_heads, "Expected Perish Body heads against Scizor");
    assert!(saw_tails, "Expected Perish Body tails against Scizor");
}

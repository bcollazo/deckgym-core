use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{get_initialized_game_with_board, get_test_game_with_board},
    Game,
};

fn find_action<F>(game: &Game, predicate: F) -> Action
where
    F: Fn(&Action) -> bool,
{
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .into_iter()
        .find(predicate)
        .expect("expected action to be available")
}

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), 0, base_hp, vec![], false, vec![])
}

#[test]
fn test_luxray_intimidating_fang_reduces_opponent_active_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A3a015Luxray)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 110);
}

#[test]
fn test_eevee_boosted_evolution_allows_first_turn_evolution_while_active() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        1,
        vec![PlayedCard::from_id(CardId::B1184Eevee)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::A1a019Vaporeon));
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Evolve { in_play_idx: 0, .. })));
}

#[test]
fn test_meloetta_strange_singing_puts_psychic_pokemon_into_hand_at_turn_start() {
    let mut game = get_initialized_game_with_board(
        0,
        1,
        3,
        vec![PlayedCard::from_id(CardId::B2070Meloetta)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A4081Natu),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 2);
    assert!(state.hands[0].iter().any(|card| card.get_id() == "A4 081"));
}

#[test]
fn test_entei_ex_legendary_pulse_draws_at_end_of_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a010EnteiEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.hands[0][0].get_name(), "Bulbasaur");
}

#[test]
fn test_aegislash_cursed_metal_boosts_psychic_attack_damage() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1129MewtwoEx)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::B1172Aegislash),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 100)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 20);
}

#[test]
fn test_togekiss_celestial_blessing_prevents_damage_for_some_seeds() {
    let mut prevented = 0;
    let mut not_prevented = 0;

    for seed in 0..40 {
        let mut game = get_initialized_game_with_board(
            seed,
            1,
            3,
            vec![PlayedCard::from_id(CardId::A4080Togekiss)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        );

        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        match game.get_state_clone().get_active(0).get_remaining_hp() {
            140 => prevented += 1,
            100 => not_prevented += 1,
            hp => panic!("unexpected HP after Celestial Blessing: {hp}"),
        }
    }

    assert!(prevented > 0);
    assert!(not_prevented > 0);
}

#[test]
fn test_dragalge_ex_poison_point_poisons_attacker() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B1160DragalgeEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert!(game.get_state_clone().get_active(0).is_poisoned());
}

#[test]
fn test_carnivine_power_link_boosts_damage_with_arceus_in_play() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a009Carnivine).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A2a071ArceusEx),
        ],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 100)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

#[test]
fn test_leafeon_ex_forest_breath_attaches_grass_energy_to_your_grass_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a010LeafeonEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let attach_action = find_action(&game, |a| {
        matches!(
            &a.action,
            SimpleAction::Attach {
                attachments,
                is_turn_energy: false
            } if attachments.as_slice() == [(1, EnergyType::Grass, 1)]
        )
    });
    game.apply_action(&attach_action);

    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][1]
            .as_ref()
            .expect("Bulbasaur should be in play")
            .attached_energy,
        vec![EnergyType::Grass]
    );
}

#[test]
fn test_misdreavus_infiltrating_inspection_is_a_noop_for_public_game_state() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A4a032Misdreavus));
    state.hands[1].clear();
    state.hands[1].push(get_card_by_enum(CardId::A1002Ivysaur));
    game.set_state(state);

    let place_action = find_action(&game, |a| matches!(a.action, SimpleAction::Place(_, 1)));
    game.apply_action(&place_action);

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), 1);
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Misdreavus should be on the bench")
            .get_name(),
        "Misdreavus"
    );
}

#[test]
fn test_chandelure_slow_sear_discards_top_card_of_opponents_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1043Chandelure)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.decks[1].cards = vec![get_card_by_enum(CardId::A1002Ivysaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.decks[1].cards.is_empty());
    assert_eq!(state.discard_piles[1].len(), 1);
    assert_eq!(state.discard_piles[1][0].get_name(), "Ivysaur");
}

#[test]
fn test_hypno_sleep_pendulum_has_both_coin_flip_outcomes() {
    let mut asleep_count = 0;
    let mut awake_count = 0;

    for seed in 0..40 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::B1306Hypno)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::UseAbility { in_play_idx: 0 },
            is_stack: false,
        });

        if game.get_state_clone().get_active(1).is_asleep() {
            asleep_count += 1;
        } else {
            awake_count += 1;
        }
    }

    assert!(asleep_count > 0);
    assert!(awake_count > 0);
}

#[test]
fn test_butterfree_powder_heal_heals_each_of_your_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1007Butterfree).with_damage(20),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 120);
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Bulbasaur should be in play")
            .get_remaining_hp(),
        60
    );
}

#[test]
fn test_shaymin_fragrant_flower_garden_heals_each_of_your_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4b030Shaymin).with_damage(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 60);
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Bulbasaur should be in play")
            .get_remaining_hp(),
        60
    );
}

#[test]
fn test_garchomp_reckless_shearing_discards_from_hand_and_draws_a_card() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2123Garchomp)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A1001Bulbasaur));
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1002Ivysaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let discard_action = find_action(&game, |a| {
        matches!(a.action, SimpleAction::DiscardOwnCards { .. })
    });
    game.apply_action(&discard_action);
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.hands[0][0].get_name(), "Ivysaur");
    assert!(state.discard_piles[0]
        .iter()
        .any(|card| card.get_name() == "Bulbasaur"));
}

#[test]
fn test_arboliva_extra_heal_heals_pokemon_ex_and_discards_an_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2a009Arboliva),
            PlayedCard::from_id(CardId::A1129MewtwoEx)
                .with_damage(80)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let heal_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::HealAndDiscardEnergy {
                in_play_idx: 1,
                heal_amount: 60,
                ..
            }
        )
    });
    game.apply_action(&heal_action);

    let state = game.get_state_clone();
    let mewtwo = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("Mewtwo ex should still be in play");
    assert_eq!(mewtwo.get_remaining_hp(), 130);
    assert_eq!(mewtwo.attached_energy.len(), 1);
}

#[test]
fn test_skeledirge_passionate_voice_boosts_fire_damage_for_turn() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2a018Skeledirge).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_card_with_base_hp(CardId::A1053Squirtle, 200)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 80);
    assert_eq!(state.get_active(0).attached_energy.len(), 2);
}

#[test]
fn test_quaquaval_torrent_boosts_damage_when_hp_is_50_or_less() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2a024Quaquaval)
            .with_remaining_hp(50)
            .with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Colorless,
            ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

#[test]
fn test_baxcalibur_ice_maker_attaches_water_energy_to_active_water_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::B2a036Baxcalibur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_active(0).attached_energy,
        vec![EnergyType::Water]
    );
}

#[test]
fn test_melmetal_hard_coat_reduces_damage_from_attacks() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1182Melmetal)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 110);
}

#[test]
fn test_armarouge_ex_armor_reduces_damage_from_attacks() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B2a020ArmarougeEx)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 130);
}

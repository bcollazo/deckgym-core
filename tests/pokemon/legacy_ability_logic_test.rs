use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{get_initialized_game, get_test_game_with_board},
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

#[test]
fn test_victreebel_fragrance_trap_switches_benched_basic_to_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1020Victreebel)],
        vec![
            PlayedCard::from_id(CardId::A1002Ivysaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let switch_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::Activate {
                player: 1,
                in_play_idx: 1
            }
        )
    });
    game.apply_action(&switch_action);

    assert_eq!(game.get_state_clone().get_active(1).get_name(), "Bulbasaur");
}

#[test]
fn test_poliwrath_counterattack_damages_attacker() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1061Poliwrath)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
    );
    state.current_player = 1;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

#[test]
fn test_gengar_ex_shadowy_spellbind_blocks_supporters() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1123GengarEx)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].push(get_card_by_enum(CardId::PA007ProfessorsResearch));
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions.iter().any(|action| {
        matches!(
            &action.action,
            SimpleAction::Play { trainer_card } if trainer_card.name == "Professor's Research"
        )
    }));
}

#[test]
fn test_gardevoir_attaches_psychic_energy_to_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1132Gardevoir)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert_eq!(active.attached_energy, vec![EnergyType::Psychic]);
}

#[test]
fn test_vaporeon_wash_out_moves_water_energy_to_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1a019Vaporeon),
            PlayedCard::from_id(CardId::A1055Blastoise).with_energy(vec![EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    let move_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::MoveEnergy {
                from_in_play_idx: 2,
                to_in_play_idx: 0,
                energy_type: EnergyType::Water,
                amount: 1,
            }
        )
    });
    game.apply_action(&move_action);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_energy, vec![EnergyType::Water]);
    assert!(state.in_play_pokemon[0][2]
        .as_ref()
        .expect("Blastoise should be in play")
        .attached_energy
        .is_empty());
}

#[test]
fn test_aerodactyl_ex_primeval_law_blocks_active_evolution() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1a046AerodactylEx)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].push(get_card_by_enum(CardId::A1002Ivysaur));
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Evolve { in_play_idx: 0, .. })));
}

#[test]
fn test_giratina_levitate_allows_retreat_with_energy_attached() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2078Giratina).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Retreat(1))));
}

#[test]
fn test_giratina_ex_broken_space_bellow_attaches_energy_and_offers_end_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2b035GiratinaEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Psychic]
    );

    let (_actor, actions) = state.generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::EndTurn)));
}

#[test]
fn test_solgaleo_ex_rising_road_switches_from_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A3122SolgaleoEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    let activate_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::Activate {
                player: 0,
                in_play_idx: 1
            }
        )
    });
    game.apply_action(&activate_action);

    assert_eq!(
        game.get_state_clone().get_active(0).get_name(),
        "Solgaleo ex"
    );
}

#[test]
fn test_shiinotic_illuminate_puts_pokemon_from_deck_into_hand() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3a027Shiinotic)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(state.hands[0].len(), 1);
    assert_eq!(state.decks[0].cards.len(), 0);
    assert_eq!(state.hands[0][0].get_name(), "Bulbasaur");
}

#[test]
fn test_celesteela_ultra_thrusters_switches_ultra_beast() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3a042Nihilego),
            PlayedCard::from_id(CardId::A3a062Celesteela),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    });

    let activate_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::Activate {
                player: 0,
                in_play_idx: 1
            }
        )
    });
    game.apply_action(&activate_action);

    assert_eq!(
        game.get_state_clone().get_active(0).get_name(),
        "Celesteela"
    );
}

#[test]
fn test_flareon_ex_combust_attaches_from_discard_and_damages_self() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3b009FlareonEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.discard_energies[0] = vec![EnergyType::Fire];
    game.set_state(state);

    let hp_before = game.get_state_clone().get_active(0).get_remaining_hp();
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert_eq!(active.attached_energy, vec![EnergyType::Fire]);
    assert_eq!(active.get_remaining_hp(), hp_before - 20);
}

#[test]
fn test_espeon_ex_psychic_healing_heals_selected_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4083EspeonEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(40),
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
            SimpleAction::Heal {
                in_play_idx: 1,
                amount: 30,
                cure_status: false
            }
        )
    });
    game.apply_action(&heal_action);

    let state = game.get_state_clone();
    let bench = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("Bulbasaur should be in play");
    assert_eq!(bench.get_remaining_hp(), 60);
}

#[test]
fn test_crobat_cunning_link_damages_opponent_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a050Crobat),
            PlayedCard::from_id(CardId::A2a071ArceusEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 40);
}

#[test]
fn test_umbreon_ex_dark_chase_switches_damaged_bench_to_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4112UmbreonEx)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1002Ivysaur).with_damage(20),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 0 },
        is_stack: false,
    });

    let activate_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::Activate {
                player: 1,
                in_play_idx: 1
            }
        )
    });
    game.apply_action(&activate_action);

    assert_eq!(game.get_state_clone().get_active(1).get_name(), "Ivysaur");
}

#[test]
fn test_oricorio_safeguard_prevents_damage_from_ex_attack() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A3066Oricorio)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

#[test]
fn test_komala_comatose_puts_it_to_sleep_on_zone_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3141Komala)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Colorless, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    });

    assert!(game.get_state_clone().get_active(0).is_asleep());
}

#[test]
fn test_zeraora_thunderclap_flash_attaches_at_end_of_first_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3a021Zeraora)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][0]
            .as_ref()
            .expect("Zeraora should be in play")
            .attached_energy,
        vec![EnergyType::Lightning]
    );
}

#[test]
fn test_sylveon_ex_happy_ribbon_draws_two_on_evolve() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1206Eevee)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A3b034SylveonEx));
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1002Ivysaur),
    ];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A3b034SylveonEx),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let draw_action = find_action(&game, |a| {
        matches!(a.action, SimpleAction::DrawCard { amount: 2 })
    });
    game.apply_action(&draw_action);
    game.play_until_stable();

    assert_eq!(game.get_state_clone().hands[0].len(), 2);
}

#[test]
fn test_snorlax_ex_full_mouth_manner_heals_at_end_of_turn() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b057SnorlaxEx).with_damage(20)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][0]
            .as_ref()
            .expect("Snorlax ex should be in play")
            .get_remaining_hp(),
        160
    );
}

#[test]
fn test_milotic_healing_ripples_heals_on_evolve() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4a021Feebas),
            PlayedCard::from_id(CardId::A1053Squirtle).with_damage(60),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::A4a022Milotic));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: get_card_by_enum(CardId::A4a022Milotic),
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });

    let heal_action = find_action(&game, |a| {
        matches!(
            a.action,
            SimpleAction::Heal {
                in_play_idx: 1,
                amount: 60,
                cure_status: false
            }
        )
    });
    game.apply_action(&heal_action);

    assert_eq!(
        game.get_state_clone().in_play_pokemon[0][1]
            .as_ref()
            .expect("Squirtle should be in play")
            .get_remaining_hp(),
        60
    );
}

#[test]
fn test_cresselia_ex_lunar_plumage_heals_on_psychic_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::PA037CresseliaEx).with_damage(40)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Psychic, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 120);
}

#[test]
fn test_ariados_trap_territory_increases_retreat_cost() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1a006Ariados),
        ],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Retreat(1))));
}

#[test]
fn test_wartortle_shell_shield_prevents_bench_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a025RaikouEx)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1a018Wartortle),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let bench_hit_action = find_action(&game, |a| match &a.action {
        SimpleAction::ApplyDamage { targets, .. } => targets
            .iter()
            .any(|(_, player, idx)| *player == 1 && *idx == 1),
        _ => false,
    });
    game.apply_action(&bench_hit_action);

    assert_eq!(
        game.get_state_clone().in_play_pokemon[1][1]
            .as_ref()
            .expect("Wartortle should be in play")
            .get_remaining_hp(),
        90
    );
}

#[test]
fn test_reuniclus_infinite_increase_raises_effective_hp_on_attach() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B1a034Reuniclus).with_damage(20)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, EnergyType::Psychic, 0)],
            is_turn_energy: true,
        },
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 100);
}

#[test]
fn test_goomy_sticky_membrane_blocks_exact_cost_attack() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2091Riolu).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::B1177Goomy)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(0))));
}

#[test]
fn test_aegislash_cursed_metal_boosts_its_own_attack_damage() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B1172Aegislash).with_energy(vec![
                EnergyType::Metal,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][0].is_none());
    assert_eq!(state.points[0], 1);
}

#[test]
fn test_eevee_ex_veevee_volve_allows_eevee_evolution() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3b056EeveeEx)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.hands[0].push(get_card_by_enum(CardId::A1a019Vaporeon));
    game.set_state(state);

    let evolve_action = find_action(&game, |a| {
        matches!(a.action, SimpleAction::Evolve { in_play_idx: 0, .. })
    });
    game.apply_action(&evolve_action);

    assert_eq!(game.get_state_clone().get_active(0).get_name(), "Vaporeon");
}

#[test]
fn test_nihilego_more_poison_increases_poison_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3a042Nihilego)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.apply_status_condition(1, 0, deckgym::models::StatusCondition::Poisoned);
    state.current_player = 0;
    state.turn_count = 3;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 50);
}

#[test]
fn test_shaymin_sky_support_reduces_active_basic_retreat_cost() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A2a069Shaymin),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Retreat(2))));
}

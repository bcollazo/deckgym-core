use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
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

/// When Miraidon ex is placed on the bench, the player should be offered a choice
/// to use Legendary Drive (switch to active + move all energy) or pass (Noop).
#[test]
fn test_legendary_drive_switches_to_active_and_moves_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1053Squirtle).with_energy(vec![EnergyType::Water]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B3a019MiraidonEx));
    game.set_state(state);

    // Place Miraidon ex on bench slot 2 (index 2 since slot 1 already has Squirtle)
    let place_action = find_action(
        &game,
        |a| matches!(a.action, SimpleAction::Place(ref c, 2) if c.get_name() == "Miraidon ex"),
    );
    game.apply_action(&place_action);

    // After placement, the player should be offered UseAbility (Legendary Drive) or Noop
    let state = game.get_state_clone();
    let (_actor, actions) = state.generate_possible_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: 2 })),
        "Legendary Drive should be offered as UseAbility after placing Miraidon ex on bench"
    );
    assert!(
        actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Noop)),
        "Noop should be offered alongside Legendary Drive"
    );

    // Choose to use Legendary Drive (directly switches and moves energy in one step)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 2 },
        is_stack: true,
    });

    // After UseAbility, Miraidon ex should be the active pokemon
    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_name(),
        "Miraidon ex",
        "Miraidon ex should now be the active pokemon after Legendary Drive"
    );

    // All energy from Bulbasaur and Squirtle should have moved to Miraidon ex
    let miraidon_energy = &state.get_active(0).attached_energy;
    assert_eq!(
        miraidon_energy.len(),
        3,
        "Miraidon ex should have all 3 energies (2 from Bulbasaur + 1 from Squirtle)"
    );

    // Bench pokemon should have no energy remaining
    for (_, pokemon) in state.enumerate_bench_pokemon(0) {
        assert!(
            pokemon.attached_energy.is_empty(),
            "{} should have no energy after Legendary Drive moved it all",
            pokemon.get_name()
        );
    }
}

/// Hadron Ray does 20 base + 20 more damage for each [L] Energy attached to Miraidon ex.
#[test]
fn test_hadron_ray_extra_damage_per_lightning_energy() {
    // Miraidon ex with 3 [L] energies: 20 + 3*20 = 80 damage
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3a019MiraidonEx).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Lightning,
            ]),
        ],
        vec![PlayedCard::new(
            get_card_by_enum(CardId::A1001Bulbasaur),
            0,
            200,
            vec![],
            false,
            vec![],
        )],
    );
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a019MiraidonEx, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // 20 + (3 * 20) = 80 damage, opponent had 200 HP → 120 remaining
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        120,
        "Hadron Ray should deal 80 damage with 3 Lightning energies (20 + 3*20)"
    );
}

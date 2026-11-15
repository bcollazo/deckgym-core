use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn attach(energy: EnergyType, in_play_idx: usize) -> Action {
    Action {
        actor: 0,
        action: SimpleAction::Attach {
            attachments: vec![(1, energy, in_play_idx)],
            is_turn_energy: true,
        },
        is_stack: false,
    }
}

#[test]
fn test_charizard_flame_aura_damages_opponent_active_on_fire_attach() {
    // Flame Aura: Whenever you attach a [R] Energy from your Energy Zone to this Pokémon,
    // do 30 damage to your opponent's Active Pokémon.
    for charizard in [CardId::C1018Charizard, CardId::C1136Charizard] {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(charizard)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );

        game.apply_action(&attach(EnergyType::Fire, 0));

        let state = game.get_state_clone();
        assert_eq!(state.get_active(0).attached_energy.len(), 1);
        assert_eq!(state.get_active(1).get_remaining_hp(), 40);
    }
}

#[test]
fn test_charizard_flame_aura_works_from_the_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::C1018Charizard),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&attach(EnergyType::Fire, 1));

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 40);
}

#[test]
fn test_charizard_flame_aura_ignores_non_fire_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1018Charizard)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.apply_action(&attach(EnergyType::Water, 0));

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
}

#[test]
fn test_charizard_flame_aura_skips_when_opponent_has_no_active() {
    // Flareon's Assisting Heater deals 40 damage, then attaches a [R] Energy from the Energy Zone
    // to a Benched Pokémon. When the 40 damage Knocks Out the opponent's Active Pokémon, the
    // attachment resolves while their Active Spot is empty. Attaching to Charizard then triggers
    // Flame Aura with no Active Pokémon to damage, so it must do nothing instead of panicking.
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3b008Flareon)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire]),
            PlayedCard::from_id(CardId::C1018Charizard),
        ],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(40),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b008Flareon, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "the opponent's Active Pokémon should be Knocked Out"
    );
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(
        actor, 0,
        "the attacker chooses where Assisting Heater's Energy goes"
    );
    let attach_to_charizard = actions
        .into_iter()
        .find(|action| {
            matches!(
                &action.action,
                SimpleAction::Attach { attachments, .. } if attachments == &vec![(1, EnergyType::Fire, 1)]
            )
        })
        .expect("attaching to Benched Charizard should be offered");

    game.apply_action(&attach_to_charizard);

    let state = game.get_state_clone();
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Charizard stays in play")
            .attached_energy,
        vec![EnergyType::Fire]
    );
    assert!(state.in_play_pokemon[1][0].is_none());
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("the opponent's Benched Pokémon stays in play")
            .get_remaining_hp(),
        70,
        "Flame Aura must not hit a Benched Pokémon"
    );
}

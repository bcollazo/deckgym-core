use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_cynthia_boosts_garchomp_dragon_claw_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    // Garchomp with energy for Dragon Claw (100 base damage)
    state.set_board(
        vec![PlayedCard::from_id(CardId::A2123Garchomp)
            .with_energy(vec![EnergyType::Water, EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(150)],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::A2152Cynthia)];
    game.set_state(state);

    // Play Cynthia
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let play_cynthia = actions
        .iter()
        .find(|a| matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.name == "Cynthia"))
        .expect("Cynthia should be playable");
    game.apply_action(play_cynthia);

    // Attack with Garchomp (Dragon Claw)
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attack = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Attack(_)))
        .expect("Garchomp should be able to attack");
    game.apply_action(attack);

    // Dragon Claw does 100 base damage, +50 from Cynthia = 150, exactly KOing the 150 HP Bulbasaur
    assert_eq!(
        game.get_state_clone().points[0],
        1,
        "Opponent's Bulbasaur should be KO'd by 150 damage (100 + 50 from Cynthia), scoring a point"
    );
}

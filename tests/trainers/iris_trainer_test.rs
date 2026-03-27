use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    state::GameOutcome,
    test_support::get_initialized_game,
};

#[test]
fn test_iris_haxorus_ko_ex_pokemon_wins_game() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    // Haxorus with energy for Frenzied Blade; Mewtwo ex with only 10 HP remaining
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2b056Haxorus).with_energy(vec![
            EnergyType::Fighting,
            EnergyType::Metal,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx).with_remaining_hp(10)],
    );
    state.hands[0] = vec![get_card_by_enum(CardId::B2b067Iris)];
    game.set_state(state);

    // Play Iris
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let play_iris = actions
        .iter()
        .find(|a| matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.name == "Iris"))
        .expect("Iris should be playable");
    game.apply_action(play_iris);

    // Attack with Haxorus (Frenzied Blade)
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attack = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Attack(_)))
        .expect("Haxorus should be able to attack");
    game.apply_action(attack);

    // Haxorus KOs ex: 2 pts (ex KO) + 1 pt (Iris bonus) = 3 pts = win
    assert_eq!(
        game.get_state_clone().winner,
        Some(GameOutcome::Win(0)),
        "Player 0 should win after Haxorus KOs opponent's Mewtwo ex with Iris active"
    );
}

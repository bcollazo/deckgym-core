use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

#[test]
fn test_mega_camerupt_ex_volcanic_kaboom_knocks_out_only_opponent_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3023MegaCameruptEx).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    assert!(
        game.get_state_clone().maybe_get_active(1).is_none(),
        "Volcanic Kaboom should knock out the only opposing Pokemon"
    );
}

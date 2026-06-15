use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn played_with_hp(card_id: CardId, hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, hp, vec![], false, vec![])
}

/// Wildly Writhe deals 120 damage to the opponent and 60 recoil to Slither Wing.
#[test]
fn test_slither_wing_wildly_writhe_damage_and_self_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a004SlitherWing).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_with_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a004SlitherWing, 0),
        is_stack: false,
    });

    let final_state = game.get_state_clone();

    // Opponent takes 120 damage (200 - 120 = 80 HP remaining)
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    assert_eq!(
        opponent_hp, 80,
        "Wildly Writhe should deal 120 damage (200 - 120 = 80)"
    );

    // Slither Wing takes 60 self-damage (120 - 60 = 60 HP remaining)
    let slither_wing_hp = final_state.get_active(0).get_remaining_hp();
    assert_eq!(
        slither_wing_hp, 60,
        "Slither Wing should take 60 self-damage (120 - 60 = 60 HP remaining)"
    );
}

/// Wildly Writhe self-damage can reduce Slither Wing's HP to 0.
#[test]
fn test_slither_wing_wildly_writhe_self_ko() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            played_with_hp(CardId::B3a004SlitherWing, 60).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Colorless,
            ]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![played_with_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    state.points = [0, 0];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a004SlitherWing, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let final_state = game.get_state_clone();

    // Slither Wing (60 HP remaining before attack) should be KO'd by its own recoil
    assert_eq!(
        final_state.points[1], 1,
        "Opponent should earn a point when Slither Wing KO's itself with recoil"
    );
}

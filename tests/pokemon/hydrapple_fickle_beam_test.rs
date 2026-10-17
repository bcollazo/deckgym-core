use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_hydrapple_fickle_beam_both_heads_deals_bonus_damage() {
    let mut saw_base_damage = false;
    let mut saw_bonus_damage = false;

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B4127Hydrapple)
                .with_energy(vec![EnergyType::Grass, EnergyType::Fire])],
            vec![PlayedCard::from_id(CardId::B4037WailordEx)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B4127Hydrapple, 0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let damage_dealt = 250 - state.get_active(1).get_remaining_hp();
        match damage_dealt {
            100 => saw_base_damage = true,
            200 => saw_bonus_damage = true,
            other => panic!("Unexpected damage dealt: {other} (seed {seed})"),
        }
    }

    assert!(
        saw_base_damage,
        "Fickle Beam should sometimes deal only its base 100 damage"
    );
    assert!(
        saw_bonus_damage,
        "Fickle Beam should sometimes deal 100 bonus damage when both coins are heads"
    );
}

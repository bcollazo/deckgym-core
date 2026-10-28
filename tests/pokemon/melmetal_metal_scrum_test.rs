//! Melmetal's Metal Scrum: "As long as this Pokémon is in play, all of your [M] Pokémon take -20
//! damage from attacks from your opponent's Pokémon. This effect doesn't stack."

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::PlayedCard,
    test_support::get_test_game_with_board,
};

/// Player 0 has `board` (first is Active); player 1 has a plain Active attacker. Applies `damage`
/// from player 1 to player 0's Pokémon at `target` and returns the HP it lost.
fn damage_taken(board: Vec<PlayedCard>, target: usize, damage: u32) -> u32 {
    let mut game =
        get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);

    let hp_before = game.get_state_clone().get_remaining_hp(0, target);
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (1, 0),
            targets: vec![(damage, 0, target)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
    hp_before - game.get_state_clone().get_remaining_hp(0, target)
}

#[test]
fn test_metal_scrum_cuts_damage_to_all_metal_pokemon() {
    let board = || {
        vec![
            PlayedCard::from_id(CardId::C1097Melmetal),
            PlayedCard::from_id(CardId::C1096Meltan),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ]
    };
    assert_eq!(damage_taken(board(), 0, 50), 30, "Melmetal is [M] itself");
    assert_eq!(
        damage_taken(board(), 1, 50),
        30,
        "Benched [M] Pokémon are covered"
    );
    assert_eq!(
        damage_taken(board(), 2, 50),
        50,
        "Non-[M] Pokémon are not covered"
    );
}

#[test]
fn test_metal_scrum_does_not_stack() {
    let board = vec![
        PlayedCard::from_id(CardId::C1097Melmetal),
        PlayedCard::from_id(CardId::C1150Melmetal),
        PlayedCard::from_id(CardId::C1096Meltan),
    ];
    assert_eq!(damage_taken(board, 2, 50), 30);
}

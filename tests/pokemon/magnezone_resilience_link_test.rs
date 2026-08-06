use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Attacks Magnezone (A2a 055) with Dusknoir's 80-damage Hammer In and returns Magnezone's
/// remaining HP. Magnezone is weak to Fire, so a [P] attacker gets no weakness bonus.
fn hammer_in_magnezone(bench: Vec<PlayedCard>) -> u32 {
    let mut defender_board = vec![PlayedCard::from_id(CardId::A2a055Magnezone)];
    defender_board.extend(bench);

    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::B1105Dusknoir)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Colorless])],
        defender_board,
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1105Dusknoir, 0),
        is_stack: false,
    });

    game.get_state_clone().get_active(1).get_remaining_hp()
}

/// Resilience Link only reduces damage while its owner has Arceus (or Arceus ex) in play.
#[test]
fn test_resilience_link_reduces_damage_with_arceus_in_play() {
    let hp = hammer_in_magnezone(vec![PlayedCard::from_id(CardId::A2a070Arceus)]);

    // Magnezone 140 HP - (80 - 30) = 90.
    assert_eq!(
        hp, 90,
        "Resilience Link should reduce the attack by 30 while Arceus is in play"
    );
}

#[test]
fn test_resilience_link_does_nothing_without_arceus_in_play() {
    let hp = hammer_in_magnezone(vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);

    // Magnezone 140 HP - 80 = 60.
    assert_eq!(
        hp, 60,
        "Resilience Link should not reduce damage without Arceus in play"
    );
}

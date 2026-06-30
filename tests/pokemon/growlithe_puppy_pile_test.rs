use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn puppy_pile_damage(bench: Vec<PlayedCard>, hand_puppy_pile_cards: Vec<CardId>) -> u32 {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    let mut player_board = vec![PlayedCard::from_id(CardId::B3b010Growlithe)
        .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])];
    player_board.extend(bench);

    // Use 200 HP to avoid knockout obscuring the damage value.
    let opponent_active = PlayedCard::new(
        get_card_by_enum(CardId::A1001Bulbasaur),
        0,
        200,
        vec![],
        false,
        vec![],
    );
    state.set_board(player_board, vec![opponent_active]);
    state.current_player = 0;

    state.hands[0] = hand_puppy_pile_cards
        .into_iter()
        .map(get_card_by_enum)
        .collect();

    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b010Growlithe, 0),
        is_stack: false,
    });

    game.get_state_clone().get_active(1).get_remaining_hp()
}

#[test]
fn test_puppy_pile_counts_attacker_itself() {
    // Only the active Growlithe with Puppy Pile is in play, none in hand.
    // 1 pokemon × 20 = 20 damage + 20 flat Fire weakness = 40 total.
    // 200 HP - 40 = 160 HP remaining.
    let remaining_hp = puppy_pile_damage(vec![], vec![]);
    assert_eq!(
        remaining_hp, 160,
        "1 Puppy Pile pokemon → 20 damage + 20 weakness = 40; 200 - 40 = 160"
    );
}

#[test]
fn test_puppy_pile_counts_bench_and_hand_pokemon() {
    // Active Growlithe (1) + benched Yamper (1) + 2 in hand = 4 × 20 = 80 damage.
    // Bulbasaur Fire weakness adds flat +20 = 100 total. 200 - 100 = 100 HP remaining.
    let remaining_hp = puppy_pile_damage(
        vec![PlayedCard::from_id(CardId::B3b025Yamper)],
        vec![CardId::B3b010Growlithe, CardId::B3b034Fidough],
    );
    assert_eq!(
        remaining_hp, 100,
        "4 Puppy Pile pokemon → 80 damage + 20 weakness = 100; 200 - 100 = 100"
    );
}

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::{attack_action, get_initialized_game},
};

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn bulbasaur_with_hp(hp: u32) -> PlayedCard {
    PlayedCard::new(
        get_card_by_enum(CardId::A1001Bulbasaur),
        0,
        hp,
        vec![],
        false,
        vec![],
    )
}

/// Flutter Mane's first Spellbinding Start blocks opponent from playing trainers next turn.
#[test]
fn test_flutter_mane_spellbinding_start_first_attack_blocks_trainers() {
    let potion = trainer_from_id(CardId::PA001Potion);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a026FlutterManeEx)
            .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
        vec![bulbasaur_with_hp(200)],
    );
    game.set_state(state);

    // First attack
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a026FlutterManeEx, 0),
        is_stack: false,
    });
    game.play_until_stable();
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    // Now it's player 1's turn — give them a trainer card
    let mut state = game.get_state_clone();
    state.hands[1] = vec![Card::Trainer(potion.clone())];
    game.set_state(state);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1, "It should be player 1's turn");
    let can_play_trainer = actions.iter().any(|a| {
        matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.id == potion.id)
    });
    assert!(
        !can_play_trainer,
        "Spellbinding Start should prevent opponent from playing trainer cards"
    );
}

/// Flutter Mane's second Spellbinding Start (not first attack) does NOT block trainers.
#[test]
fn test_flutter_mane_spellbinding_start_second_attack_no_trainer_block() {
    let potion = trainer_from_id(CardId::PA001Potion);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 1;

    let mut flutter_mane = PlayedCard::from_id(CardId::B3a026FlutterManeEx)
        .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic]);
    // Simulate that this Pokémon has already attacked since coming into play
    flutter_mane.has_attacked_since_play = true;

    state.set_board(vec![flutter_mane], vec![bulbasaur_with_hp(200)]);
    game.set_state(state);

    // Second attack — no first-attack bonus
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a026FlutterManeEx, 0),
        is_stack: false,
    });
    game.play_until_stable();
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    // Give opponent a trainer card
    let mut state = game.get_state_clone();
    state.hands[1] = vec![Card::Trainer(potion.clone())];
    game.set_state(state);

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1, "It should be player 1's turn");
    let can_play_trainer = actions.iter().any(|a| {
        matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.id == potion.id)
    });
    assert!(
        can_play_trainer,
        "Subsequent Spellbinding Start should NOT block trainer cards"
    );
}

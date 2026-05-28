use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn attach_tool_to_active(game: &mut deckgym::Game<'static>, tool_id: CardId) {
    let trainer_card = trainer_from_id(tool_id);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    });
    let state = game.get_state_clone();
    let (_actor, choices) = state.generate_possible_actions();
    let attach = choices
        .iter()
        .find(|a| matches!(a.action, SimpleAction::AttachTool { in_play_idx: 0, .. }))
        .cloned()
        .expect("Expected attach tool action for slot 0");
    game.apply_action(&attach);
}

#[test]
fn test_ancient_booster_increases_hp_for_ancient_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Brute Bonnet is an Ancient Pokémon with 100 HP
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a003BruteBonnet)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::B3a069AncientBoosterEnergyCapsule)];
    game.set_state(state);

    attach_tool_to_active(&mut game, CardId::B3a069AncientBoosterEnergyCapsule);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(active.attached_tool.is_some());
    // Brute Bonnet base HP = 100, +40 from Ancient Booster Energy Capsule = 140
    assert_eq!(active.get_remaining_hp(), 140);
}

#[test]
fn test_ancient_booster_no_hp_bonus_for_non_ancient_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is not an Ancient Pokémon; base HP = 70
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::B3a069AncientBoosterEnergyCapsule)];
    game.set_state(state);

    attach_tool_to_active(&mut game, CardId::B3a069AncientBoosterEnergyCapsule);

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(active.attached_tool.is_some());
    // No HP bonus — Bulbasaur is not Ancient
    assert_eq!(active.get_remaining_hp(), 70);
}

#[test]
fn test_future_booster_increases_damage_to_opponent_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Iron Moth is a Future Pokémon
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3a005IronMoth)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)], // 70 HP
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::B3a070FutureBoosterEnergyCapsule)];
    game.set_state(state);

    attach_tool_to_active(&mut game, CardId::B3a070FutureBoosterEnergyCapsule);

    // Apply 50 base damage from player 0's active (Iron Moth) to player 1's active
    // With +20 from the capsule, total = 70, which KOs Bulbasaur (70 HP)
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(50, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert!(
        state.in_play_pokemon[1][0].is_none(),
        "Bulbasaur (70 HP) should be KO'd by 50 base + 20 boost = 70 damage"
    );
}

#[test]
fn test_future_booster_no_damage_bonus_for_non_future_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is not a Future Pokémon; attach the capsule to it
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)], // 60 HP
    );
    state.current_player = 0;
    state.hands[0] = vec![get_card_by_enum(CardId::B3a070FutureBoosterEnergyCapsule)];
    game.set_state(state);

    attach_tool_to_active(&mut game, CardId::B3a070FutureBoosterEnergyCapsule);

    // Apply 40 base damage from Bulbasaur to opponent's Charmander (60 HP)
    // No +20 bonus since Bulbasaur is not Future; 60 - 40 = 20 HP remaining
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(40, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    let defender = state.in_play_pokemon[1][0]
        .as_ref()
        .expect("Charmander should survive");
    assert_eq!(
        defender.get_remaining_hp(),
        20,
        "No damage bonus for non-Future Pokémon; 60 - 40 = 20 HP remaining"
    );
}

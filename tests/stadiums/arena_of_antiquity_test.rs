use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn trainer_from_id(card_id: CardId) -> crate::deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

/// Sets up a game with Machop (player 0) vs Machamp ex (player 1),
/// optionally with Arena of Antiquity active.
fn setup_game_with_fighting_vs_ex(with_stadium: bool) -> crate::deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1143Machop).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1146MachampEx)],
    );
    state.current_player = 0;
    state.turn_count = 2;

    if with_stadium {
        state.hands[0] = vec![get_card_by_enum(CardId::B3154ArenaofAntiquity)];
    }

    game.set_state(state);

    if with_stadium {
        let trainer_card = trainer_from_id(CardId::B3154ArenaofAntiquity);
        let play_action = Action {
            actor: 0,
            action: SimpleAction::Play { trainer_card },
            is_stack: false,
        };
        game.apply_action(&play_action);
    }

    game
}

fn get_remaining_hp_after_damage(game: &mut crate::deckgym::Game<'static>, damage: u32) -> u32 {
    let action = Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(damage, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    };
    game.apply_action(&action);
    game.get_state_clone().in_play_pokemon[1][0]
        .as_ref()
        .map(|p| p.get_remaining_hp())
        .unwrap_or(0)
}

#[test]
fn test_arena_of_antiquity_boosts_fighting_vs_ex() {
    let machamp_ex_hp = match get_card_by_enum(CardId::A1146MachampEx) {
        Card::Pokemon(p) => p.hp,
        _ => panic!("Expected Pokemon"),
    };

    // Without stadium: deal 20 damage, measure remaining HP
    let mut game_no_stadium = setup_game_with_fighting_vs_ex(false);
    let remaining_no_stadium = get_remaining_hp_after_damage(&mut game_no_stadium, 20);

    // With stadium: deal 20 damage, measure remaining HP (should be 20 less)
    let mut game_with_stadium = setup_game_with_fighting_vs_ex(true);
    let remaining_with_stadium = get_remaining_hp_after_damage(&mut game_with_stadium, 20);

    let damage_without = machamp_ex_hp - remaining_no_stadium;
    let damage_with = machamp_ex_hp - remaining_with_stadium;

    assert_eq!(
        damage_with.saturating_sub(damage_without),
        20,
        "Arena of Antiquity should add +20 damage for Fighting vs ex (without={damage_without}, with={damage_with})"
    );
}

#[test]
fn test_arena_of_antiquity_no_boost_vs_non_ex() {
    // Arena of Antiquity does NOT boost Fighting vs non-ex pokemon
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1143Machop).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)], // non-ex
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3154ArenaofAntiquity)];
    game.set_state(state);

    let bulbasaur_hp = match get_card_by_enum(CardId::A1001Bulbasaur) {
        Card::Pokemon(p) => p.hp,
        _ => panic!("Expected Pokemon"),
    };

    // Play Arena of Antiquity
    let trainer_card = trainer_from_id(CardId::B3154ArenaofAntiquity);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Apply 20 damage
    let remaining = get_remaining_hp_after_damage(&mut game, 20);

    let damage = bulbasaur_hp - remaining;
    // Damage should be exactly 20 (no +20 bonus since target is not ex)
    assert_eq!(
        damage, 20,
        "Arena of Antiquity should NOT boost damage vs non-ex (expected 20, got {damage})"
    );
}

#[test]
fn test_arena_of_antiquity_no_boost_for_non_fighting() {
    // Arena of Antiquity does NOT boost non-Fighting pokemon vs ex
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur (Grass type) vs Machamp ex
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1146MachampEx)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![get_card_by_enum(CardId::B3154ArenaofAntiquity)];
    game.set_state(state);

    let ex_hp = match get_card_by_enum(CardId::A1146MachampEx) {
        Card::Pokemon(p) => p.hp,
        _ => panic!("Expected Pokemon"),
    };

    // Play Arena of Antiquity
    let trainer_card = trainer_from_id(CardId::B3154ArenaofAntiquity);
    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Apply 20 damage
    let remaining = get_remaining_hp_after_damage(&mut game, 20);

    let damage = ex_hp - remaining;
    // Should be 20 (no bonus for non-Fighting type)
    assert_eq!(
        damage, 20,
        "Arena of Antiquity should NOT boost non-Fighting attacks vs ex (expected 20, got {damage})"
    );
}

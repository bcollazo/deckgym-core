use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, play_trainer, trainer_from_id},
    Game, State,
};

/// Player 0 sets up `defender`, optionally plays `trainer`, ends the turn, and then player 1's
/// Venusaur ex uses Razor Leaf (60 damage) into it. Returns the resulting state.
///
/// Mirrors the Jasmine test harness — going through a real EndTurn is what makes "during your
/// opponent's next turn" effects actually be in scope when the attack lands.
fn razor_leaf_into(defender: PlayedCard, trainer: Option<CardId>) -> State {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![defender, PlayedCard::from_id(CardId::A1053Squirtle)],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );

    if let Some(card_id) = trainer {
        let trainer_card = trainer_from_id(card_id);
        let mut state = game.get_state_clone();
        state.hands[0].push(Card::Trainer(trainer_card.clone()));
        game.set_state(state);
        play_trainer(&mut game, 0, trainer_card);
    }

    end_turn_and_attack(&mut game);
    game.get_state_clone()
}

fn end_turn_and_attack(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1004VenusaurEx, 0),
        is_stack: false,
    });
}

/// Blue: "During your opponent's next turn, all of your Pokémon take -10 damage from attacks from
/// your opponent's Pokémon."
#[test]
fn test_blue_reduces_damage_to_all_of_your_pokemon() {
    // Chansey has 120 HP, so it survives Razor Leaf either way and the difference is visible.
    let chansey = PlayedCard::from_id(CardId::A1202Chansey);
    assert_eq!(
        razor_leaf_into(chansey.clone(), None).get_remaining_hp(0, 0),
        120 - 60
    );
    assert_eq!(
        razor_leaf_into(chansey, Some(CardId::A1a067Blue)).get_remaining_hp(0, 0),
        120 - 50,
        "Blue should reduce Razor Leaf's 60 damage to 50"
    );
}

/// Beast Wall: "...During your opponent's next turn, all of your Ultra Beasts take -20 damage from
/// attacks from your opponent's Pokémon."
#[test]
fn test_beast_wall_reduces_damage_for_ultra_beasts() {
    let nihilego = PlayedCard::from_id(CardId::A3a042Nihilego);
    assert_eq!(
        razor_leaf_into(nihilego.clone(), None).get_remaining_hp(0, 0),
        70 - 60
    );
    assert_eq!(
        razor_leaf_into(nihilego, Some(CardId::A3a063BeastWall)).get_remaining_hp(0, 0),
        70 - 40,
        "Beast Wall should reduce Razor Leaf's 60 damage to 40 for an Ultra Beast"
    );
}

/// Beast Wall names Ultra Beasts specifically — a non-Ultra-Beast gets nothing.
#[test]
fn test_beast_wall_does_not_protect_non_ultra_beasts() {
    let chansey = PlayedCard::from_id(CardId::A1202Chansey);
    assert_eq!(
        razor_leaf_into(chansey, Some(CardId::A3a063BeastWall)).get_remaining_hp(0, 0),
        120 - 60,
        "Chansey is not an Ultra Beast and should take full damage"
    );
}

/// Beast Wall: "You can use this card only if your opponent hasn't gotten any points."
#[test]
fn test_beast_wall_is_unplayable_once_the_opponent_has_a_point() {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![PlayedCard::from_id(CardId::A3a042Nihilego)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let beast_wall = trainer_from_id(CardId::A3a063BeastWall);

    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(beast_wall.clone())];
    state.points[1] = 1;
    game.set_state(state);
    assert!(
        !can_play(&game, &beast_wall.id),
        "Beast Wall should not be playable while the opponent has a point"
    );

    let mut state = game.get_state_clone();
    state.points[1] = 0;
    game.set_state(state);
    assert!(
        can_play(&game, &beast_wall.id),
        "Beast Wall should be playable while the opponent has no points"
    );
}

fn can_play(game: &Game<'static>, trainer_id: &str) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions.iter().any(|a| {
        matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.id == trainer_id)
    })
}

/// Hala: "During your opponent's next turn, if your Hariyama or Crabominable would be Knocked Out
/// by damage from an attack, it is not Knocked Out and its remaining HP becomes 10."
#[test]
fn test_hala_leaves_hariyama_at_10_hp_instead_of_knocking_it_out() {
    let hariyama = PlayedCard::from_id(CardId::A3091Hariyama).with_remaining_hp(20);
    let state = razor_leaf_into(hariyama, Some(CardId::B1222Hala));

    let survivor = state.in_play_pokemon[0][0]
        .as_ref()
        .expect("Hala should have kept Hariyama in play");
    assert_eq!(survivor.get_name(), "Hariyama");
    assert_eq!(
        survivor.get_remaining_hp(),
        10,
        "Hala should leave Hariyama at exactly 10 HP"
    );
    assert_eq!(
        state.points[1], 0,
        "The attacker should not score a point for a Pokémon Hala saved"
    );
}

/// Without Hala the same Hariyama is knocked out, confirming the test setup is lethal.
#[test]
fn test_hariyama_is_knocked_out_without_hala() {
    let hariyama = PlayedCard::from_id(CardId::A3091Hariyama).with_remaining_hp(20);
    let state = razor_leaf_into(hariyama, None);

    assert!(state.in_play_pokemon[0][0].is_none() || state.get_active(0).get_name() != "Hariyama");
    assert_eq!(state.points[1], 1);
}

/// Hala only covers the named Pokémon — anything else is knocked out normally.
#[test]
fn test_hala_does_not_protect_other_pokemon() {
    let charmander = PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(20);
    let state = razor_leaf_into(charmander, Some(CardId::B1222Hala));

    assert!(
        state.in_play_pokemon[0][0].is_none() || state.get_active(0).get_name() != "Charmander",
        "Charmander is not covered by Hala and should be knocked out"
    );
    assert_eq!(state.points[1], 1);
}

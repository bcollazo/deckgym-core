use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{get_test_game_with_board, trainer_from_id},
    Game,
};

/// Gholdengo's "Luxury Coin": "Once during your turn, when you flip any coins for an effect of
/// your Trainer cards, you may ignore all results of those coin flips and begin flipping those
/// coins again. You can't use more than 1 Luxury Coin Ability each turn."
///
/// Electric Generator: "Flip a coin. If heads, take a [L] Energy from your Energy Zone and attach
/// it to 1 of your Benched [L] Pokémon."
fn game_with_gholdengo_and_electric_generator(with_gholdengo: bool) -> Game<'static> {
    let mut board = vec![PlayedCard::from_id(CardId::A1094Pikachu)];
    if with_gholdengo {
        board.push(PlayedCard::from_id(CardId::B4a051Gholdengo));
    } else {
        board.push(PlayedCard::from_id(CardId::A1001Bulbasaur));
    }
    let mut game =
        get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::B2a086ElectricGenerator));
    state.energy_zone[0].current = Some(EnergyType::Lightning);
    game.set_state(state);
    game
}

fn play_electric_generator(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::B2a086ElectricGenerator),
        },
        is_stack: false,
    });
}

#[test]
fn test_luxury_coin_offers_a_reflip_for_a_trainer_cards_coin() {
    let mut game = game_with_gholdengo_and_electric_generator(true);
    play_electric_generator(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        2,
        "Expected a decline/accept pair, got: {:?}",
        choices.iter().map(|c| &c.action).collect::<Vec<_>>()
    );
    assert!(choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::Noop)));
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 1 })),
        "Gholdengo (bench idx 1) should be able to re-flip"
    );
}

#[test]
fn test_luxury_coin_not_offered_without_gholdengo() {
    let mut game = game_with_gholdengo_and_electric_generator(false);
    play_electric_generator(&mut game);

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { .. })),
        "no Gholdengo in play, so nothing to re-flip with"
    );
}

/// A Trainer card without any coin flip should resolve immediately.
#[test]
fn test_luxury_coin_not_offered_for_a_trainer_without_coin_flips() {
    let mut board = vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30)];
    board.push(PlayedCard::from_id(CardId::B4a051Gholdengo));
    let mut game =
        get_test_game_with_board(board, vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    let mut state = game.get_state_clone();
    state.hands[0].clear();
    state.hands[0].push(get_card_by_enum(CardId::PA001Potion));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: match get_card_by_enum(CardId::PA001Potion) {
                Card::Trainer(t) => t,
                _ => unreachable!(),
            },
        },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { .. })),
        "Potion flips no coins, so Luxury Coin has nothing to re-flip"
    );
}

/// Accepting the reflip burns the once-per-turn Ability, so a second coin-flipping Trainer card
/// in the same turn gets no prompt.
#[test]
fn test_luxury_coin_can_only_be_used_once_per_turn() {
    let mut game = game_with_gholdengo_and_electric_generator(true);
    let mut state = game.get_state_clone();
    state.hands[0].push(get_card_by_enum(CardId::B2a086ElectricGenerator));
    game.set_state(state);

    play_electric_generator(&mut game);
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let accept = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::UseAbility { .. }))
        .expect("the reflip should be offered")
        .clone();
    game.apply_action(&accept);

    // Resolve any follow-up choice the re-flipped Electric Generator produced.
    loop {
        let (_, choices) = game.get_state_clone().generate_possible_actions();
        match choices
            .iter()
            .find(|c| matches!(c.action, SimpleAction::Attach { .. }))
        {
            Some(action) => {
                let action = action.clone();
                game.apply_action(&action);
            }
            None => break,
        }
    }

    play_electric_generator(&mut game);
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::UseAbility { .. })),
        "Luxury Coin is once per turn"
    );
}

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::get_initialized_game,
};

fn lana_card() -> deckgym::models::TrainerCard {
    match get_card_by_enum(CardId::A3152Lana) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Lana should be a Trainer card"),
    }
}

/// Lana: "You can use this card only if you have Araquanid in play. Switch in 1 of your
/// opponent's Benched Pokémon to the Active Spot."
#[test]
fn test_cannot_play_lana_without_araquanid() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.hands[0].clear();
    state.hands[0].push(Card::Trainer(lana_card()));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let can_play = actions
        .iter()
        .any(|a| matches!(&a.action, SimpleAction::Play { .. }));
    assert!(
        !can_play,
        "Should not be able to play Lana without Araquanid in play, actions: {actions:?}"
    );
}

/// Playing Lana with Araquanid in play lets the player switch in any of the opponent's
/// Benched Pokémon (even undamaged ones) to the Active Spot.
#[test]
fn test_lana_switches_in_opponents_undamaged_benched_pokemon() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B4047Araquanid),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.hands[0].clear();
    state.hands[0].push(Card::Trainer(lana_card()));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: lana_card(),
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        1,
        "Opponent has 1 undamaged Benched Pokémon, should still be a valid Lana target"
    );
    assert!(matches!(
        choices[0].action,
        SimpleAction::Activate {
            player: 1,
            in_play_idx: 1
        }
    ));
    game.apply_action(&choices[0]);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).card.get_name(),
        "Bulbasaur",
        "Opponent's Bulbasaur should now be Active"
    );
}

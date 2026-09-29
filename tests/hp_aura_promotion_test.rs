use deckgym::actions::{Action, SimpleAction};
use deckgym::card_ids::CardId;
use deckgym::models::Card;
use deckgym::state::PlayedCard;
use deckgym::{Game, State};

#[test]
fn hp_aura_knockout_removes_stale_promotion_choice() {
    let mut state = State::default();
    state.turn_count = 3;
    state.set_board(
        vec![PlayedCard::from_id(CardId::A3b014Salandit)],
        vec![
            PlayedCard::from_id(CardId::B1018Lilligant).with_damage(110),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B1018Lilligant).with_damage(100),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    // Both Lilligant start alive with two +20 HP auras. Losing the Active
    // removes an aura and knocks out the damaged Benched Lilligant too.
    assert!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .unwrap()
            .get_remaining_hp()
            > 0
    );
    let Card::Pokemon(card) = &state.get_active(0).card else {
        panic!()
    };
    let attack = card.attacks[0].clone();
    let mut game = Game::from_state(state, vec![], 0);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(attack),
        is_stack: false,
    });
    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][2].is_none());
    assert_eq!(state.points[0], 2);
    let (_, actions) = state.generate_possible_actions();
    assert_eq!(actions.len(), 2);
    for action in &actions {
        assert!(matches!(
            action.action,
            SimpleAction::Activate {
                player: 1,
                in_play_idx: 1 | 3
            }
        ));
    }
    game.apply_action(&actions[0]);
    assert!(game.get_state_clone().maybe_get_active(1).is_some());
}

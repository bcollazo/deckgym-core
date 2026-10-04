//! On-play abilities requiring the owner's turn must not queue during setup.
use deckgym::actions::{Action, SimpleAction};
use deckgym::card_ids::CardId;
use deckgym::database::get_card_by_enum;
use deckgym::state::PlayedCard;
use deckgym::{Game, State};

fn place_on_bench(card: CardId, turn: u8, actor: usize) -> State {
    let mut state = State::default();
    state.turn_count = turn;
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B1a024Magnemite),
            PlayedCard::from_id(CardId::B1a024Magnemite),
        ],
        vec![
            PlayedCard::from_id(CardId::A4032Magby),
            PlayedCard::from_id(CardId::A4032Magby),
        ],
    );
    let card = get_card_by_enum(card);
    state.hands[actor] = vec![card.clone()];
    let mut game = Game::from_state(state, vec![], 42);
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Place(card, 2),
        is_stack: false,
    });
    game.get_state_clone()
}

#[test]
fn on_play_abilities_do_not_create_setup_or_opponent_turn_prompts() {
    for card in [CardId::B3a019MiraidonEx, CardId::B3a047RoaringMoon] {
        for actor in 0..2 {
            let state = place_on_bench(card, 0, actor);
            assert!(
                state.move_generation_stack.is_empty(),
                "setup must not trigger on-play abilities"
            );
        }
        let state = place_on_bench(card, 1, 1);
        assert!(
            state.move_generation_stack.is_empty(),
            "ability requires the owner's turn"
        );
    }
}

#[test]
fn on_play_abilities_still_offer_the_correct_owner_choice_during_their_turn() {
    for card in [CardId::B3a019MiraidonEx, CardId::B3a047RoaringMoon] {
        let state = place_on_bench(card, 1, 0);
        assert_eq!(
            state.move_generation_stack,
            vec![(
                0,
                vec![
                    SimpleAction::UseAbility { in_play_idx: 2 },
                    SimpleAction::Noop
                ]
            )]
        );
    }
}

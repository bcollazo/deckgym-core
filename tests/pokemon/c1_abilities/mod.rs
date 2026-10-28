use deckgym::{
    actions::{Action, SimpleAction},
    models::PlayedCard,
    test_support::get_test_game_with_board,
    Game,
};

fn use_ability(game: &mut Game<'static>, in_play_idx: usize) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx },
        is_stack: false,
    });
}

fn offers_ability(game: &Game<'static>, in_play_idx: usize) -> bool {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions.iter().any(|action| {
        matches!(action.action, SimpleAction::UseAbility { in_play_idx: i } if i == in_play_idx)
    })
}

mod pawmot;

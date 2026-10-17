use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    state::State,
    test_support::{attack_action, get_test_game_with_board, trainer_from_id},
    Game,
};

fn rare_candy_is_offered(state: &State) -> bool {
    let (actor, actions) = state.generate_possible_actions();
    actor == 0
        && actions.iter().any(|action| {
            matches!(&action.action, SimpleAction::Play { trainer_card }
                if trainer_card == &trainer_from_id(CardId::A3144RareCandy))
        })
}

/// Player 0 has a Bulbasaur in the Active Spot, a Bulbasaur on the Bench, and Rare Candy and
/// Venusaur in hand; the opponent's Active Pokémon is `opponent_active`.
fn game_with_rare_candy(opponent_active: PlayedCard) -> Game<'static> {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![opponent_active],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![
        get_card_by_enum(CardId::A3144RareCandy),
        get_card_by_enum(CardId::A1003Venusaur),
    ];
    game.set_state(state);
    game
}

fn play_rare_candy(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::A3144RareCandy),
        },
        is_stack: false,
    });
}

fn evolve_targets(state: &State) -> Vec<usize> {
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    actions
        .iter()
        .filter_map(|action| match &action.action {
            SimpleAction::Evolve { in_play_idx, .. } => Some(*in_play_idx),
            _ => None,
        })
        .collect()
}

#[test]
fn test_rare_candy_can_evolve_active_and_bench_without_locks() {
    let mut game = game_with_rare_candy(PlayedCard::from_id(CardId::A1001Bulbasaur));
    assert!(rare_candy_is_offered(&game.get_state_clone()));

    play_rare_candy(&mut game);

    assert_eq!(evolve_targets(&game.get_state_clone()), vec![0, 1]);
}

#[test]
fn test_rare_candy_cannot_evolve_active_against_primeval_law() {
    let mut game = game_with_rare_candy(PlayedCard::from_id(CardId::A1a046AerodactylEx));
    assert!(rare_candy_is_offered(&game.get_state_clone()));

    play_rare_candy(&mut game);

    // Primeval Law only protects the Active Pokémon; the Benched Bulbasaur can still evolve.
    assert_eq!(evolve_targets(&game.get_state_clone()), vec![1]);
}

#[test]
fn test_rare_candy_not_offered_if_only_active_is_protected_by_primeval_law() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1a046AerodactylEx)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![
        get_card_by_enum(CardId::A3144RareCandy),
        get_card_by_enum(CardId::A1003Venusaur),
    ];
    game.set_state(state);

    assert!(!rare_candy_is_offered(&game.get_state_clone()));
}

/// Player 1's Malamar is Active and can attack; player 1 takes their turn (attacking with
/// Evolution Jammer if `jam`) and ends it, so it becomes player 0's turn with Rare Candy in hand.
fn game_on_turn_after_malamar(jam: bool) -> Game<'static> {
    let mut malamar = PlayedCard::from_id(CardId::B3112Malamar);
    malamar.attached_energy = vec![EnergyType::Darkness, EnergyType::Darkness];
    let mut game = game_with_rare_candy(malamar);

    let mut state = game.get_state_clone();
    state.current_player = 1;
    state.turn_count = 4;
    game.set_state(state);
    if jam {
        game.apply_action(&Action {
            actor: 1,
            action: attack_action(CardId::B3112Malamar, 0),
            is_stack: false,
        });
    }
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(game.get_state_clone().current_player, 0);
    game
}

#[test]
fn test_rare_candy_offered_the_turn_after_malamar_does_not_jam() {
    let game = game_on_turn_after_malamar(false);
    assert!(rare_candy_is_offered(&game.get_state_clone()));
}

#[test]
fn test_rare_candy_not_offered_after_evolution_jammer() {
    let game = game_on_turn_after_malamar(true);
    assert!(!rare_candy_is_offered(&game.get_state_clone()));
}

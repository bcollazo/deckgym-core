use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
};

mod common;

/// Test Charmeleon B1a 012 - Ignition ability
/// Should trigger on evolution, offering to attach Fire energy
#[test]
fn test_charmeleon_ignition() {
    let setup_game = || {
        let mut game = get_initialized_game(0);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        state.set_board(
            vec![PlayedCard::from_id(CardId::A1033Charmander)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );

        let charmeleon = get_card_by_enum(CardId::B1a012Charmeleon);
        state.hands[0].push(charmeleon.clone());

        game.set_state(state);
        (game, charmeleon)
    };

    let evolve = |game: &mut deckgym::Game, charmeleon: Card| {
        let evolve_action = Action {
            actor: 0,
            action: SimpleAction::Evolve {
                in_play_idx: 0,
                evolution: if let Card::Pokemon(pc) = charmeleon {
                    Card::Pokemon(pc)
                } else {
                    panic!("Expected Pokemon card")
                },
                from_deck: false,
            },
            is_stack: false,
        };
        game.apply_action(&evolve_action);
    };

    // Test 1: Ability triggers on evolution with 2 options
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let active = state.get_active(0);
        if let Card::Pokemon(pokemon) = &active.card {
            assert_eq!(pokemon.name, "Charmeleon");
        }

        let (_actor, moves) = state.generate_possible_actions();
        assert_eq!(moves.len(), 2);
        assert!(
            moves
                .iter()
                .any(|a| matches!(a.action, SimpleAction::Attach { .. })),
            "Expected an Attach option for the ability"
        );
        assert!(
            moves.iter().any(|a| matches!(a.action, SimpleAction::Noop)),
            "Expected a Noop option for declining the ability"
        );
    }

    // Test 2: User accepts and attaches Fire energy
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let (_actor, moves) = state.generate_possible_actions();
        game.apply_action(&moves[0]);
        let state = game.get_state_clone();

        let charmeleon_active = state.get_active(0);
        assert_eq!(charmeleon_active.attached_energy.len(), 1);
        assert_eq!(charmeleon_active.attached_energy[0], EnergyType::Fire);
    }

    // Test 3: User declines and doesn't attach energy
    {
        let (mut game, charmeleon) = setup_game();
        evolve(&mut game, charmeleon);
        let state = game.get_state_clone();

        let (_actor, moves) = state.generate_possible_actions();
        game.apply_action(&moves[1]);
        let state = game.get_state_clone();

        let charmeleon_active = state.get_active(0);
        assert_eq!(charmeleon_active.attached_energy.len(), 0);
    }
}

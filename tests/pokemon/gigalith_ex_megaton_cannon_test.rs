use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

fn gigalith_ex_board() -> deckgym::Game<'static> {
    get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4229GigalithEx).with_energy(vec![
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
                EnergyType::Fighting,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1004VenusaurEx),
        ],
    )
}

/// Gigalith ex's Megaton Cannon lets the attacker pick any of the opponent's Pokémon —
/// including a Benched one — and does 140 damage to it.
#[test]
fn test_gigalith_ex_megaton_cannon_hits_chosen_benched_pokemon() {
    let mut game = gigalith_ex_board();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4229GigalithEx, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. })));

    let bench_target = choices
        .iter()
        .find(|choice| {
            matches!(&choice.action, SimpleAction::ApplyDamage { targets, .. }
                if targets.iter().any(|(_, _, in_play_idx)| *in_play_idx == 1))
        })
        .expect("Megaton Cannon should be able to target a Benched Pokémon")
        .clone();
    game.apply_action(&bench_target);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), VENUSAUR_EX_MAX_HP);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Benched Venusaur ex should still be in play")
            .get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 140
    );
}

/// After using Megaton Cannon, Gigalith ex can't attack during its next turn.
#[test]
fn test_gigalith_ex_cannot_attack_the_turn_after_megaton_cannon() {
    let mut game = gigalith_ex_board();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4229GigalithEx, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    game.apply_action(&choices[0].clone());

    // Pass through the opponent's turn to get back to Gigalith ex's next turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action.action, SimpleAction::Attack(_))),
        "Gigalith ex should not be able to attack the turn after Megaton Cannon"
    );
}

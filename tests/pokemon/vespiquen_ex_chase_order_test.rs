use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;

fn vespiquen_ex_board(bench: Vec<PlayedCard>) -> deckgym::Game<'static> {
    let mut player_board = vec![PlayedCard::from_id(CardId::B4011VespiquenEx)
        .with_energy(vec![EnergyType::Grass, EnergyType::Grass])];
    player_board.extend(bench);
    get_test_game_with_board(
        player_board,
        vec![PlayedCard::from_id(CardId::A1004VenusaurEx)],
    )
}

fn use_chase_order(game: &mut deckgym::Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4011VespiquenEx, 0),
        is_stack: false,
    });
}

/// Discarding a Benched Basic [G] Pokémon makes Chase Order do 70 more damage, and the
/// discarded Pokémon leaves play.
#[test]
fn test_vespiquen_ex_chase_order_discard_boosts_damage() {
    let mut game = vespiquen_ex_board(vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    use_chase_order(&mut game);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let discard_choice = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::DiscardOwnBenchedThenDamage { in_play_idx: 1, .. }
            )
        })
        .expect("Chase Order should offer discarding the Benched Basic [G] Bulbasaur")
        .clone();
    game.apply_action(&discard_choice);

    // The boosted damage is queued as a single follow-up application.
    let (actor, follow_up) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(follow_up.len(), 1);
    game.apply_action(&follow_up[0].clone());

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 140
    );
    assert!(state.in_play_pokemon[0][1].is_none());
    assert_eq!(state.discard_piles[0].len(), 1);
    assert_eq!(state.discard_piles[0][0].get_name(), "Bulbasaur");
}

/// The discard is optional: declining it keeps the Bench intact and deals only the base damage.
#[test]
fn test_vespiquen_ex_chase_order_can_decline_the_discard() {
    let mut game = vespiquen_ex_board(vec![PlayedCard::from_id(CardId::A1001Bulbasaur)]);
    use_chase_order(&mut game);

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let keep_bench_choice = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::ApplyDamage { .. }))
        .expect("Chase Order should offer attacking without discarding")
        .clone();
    game.apply_action(&keep_bench_choice);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 70
    );
    assert!(state.in_play_pokemon[0][1].is_some());
}

/// Only Benched Basic [G] Pokémon can be discarded: an evolved [G] Pokémon and a Basic of
/// another type are both ineligible, so no choice is offered at all.
#[test]
fn test_vespiquen_ex_chase_order_ignores_ineligible_bench() {
    let mut game = vespiquen_ex_board(vec![
        PlayedCard::from_id(CardId::A1002Ivysaur),
        PlayedCard::from_id(CardId::A1033Charmander),
    ]);
    use_chase_order(&mut game);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 70
    );
    assert!(state.in_play_pokemon[0][1].is_some());
    assert!(state.in_play_pokemon[0][2].is_some());
}

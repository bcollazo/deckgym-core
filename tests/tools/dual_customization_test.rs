//! Revavroom (B4 115) — "Dual Customization: This Pokémon may have up to 2 Pokémon Tool cards
//! attached to it."
//!
//! Everything here drives the public `Game` API, in the style of
//! `test_raikou_rocky_helmet_promotion_order`.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    effects::CardEffect,
    models::{Card, PlayedCard},
    state::GameOutcome,
    test_support::get_test_game_with_board,
    Game,
};

fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

/// Plays `tool_id` from `actor`'s hand and returns the board slots the engine offers as
/// attachment targets.
fn attach_targets_after_playing(
    game: &mut Game<'static>,
    actor: usize,
    tool_id: CardId,
) -> Vec<usize> {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(tool_id),
        },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let mut indices: Vec<usize> = choices
        .iter()
        .filter_map(|choice| match choice.action {
            SimpleAction::AttachTool { in_play_idx, .. } => Some(in_play_idx),
            _ => None,
        })
        .collect();
    indices.sort_unstable();
    indices.dedup();
    indices
}

fn apply_attack_damage(
    game: &mut Game<'static>,
    actor: usize,
    damage: u32,
    target: (usize, usize),
) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (actor, 0),
            targets: vec![(damage, target.0, target.1)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
}

#[test]
fn test_revavroom_holds_two_tools_and_both_effects_apply() {
    // Rocky Helmet first, Giant Cape second: the HP bonus has to be found in the *second* slot.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))],
    );

    let state = game.get_state_clone();
    let revavroom = state.get_active(1);
    assert_eq!(revavroom.attached_tools.len(), 2);
    // Revavroom's printed HP is 120; Giant Cape adds 20.
    assert_eq!(revavroom.get_remaining_hp(), 140);

    apply_attack_damage(&mut game, 0, 100, (1, 0));

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 40);
    // Rocky Helmet's 20 recoil still fires from the first slot.
    assert_eq!(state.get_active(0).get_remaining_hp(), 50);
}

#[test]
fn test_tool_lookup_is_order_independent() {
    // The mirror of the test above: Giant Cape first, Rocky Helmet second.
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 140);

    apply_attack_damage(&mut game, 0, 100, (1, 0));

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 40);
    assert_eq!(state.get_active(0).get_remaining_hp(), 50);
}

#[test]
fn test_revavroom_can_be_offered_a_second_tool() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4115Revavroom)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A2148RockyHelmet)];
    game.set_state(state);

    let targets = attach_targets_after_playing(&mut game, 0, CardId::A2148RockyHelmet);
    assert_eq!(targets, vec![0, 1]);

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let attach = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::AttachTool { in_play_idx: 0, .. }
            )
        })
        .expect("Revavroom should be offered a second Tool")
        .clone();
    game.apply_action(&attach);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_tools.len(), 2);
}

#[test]
fn test_normal_pokemon_is_not_offered_a_second_tool() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A2148RockyHelmet)];
    game.set_state(state);

    let targets = attach_targets_after_playing(&mut game, 0, CardId::A2148RockyHelmet);
    assert_eq!(targets, vec![1]);
}

#[test]
fn test_revavroom_is_not_offered_a_duplicate_tool() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4115Revavroom)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A2147GiantCape)];
    game.set_state(state);

    let targets = attach_targets_after_playing(&mut game, 0, CardId::A2147GiantCape);
    assert_eq!(targets, vec![1]);
}

#[test]
fn test_abilities_disabled_returns_revavroom_to_one_tool_slot() {
    let mut revavroom = PlayedCard::from_id(CardId::B4115Revavroom)
        .with_tool(get_card_by_enum(CardId::A2147GiantCape));
    revavroom.add_effect(CardEffect::AbilitiesDisabled, 200);

    let mut game = get_test_game_with_board(
        vec![revavroom, PlayedCard::from_id(CardId::A1033Charmander)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A2148RockyHelmet)];
    game.set_state(state);

    let targets = attach_targets_after_playing(&mut game, 0, CardId::A2148RockyHelmet);
    assert_eq!(targets, vec![1]);
}

#[test]
fn test_abilities_disabled_keeps_a_second_tool_already_attached() {
    // Losing the Ability does not knock a Tool off — it only stops a third from being added,
    // and both attached Tools keep working.
    let mut revavroom = PlayedCard::from_id(CardId::B4115Revavroom)
        .with_tool(get_card_by_enum(CardId::A2147GiantCape))
        .with_tool(get_card_by_enum(CardId::A2148RockyHelmet));
    revavroom.add_effect(CardEffect::AbilitiesDisabled, 200);

    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![revavroom],
    );

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).attached_tools.len(), 2);
    assert_eq!(state.get_active(1).get_remaining_hp(), 140);

    apply_attack_damage(&mut game, 0, 100, (1, 0));
    assert_eq!(game.get_state_clone().get_active(0).get_remaining_hp(), 50);
}

#[test]
fn test_ko_discards_both_tools() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    let mut state = game.get_state_clone();
    state.points = [0, 0];
    game.set_state(state);

    apply_attack_damage(&mut game, 0, 200, (1, 0));

    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[1][0].is_none());
    for tool in [CardId::A2147GiantCape, CardId::A2148RockyHelmet] {
        assert!(
            state.discard_piles[1].contains(&get_card_by_enum(tool)),
            "expected {tool:?} in the opponent's discard pile"
        );
    }
}

#[test]
fn test_guzma_discards_both_tools() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A3151Guzma)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::A3151Guzma),
        },
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_ne!(state.winner, Some(GameOutcome::Win(0)));
    assert!(state.get_active(1).attached_tools.is_empty());
    assert_eq!(state.discard_piles[1].len(), 2);
    // Losing Giant Cape drops the effective HP back to the printed 120.
    assert_eq!(state.get_active(1).get_remaining_hp(), 120);
}

#[test]
fn test_field_blower_can_pick_either_of_two_tools() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4115Revavroom)
            .with_tool(get_card_by_enum(CardId::A2147GiantCape))
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::B3147FieldBlower)];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: trainer_from_id(CardId::B3147FieldBlower),
        },
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let discard_choices: Vec<&Action> = choices
        .iter()
        .filter(|choice| matches!(choice.action, SimpleAction::DiscardToolFromPokemon { .. }))
        .collect();
    assert_eq!(
        discard_choices.len(),
        2,
        "Field Blower should offer one choice per attached Tool"
    );

    let chosen = discard_choices[1].clone();
    game.apply_action(&chosen);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).attached_tools.len(), 1);
}

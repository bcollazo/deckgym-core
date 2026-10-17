use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{get_initialized_game_with_board, get_test_game_with_board},
};

fn use_ability(game: &mut deckgym::Game<'static>, actor: usize, in_play_idx: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::UseAbility { in_play_idx },
        is_stack: false,
    });
}

fn offers_ability(game: &deckgym::Game<'_>, in_play_idx: usize) -> bool {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions.iter().any(
        |a| matches!(a.action, SimpleAction::UseAbility { in_play_idx: idx } if idx == in_play_idx),
    )
}

/// Tyranitar's Energy Plunder: "Once during your turn, you may move all [D] Energy from each of
/// your Pokémon to this Pokémon."
#[test]
fn test_energy_plunder_gathers_all_darkness_energy_onto_tyranitar() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4119Tyranitar).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![
                EnergyType::Darkness,
                EnergyType::Darkness,
                EnergyType::Grass,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(offers_ability(&game, 0));
    use_ability(&mut game, 0, 0);

    let state = game.get_state_clone();
    let tyranitar = state.in_play_pokemon[0][0].as_ref().unwrap();
    assert_eq!(
        tyranitar
            .attached_energy
            .iter()
            .filter(|e| **e == EnergyType::Darkness)
            .count(),
        3,
        "all three [D] Energy should be on Tyranitar"
    );
    let bulbasaur = state.in_play_pokemon[0][1].as_ref().unwrap();
    assert_eq!(
        bulbasaur.attached_energy,
        vec![EnergyType::Grass],
        "non-[D] Energy stays where it is"
    );
}

#[test]
fn test_energy_plunder_not_offered_without_darkness_energy_elsewhere() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4119Tyranitar).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!offers_ability(&game, 0));
}

/// Swellow's Repelling Wind only works on an opponent's Active *Basic* Pokémon.
#[test]
fn test_repelling_wind_requires_a_basic_opponent_active() {
    let against_basic = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2133Swellow)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    assert!(offers_ability(&against_basic, 0));

    let against_stage_1 = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2133Swellow)],
        vec![
            PlayedCard::from_id(CardId::A1002Ivysaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    assert!(
        !offers_ability(&against_stage_1, 0),
        "Repelling Wind should not be offered against a Stage 1 Active"
    );
}

#[test]
fn test_repelling_wind_switches_the_opponents_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2133Swellow)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );
    use_ability(&mut game, 0, 0);

    // The opponent chooses the new Active Pokémon.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    let activate = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::Activate { .. }))
        .expect("opponent should be choosing a new Active Pokemon")
        .clone();
    game.apply_action(&activate);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Charmander");
}

/// Grafaiai's Poison Coating: a coin flip; on heads the opponent's Active Pokémon is Poisoned.
#[test]
fn test_poison_coating_poisons_on_heads_only() {
    let mut saw_poisoned = false;
    let mut saw_clean = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A2b051Grafaiai)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        use_ability(&mut game, 0, 0);
        if game.get_state_clone().get_active(1).is_poisoned() {
            saw_poisoned = true;
        } else {
            saw_clean = true;
        }
    }

    assert!(saw_poisoned && saw_clean);
}

/// Rillaboom's Captivating Rhythm: on heads, the *acting* player picks which of the opponent's
/// Benched Pokémon is dragged up.
#[test]
fn test_captivating_rhythm_can_drag_up_a_benched_pokemon() {
    let mut saw_switch = false;
    let mut saw_no_switch = false;

    for seed in 0..40u64 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::B1027Rillaboom)],
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur),
                PlayedCard::from_id(CardId::A1033Charmander),
            ],
        );
        use_ability(&mut game, 0, 0);

        let (actor, choices) = game.get_state_clone().generate_possible_actions();
        let activate = (actor == 0)
            .then(|| {
                choices
                    .iter()
                    .find(|c| matches!(c.action, SimpleAction::Activate { player: 1, .. }))
                    .cloned()
            })
            .flatten();
        match activate {
            Some(action) => {
                game.apply_action(&action);
                assert_eq!(
                    game.get_state_clone().get_active(1).get_name(),
                    "Charmander"
                );
                saw_switch = true;
            }
            None => saw_no_switch = true,
        }
    }

    assert!(saw_switch, "expected at least one heads");
    assert!(saw_no_switch, "expected at least one tails");
}

/// Ambipom's Catching Tail: "Once during your turn, you may put a random Pokémon Tool card from
/// your deck into your hand."
#[test]
fn test_catching_tail_pulls_a_tool_from_the_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b059Ambipom)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        deckgym::database::get_card_by_enum(CardId::A2148RockyHelmet),
        deckgym::database::get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    game.set_state(state);

    assert!(offers_ability(&game, 0));
    use_ability(&mut game, 0, 0);

    let state = game.get_state_clone();
    assert!(
        state.hands[0]
            .iter()
            .any(|c| c.get_name() == "Rocky Helmet"),
        "Catching Tail should have put the deck's only Tool into hand"
    );
}

#[test]
fn test_catching_tail_not_offered_without_a_tool_in_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b059Ambipom)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![deckgym::database::get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);
    assert!(!offers_ability(&game, 0));
}

/// Porygon's Data Scan (and the other look-at-hidden-cards abilities) are information-only, so
/// they are never offered as actions.
#[test]
fn test_look_at_cards_abilities_are_not_offered() {
    let game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1209Porygon)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!offers_ability(&game, 0));
}

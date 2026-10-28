use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Dewgong - Dual Blizzard: discard 2 [W] Energy from this Pokémon, and this attack does 60
/// damage to 2 of your opponent's Pokémon.
#[test]
fn test_dewgong_dual_blizzard_hits_two_chosen_pokemon_and_discards_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1033Dewgong).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A1129MewtwoEx),
            PlayedCard::from_id(CardId::B4197WailordEx),
        ],
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::C1033Dewgong, 0),
        is_stack: false,
    });

    // Any pair out of the 3 opposing Pokémon can be picked.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 3);

    // Pick the pair of the two Benched Pokémon.
    let pick = choices
        .iter()
        .find(|choice| match &choice.action {
            SimpleAction::ApplyDamage { targets, .. } => {
                targets.iter().all(|(_, _, idx)| *idx != 0)
            }
            _ => false,
        })
        .expect("a pair of Benched targets should be offered")
        .clone();
    game.apply_action(&pick);
    game.play_until_stable();

    let state = game.get_state_clone();
    let hp = |idx: usize| {
        state.in_play_pokemon[1][idx]
            .as_ref()
            .unwrap()
            .get_remaining_hp()
    };
    assert_eq!(hp(0), 150);
    assert_eq!(hp(1), 150 - 60);
    assert_eq!(hp(2), 250 - 60);
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Water, EnergyType::Water]
    );
    assert_eq!(state.discard_energies[0], vec![EnergyType::Water; 2]);
}

/// With only the Active Pokémon in play, Dual Blizzard can only hit that one.
#[test]
fn test_dewgong_dual_blizzard_with_lone_target_hits_it_once() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1033Dewgong).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::C1033Dewgong, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 60);
    assert_eq!(state.get_active(0).attached_energy, vec![EnergyType::Water]);
}

/// Palkia - Spacial Rend: 70 damage, and if the opponent has a Stadium in play, discard it.
#[test]
fn test_palkia_spacial_rend_discards_the_stadium_in_play() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1038Palkia).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    let mut state = game.get_state_clone();
    state.active_stadium = Some(get_card_by_enum(CardId::B4a072Arcade));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::C1038Palkia, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 70);
    assert!(state.active_stadium.is_none());
}

/// Palkia deals its damage normally when there is no Stadium to discard.
#[test]
fn test_palkia_spacial_rend_without_stadium_just_damages() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1038Palkia).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::C1038Palkia, 0),
        is_stack: false,
    });
    game.play_until_stable();
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250 - 70
    );
}

fn evolve_froakie_into_frogadier(deck_extra: Vec<deckgym::models::Card>) -> deckgym::Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1040Froakie)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let mut state = game.get_state_clone();
    let frogadier = get_card_by_enum(CardId::C1041Frogadier);
    state.hands[0].push(frogadier.clone());
    state.decks[0].cards = deck_extra;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Evolve {
            evolution: frogadier,
            in_play_idx: 0,
            from_deck: false,
        },
        is_stack: false,
    });
    game
}

/// Frogadier - Sudden Duplication: on evolving from hand, may put a random Frogadier from the
/// deck onto the Bench.
#[test]
fn test_frogadier_sudden_duplication_benches_a_frogadier_from_deck() {
    let mut game = evolve_froakie_into_frogadier(vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1088Frogadier),
    ]);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(choices
        .iter()
        .any(|c| matches!(c.action, SimpleAction::Noop)));
    let use_ability = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::UseAbility { in_play_idx: 0 }))
        .expect("Sudden Duplication should be offered")
        .clone();
    game.apply_action(&use_ability);
    game.play_until_stable();

    let state = game.get_state_clone();
    let benched = state.in_play_pokemon[0][1]
        .as_ref()
        .expect("a Frogadier should have been benched");
    assert_eq!(benched.get_name(), "Frogadier");
    assert_eq!(state.decks[0].cards.len(), 1);
}

/// Without a Frogadier in the deck, the ability has nothing to put onto the Bench.
#[test]
fn test_frogadier_sudden_duplication_does_nothing_without_frogadier_in_deck() {
    let mut game = evolve_froakie_into_frogadier(vec![get_card_by_enum(CardId::A1001Bulbasaur)]);
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    if let Some(use_ability) = choices
        .iter()
        .find(|c| matches!(c.action, SimpleAction::UseAbility { .. }))
    {
        game.apply_action(&use_ability.clone());
        game.play_until_stable();
    }
    let state = game.get_state_clone();
    assert!(state.in_play_pokemon[0][1].is_none());
    assert_eq!(state.decks[0].cards.len(), 1);
}

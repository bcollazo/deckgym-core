use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Tapu Koko - Volt Switch: after damage, switch with a Benched [L] Pokémon (and only a [L] one).
#[test]
fn test_tapu_koko_volt_switch_only_offers_lightning_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3068TapuKoko).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Lightning,
            ]),
            // Bulbasaur is [G] and must not be offered.
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A3068TapuKoko),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3068TapuKoko, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(
        choices.len(),
        1,
        "only the Benched [L] Pokémon may be switched in"
    );
    assert!(matches!(
        choices[0].action,
        SimpleAction::Activate {
            player: 0,
            in_play_idx: 2
        }
    ));

    game.apply_action(&choices[0].clone());
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 80);
    assert!(state.get_active(0).attached_energy.is_empty());
}

/// Eldegoss - Float Up: the player MAY shuffle Eldegoss and everything attached back into the deck.
#[test]
fn test_eldegoss_float_up_can_shuffle_itself_and_attachments_into_the_deck() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2016Eldegoss)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let deck_size_before = game.get_state_clone().decks[0].cards.len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2016Eldegoss, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let shuffle_action = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::ShuffleSelfAndAttachmentsIntoDeck { in_play_idx: 0 }
            )
        })
        .expect("Float Up should offer shuffling itself into the deck")
        .clone();
    assert!(
        choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::Noop)),
        "the effect is optional, so declining must be offered"
    );

    game.apply_action(&shuffle_action);

    // Vacating the Active Spot prompts a promotion from the Bench.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let promote = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::Activate {
                    player: 0,
                    in_play_idx: 1
                }
            )
        })
        .expect("Bulbasaur should be promotable")
        .clone();
    game.apply_action(&promote);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 110);
    // Eldegoss went back to the deck and Bulbasaur was promoted; its Energy was discarded.
    assert_eq!(state.decks[0].cards.len(), deck_size_before + 1);
    assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
    assert_eq!(state.discard_energies[0].len(), 2);
}

/// Float Up is optional: declining leaves Eldegoss in the Active Spot with its Energy.
#[test]
fn test_eldegoss_float_up_can_be_declined() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2016Eldegoss).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B2016Eldegoss, 0),
        is_stack: false,
    });

    let (_, choices) = game.get_state_clone().generate_possible_actions();
    let decline = choices
        .iter()
        .find(|choice| matches!(choice.action, SimpleAction::Noop))
        .expect("declining should be offered")
        .clone();
    game.apply_action(&decline);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_name(), "Eldegoss");
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
}

/// Accelgor - Deck and Cover: Poison + Paralysis, then Accelgor goes back into the deck.
#[test]
fn test_accelgor_deck_and_cover_poisons_paralyzes_then_shuffles_itself_away() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4014Accelgor)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let deck_size_before = game.get_state_clone().decks[0].cards.len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4014Accelgor, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.decks[0].cards.len(), deck_size_before + 1);
    assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
    assert_eq!(state.discard_energies[0].len(), 2);
}

/// Liepard - Snatch and Flee: a random opposing hand card goes back to their deck, and Liepard
/// shuffles itself away too.
#[test]
fn test_liepard_snatch_and_flee_returns_a_hand_card_and_itself() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B1a048Liepard).with_energy(vec![EnergyType::Darkness]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let state = game.get_state_clone();
    let opponent_hand_before = state.hands[1].len();
    let opponent_deck_before = state.decks[1].cards.len();
    assert!(opponent_hand_before > 0, "opponent should hold cards");

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B1a048Liepard, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), opponent_hand_before - 1);
    assert_eq!(state.decks[1].cards.len(), opponent_deck_before + 1);
    assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
}

/// Tsareena - Kick Down: a random opposing hand card is shuffled into their deck. Tsareena stays.
#[test]
fn test_tsareena_kick_down_shuffles_a_random_hand_card_back() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b005Tsareena).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let state = game.get_state_clone();
    let opponent_hand_before = state.hands[1].len();
    let opponent_deck_before = state.decks[1].cards.len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b005Tsareena, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), opponent_hand_before - 1);
    assert_eq!(state.decks[1].cards.len(), opponent_deck_before + 1);
    assert_eq!(state.get_active(0).get_name(), "Tsareena");
    assert_eq!(state.get_active(1).get_remaining_hp(), 100);
}

/// Purugly - Interrupt: the attacker picks which revealed card goes back into the opponent's deck.
#[test]
fn test_purugly_interrupt_lets_the_attacker_choose_the_card_to_shuffle_away() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A2140Purugly).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );

    let state = game.get_state_clone();
    let opponent_hand_before = state.hands[1].len();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A2140Purugly, 0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices.is_empty());
    assert!(choices
        .iter()
        .all(|choice| matches!(choice.action, SimpleAction::ShuffleOpponentSupporter { .. })));

    game.apply_action(&choices[0].clone());
    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), opponent_hand_before - 1);
    assert_eq!(state.get_active(1).get_remaining_hp(), 90);
}

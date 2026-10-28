use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
};

fn attack(game: &mut deckgym::Game<'static>, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
    game.play_until_stable();
}

/// Sawsbuck - Leaf Litter Tackle: 100 damage, then discard a [G] Energy from this Pokémon.
#[test]
fn test_sawsbuck_leaf_litter_tackle_discards_a_grass_energy() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1010Sawsbuck).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Fire,
            EnergyType::Fire,
        ])],
        vec![PlayedCard::from_id(CardId::B4197WailordEx)],
    );
    attack(&mut game, CardId::C1010Sawsbuck);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 150);
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Fire, EnergyType::Fire]
    );
    assert_eq!(state.discard_energies[0], vec![EnergyType::Grass]);
}
/// Amoonguss - Astonish: a random card from the opponent's hand is shuffled into their deck.
#[test]
fn test_amoonguss_astonish_shuffles_a_random_hand_card_back() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::C1012Amoonguss)
            .with_energy(vec![EnergyType::Grass, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1211Snorlax)],
    );
    let state = game.get_state_clone();
    let hand_before = state.hands[1].len();
    let deck_before = state.decks[1].cards.len();

    attack(&mut game, CardId::C1012Amoonguss);

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), hand_before - 1);
    assert_eq!(state.decks[1].cards.len(), deck_before + 1);
}
/// Volcanion - Dynamite Steam: flip 2 coins, 110 damage for each heads.
#[test]
fn test_volcanion_dynamite_steam_damage_scales_with_heads() {
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::C1019Volcanion).with_energy(vec![
                    EnergyType::Fire,
                    EnergyType::Fire,
                    EnergyType::Fire,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        attack(&mut game, CardId::C1019Volcanion);
        let state = game.get_state_clone();
        seen.insert(250 - state.get_active(1).get_remaining_hp());
    }
    assert_eq!(seen, [0, 110, 220].into_iter().collect());
}
/// Ceruledge - Rally Back: +70 damage if one of our Pokémon was Knocked Out by an attack last turn.
#[test]
fn test_ceruledge_rally_back_adds_damage_after_a_knockout_last_turn() {
    for (knocked_out, expected_damage) in [(true, 140), (false, 70)] {
        let mut game = get_test_game_with_board(
            vec![
                PlayedCard::from_id(CardId::C1028Ceruledge).with_energy(vec![
                    EnergyType::Fire,
                    EnergyType::Fire,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::B4197WailordEx)],
        );
        let mut state = game.get_state_clone();
        state.set_knocked_out_by_opponent_attack_last_turn(knocked_out);
        game.set_state(state);
        attack(&mut game, CardId::C1028Ceruledge);

        let state = game.get_state_clone();
        assert_eq!(
            250 - state.get_active(1).get_remaining_hp(),
            expected_damage
        );
    }
}

/// Venusaur - Sunny Heal: once during your turn, you may heal 30 damage from your Active Pokémon.
#[test]
fn test_venusaur_sunny_heal_heals_active_once_per_turn() {
    use deckgym::actions::SimpleAction;

    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(50),
            PlayedCard::from_id(CardId::C1003Venusaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    let ability = Action {
        actor: 0,
        action: SimpleAction::UseAbility { in_play_idx: 1 },
        is_stack: false,
    };
    assert!(game
        .get_state_clone()
        .generate_possible_actions()
        .1
        .contains(&ability));
    game.apply_action(&ability);
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 70 - 50 + 30);
    assert!(!state.generate_possible_actions().1.contains(&ability));
}

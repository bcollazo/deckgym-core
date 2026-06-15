use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_test_game_with_board,
};

/// Builds a Grovyle (Stage 1) that evolved from Treecko, with the given attached energy. The
/// `cards_behind` records the under-card (Treecko) so Celebi's Time Recall can grant Treecko's
/// "Pound" attack to the active Grovyle.
fn grovyle_over_treecko(energy: Vec<EnergyType>) -> PlayedCard {
    let mut grovyle = PlayedCard::from_id(CardId::B3006Grovyle).with_energy(energy);
    grovyle.cards_behind = vec![get_card_by_enum(CardId::B3005Treecko)];
    grovyle
}

fn offered_attack_titles(game: &deckgym::Game<'_>) -> Vec<String> {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .filter_map(|action| match &action.action {
            SimpleAction::Attack(attack) => Some(attack.title.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn test_celebi_time_recall_grants_previous_evolution_attack() {
    // Grovyle (evolved from Treecko) is Active with a single Grass energy. Grovyle's own
    // Slicing Snipe needs [G][G], so it is not usable; but Celebi's Time Recall grants Treecko's
    // Pound ([G], 20 damage), which should be offered and deal its damage.
    let mut game = get_test_game_with_board(
        vec![
            grovyle_over_treecko(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::B3004Celebi),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let titles = offered_attack_titles(&game);
    assert!(
        titles.iter().any(|title| title == "Pound"),
        "Celebi's Time Recall should grant Treecko's Pound to the evolved Grovyle, got {titles:?}"
    );

    let hp_before = game.get_state_clone().get_active(1).get_remaining_hp();

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    let pound = actions
        .iter()
        .find(|action| matches!(&action.action, SimpleAction::Attack(a) if a.title == "Pound"))
        .expect("Pound should be offered")
        .clone();
    game.apply_action(&pound);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        hp_before - 20,
        "Copied Pound from a previous evolution should deal 20 damage"
    );
}

#[test]
fn test_no_time_recall_without_celebi() {
    // Same board, but no Celebi in play: the previous-evolution Pound should NOT be offered, and
    // Grovyle cannot pay for its own Slicing Snipe with a single Grass energy.
    let game = get_test_game_with_board(
        vec![grovyle_over_treecko(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let titles = offered_attack_titles(&game);
    assert!(
        !titles.iter().any(|title| title == "Pound"),
        "Without Celebi, previous-evolution attacks should not be available, got {titles:?}"
    );
}

#[test]
fn test_time_recall_still_requires_energy() {
    // Celebi is in play, but Grovyle has no energy: Treecko's Pound (needs [G]) must not be
    // offered, since Time Recall still requires the necessary Energy.
    let game = get_test_game_with_board(
        vec![
            grovyle_over_treecko(vec![]),
            PlayedCard::from_id(CardId::B3004Celebi),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let titles = offered_attack_titles(&game);
    assert!(
        !titles.iter().any(|title| title == "Pound"),
        "Time Recall should still require the necessary Energy, got {titles:?}"
    );
}

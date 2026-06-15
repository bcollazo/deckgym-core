use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Attack, EnergyType, PlayedCard},
    test_support::{get_test_game_with_board, nth_attack},
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
    offered_attacks(game)
        .into_iter()
        .map(|attack| attack.title)
        .collect()
}

fn offered_attacks(game: &deckgym::Game<'_>) -> Vec<Attack> {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .filter_map(|action| match &action.action {
            SimpleAction::Attack(attack) => Some(attack.clone()),
            _ => None,
        })
        .collect()
}

/// Slurpuff (Stage 1) evolved from Swirlix. Both have an attack named "Sweets Relay", but with
/// different cost/damage/effect, so Time Recall must offer BOTH as distinct choices.
fn slurpuff_over_swirlix(energy: Vec<EnergyType>) -> PlayedCard {
    let mut slurpuff = PlayedCard::from_id(CardId::A3b032Slurpuff).with_energy(energy);
    slurpuff.cards_behind = vec![get_card_by_enum(CardId::A3b031Swirlix)];
    slurpuff
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

#[test]
fn test_time_recall_offers_both_same_named_attacks_with_different_effects() {
    // Slurpuff (active, evolved from Swirlix) and its under-card Swirlix both have an attack
    // named "Sweets Relay", but the two differ in cost/damage/effect. With Celebi's Time Recall
    // in play, the player should be able to use EITHER one — so both distinct attacks must be
    // offered, not collapsed by name.
    //
    // NOTE: the "Sweets Relay" effect is not yet implemented, so we only assert what is OFFERED
    // (move generation does not evaluate attack effects) and do not apply the attacks.
    let game = get_test_game_with_board(
        vec![
            slurpuff_over_swirlix(vec![EnergyType::Colorless, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::B3004Celebi),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let sweets_relays: Vec<Attack> = offered_attacks(&game)
        .into_iter()
        .filter(|attack| attack.title == "Sweets Relay")
        .collect();

    // Slurpuff's own Sweets Relay: 30 damage, "+60" effect.
    let slurpuff_relay = nth_attack(CardId::A3b032Slurpuff, 0);
    // Swirlix's Sweets Relay (granted by Time Recall): 10 damage, "+20" effect.
    let swirlix_relay = nth_attack(CardId::A3b031Swirlix, 0);
    assert_ne!(
        slurpuff_relay, swirlix_relay,
        "Sanity: the two Sweets Relay attacks should be different structs"
    );

    assert!(
        sweets_relays.contains(&slurpuff_relay),
        "Slurpuff's own Sweets Relay should be offered, got {sweets_relays:?}"
    );
    assert!(
        sweets_relays.contains(&swirlix_relay),
        "Swirlix's Sweets Relay should be granted by Time Recall, got {sweets_relays:?}"
    );
    assert_eq!(
        sweets_relays.len(),
        2,
        "Exactly the two distinct Sweets Relay attacks should be offered, got {sweets_relays:?}"
    );
}

#[test]
fn test_without_celebi_only_own_sweets_relay_offered() {
    // Same Slurpuff, but no Celebi: only Slurpuff's own Sweets Relay is available; Swirlix's
    // previous-evolution variant must not appear.
    let game = get_test_game_with_board(
        vec![slurpuff_over_swirlix(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let sweets_relays: Vec<Attack> = offered_attacks(&game)
        .into_iter()
        .filter(|attack| attack.title == "Sweets Relay")
        .collect();

    let slurpuff_relay = nth_attack(CardId::A3b032Slurpuff, 0);
    assert_eq!(
        sweets_relays,
        vec![slurpuff_relay],
        "Without Celebi, only Slurpuff's own Sweets Relay should be offered, got {sweets_relays:?}"
    );
}

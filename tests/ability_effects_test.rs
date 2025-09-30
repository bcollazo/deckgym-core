use common::get_initialized_game;
use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    generate_possible_actions,
    types::{EnergyType, PlayedCard},
};

mod common;

#[test]
fn test_serperior_jungle_totem_ability() {
    // Serperior's Jungle Totem: Each Grass Energy attached to your Grass Pokémon provides 2 Grass Energy
    // Bulbasaur's Vine Whip requires 1 Grass + 1 Colorless (2 total)
    // With Jungle Totem, 1 Grass energy should count as 2, making the attack usable

    let bulbasaur_card = get_card_by_enum(CardId::A1001Bulbasaur);
    let serperior_card = get_card_by_enum(CardId::A1a006Serperior);

    // Initialize with basic decks
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Ensure we're testing with the correct player
    let test_player = state.current_player;

    // Set up test_player with Bulbasaur in active position with only 1 Grass energy
    let bulbasaur = PlayedCard::new(
        bulbasaur_card.clone(),
        70,                      // remaining_hp
        70,                      // total_hp
        vec![EnergyType::Grass], // Only 1 Grass energy (normally not enough for Vine Whip)
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][0] = Some(bulbasaur);

    // Set up Serperior on the bench
    let serperior = PlayedCard::new(
        serperior_card.clone(),
        110,    // remaining_hp
        110,    // total_hp
        vec![], // No energy needed for ability
        false,
        vec![],
    );
    state.in_play_pokemon[test_player][1] = Some(serperior);

    // Clear the move generation stack so we can test attack generation
    state.move_generation_stack.clear();

    game.set_state(state.clone());

    // Generate possible actions
    let (actor, actions) = generate_possible_actions(&state);

    // Check if attack action is available
    let has_attack_action = actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::Attack(_)));

    assert_eq!(
        actor, test_player,
        "Current player should match test_player"
    );
    assert!(
        has_attack_action,
        "With Serperior's Jungle Totem, Bulbasaur should be able to attack with only 1 Grass energy"
    );

    // Verify the specific attack index (should be attack 0 - Vine Whip)
    let attack_actions: Vec<_> = actions
        .iter()
        .filter(|action| matches!(action.action, SimpleAction::Attack(_)))
        .collect();

    assert_eq!(
        attack_actions.len(),
        1,
        "Should have exactly one attack action available"
    );

    if let SimpleAction::Attack(index) = attack_actions[0].action {
        assert_eq!(index, 0, "Attack index should be 0 (Vine Whip)");
    }
}

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn game_with_rainbow_cave(
    current: Option<EnergyType>,
    next: Option<EnergyType>,
) -> deckgym::Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 3;
    state.active_stadium = Some(get_card_by_enum(CardId::B4155RainbowCave));
    state.energy_zone[0].current = current;
    state.energy_zone[0].next = next;
    game.set_state(state);
    game
}

fn use_stadium(game: &mut deckgym::Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::UseStadium,
        is_stack: false,
    });
}

fn has_use_stadium(game: &deckgym::Game<'static>) -> bool {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|action| matches!(action.action, SimpleAction::UseStadium))
}

#[test]
fn test_rainbow_cave_discards_current_energy_and_produces_the_next_one() {
    let mut game = game_with_rainbow_cave(Some(EnergyType::Fire), Some(EnergyType::Water));

    assert!(
        has_use_stadium(&game),
        "UseStadium should be available while an Energy is generated in the Energy Zone"
    );

    use_stadium(&mut game, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.energy_zone[0].current,
        Some(EnergyType::Water),
        "The previewed next Energy should be produced as the new current Energy"
    );
    assert!(
        state.energy_zone[0].next.is_some(),
        "A fresh next Energy should be queued up behind it"
    );
    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Fire],
        "The generated Energy should end up in the discard pile"
    );
    assert!(
        state.has_used_stadium[0],
        "Rainbow Cave is a once-per-turn stadium effect"
    );
    assert!(
        !has_use_stadium(&game),
        "Player 0 should not be able to use Rainbow Cave twice in one turn"
    );
}

#[test]
fn test_rainbow_cave_unavailable_when_energy_zone_is_empty() {
    // `current` is None once the player has already attached their Energy for the turn,
    // so there is nothing left to discard.
    let game = game_with_rainbow_cave(None, Some(EnergyType::Water));

    assert!(
        !has_use_stadium(&game),
        "UseStadium should not be offered when no Energy is generated in the Energy Zone"
    );
}

#[test]
fn test_rainbow_cave_new_energy_is_still_attachable_this_turn() {
    let mut game = game_with_rainbow_cave(Some(EnergyType::Fire), Some(EnergyType::Water));
    use_stadium(&mut game, 0);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attaches_water = actions.iter().any(|action| match &action.action {
        SimpleAction::Attach {
            attachments,
            is_turn_energy: true,
        } => attachments
            .iter()
            .any(|(_, energy, _)| *energy == EnergyType::Water),
        _ => false,
    });
    assert!(
        attaches_water,
        "The newly produced Energy should be attachable during the same turn"
    );
}

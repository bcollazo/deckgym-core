use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{get_initialized_game_with_board, get_test_game_with_board},
};

/// True if the current player can retreat the active Pokémon into bench slot 1 with the energy
/// currently attached (i.e. the modified retreat cost is affordable).
fn can_retreat_to_bench_1(game: &deckgym::Game<'_>) -> bool {
    let (_, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Retreat(1)))
}

/// Heatran's Speed Link: "If you have Arceus or Arceus ex in play, this Pokémon has no Retreat
/// Cost." Heatran's printed cost is [C][C][C], so with no Energy attached it can only retreat
/// while Arceus is on the board.
#[test]
fn test_speed_link_zeroes_retreat_cost_with_arceus_in_play() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a013Heatran),
            PlayedCard::from_id(CardId::A2a071ArceusEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&game));
}

#[test]
fn test_speed_link_does_nothing_without_arceus() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2a013Heatran),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&game));
}

/// Latios's Fantastical Floating: "If you have Latias in play, this Pokémon has no Retreat Cost."
#[test]
fn test_fantastical_floating_zeroes_retreat_cost_with_latias_in_play() {
    let with_latias = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4a037Latios),
            PlayedCard::from_id(CardId::A4a036Latias),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&with_latias));

    let without_latias = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4a037Latios),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&without_latias));
}

/// Jumpluff's Fluffy Flight: "Your Active Pokémon has no Retreat Cost." Works from the Bench and
/// applies to whichever of your Pokémon is Active.
#[test]
fn test_fluffy_flight_zeroes_active_retreat_cost_from_bench() {
    let game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A4015Jumpluff),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&game));
}

/// Tatsugiri's Retreat Directive only frees an Active *Dondozo*.
#[test]
fn test_retreat_directive_only_applies_to_dondozo() {
    let with_dondozo = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A2b020Dondozo),
            PlayedCard::from_id(CardId::A2b021Tatsugiri),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&with_dondozo));

    let with_snorlax = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1211Snorlax),
            PlayedCard::from_id(CardId::A2b021Tatsugiri),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&with_snorlax));
}

/// Wimpod's Wimp Out: "During your first turn, this Pokémon has no Retreat Cost." Wimpod's
/// printed cost is [C][C][C].
#[test]
fn test_wimp_out_only_on_your_first_turn() {
    // turn_count 1 is player 0's first turn.
    let first_turn = get_initialized_game_with_board(
        0,
        0,
        1,
        vec![
            PlayedCard::from_id(CardId::A3021Wimpod),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&first_turn));

    let later_turn = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3021Wimpod),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&later_turn));
}

/// Alolan Raichu's Surge Surfer: "If a Stadium is in play, this Pokémon has no Retreat Cost."
#[test]
fn test_surge_surfer_needs_a_stadium_in_play() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B2050AlolanRaichu),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&game));

    let mut state = game.get_state_clone();
    state.set_active_stadium(deckgym::database::get_card_by_enum(
        CardId::B3154ArenaofAntiquity,
    ));
    game.set_state(state);
    assert!(can_retreat_to_bench_1(&game));
}

/// Beldum's Conductive Body: "If you have another Beldum in play, this Pokémon's Retreat Cost is
/// 2 less." Beldum's printed cost is [C][C], so a second Beldum makes it free.
#[test]
fn test_conductive_body_needs_another_beldum() {
    let alone = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4106Beldum),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&alone));

    let paired = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4106Beldum),
            PlayedCard::from_id(CardId::PB086Beldum),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(can_retreat_to_bench_1(&paired));

    // With one Energy attached the lone Beldum still can't pay a 2-Energy cost.
    let one_energy = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4106Beldum).with_energy(vec![EnergyType::Metal]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    assert!(!can_retreat_to_bench_1(&one_energy));
}

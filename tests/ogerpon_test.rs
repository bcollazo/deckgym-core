// Tests for Ogerpon-family cards from Fantastical Parade (B2).
//
// Cards covered:
//   B2 017 / B2 180 / B2 194 – Teal Mask Ogerpon ex
//     Ability: Soothing Wind (passive)
//     Attack:  Energized Leaves – GG, 60 (+60 if combined active energy ≥ 5)
//   B2 027 – Hearthflame Mask Ogerpon
//     Attack:  Hearthflame Dance – RC, 40 (flip coin; heads: attach 2 [R] to a bench)
//   B2 048 – Wellspring Mask Ogerpon
//     Attack:  Wellspring Dance – WC, 40 (flip coin; heads: also 40 to opponent bench)
//   B2 093 – Cornerstone Mask Ogerpon
//     Attack:  Cornerstone Dance – FC, 40 (flip coin; heads: -100 dmg reduction next turn)

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition, TrainerCard},
    test_support::get_initialized_game,
};

/// Build a PlayedCard with a custom base HP (useful for avoiding KOs in damage tests).
fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(t) => t,
        _ => panic!("Expected trainer card"),
    }
}

fn play_will(game: &mut deckgym::Game<'static>, actor: usize) {
    let will = trainer_from_id(CardId::A4156Will);
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });
}

// ── Teal Mask Ogerpon ex – Energized Leaves ──────────────────────────────────

/// When combined active energy < 5, Energized Leaves deals only its base 60 damage.
#[test]
fn test_energized_leaves_base_damage_below_threshold() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Ogerpon has 2 energy; opponent's active has 2 → total 4, below the 5 threshold.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2017TealMaskOgerponEx)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Bulbasaur HP 70 – 60 base = 10
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 10,
        "Energized Leaves should deal only 60 damage when combined energy < 5"
    );
}

/// When combined active energy ≥ 5, Energized Leaves deals 60 + 60 = 120 damage.
#[test]
fn test_energized_leaves_extra_damage_at_threshold() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Ogerpon 2 energy + opponent 3 energy = 5 total → bonus applies.
    // Use a 200 HP Bulbasaur so it survives the 120 damage.
    // Bulbasaur is weak to Fire, not Grass, so no weakness modifier from Teal Mask Ogerpon ex.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2017TealMaskOgerponEx)
            .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
        vec![
            played_card_with_base_hp(CardId::A1001Bulbasaur, 200).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Grass,
                EnergyType::Grass,
            ]),
        ],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // 200 – 120 = 80
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 80,
        "Energized Leaves should deal 120 damage when combined energy ≥ 5"
    );
}

// ── Teal Mask Ogerpon ex – Soothing Wind (ability) ───────────────────────────

/// A Pokémon with energy on a team that has Soothing Wind cannot be inflicted with
/// a Special Condition via apply_status_condition.
#[test]
fn test_soothing_wind_prevents_status_on_energized_pokemon() {
    let game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Teal Mask Ogerpon ex on bench; active Bulbasaur has energy attached.
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::B2017TealMaskOgerponEx),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // Attempt to inflict Poison on player 0's active (has energy → protected).
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);

    assert!(
        !state.get_active(0).is_poisoned(),
        "Soothing Wind should prevent Poisoned on an energized Pokémon"
    );
}

/// A Pokémon WITHOUT energy on a Soothing Wind team CAN still be inflicted.
#[test]
fn test_soothing_wind_allows_status_on_unenergized_pokemon() {
    let game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            // No energy on active
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::B2017TealMaskOgerponEx),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // No energy → Soothing Wind does not protect.
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);

    assert!(
        state.get_active(0).is_poisoned(),
        "Soothing Wind should not protect a Pokémon with no energy"
    );
}

/// Pokémon with energy on a Soothing Wind team are protected from multiple status types.
#[test]
fn test_soothing_wind_prevents_paralysis() {
    let game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::B2017TealMaskOgerponEx),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    state.apply_status_condition(0, 0, StatusCondition::Paralyzed);

    assert!(
        !state.get_active(0).is_paralyzed(),
        "Soothing Wind should prevent Paralysis on an energized Pokémon"
    );
}

// ── Hearthflame Mask Ogerpon – Hearthflame Dance ─────────────────────────────

/// Hearthflame Dance always deals its 40 base damage to the active opponent.
#[test]
fn test_hearthflame_dance_deals_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Charmander is Fire type (weak to Water, not Fire) so no weakness modifier applies.
    // Use a 100 HP Charmander to survive the 40 damage.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2027HearthflameMaskOgerpon)
            .with_energy(vec![EnergyType::Fire, EnergyType::Colorless])],
        vec![played_card_with_base_hp(CardId::A1033Charmander, 100)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Charmander 100 HP – 40 = 60 (regardless of coin flip)
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 60,
        "Hearthflame Dance should always deal 40 damage to the active opponent"
    );
}

/// On heads (forced via Will), Hearthflame Dance offers an Attach choice for bench Pokémon.
#[test]
fn test_hearthflame_dance_heads_queues_bench_attach_choice() {
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B2027HearthflameMaskOgerpon)
                .with_energy(vec![EnergyType::Fire, EnergyType::Colorless]),
            PlayedCard::from_id(CardId::A1001Bulbasaur), // bench target
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    // Force next coin flip to be heads via Will.
    play_will(&mut game, 0);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // The engine should now be waiting for player 0 to choose a bench Pokémon to attach to.
    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::Attach { .. })),
        "On heads, Hearthflame Dance should offer Attach choices for bench Pokémon"
    );
}

// ── Wellspring Mask Ogerpon – Wellspring Dance ────────────────────────────────

/// Wellspring Dance always deals its 40 base damage to the active opponent.
#[test]
fn test_wellspring_dance_deals_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is Grass type (weak to Fire, not Water) so no weakness modifier applies.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2048WellspringMaskOgerpon)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Bulbasaur 70 HP – 40 = 30 (regardless of coin flip)
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 30,
        "Wellspring Dance should always deal 40 damage to the active opponent"
    );
}

/// On heads (forced via Will), Wellspring Dance offers a bench-damage ApplyDamage choice.
#[test]
fn test_wellspring_dance_heads_queues_bench_damage_choice() {
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2048WellspringMaskOgerpon)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),  // active
            PlayedCard::from_id(CardId::A1033Charmander), // bench target
        ],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    play_will(&mut game, 0);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        choices
            .iter()
            .any(|c| matches!(c.action, SimpleAction::ApplyDamage { .. })),
        "On heads, Wellspring Dance should offer bench-damage choices"
    );
}

// ── Cornerstone Mask Ogerpon – Cornerstone Dance ─────────────────────────────

/// Cornerstone Dance always deals its 40 base damage to the active opponent.
#[test]
fn test_cornerstone_dance_deals_base_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Bulbasaur is Grass type (weak to Fire, not Fighting) so no weakness modifier applies.
    state.set_board(
        vec![PlayedCard::from_id(CardId::B2093CornerstoneMaskOgerpon)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.current_player = 0;
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Bulbasaur 70 HP – 40 = 30 (regardless of coin flip)
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(hp, 30, "Cornerstone Dance should always deal 40 damage");
}

/// On heads (forced via Will), Cornerstone Dance reduces damage taken by 100 for one turn.
#[test]
fn test_cornerstone_dance_heads_reduces_incoming_damage() {
    let will = trainer_from_id(CardId::A4156Will);

    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::B2093CornerstoneMaskOgerpon)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless])
            .with_remaining_hp(80)],
        vec![
            // Rampardos (Head Smash: 130 dmg) – normally would KO Ogerpon but -100 saves it.
            PlayedCard::from_id(CardId::A2089Rampardos).with_energy(vec![EnergyType::Fighting]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    state.current_player = 0;
    state.turn_count = 2;
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    // Player 0: Will → force heads, then Cornerstone Dance.
    play_will(&mut game, 0);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    // End player 0's turn.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Player 1 attacks with Rampardos (Head Smash = 130 dmg; Ogerpon has 80 HP).
    // Without the -100 reduction, Ogerpon would be KO'd. With it, it survives.
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    let ogerpon_hp = game.get_state_clone().get_active(0).get_remaining_hp();
    assert!(
        ogerpon_hp > 0,
        "Cornerstone Dance's -100 damage reduction should keep Ogerpon alive (130 - 100 = 30 dmg on 80 HP)"
    );
}

// ── Soothing Wind – cure on play ─────────────────────────────────────────────

/// When Teal Mask Ogerpon ex is played to the bench, it immediately cures status
/// conditions from any energy-bearing Pokémon on that player's side.
#[test]
fn test_soothing_wind_cures_on_play() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0 has a poisoned Bulbasaur with energy in the active spot.
    // Ogerpon is in hand – not yet in play, so no Soothing Wind effect.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    // Poison the active via apply_status_condition – Ogerpon not in play yet, so no prevention.
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    assert!(
        state.get_active(0).is_poisoned(),
        "Bulbasaur should be Poisoned before Ogerpon enters play"
    );

    // Add Ogerpon to player 0's hand and place it on the bench.
    let ogerpon_card = get_card_by_enum(CardId::B2017TealMaskOgerponEx);
    state.hands[0].push(ogerpon_card.clone());
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Place(ogerpon_card, 1),
        is_stack: false,
    });

    assert!(
        !game.get_state_clone().get_active(0).is_poisoned(),
        "Soothing Wind should cure Poison the moment Ogerpon enters play"
    );
}

// ── Venoshock interaction ─────────────────────────────────────────────────────

/// Salandit's Venoshock deals 10 + 40 = 50 damage when the target is Poisoned.
/// Uses Charmander (Fire type, weak to Water) so Salandit's Fire attacks have no weakness bonus.
#[test]
fn test_venoshock_deals_extra_damage_when_poisoned() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Salandit (Fire) with 1 Colorless energy (enough for Venoshock).
    // Player 1: Charmander (Fire, 60 HP, weak to Water – NOT weak to Fire) – will be poisoned.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a015Salandit).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    state.current_player = 0;

    // Poison player 1's active (no Soothing Wind in play yet).
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Charmander 60 HP – 50 (10 base + 40 poison bonus, no weakness) = 10.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 10,
        "Venoshock should deal 50 damage to a Poisoned target"
    );
}

/// When Ogerpon is played to the bench, it cures Poison from the active, and
/// a subsequent Venoshock deals only base 10 damage (no poison bonus).
#[test]
fn test_soothing_wind_cures_before_venoshock_no_extra_damage() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    // Player 0: Salandit (Fire) active with energy (attacker).
    // Player 1: Charmander (Fire, 60 HP, weak to Water – NOT weak to Fire) with energy + Ogerpon in hand.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1a015Salandit).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire])],
    );

    // Poison Charmander via apply_status_condition – Ogerpon not in play yet, no prevention.
    state.apply_status_condition(1, 0, StatusCondition::Poisoned);
    assert!(
        state.get_active(1).is_poisoned(),
        "Charmander should be Poisoned before Ogerpon enters play"
    );

    // Player 1 plays Ogerpon to bench → Soothing Wind immediately cures Charmander.
    let ogerpon_card = get_card_by_enum(CardId::B2017TealMaskOgerponEx);
    state.hands[1].push(ogerpon_card.clone());
    game.set_state(state);

    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::Place(ogerpon_card, 1),
        is_stack: false,
    });

    assert!(
        !game.get_state_clone().get_active(1).is_poisoned(),
        "Soothing Wind should have cured Charmander's Poison when Ogerpon was played"
    );

    // Now player 0 attacks with Venoshock – Charmander is no longer Poisoned,
    // so only base 10 damage should be dealt (no weakness bonus since Charmander isn't weak to Fire).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });

    // Charmander 60 HP – 10 (base only, no poison bonus, no weakness) = 50.
    let hp = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(
        hp, 50,
        "Venoshock should deal only base 10 damage after Soothing Wind cured the Poison"
    );
}

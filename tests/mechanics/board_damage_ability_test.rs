use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Runs one attack from player 0's Active Pokémon and returns the damage taken by player 1's
/// Active Pokémon.
fn damage_dealt_to_defender(
    attacker: PlayedCard,
    attacker_card: CardId,
    defender_board: Vec<PlayedCard>,
    attacker_bench: Vec<PlayedCard>,
) -> u32 {
    let mut player_board = vec![attacker];
    player_board.extend(attacker_bench);
    let before = defender_board[0].get_remaining_hp();
    let mut game = get_test_game_with_board(player_board, defender_board);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(attacker_card, 0),
        is_stack: false,
    });
    let state = game.get_state_clone();
    let after = state.in_play_pokemon[1][0]
        .as_ref()
        .map(|p| p.get_remaining_hp())
        .unwrap_or(0);
    before - after
}

/// Piloswine's Thick Fat: "This Pokémon takes -20 damage from attacks from [R] or [W] Pokémon."
#[test]
fn test_piloswine_thick_fat_reduces_water_attacks() {
    // Primarina's Surf is 60; Piloswine is weak to [M], so no weakness bonus applies.
    let damage = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A3048Primarina)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless]),
        CardId::A3048Primarina,
        vec![PlayedCard::from_id(CardId::A2032Piloswine)],
        vec![],
    );
    assert_eq!(damage, 40, "Surf's 60 should be reduced by 20");
}

/// Staraptor's Defensive Whirlwind: "-30 damage from attacks from [F] Pokémon."
#[test]
fn test_staraptor_defensive_whirlwind_reduces_fighting_attacks() {
    let damage = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A1a046AerodactylEx)
            .with_energy(vec![EnergyType::Fighting, EnergyType::Colorless]),
        CardId::A1a046AerodactylEx,
        vec![PlayedCard::from_id(CardId::PA047Staraptor)],
        vec![],
    );
    assert_eq!(damage, 50, "Land Crush's 80 should be reduced by 30");
}

/// Eiscue's Ice Face: "-40 damage from attacks" only while it is at full HP.
#[test]
fn test_eiscue_ice_face_only_applies_at_full_hp() {
    let at_full_hp = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]),
        CardId::A3135Toucannon,
        vec![PlayedCard::from_id(CardId::B1080Eiscue)],
        vec![],
    );
    assert_eq!(at_full_hp, 30, "Drill Peck's 70 should be reduced by 40");

    let already_damaged = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]),
        CardId::A3135Toucannon,
        vec![
            PlayedCard::from_id(CardId::B1080Eiscue).with_damage(10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![],
    );
    assert_eq!(
        already_damaged, 70,
        "Ice Face stops working once Eiscue has damage on it"
    );
}

/// Falinks's Coordinated Unit boosts its own attacks by 20 and reduces incoming damage by 20,
/// but only while a second Falinks is in play.
#[test]
fn test_falinks_coordinated_unit_boosts_damage_with_a_second_falinks() {
    let alone = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::B2092Falinks).with_energy(vec![EnergyType::Fighting]),
        CardId::B2092Falinks,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![],
    );
    assert_eq!(alone, 20, "Invade's base damage");

    let paired = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::B2092Falinks).with_energy(vec![EnergyType::Fighting]),
        CardId::B2092Falinks,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![PlayedCard::from_id(CardId::B2172Falinks)],
    );
    assert_eq!(paired, 40, "Invade's 20 plus the Coordinated Unit's +20");
}

#[test]
fn test_falinks_coordinated_unit_reduces_incoming_damage_with_a_second_falinks() {
    let alone = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]),
        CardId::A3135Toucannon,
        vec![
            PlayedCard::from_id(CardId::B2092Falinks),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![],
    );
    assert_eq!(alone, 70);

    let paired = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A3135Toucannon).with_energy(vec![EnergyType::Colorless]),
        CardId::A3135Toucannon,
        vec![
            PlayedCard::from_id(CardId::B2092Falinks),
            PlayedCard::from_id(CardId::B2172Falinks),
        ],
        vec![],
    );
    assert_eq!(paired, 50, "Coordinated Unit takes 20 off");
}

/// Unown GUARD only works while its owner has another Unown in play with a *different* Ability.
#[test]
fn test_unown_guard_needs_an_unown_with_a_different_ability() {
    // Falinks's Invade does 20; Unown only has 60 HP, so use a small attack to keep the
    // measurement away from the knock-out clamp.
    let guard_alone = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::B2092Falinks).with_energy(vec![EnergyType::Fighting]),
        CardId::B2092Falinks,
        vec![
            PlayedCard::from_id(CardId::A4084Unown),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![],
    );
    assert_eq!(guard_alone, 20, "GUARD alone does nothing");

    let guard_with_check = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::B2092Falinks).with_energy(vec![EnergyType::Fighting]),
        CardId::B2092Falinks,
        vec![
            PlayedCard::from_id(CardId::A4084Unown),
            PlayedCard::from_id(CardId::A2a034Unown),
        ],
        vec![],
    );
    assert_eq!(
        guard_with_check, 10,
        "GUARD takes 10 off with CHECK in play"
    );
}

/// Unown POWER adds +10 to the owner's attacks while another differently-abled Unown is in play.
#[test]
fn test_unown_power_needs_an_unown_with_a_different_ability() {
    let power_alone = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A4085Unown).with_energy(vec![EnergyType::Colorless]),
        CardId::A4085Unown,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![],
    );
    assert_eq!(power_alone, 20, "Hidden Power's base damage");

    let power_with_check = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A4085Unown).with_energy(vec![EnergyType::Colorless]),
        CardId::A4085Unown,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![PlayedCard::from_id(CardId::A2a034Unown)],
    );
    assert_eq!(power_with_check, 30, "+10 while CHECK is in play");
}

/// Politoed's Lordly Cheering: "As long as this Pokémon is on your Bench, attacks used by your
/// Pokémon that evolve from Poliwhirl do +40 damage to your opponent's Active Pokémon."
#[test]
fn test_lordly_cheering_boosts_poliwhirl_evolutions_from_the_bench() {
    let without_politoed = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A1061Poliwrath).with_energy(vec![
            EnergyType::Water,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ]),
        CardId::A1061Poliwrath,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![],
    );
    assert_eq!(without_politoed, 80);

    let with_politoed = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A1061Poliwrath).with_energy(vec![
            EnergyType::Water,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ]),
        CardId::A1061Poliwrath,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![PlayedCard::from_id(CardId::A4040Politoed)],
    );
    assert_eq!(with_politoed, 120, "Mega Punch's 80 plus Lordly Cheering");
}

/// The boost is bench-only: an Active Politoed does not cheer for itself.
#[test]
fn test_lordly_cheering_does_not_apply_from_the_active_spot() {
    let damage = damage_dealt_to_defender(
        PlayedCard::from_id(CardId::A4040Politoed)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless]),
        CardId::A4040Politoed,
        vec![PlayedCard::from_id(CardId::A3135Toucannon)],
        vec![],
    );
    assert_eq!(damage, 60, "Hyper Voice's base damage, no self-cheering");
}

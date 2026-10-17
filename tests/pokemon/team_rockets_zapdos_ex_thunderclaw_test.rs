use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

const VENUSAUR_EX_MAX_HP: u32 = 190;
const BULBASAUR_MAX_HP: u32 = 70;

/// Team Rocket's Zapdos ex's Thunderclaw: 90 damage to the Defending Pokémon, plus the attacker
/// may choose 1 of the opponent's Benched Pokémon that already has damage on it to deal 50 more
/// damage to. Undamaged Benched Pokémon aren't offered as targets.
#[test]
fn test_zapdos_ex_thunderclaw_only_offers_damaged_bench_as_target() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a021TeamRocketsZapdosEx).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(BULBASAUR_MAX_HP - 10),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a021TeamRocketsZapdosEx, 1),
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(!choices.is_empty());
    for choice in &choices {
        let SimpleAction::ApplyDamage { targets, .. } = &choice.action else {
            panic!("Expected ApplyDamage choices");
        };
        assert!(
            targets.iter().all(|(_, _, in_play_idx)| *in_play_idx != 2),
            "Undamaged benched Bulbasaur (idx 2) should never be offered as a target"
        );
    }

    let choice = choices[0].clone();
    game.apply_action(&choice);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 90
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Damaged benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        BULBASAUR_MAX_HP - 10 - 50
    );
    assert_eq!(
        state.in_play_pokemon[1][2]
            .as_ref()
            .expect("Undamaged benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        BULBASAUR_MAX_HP
    );
}

/// "This attack ALSO does 50 damage to 1 of your opponent's Benched Pokémon that has damage on
/// it": when no Benched Pokémon is damaged the bonus fizzles, but the 90 to the Defending Pokémon
/// still lands.
#[test]
fn test_zapdos_ex_thunderclaw_still_damages_active_without_a_damaged_bench() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B4a021TeamRocketsZapdosEx).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4a021TeamRocketsZapdosEx, 1),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        VENUSAUR_EX_MAX_HP - 90,
        "Thunderclaw's printed 90 damage must land even with no eligible bench target"
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Undamaged benched Bulbasaur should still be in play")
            .get_remaining_hp(),
        BULBASAUR_MAX_HP,
        "Undamaged benched Bulbasaur is not an eligible target and must be untouched"
    );
}

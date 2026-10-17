//! Hit-back Abilities are recognised by their text, so every printing works and an effect that
//! removes Abilities (Alolan Muk's Power of Alchemy, Budew's Prickly Powder) switches them off.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

/// Player 0's Active Pokémon deals 10 damage to player 1's Active Pokémon.
fn active_hits_active(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(10, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });
}

fn snorlax_with_energy() -> PlayedCard {
    PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![EnergyType::Colorless; 4])
}

fn attacker_hp_after_being_hit_by_snorlax(defender: CardId, extra: Vec<PlayedCard>) -> u32 {
    let mut game = get_test_game_with_board(
        vec![snorlax_with_energy()],
        std::iter::once(PlayedCard::from_id(defender))
            .chain(extra)
            .collect(),
    );
    active_hits_active(&mut game);
    game.get_state_clone().get_active(0).get_remaining_hp()
}

/// Iron Jugulis - Automated Combat: "If this Pokémon is in the Active Spot and is damaged by an
/// attack from your opponent's Pokémon, do 20 damage to the Attacking Pokémon."
#[test]
fn test_iron_jugulis_automated_combat_hits_back() {
    let full_hp = PlayedCard::from_id(CardId::A1211Snorlax).get_remaining_hp();
    assert_eq!(
        attacker_hp_after_being_hit_by_snorlax(CardId::B3a046IronJugulis, vec![]),
        full_hp - 20
    );
}

/// Dragalge ex - Poison Point is printed on B1 160, B1 263, B1 281, B3 231 and B4b 148 alike.
#[test]
fn test_every_dragalge_ex_printing_poisons_the_attacker() {
    for dragalge in [
        CardId::B1160DragalgeEx,
        CardId::B1263DragalgeEx,
        CardId::B1281DragalgeEx,
        CardId::B3231DragalgeEx,
        CardId::B4b148DragalgeEx,
    ] {
        let mut game = get_test_game_with_board(
            vec![snorlax_with_energy()],
            vec![PlayedCard::from_id(dragalge)],
        );
        active_hits_active(&mut game);
        assert!(
            game.get_state_clone().get_active(0).is_poisoned(),
            "{dragalge:?} should poison the Attacking Pokémon"
        );
    }
}

/// Power of Alchemy: "Basic Pokémon in play (both yours and your opponent's) have no Abilities."
/// Druddigon is a Basic, so its Rough Skin does nothing next to an Alolan Muk.
#[test]
fn test_power_of_alchemy_switches_off_a_hit_back_ability() {
    let full_hp = PlayedCard::from_id(CardId::A1211Snorlax).get_remaining_hp();
    assert_eq!(
        attacker_hp_after_being_hit_by_snorlax(CardId::A1a056Druddigon, vec![]),
        full_hp - 20,
        "control: Rough Skin hits back"
    );
    assert_eq!(
        attacker_hp_after_being_hit_by_snorlax(
            CardId::A1a056Druddigon,
            vec![PlayedCard::from_id(CardId::B2097AlolanMuk)]
        ),
        full_hp,
        "Rough Skin is off while Power of Alchemy is in play"
    );
}

/// Same for Poison Point on a Basic... Dragalge ex is a Stage 1, so Muk leaves it alone.
#[test]
fn test_power_of_alchemy_does_not_stop_a_stage_1_hit_back() {
    let mut game = get_test_game_with_board(
        vec![snorlax_with_energy()],
        vec![
            PlayedCard::from_id(CardId::B3231DragalgeEx),
            PlayedCard::from_id(CardId::B2097AlolanMuk),
        ],
    );
    active_hits_active(&mut game);
    assert!(game.get_state_clone().get_active(0).is_poisoned());
}

/// Budew - Prickly Powder: "The Defending Pokémon loses all Abilities." A hit that lands after
/// the effect is set up gets no Rough Skin recoil.
#[test]
fn test_prickly_powder_switches_off_a_hit_back_ability() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3013Budew)],
        vec![PlayedCard::from_id(CardId::A1a056Druddigon)],
    );
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3013Budew, 0),
        is_stack: false,
    });
    // Prickly Powder's own hit resolves before its effect lands, so it is the later hit that
    // shows whether Rough Skin is still on.
    let hp_before = game.get_state_clone().get_active(0).get_remaining_hp();
    active_hits_active(&mut game);
    assert_eq!(
        game.get_state_clone().get_active(0).get_remaining_hp(),
        hp_before
    );
}

//! The hit back left by an attack takes Weakness.
//!
//! Togedemaru's Bristling Spikes (and Cursed Jewel, Spike Armor, Needle Lariat, Shell Trap) read:
//! "During your opponent's next turn, if this Pokémon is damaged by an attack, do 30 damage to
//! the Attacking Pokémon." That damage is part of the attack that set it up, so it is attack
//! damage to the opponent's Active Pokémon and Weakness applies. The text for Rocky Helmet and
//! for hit-back Abilities (Rough Skin and its reprints) says nothing of the kind: their damage
//! stays flat.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
    Game,
};

const ALTARIA_HP: u32 = 190;
const BULBASAUR_HP: u32 = 70;

/// Togedemaru (Metal) uses Bristling Spikes, then the opponent's `attacker` hits it with its
/// first attack. Returns the game after that attack.
fn togedemaru_spikes_then_attacked_by(
    attacker: PlayedCard,
    attacker_id: CardId,
    stadium: Option<CardId>,
) -> Game<'static> {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b048Togedemaru)
            .with_energy(vec![EnergyType::Metal, EnergyType::Metal])],
        vec![attacker],
    );
    if let Some(stadium) = stadium {
        let mut state = game.get_state_clone();
        state.set_active_stadium(get_card_by_enum(stadium));
        game.set_state(state);
    }
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3b048Togedemaru, 0),
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(attacker_id, 0),
        is_stack: false,
    });
    game
}

fn mega_altaria() -> PlayedCard {
    // Mega Altaria ex is weak to [M], the type of Togedemaru.
    PlayedCard::from_id(CardId::B1102MegaAltariaEx)
        .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])
}

#[test]
fn test_hit_back_left_by_an_attack_takes_weakness() {
    let game = togedemaru_spikes_then_attacked_by(mega_altaria(), CardId::B1102MegaAltariaEx, None);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        ALTARIA_HP - 2 * (30 + 20),
        "Bristling Spikes does 30 + 20 Weakness to Mega Altaria ex, and so does its hit back"
    );
}

#[test]
fn test_hit_back_left_by_an_attack_without_weakness_stays_30() {
    // Bulbasaur is [G], weak to [R], not to Togedemaru's [M].
    let bulbasaur = PlayedCard::from_id(CardId::A1001Bulbasaur)
        .with_energy(vec![EnergyType::Grass, EnergyType::Colorless]);
    let game = togedemaru_spikes_then_attacked_by(bulbasaur, CardId::A1001Bulbasaur, None);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        BULBASAUR_HP - 2 * 30,
        "No Weakness, so Bristling Spikes and its hit back do just 30 each"
    );
}

#[test]
fn test_hit_back_left_by_an_attack_doubles_under_bounded_field() {
    let game = togedemaru_spikes_then_attacked_by(
        mega_altaria(),
        CardId::B1102MegaAltariaEx,
        Some(CardId::B3155BoundedField),
    );

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        ALTARIA_HP - 2 * (30 * 2),
        "Bounded Field turns Weakness into x2 for the attack and its hit back (30 x 2 = 60 each), \
         as recorded in game with Spike Armor (40 x 2 = 80)"
    );
}

#[test]
fn test_rocky_helmet_recoil_takes_no_weakness() {
    // Rocky Helmet: "If the Pokémon this card is attached to is in the Active Spot and is
    // damaged by an attack from your opponent's Pokémon, do 20 damage to the Attacking Pokémon."
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3b048Togedemaru)
            .with_tool(get_card_by_enum(CardId::A2148RockyHelmet))],
        vec![mega_altaria()],
    );
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::B1102MegaAltariaEx, 0),
        is_stack: false,
    });

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        ALTARIA_HP - 20,
        "Rocky Helmet does a flat 20 even to a Pokémon weak to the holder's type"
    );
}

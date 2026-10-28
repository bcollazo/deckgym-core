use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

fn attack(game: &mut deckgym::Game<'static>, card_id: CardId) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(card_id, 0),
        is_stack: false,
    });
}

fn hp_lost_by_defender(defender: PlayedCard) -> u32 {
    let full_hp = defender.get_remaining_hp();
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass; 2])],
        vec![defender],
    );
    attack(&mut game, CardId::A1001Bulbasaur);
    full_hp - game.get_state_clone().get_remaining_hp(1, 0)
}

/// Eviolite: "The Basic Pokémon this card is attached to takes -20 damage from attacks from your
/// opponent's Pokémon."
#[test]
fn test_eviolite_reduces_damage_to_a_basic_holder_by_20() {
    let eviolite = get_card_by_enum(CardId::C1123Eviolite);
    let bare = hp_lost_by_defender(PlayedCard::from_id(CardId::A1033Charmander));
    let with_tool =
        hp_lost_by_defender(PlayedCard::from_id(CardId::A1033Charmander).with_tool(eviolite));
    assert!(bare >= 20);
    assert_eq!(with_tool, bare - 20);
}

#[test]
fn test_eviolite_does_nothing_on_an_evolved_holder() {
    let eviolite = get_card_by_enum(CardId::C1123Eviolite);
    let bare = hp_lost_by_defender(PlayedCard::from_id(CardId::A1034Charmeleon));
    let with_tool =
        hp_lost_by_defender(PlayedCard::from_id(CardId::A1034Charmeleon).with_tool(eviolite));
    assert_eq!(with_tool, bare);
}

fn pikachu_ex_damage(tool: bool) -> u32 {
    let mut attacker =
        PlayedCard::from_id(CardId::A1096PikachuEx).with_energy(vec![EnergyType::Lightning; 3]);
    if tool {
        attacker = attacker.with_tool(get_card_by_enum(CardId::C1124LightBall));
    }
    let mut game = get_test_game_with_board(
        vec![attacker, PlayedCard::from_id(CardId::A1094Pikachu)],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    attack(&mut game, CardId::A1096PikachuEx);
    120 - game.get_state_clone().get_remaining_hp(1, 0)
}

/// Light Ball: "Attacks used by the Pikachu ex this card is attached to do +30 damage to your
/// opponent's Active Pokémon."
#[test]
fn test_light_ball_adds_30_damage_for_pikachu_ex() {
    assert_eq!(pikachu_ex_damage(true), pikachu_ex_damage(false) + 30);
}

#[test]
fn test_light_ball_does_nothing_for_other_pokemon() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
            .with_energy(vec![EnergyType::Grass; 2])
            .with_tool(get_card_by_enum(CardId::C1124LightBall))],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    attack(&mut game, CardId::A1001Bulbasaur);
    assert_eq!(120 - game.get_state_clone().get_remaining_hp(1, 0), 40);
}

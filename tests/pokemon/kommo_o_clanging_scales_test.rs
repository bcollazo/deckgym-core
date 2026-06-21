use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32, energy: Vec<EnergyType>) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), 0, base_hp, energy, false, vec![])
}

/// Kommo-o's "Clanging Scales": "During your opponent's next turn, this Pokémon
/// takes +30 damage from attacks." The vulnerability applies to Kommo-o itself,
/// only during the very next turn.
#[test]
fn test_kommo_o_clanging_scales_increases_damage_taken_next_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A3127Kommoo)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Fighting])],
        vec![played_card_with_base_hp(
            CardId::A1001Bulbasaur,
            300,
            vec![EnergyType::Grass, EnergyType::Colorless],
        )],
    );
    state.current_player = 0;
    game.set_state(state);

    // Kommo-o uses Clanging Scales: 130 damage to Bulbasaur + self vulnerability.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3127Kommoo, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        170,
        "Bulbasaur should take 130 damage from Clanging Scales (300 - 130 = 170)"
    );

    // End Kommo-o's turn; opponent's "next turn" begins.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    // Bulbasaur attacks Kommo-o with Vine Whip (40 damage).
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    // Kommo-o: 150 - (40 + 30) = 80.
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        80,
        "Kommo-o should take 30 extra damage from its own Clanging Scales vulnerability"
    );
}

/// The vulnerability only lasts during the opponent's next turn: once that turn
/// passes, a later attack should not take the extra damage.
#[test]
fn test_kommo_o_clanging_scales_vulnerability_expires_after_one_turn() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![PlayedCard::from_id(CardId::A3127Kommoo)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Fighting])],
        vec![played_card_with_base_hp(
            CardId::A1001Bulbasaur,
            300,
            vec![EnergyType::Grass, EnergyType::Colorless],
        )],
    );
    state.current_player = 0;
    game.set_state(state);

    // Kommo-o uses Clanging Scales.
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A3127Kommoo, 0),
        is_stack: false,
    });
    // End Kommo-o's turn (opponent's "next turn" begins).
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    // Opponent's next turn passes without attacking.
    game.apply_action(&Action {
        actor: 1,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    // Kommo-o's turn passes.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });

    let kommo_o_hp_before = game.get_state_clone().get_active(0).get_remaining_hp();

    // Now Bulbasaur attacks; the vulnerability should have already expired.
    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1001Bulbasaur, 0),
        is_stack: false,
    });

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).get_remaining_hp(),
        kommo_o_hp_before - 40,
        "Vulnerability should have expired; Kommo-o should only take 40 damage from Vine Whip"
    );
}

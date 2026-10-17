use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
};

/// Cheren: "During your opponent's next turn, all of your Watchog and Stoutland take -100 damage
/// from attacks from your opponent's Pokémon ex."
///
/// Player 0 puts `defender` in the Active Spot, optionally plays Cheren, ends the turn, and then
/// player 1's `attacker` uses its first attack. Returns the defender's remaining HP.
fn attack_defender(defender: PlayedCard, attacker: CardId, play_cheren: bool) -> u32 {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![defender],
        vec![PlayedCard::from_id(attacker).with_energy(vec![
            EnergyType::Grass,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
    );

    if play_cheren {
        let cheren = match get_card_by_enum(CardId::B3151Cheren) {
            Card::Trainer(trainer_card) => trainer_card,
            _ => panic!("Cheren should be a Trainer card"),
        };
        let mut state = game.get_state_clone();
        state.hands[0].push(Card::Trainer(cheren.clone()));
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: cheren,
            },
            is_stack: false,
        });
    }

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    game.apply_action(&Action {
        actor: 1,
        action: attack_action(attacker, 0),
        is_stack: false,
    });

    game.get_state_clone().get_remaining_hp(0, 0)
}

#[test]
fn test_cheren_reduces_damage_from_pokemon_ex() {
    // Venusaur ex's Razor Leaf does 60, fully absorbed by Cheren's -100.
    let watchog = PlayedCard::from_id(CardId::B1200Watchog);
    assert_eq!(
        attack_defender(watchog.clone(), CardId::A1004VenusaurEx, false),
        100 - 60
    );
    assert_eq!(attack_defender(watchog, CardId::A1004VenusaurEx, true), 100);
}

#[test]
fn test_cheren_does_not_reduce_damage_from_non_ex() {
    // Bulbasaur's Vine Whip does 40, and Bulbasaur is not a Pokémon ex.
    let stoutland = PlayedCard::from_id(CardId::B1203Stoutland);
    assert_eq!(
        attack_defender(stoutland.clone(), CardId::A1001Bulbasaur, false),
        150 - 40
    );
    assert_eq!(
        attack_defender(stoutland, CardId::A1001Bulbasaur, true),
        150 - 40
    );
}

#[test]
fn test_cheren_does_not_protect_other_pokemon() {
    // Snorlax is neither Watchog nor Stoutland, so it takes the full 60 from a Pokémon ex.
    let snorlax = PlayedCard::from_id(CardId::A1211Snorlax);
    assert_eq!(
        attack_defender(snorlax, CardId::A1004VenusaurEx, true),
        150 - 60
    );
}

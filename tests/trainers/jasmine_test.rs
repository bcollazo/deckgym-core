use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};

/// Jasmine: "During your opponent's next turn, all of your Steelix and Skarmory ex take -50
/// damage from attacks from your opponent's Pokémon."
///
/// Player 0 sets up `defender`, optionally plays Jasmine, ends the turn, and then player 1's
/// Venusaur ex uses Razor Leaf (60) into it. Returns the defender's remaining HP.
fn razor_leaf_into(defender: PlayedCard, play_jasmine: bool) -> u32 {
    let mut game = get_initialized_game_with_board(
        0,
        0,
        3,
        vec![defender],
        vec![
            PlayedCard::from_id(CardId::A1004VenusaurEx).with_energy(vec![
                EnergyType::Grass,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
    );

    if play_jasmine {
        let jasmine = match get_card_by_enum(CardId::A4160Jasmine) {
            Card::Trainer(trainer_card) => trainer_card,
            _ => panic!("Jasmine should be a Trainer card"),
        };
        let mut state = game.get_state_clone();
        state.hands[0].push(Card::Trainer(jasmine.clone()));
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: jasmine,
            },
            is_stack: false,
        });
    }

    end_turn_and_attack(&mut game);
    game.get_state_clone().get_remaining_hp(0, 0)
}

fn end_turn_and_attack(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    game.apply_action(&Action {
        actor: 1,
        action: attack_action(CardId::A1004VenusaurEx, 0),
        is_stack: false,
    });
}

#[test]
fn test_jasmine_reduces_damage_to_steelix() {
    let steelix = PlayedCard::from_id(CardId::A4122Steelix);
    assert_eq!(razor_leaf_into(steelix.clone(), false), 150 - 60);
    assert_eq!(razor_leaf_into(steelix, true), 150 - 10);
}

#[test]
fn test_jasmine_does_not_protect_other_pokemon() {
    // Registeel is a [M] Pokemon that Jasmine does not name, so it takes the full 60.
    let registeel = PlayedCard::from_id(CardId::A2112Registeel);
    assert_eq!(razor_leaf_into(registeel.clone(), false), 110 - 60);
    assert_eq!(razor_leaf_into(registeel, true), 110 - 60);
}

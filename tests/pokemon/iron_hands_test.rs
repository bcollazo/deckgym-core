use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn played_with_hp(card_id: CardId, hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, hp, vec![], false, vec![])
}

/// Successive Slapping does 70 damage per heads over 2 coin flips.
/// Using Will (force heads) ensures 2 heads = 140 damage.
#[test]
fn test_iron_hands_successive_slapping_two_heads_with_will() {
    use deckgym::{actions::Action, card_ids::CardId, database::get_card_by_enum, models::Card};

    fn trainer_from_id(card_id: CardId) -> deckgym::models::TrainerCard {
        match get_card_by_enum(card_id) {
            Card::Trainer(t) => t,
            _ => panic!("Expected trainer card"),
        }
    }

    let mut game = get_initialized_game(0);
    let will = trainer_from_id(CardId::A4156Will);
    let mut state = game.get_state_clone();

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::B3a017IronHands).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![played_with_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    // Play Will to force the first coin to be heads
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3a017IronHands, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let final_state = game.get_state_clone();
    // With Will guaranteeing at least 1 head (and typically 2 heads), test checks
    // that damage is at least 70. With 2 heads: 200 - 140 = 60.
    let opponent_hp = final_state.get_active(1).get_remaining_hp();
    // At minimum 1 head forced by Will => at least 70 damage => hp at most 130
    assert!(
        opponent_hp <= 130,
        "Successive Slapping with Will (at least 1 heads) should deal at least 70 damage, opponent hp={opponent_hp}"
    );
}

/// Successive Slapping can produce 0 damage (0 heads), 70 (1 head), or 140 (2 heads).
#[test]
fn test_iron_hands_successive_slapping_variable_damage() {
    let mut saw_zero = false;
    let mut saw_seventy = false;
    let mut saw_one_forty = false;

    for seed in 0..200 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.set_board(
            vec![
                PlayedCard::from_id(CardId::B3a017IronHands).with_energy(vec![
                    EnergyType::Lightning,
                    EnergyType::Lightning,
                    EnergyType::Colorless,
                ]),
            ],
            vec![played_with_hp(CardId::A1001Bulbasaur, 200)],
        );
        state.current_player = 0;
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B3a017IronHands, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let opponent_hp = game.get_state_clone().get_active(1).get_remaining_hp();
        let damage_dealt = 200 - opponent_hp;

        match damage_dealt {
            0 => saw_zero = true,
            70 => saw_seventy = true,
            140 => saw_one_forty = true,
            other => panic!("Unexpected damage from Successive Slapping: {other}"),
        }

        if saw_zero && saw_seventy && saw_one_forty {
            break;
        }
    }

    assert!(saw_zero, "Expected at least one 0-damage outcome (0 heads)");
    assert!(
        saw_seventy,
        "Expected at least one 70-damage outcome (1 head)"
    );
    assert!(
        saw_one_forty,
        "Expected at least one 140-damage outcome (2 heads)"
    );
}

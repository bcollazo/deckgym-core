use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn played_with_hp(card_id: CardId, hp: u32) -> PlayedCard {
    let card = deckgym::database::get_card_by_enum(card_id);
    PlayedCard::new(card, 0, hp, vec![], false, vec![])
}

/// Thermal Gust: 3-coin tiered damage attack.
/// With Will (force first coin heads), at least 1 head is guaranteed.
/// Uses Squirtle (Water, not weak to Fire) to avoid weakness modifier.
#[test]
fn test_iron_moth_thermal_gust_at_least_one_head_with_will() {
    use deckgym::{database::get_card_by_enum, models::Card};

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
        vec![PlayedCard::from_id(CardId::B3a005IronMoth)
            .with_energy(vec![EnergyType::Fire, EnergyType::Colorless])],
        vec![played_with_hp(CardId::A1053Squirtle, 200)],
    );
    state.current_player = 0;
    state.turn_count = 1;
    state.hands[0] = vec![Card::Trainer(will.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play { trainer_card: will },
        is_stack: false,
    });

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    let final_state = game.get_state_clone();
    let damage_dealt = 200 - final_state.get_active(1).get_remaining_hp();
    // Will forces first coin to be heads, so at least 1 head => at least 30 damage
    assert!(
        damage_dealt >= 30,
        "Iron Moth with Will should deal at least 30 damage (1+ heads), got {damage_dealt}"
    );
}

/// Thermal Gust produces exactly 4 outcomes: 10 (0 heads), 30 (1), 60 (2), 130 (3).
/// Uses Squirtle (Water, not weak to Fire) to avoid the +20 weakness modifier.
#[test]
fn test_iron_moth_thermal_gust_all_damage_tiers() {
    let mut saw_ten = false;
    let mut saw_thirty = false;
    let mut saw_sixty = false;
    let mut saw_one_thirty = false;

    for seed in 0..400 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.set_board(
            vec![PlayedCard::from_id(CardId::B3a005IronMoth)
                .with_energy(vec![EnergyType::Fire, EnergyType::Colorless])],
            vec![played_with_hp(CardId::A1053Squirtle, 200)],
        );
        state.current_player = 0;
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let damage_dealt = 200 - game.get_state_clone().get_active(1).get_remaining_hp();

        match damage_dealt {
            10 => saw_ten = true,
            30 => saw_thirty = true,
            60 => saw_sixty = true,
            130 => saw_one_thirty = true,
            other => panic!("Unexpected damage from Thermal Gust: {other}"),
        }

        if saw_ten && saw_thirty && saw_sixty && saw_one_thirty {
            break;
        }
    }

    assert!(saw_ten, "Expected 10-damage outcome (0 heads)");
    assert!(saw_thirty, "Expected 30-damage outcome (1 head, +20)");
    assert!(saw_sixty, "Expected 60-damage outcome (2 heads, +50)");
    assert!(
        saw_one_thirty,
        "Expected 130-damage outcome (3 heads, +120)"
    );
}

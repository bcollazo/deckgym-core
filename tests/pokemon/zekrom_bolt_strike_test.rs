use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(card_id), 0, base_hp, vec![], false, vec![])
}

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn use_bolt_strike(seed: u64, force_heads: bool) -> (u32, u32) {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    let will = trainer_from_id(CardId::A4156Will);

    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3059Zekrom).with_energy(vec![
            EnergyType::Lightning,
            EnergyType::Lightning,
            EnergyType::Lightning,
        ])],
        vec![played_card_with_base_hp(CardId::A1001Bulbasaur, 200)],
    );
    state.hands[0] = if force_heads {
        vec![Card::Trainer(will.clone())]
    } else {
        vec![]
    };
    game.set_state(state);

    if force_heads {
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play { trainer_card: will },
            is_stack: false,
        });
    }

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Attack(0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    (
        state.get_active(0).get_remaining_hp(),
        state.get_active(1).get_remaining_hp(),
    )
}

#[test]
fn test_zekrom_bolt_strike_heads_has_no_self_damage() {
    for seed in 0..50 {
        let (zekrom_hp, opponent_hp) = use_bolt_strike(seed, true);

        assert_eq!(
            opponent_hp, 90,
            "Seed {seed}: Bolt Strike should deal 110 damage"
        );
        assert_eq!(
            zekrom_hp, 120,
            "Seed {seed}: Will should force heads, so Bolt Strike should not damage Zekrom"
        );
    }
}

#[test]
fn test_zekrom_bolt_strike_tails_damages_itself() {
    let mut saw_tails = false;

    for seed in 0..200 {
        let (zekrom_hp, opponent_hp) = use_bolt_strike(seed, false);

        assert_eq!(
            opponent_hp, 90,
            "Seed {seed}: Bolt Strike should deal 110 damage regardless of coin flip"
        );

        if zekrom_hp == 90 {
            saw_tails = true;
        } else {
            assert_eq!(
                zekrom_hp, 120,
                "Seed {seed}: Bolt Strike should only leave Zekrom at 120 HP on heads or 90 HP on tails"
            );
        }
    }

    assert!(
        saw_tails,
        "Expected at least one tails outcome where Bolt Strike does 30 damage to Zekrom"
    );
}

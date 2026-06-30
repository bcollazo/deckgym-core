use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::{attack_action, get_initialized_game},
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

fn use_bundled_pump(seed: u64, force_heads: bool) -> (u32, u32) {
    let mut game = get_initialized_game(seed);
    let mut state = game.get_state_clone();
    let will = trainer_from_id(CardId::A4156Will);

    state.current_player = 0;
    state.turn_count = 1;
    state.set_board(
        vec![PlayedCard::from_id(CardId::B3b020IronBundle)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
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
            action: deckgym::actions::SimpleAction::Play { trainer_card: will },
            is_stack: false,
        });
    }

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3b020IronBundle, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    (
        state.get_active(0).get_remaining_hp(),
        state.get_active(1).get_remaining_hp(),
    )
}

/// Bundled Pump on heads deals 100 damage (50 + 50 extra) and no self damage.
#[test]
fn test_iron_bundle_bundled_pump_heads_extra_damage_no_self_damage() {
    for seed in 0..50 {
        let (iron_bundle_hp, opponent_hp) = use_bundled_pump(seed, true);

        assert_eq!(
            opponent_hp, 100,
            "Seed {seed}: Bundled Pump (heads) should deal 100 damage (200 - 100 = 100)"
        );
        assert_eq!(
            iron_bundle_hp, 80,
            "Seed {seed}: Will forces heads, so Bundled Pump should not damage Iron Bundle"
        );
    }
}

/// Bundled Pump on tails deals 50 damage to opponent and 50 self damage.
#[test]
fn test_iron_bundle_bundled_pump_tails_self_damage() {
    let mut saw_tails = false;

    for seed in 0..200 {
        let (iron_bundle_hp, opponent_hp) = use_bundled_pump(seed, false);

        if iron_bundle_hp == 30 {
            // Tails: 50 self damage (80 - 50 = 30), opponent takes base 50
            saw_tails = true;
            assert_eq!(
                opponent_hp, 150,
                "Seed {seed}: Bundled Pump (tails) should deal 50 damage (200 - 50 = 150)"
            );
        } else {
            // Heads: no self damage, opponent takes 100
            assert_eq!(
                iron_bundle_hp, 80,
                "Seed {seed}: Bundled Pump should leave Iron Bundle at 80 HP on heads or 30 HP on tails"
            );
            assert_eq!(
                opponent_hp, 100,
                "Seed {seed}: Bundled Pump (heads) should deal 100 damage"
            );
        }
    }

    assert!(
        saw_tails,
        "Expected at least one tails outcome where Bundled Pump does 50 damage to Iron Bundle"
    );
}

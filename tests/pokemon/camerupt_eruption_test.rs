use crate::deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
    test_support::get_initialized_game,
};

fn played_card_with_base_hp(card_id: CardId, base_hp: u32) -> PlayedCard {
    let card = get_card_by_enum(card_id);
    PlayedCard::new(card, 0, base_hp, vec![], false, vec![])
}

fn trainer_from_id(card_id: CardId) -> TrainerCard {
    match get_card_by_enum(card_id) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Expected trainer card"),
    }
}

fn play_trainer(game: &mut crate::deckgym::Game<'static>, actor: usize, trainer_card: TrainerCard) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    });
}

#[test]
fn test_camerupt_eruption_damage_matches_discarded_fire_energy() {
    for seed in 0..200 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3022Camerupt).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ])],
            vec![played_card_with_base_hp(CardId::B3023MegaCameruptEx, 210)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let attacker = state.get_active(0);
        let defender = state.get_active(1);

        let discarded_fire = 3 - attacker.attached_energy.len() as u32;
        let expected_damage = 60 + (discarded_fire * 30);
        let expected_remaining_hp = 210 - expected_damage;

        assert_eq!(
            defender.get_remaining_hp(),
            expected_remaining_hp,
            "Seed {seed}: damage should scale with the Fire energy Camerupt discarded"
        );
    }
}

#[test]
fn test_camerupt_eruption_will_forces_at_least_one_discard_and_bonus_damage() {
    let will = trainer_from_id(CardId::A4156Will);

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3022Camerupt).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ])],
            vec![played_card_with_base_hp(CardId::B3023MegaCameruptEx, 210)],
        );
        state.hands[0] = vec![Card::Trainer(will.clone())];
        game.set_state(state);

        play_trainer(&mut game, 0, will.clone());
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let attacker = state.get_active(0);
        let defender = state.get_active(1);

        assert!(
            attacker.attached_energy.len() <= 2,
            "Seed {seed}: Will should force Eruption to discard at least 1 Fire energy"
        );
        assert!(
            defender.get_remaining_hp() <= 120,
            "Seed {seed}: Will should force Eruption to deal at least 90 damage"
        );
    }
}

use deckgym::{
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

fn play_trainer(game: &mut deckgym::Game<'static>, actor: usize, trainer_card: TrainerCard) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::Play { trainer_card },
        is_stack: false,
    });
}

#[test]
fn test_emboar_flare_storm_adds_damage_for_fire_energy_heads() {
    let will = trainer_from_id(CardId::A4156Will);

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3028Emboar).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Fire,
            ])],
            vec![played_card_with_base_hp(CardId::B3023MegaCameruptEx, 250)],
        );
        state.hands[0] = vec![Card::Trainer(will.clone())];
        game.set_state(state);

        play_trainer(&mut game, 0, will.clone());
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let remaining_hp = state.get_active(1).get_remaining_hp();
        assert!(
            [150, 120, 90].contains(&remaining_hp),
            "Seed {seed}: Will should force at least one heads while Flare Storm flips once per Fire Energy"
        );
    }
}

#[test]
fn test_emboar_flare_storm_flips_only_for_fire_energy() {
    let will = trainer_from_id(CardId::A4156Will);

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B3028Emboar).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])],
            vec![played_card_with_base_hp(CardId::B3023MegaCameruptEx, 250)],
        );
        state.hands[0] = vec![Card::Trainer(will.clone())];
        game.set_state(state);

        play_trainer(&mut game, 0, will.clone());
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            150,
            "Seed {seed}: Flare Storm should ignore non-Fire attached Energy when counting coins"
        );
    }
}

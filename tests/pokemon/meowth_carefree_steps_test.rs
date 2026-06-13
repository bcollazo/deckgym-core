use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game_with_board,
};

#[test]
fn test_carefree_steps_meowth_still_poisoned_when_damage_prevented() {
    // Venomoth's Poison Powder deals 30 damage and Poisons the opponent's
    // Active Pokemon. Meowth's Carefree Steps may prevent the 30 damage on a
    // coin flip, but the Poison status should be applied either way.
    let mut prevented_count = 0;
    let mut damaged_count = 0;

    for seed in 0..100 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            1,
            vec![PlayedCard::from_id(CardId::A1017Venomoth).with_energy(vec![EnergyType::Grass])],
            vec![PlayedCard::from_id(CardId::B2124Meowth)],
        );

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        let meowth = state.get_active(1);
        assert!(
            meowth.is_poisoned(),
            "Seed {seed}: Meowth should be Poisoned by Poison Powder regardless of Carefree Steps"
        );

        match meowth.get_remaining_hp() {
            50 => prevented_count += 1,
            20 => damaged_count += 1,
            hp => panic!("Seed {seed}: unexpected Meowth HP after attack: {hp}"),
        }
    }

    assert!(
        prevented_count > 0,
        "Expected some seeds where Carefree Steps prevents the damage"
    );
    assert!(
        damaged_count > 0,
        "Expected some seeds where Carefree Steps doesn't prevent the damage"
    );
}

#[test]
fn test_palkia_ex_dimensional_storm_each_benched_meowth_flips_independently() {
    // Palkia ex's Dimensional Storm does 150 damage to the opponent's Active
    // Pokemon and 20 damage to each of their Benched Pokemon. Each Benched
    // Meowth's Carefree Steps coin flip should be independent of the other.
    let mut both_prevented = 0;
    let mut only_first_prevented = 0;
    let mut only_second_prevented = 0;
    let mut neither_prevented = 0;

    for seed in 0..200 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            1,
            vec![PlayedCard::from_id(CardId::A2049PalkiaEx).with_energy(vec![
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Water,
                EnergyType::Colorless,
            ])],
            vec![
                PlayedCard::from_id(CardId::A1004VenusaurEx),
                PlayedCard::from_id(CardId::B2124Meowth),
                PlayedCard::from_id(CardId::B2124Meowth),
            ],
        );

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(1),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        assert_eq!(
            state.get_remaining_hp(1, 0),
            40,
            "Seed {seed}: opponent's Active should take the full 150 damage from Dimensional Storm"
        );

        let bench_1_hp = state.get_remaining_hp(1, 1);
        let bench_2_hp = state.get_remaining_hp(1, 2);

        match (bench_1_hp, bench_2_hp) {
            (50, 50) => both_prevented += 1,
            (50, 30) => only_first_prevented += 1,
            (30, 50) => only_second_prevented += 1,
            (30, 30) => neither_prevented += 1,
            (hp1, hp2) => panic!("Seed {seed}: unexpected Benched Meowth HPs: {hp1}, {hp2}"),
        }
    }

    assert!(
        both_prevented > 0,
        "Expected some seeds where both Meowths prevent their damage"
    );
    assert!(
        only_first_prevented > 0,
        "Expected some seeds where only the first Meowth prevents its damage"
    );
    assert!(
        only_second_prevented > 0,
        "Expected some seeds where only the second Meowth prevents its damage"
    );
    assert!(
        neither_prevented > 0,
        "Expected some seeds where neither Meowth prevents its damage"
    );
}

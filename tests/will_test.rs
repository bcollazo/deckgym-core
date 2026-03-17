use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerCard},
};

mod common;

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
fn test_will_forces_heads_for_mesagoza_coin_flip() {
    let will = trainer_from_id(CardId::A4156Will);
    let searched_pokemon = get_card_by_enum(CardId::A1033Charmander);

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        state.active_stadium = Some(get_card_by_enum(CardId::B2a093Mesagoza));
        state.decks[0].cards = vec![searched_pokemon.clone()];
        state.hands[0] = vec![Card::Trainer(will.clone())];
        game.set_state(state);

        play_trainer(&mut game, 0, will.clone());
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::UseStadium,
            is_stack: false,
        });

        let state = game.get_state_clone();
        assert!(
            state.hands[0].contains(&searched_pokemon),
            "Seed {seed}: Mesagoza should hit heads after Will"
        );
        assert!(
            state.decks[0].cards.is_empty(),
            "Seed {seed}: Mesagoza should have moved the only Pokemon from deck to hand"
        );
    }
}

#[test]
fn test_will_forces_heads_for_lucky_ice_pop_coin_flip() {
    let will = trainer_from_id(CardId::A4156Will);
    let lucky_ice_pop = trainer_from_id(CardId::B2145LuckyIcePop);
    let lucky_ice_pop_card = Card::Trainer(lucky_ice_pop.clone());

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20)],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        state.hands[0] = vec![
            Card::Trainer(will.clone()),
            Card::Trainer(lucky_ice_pop.clone()),
        ];
        game.set_state(state);

        play_trainer(&mut game, 0, will.clone());
        play_trainer(&mut game, 0, lucky_ice_pop.clone());

        let state = game.get_state_clone();
        let active = state.get_active(0);
        assert_eq!(
            active.get_remaining_hp(),
            70,
            "Seed {seed}: Lucky Ice Pop should still heal 20"
        );
        assert!(
            state.hands[0].contains(&lucky_ice_pop_card),
            "Seed {seed}: Lucky Ice Pop should return to hand after Will"
        );
        assert!(
            !state.discard_piles[0].contains(&lucky_ice_pop_card),
            "Seed {seed}: Lucky Ice Pop should not remain in discard after Will"
        );
    }
}

#[test]
fn test_will_forces_heads_for_single_coin_damage_attack() {
    let will = trainer_from_id(CardId::A4156Will);

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1022Exeggutor).with_energy(vec![EnergyType::Grass])],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
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
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            10,
            "Seed {seed}: Exeggutor Stomp should always get +30 after Will"
        );
    }
}

#[test]
fn test_will_only_forces_first_coin_on_multi_coin_attack() {
    let will = trainer_from_id(CardId::A4156Will);
    let mut saw_one_heads = false;
    let mut saw_two_heads = false;

    for seed in 0..200 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A2098Sneasel).with_energy(vec![EnergyType::Darkness])],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
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

        let hp = game.get_state_clone().get_active(1).get_remaining_hp();
        assert_ne!(
            hp, 50,
            "Seed {seed}: First coin should never be tails after Will"
        );
        if hp == 30 {
            saw_one_heads = true;
        }
        if hp == 10 {
            saw_two_heads = true;
        }
    }

    assert!(
        saw_one_heads,
        "Expected at least one outcome with exactly one heads (second coin tails)"
    );
    assert!(
        saw_two_heads,
        "Expected at least one outcome with two heads (second coin heads)"
    );
}

#[test]
fn test_will_forces_heads_for_coin_flip_effect_attack() {
    let will = trainer_from_id(CardId::A4156Will);

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1140Dugtrio).with_energy(vec![EnergyType::Fighting])],
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur)
                    .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            ],
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

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::EndTurn,
            is_stack: false,
        });
        game.play_until_stable();

        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(0).get_remaining_hp(),
            70,
            "Seed {seed}: Dugtrio should always block the next attack after Will"
        );
    }
}

#[test]
fn test_will_does_not_force_enemy_coin_flips_meowth() {
    let will = trainer_from_id(CardId::A4156Will);
    let mut prevented_count = 0;
    let mut damaged_count = 0;

    for seed in 0..300 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur)
                    .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            ],
            vec![PlayedCard::from_id(CardId::B2124Meowth)],
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

        let hp = game.get_state_clone().get_active(1).get_remaining_hp();
        if hp == 50 {
            prevented_count += 1;
        } else if hp == 10 {
            damaged_count += 1;
        } else {
            panic!("Unexpected Meowth HP after attack: {hp}");
        }
    }

    assert!(
        prevented_count > 0,
        "Expected some seeds where enemy Meowth flips heads and prevents damage"
    );
    assert!(
        damaged_count > 0,
        "Expected some seeds where enemy Meowth flips tails and takes damage"
    );
}

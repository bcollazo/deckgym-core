use common::get_initialized_game;
use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition, TrainerCard},
};

mod common;

fn make_will_trainer_card() -> TrainerCard {
    match get_card_by_enum(CardId::A4156Will) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn make_electric_generator_card() -> TrainerCard {
    match get_card_by_enum(CardId::B2a086ElectricGenerator) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

fn make_lucky_ice_pop_card() -> TrainerCard {
    match get_card_by_enum(CardId::B2145LuckyIcePop) {
        Card::Trainer(tc) => tc,
        _ => panic!("Expected trainer card"),
    }
}

/// Test that Will guarantees first coin flip is heads for single coin flip attacks.
/// Skiddo's "Surprise Attack" does 40 damage but does nothing on tails.
/// With Will active, it should ALWAYS do 40 damage (first flip guaranteed heads).
#[test]
fn test_will_single_coin_flip_attack() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Skiddo (with Surprise Attack - CoinFlipNoEffect) vs Squirtle
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1031Skiddo).with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );

        // Add Will to hand and play it
        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3; // Ensure not first turn
        game.set_state(state);

        // Play Will
        let will_action = Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        };
        game.apply_action(&will_action);

        // Now attack with Surprise Attack (index 0)
        let attack_action = Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        };
        game.apply_action(&attack_action);

        let state = game.get_state_clone();
        let opponent_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);

        // Squirtle has 60 HP, Surprise Attack does 40 damage
        // With Will, first coin flip is always heads, so attack should ALWAYS succeed
        assert_eq!(
            opponent_hp, 20,
            "Seed {seed}: Surprise Attack should always succeed with Will (60 - 40 = 20 HP)"
        );
    }
}

/// Test that Will is consumed after the first coin flip sequence.
/// After using Will and attacking, subsequent attacks should have normal 50/50 odds.
#[test]
fn test_will_consumed_after_use() {
    let mut heads_count = 0;
    let mut tails_count = 0;

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Skiddo vs Squirtle, with two Skiddos so we can attack twice
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1031Skiddo).with_energy(vec![EnergyType::Colorless])],
            vec![
                PlayedCard::from_id(CardId::A1053Squirtle),
                PlayedCard::from_id(CardId::A1053Squirtle), // Second Squirtle on bench
            ],
        );

        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will and attack (this consumes Will)
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        // The first attack should have succeeded (Will active)
        let state = game.get_state_clone();
        let first_opponent_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);
        assert_eq!(
            first_opponent_hp, 20,
            "Seed {seed}: First attack with Will should always succeed"
        );

        // Skip opponent's turn - manually set up second attack scenario
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 5; // Another turn

        // Heal opponent and give Skiddo energy again
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1031Skiddo).with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );
        game.set_state(state);

        // Second attack (Will should be consumed, so 50/50)
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let second_opponent_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);

        // Now it should be probabilistic
        if second_opponent_hp == 20 {
            heads_count += 1;
        } else if second_opponent_hp == 60 {
            tails_count += 1;
        }
    }

    // Without Will, we should see both outcomes
    assert!(
        heads_count > 0 && tails_count > 0,
        "Second attack (without Will) should have both outcomes: heads={heads_count}, tails={tails_count}"
    );
}

/// Test that Will effect expires at end of turn (even without a coin flip).
/// Will only applies "on this turn", so if you don't flip a coin during that turn,
/// the effect is lost.
#[test]
fn test_will_expires_at_end_of_turn() {
    let mut heads_count = 0;
    let mut tails_count = 0;

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Skiddo (coin flip attack)
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1031Skiddo).with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );

        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will (but don't attack this turn)
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Advance to next turn without using coin flip - Will should expire
        let mut state = game.get_state_clone();
        state.turn_count = 5; // Next turn (Will was on turn 3)
        state.current_player = 0;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1031Skiddo).with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );
        game.set_state(state);

        // Attack with Skiddo - Will should have expired
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let opponent_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);

        // Will expired - attack should be probabilistic
        if opponent_hp == 20 {
            heads_count += 1;
        } else if opponent_hp == 60 {
            tails_count += 1;
        }
    }

    // Without Will (expired), we should see both outcomes
    assert!(
        heads_count > 0 && tails_count > 0,
        "After Will expires, attack should be probabilistic: heads={heads_count}, tails={tails_count}"
    );
}

/// Test Will with Lucky Ice Pop (trainer coin flip)
/// Lucky Ice Pop: Heal 20 damage. Flip a coin - heads = return to hand, tails = stay in discard
/// With Will, coin flip should always be heads (card returns to hand).
#[test]
fn test_will_with_lucky_ice_pop() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: damaged active Pokemon
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30)],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );

        let will_card = make_will_trainer_card();
        let ice_pop_card = make_lucky_ice_pop_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.hands[0].push(Card::Trainer(ice_pop_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will first
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Then play Lucky Ice Pop
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: ice_pop_card.clone(),
            },
            is_stack: false,
        });

        let state = game.get_state_clone();
        let ice_pop_in_hand = state.hands[0].contains(&Card::Trainer(ice_pop_card.clone()));
        let ice_pop_in_discard = state.discard_piles[0].contains(&Card::Trainer(ice_pop_card));

        // With Will, coin should always be heads - card returns to hand
        assert!(
            ice_pop_in_hand && !ice_pop_in_discard,
            "Seed {seed}: With Will, Lucky Ice Pop should always return to hand (heads)"
        );
    }
}

/// Test Will with Electric Generator (trainer coin flip)
/// Electric Generator: Flip a coin - heads = attach Lightning energy to benched Lightning Pokemon
/// With Will, coin flip should always be heads.
#[test]
fn test_will_with_electric_generator() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Active Pokemon + Lightning benched Pokemon
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A1001Bulbasaur),
                PlayedCard::from_id(CardId::A1094Pikachu), // Lightning type on bench
            ],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );

        let will_card = make_will_trainer_card();
        let generator_card = make_electric_generator_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.hands[0].push(Card::Trainer(generator_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Check initial energy on Pikachu (bench slot 1)
        {
            let state = game.get_state_clone();
            let initial_energy_count = state.in_play_pokemon[0][1]
                .as_ref()
                .map(|p| p.attached_energy.len())
                .unwrap_or(0);
            assert_eq!(
                initial_energy_count, 0,
                "Pikachu should start with no energy"
            );
        }

        // Play Will first
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Then play Electric Generator
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: generator_card,
            },
            is_stack: false,
        });

        // Play until stable (to resolve any stacked actions like choosing where to attach)
        game.play_until_stable();

        let state = game.get_state_clone();
        let pikachu_energy = state.in_play_pokemon[0][1]
            .as_ref()
            .map(|p| p.attached_energy.len())
            .unwrap_or(0);

        // With Will, Electric Generator should always flip heads and attach energy
        assert_eq!(
            pikachu_energy, 1,
            "Seed {seed}: With Will, Electric Generator should always succeed (attach energy)"
        );
    }
}

/// Test Will with confusion - confused attacker's coin flip should always be heads
/// When confused, you flip a coin - heads = attack works, tails = attack fails
/// With Will, the confusion flip should always be heads (attack succeeds).
#[test]
fn test_will_with_confusion() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Confused Charizard attacking
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1035Charizard)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire, EnergyType::Fire])
                .with_status(StatusCondition::Confused)],
            vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        );

        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Attack while confused - with Will, confusion flip should be heads
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0), // Fire Blast
            is_stack: false,
        });

        let state = game.get_state_clone();

        // Fire Blast does 150 damage - Squirtle (60 HP) should be knocked out
        // With Will, confusion flip is heads, so attack succeeds
        let squirtle_ko = state.maybe_get_active(1).is_none()
            || state
                .maybe_get_active(1)
                .map(|p| p.get_remaining_hp() < 60)
                .unwrap_or(true);

        assert!(
            squirtle_ko,
            "Seed {seed}: With Will active, confusion flip should be heads (attack succeeds)"
        );
    }
}

/// Test that Will does NOT affect opponent's coin flips.
/// When opponent has a coin flip ability or card, Will should not help the player.
/// This uses Meowth's "Pay Day" ability (coin flip to prevent damage).
#[test]
fn test_will_does_not_affect_opponent_coin_flip() {
    // Meowth has CoinFlipToPreventDamage ability - defender flips, not attacker
    // Player's Will should NOT make opponent's defensive flip always heads

    let mut damage_prevented_count = 0;
    let mut damage_dealt_count = 0;

    for seed in 0..100 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Player attacks Meowth (which has coin flip to prevent damage)
        state.set_board(
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass])],
            vec![PlayedCard::from_id(CardId::B2124Meowth)], // Meowth with CoinFlipToPreventDamage ability
        );

        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Attack Meowth - Meowth's coin flip is the OPPONENT's flip, not ours
        let initial_meowth_hp = game.get_state_clone().get_active(1).get_remaining_hp();

        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let meowth_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);

        if meowth_hp < initial_meowth_hp {
            damage_dealt_count += 1;
        } else {
            damage_prevented_count += 1;
        }
    }

    // Will should NOT affect opponent's coin flip
    // We should see both outcomes (damage dealt and damage prevented)
    assert!(
        damage_dealt_count > 0 && damage_prevented_count > 0,
        "Opponent's coin flip should be unaffected by Will: dealt={damage_dealt_count}, prevented={damage_prevented_count}"
    );
}

/// Test Will with multi-coin attacks (e.g., Kangaskhan's Dizzy Punch which flips 2 coins)
/// Will only guarantees the FIRST coin is heads.
/// For a 2-coin attack: outcomes should be 50% (H,H) and 50% (H,T), never (T,_)
#[test]
fn test_will_multi_coin_attack() {
    // Using Kangaskhan's Dizzy Punch: flip 2 coins, 30 damage per heads
    // Outcomes: 0 heads = 0 damage, 1 head = 30 damage, 2 heads = 60 damage
    // Without Will: 25% 0, 50% 30, 25% 60
    // With Will (first flip heads): 50% 1 head (30), 50% 2 heads (60), 0% 0 heads

    let mut zero_heads = 0;
    let mut one_head = 0;
    let mut two_heads = 0;

    for seed in 0..200 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;

        // Setup: Kangaskhan (Dizzy Punch) vs Squirtle
        // Kangaskhan: A1203, Dizzy Punch is attack index 0, requires [C][C][C]
        state.set_board(
            vec![
                PlayedCard::from_id(CardId::A1203Kangaskhan).with_energy(vec![
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)], // 70 HP
        );

        let will_card = make_will_trainer_card();
        state.hands[0].push(Card::Trainer(will_card.clone()));
        state.turn_count = 3;
        game.set_state(state);

        // Play Will
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Play {
                trainer_card: will_card,
            },
            is_stack: false,
        });

        // Attack with Dizzy Punch
        let initial_hp = 70;
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0), // Dizzy Punch
            is_stack: false,
        });

        let state = game.get_state_clone();
        let remaining_hp = state
            .maybe_get_active(1)
            .map(|p| p.get_remaining_hp())
            .unwrap_or(0);
        let damage = initial_hp - remaining_hp;

        match damage {
            0 => zero_heads += 1,
            30 => one_head += 1,
            60 => two_heads += 1,
            _ => panic!("Unexpected damage: {damage}"),
        }
    }

    // With Will active, first coin is always heads
    // So we should never get 0 heads (0 damage)
    assert_eq!(
        zero_heads, 0,
        "With Will, should never get 0 heads (first flip is guaranteed)"
    );
    // We should see both 1 head and 2 heads outcomes
    assert!(
        one_head > 0 && two_heads > 0,
        "Should see both 1 head and 2 heads outcomes: one={one_head}, two={two_heads}"
    );
}

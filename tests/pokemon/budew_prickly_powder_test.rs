use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Cloyster's Shell Armor ("This Pokémon takes -10 damage from attacks.") normally softens an
/// incoming hit; this is the control for the Prickly Powder test below.
#[test]
fn test_cloysters_shell_armor_reduces_damage_without_prickly_powder() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![PlayedCard::from_id(CardId::A1067Cloyster)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(50, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    // 120 HP - (50 - 10) = 80
    assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 80);
}

/// Budew - Prickly Powder: "The Defending Pokémon loses all Abilities." Shell Armor stops working.
#[test]
fn test_budew_prickly_powder_strips_the_defenders_passive_ability() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::B3013Budew),
            PlayedCard::from_id(CardId::A1211Snorlax).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1067Cloyster)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3013Budew, 0),
        is_stack: false,
    });
    // Prickly Powder's own 10 damage is still soaked by Shell Armor (the effect only lands after
    // that hit resolves); what matters is the effect it leaves behind.
    let hp_before = game.get_state_clone().get_active(1).get_remaining_hp();
    assert_eq!(hp_before, 120);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(50, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    // The full 50 lands: Shell Armor is gone.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        hp_before - 50
    );
}

/// The effect lasts only while the Defending Pokémon stays Active: switching it out restores its
/// Ability.
#[test]
fn test_budew_prickly_powder_wears_off_when_the_defender_leaves_the_active_spot() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3013Budew)],
        vec![
            PlayedCard::from_id(CardId::A1067Cloyster),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3013Budew, 0),
        is_stack: false,
    });

    // The opponent swaps Cloyster to the Bench and back, which clears effects on it.
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    for _ in 0..2 {
        game.apply_action(&Action {
            actor: 1,
            action: SimpleAction::Activate {
                player: 1,
                in_play_idx: 1,
            },
            is_stack: false,
        });
        let mut state = game.get_state_clone();
        state.current_player = 1;
        game.set_state(state);
    }

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Cloyster");
    let hp_before = state.get_active(1).get_remaining_hp();

    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::ApplyDamage {
            attacking_ref: (0, 0),
            targets: vec![(50, 1, 0)],
            is_from_active_attack: true,
        },
        is_stack: false,
    });

    // Shell Armor is back: only 40 gets through.
    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        hp_before - 40
    );
}

/// A Pokémon that has lost its Abilities also stops offering activated ones.
#[test]
fn test_budew_prickly_powder_removes_activated_abilities_from_the_defender() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3013Budew)],
        vec![
            PlayedCard::from_id(CardId::A1007Butterfree),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(30),
        ],
    );

    // Control: Butterfree's Powder Heal is offered on the opponent's turn.
    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 0 })),
        "Butterfree should normally offer its ability"
    );

    // Now strip it with Prickly Powder.
    let mut state = game.get_state_clone();
    state.current_player = 0;
    game.set_state(state);
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B3013Budew, 0),
        is_stack: false,
    });

    let mut state = game.get_state_clone();
    state.current_player = 1;
    game.set_state(state);
    let (_, choices) = game.get_state_clone().generate_possible_actions();
    assert!(
        !choices
            .iter()
            .any(|choice| matches!(choice.action, SimpleAction::UseAbility { in_play_idx: 0 })),
        "Prickly Powder should take Butterfree's ability away"
    );
}

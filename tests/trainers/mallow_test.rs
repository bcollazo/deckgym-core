use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn mallow_card() -> deckgym::models::TrainerCard {
    match get_card_by_enum(CardId::A3154Mallow) {
        Card::Trainer(trainer_card) => trainer_card,
        _ => panic!("Mallow should be a Trainer card"),
    }
}

/// Mallow: "Heal all damage from 1 of your Shiinotic or Tsareena. If you do, discard all Energy
/// from that Pokémon."
#[test]
fn test_mallow_fully_heals_tsareena_and_discards_all_its_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A3020Tsareena)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass])
                .with_damage(90),
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_energy(vec![EnergyType::Grass])
                .with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.hands[0].push(Card::Trainer(mallow_card()));
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: mallow_card(),
        },
        is_stack: false,
    });

    // Only Tsareena is a valid target; Bulbasaur is damaged but is neither Shiinotic nor Tsareena.
    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(actions.len(), 1);
    let heal_action = actions[0].clone();
    assert!(matches!(
        heal_action.action,
        SimpleAction::HealAndDiscardEnergy { in_play_idx: 0, .. }
    ));
    game.apply_action(&heal_action);

    let state = game.get_state_clone();
    let tsareena = state.get_active(0);
    assert_eq!(tsareena.get_remaining_hp(), 130);
    assert!(tsareena.attached_energy.is_empty());
    assert_eq!(
        state.discard_energies[0],
        vec![EnergyType::Grass, EnergyType::Grass]
    );

    // Bulbasaur is untouched.
    assert_eq!(state.get_remaining_hp(0, 1), 70 - 20);
}

/// Without a damaged Shiinotic or Tsareena, Mallow can't be played at all.
#[test]
fn test_mallow_is_not_playable_without_a_damaged_target() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.set_board(
        // An undamaged Tsareena would heal nothing, so playing Mallow would be a no-op.
        vec![
            PlayedCard::from_id(CardId::A3020Tsareena)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_damage(20),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );
    state.hands[0].push(Card::Trainer(mallow_card()));
    game.set_state(state);

    let (_, actions) = game.get_state_clone().generate_possible_actions();
    assert!(!actions.iter().any(|action| matches!(
        &action.action,
        SimpleAction::Play { trainer_card } if trainer_card.name == "Mallow"
    )));
}

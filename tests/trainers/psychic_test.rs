use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

/// Psychic (Supporter): "You can use this card only if your Pokémon in the Active Spot has the
/// Psychic attack. Choose 1 of your opponent's Benched Pokémon and move a random Energy from it
/// to your opponent's Active Pokémon."
#[test]
fn test_psychic_moves_energy_from_opponent_bench_to_opponent_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Alakazam has the Psychic attack, so the Supporter is usable.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1117Alakazam)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander)
                .with_energy(vec![EnergyType::Fire, EnergyType::Fire]),
        ],
    );

    let psychic = make_trainer_card(CardId::B4150Psychic);
    state.hands[0] = vec![Card::Trainer(psychic.clone())];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: psychic,
        },
        is_stack: false,
    });

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0, "The player who played Psychic makes the choice");
    assert_eq!(
        choices.len(),
        1,
        "Only the benched Charmander has Energy to move"
    );
    assert!(matches!(
        choices[0].action,
        SimpleAction::MoveRandomOpponentEnergyToActive {
            from_in_play_idx: 1
        }
    ));
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).attached_energy, vec![EnergyType::Fire]);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .expect("Charmander should still be benched")
            .attached_energy,
        vec![EnergyType::Fire]
    );
}

#[test]
fn test_psychic_cannot_be_played_without_psychic_attack_active() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Bulbasaur's only attack is Vine Whip, so Psychic can't be used.
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander).with_energy(vec![EnergyType::Fire]),
        ],
    );

    let psychic = make_trainer_card(CardId::B4150Psychic);
    state.hands[0] = vec![Card::Trainer(psychic)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(
            |choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B4 150")
        ),
        "Psychic shouldn't be playable unless the Active Pokemon has the Psychic attack"
    );
}

#[test]
fn test_psychic_cannot_be_played_when_opponent_bench_has_no_energy() {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1117Alakazam)],
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
    );

    let psychic = make_trainer_card(CardId::B4150Psychic);
    state.hands[0] = vec![Card::Trainer(psychic)];
    game.set_state(state);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    assert!(
        !choices.iter().any(
            |choice| matches!(&choice.action, SimpleAction::Play { trainer_card } if trainer_card.id == "B4 150")
        ),
        "Psychic shouldn't be playable when no Benched opponent Pokemon has Energy"
    );
}

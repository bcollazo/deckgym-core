use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerType},
    state::GameOutcome,
    test_support::get_initialized_game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

// --- Korrina Tests ---

#[test]
fn test_korrina_boosts_fighting_damage_against_ex() {
    // Korrina: "During this turn, attacks used by your [F] Pokémon do +30 damage to your
    // opponent's Active Pokémon ex."
    // Hitmonlee Kick = 30 base. With Korrina it should do 60 → KO a 50 HP Mewtwo ex.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2b040Hitmonlee).with_energy(vec![EnergyType::Fighting])],
        vec![PlayedCard::from_id(CardId::A1129MewtwoEx).with_remaining_hp(50)],
    );

    let korrina = make_trainer_card(CardId::B3149Korrina);
    state.hands[0] = vec![Card::Trainer(korrina.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: korrina,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Attack with Hitmonlee (Kick - index 0)
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attack = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Attack(0)))
        .expect("Hitmonlee should be able to use Kick");
    game.apply_action(attack);

    // 30 base + 30 Korrina = 60 damage on a 50 HP Mewtwo ex → KO
    assert_eq!(
        game.get_state_clone().winner,
        Some(GameOutcome::Win(0)),
        "Player 0 should win: Korrina boosts Hitmonlee's Kick to KO Mewtwo ex"
    );
}

#[test]
fn test_korrina_does_not_boost_against_non_ex() {
    // Korrina only boosts against ex. Against a non-ex opponent, damage should be base 30.
    // Hitmonlee Kick = 30. A 40 HP non-ex opponent should survive.
    let mut game = get_initialized_game(1);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A2b040Hitmonlee).with_energy(vec![EnergyType::Fighting])],
        // Bulbasaur has 70 HP; use 40 remaining so it survives 30 but dies to 60
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(40)],
    );

    let korrina = make_trainer_card(CardId::B3149Korrina);
    state.hands[0] = vec![Card::Trainer(korrina.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: korrina,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let attack = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Attack(0)))
        .expect("Hitmonlee should be able to use Kick");
    game.apply_action(attack);

    // Bulbasaur should still be alive (30 damage, 40 HP → 10 remaining)
    let state = game.get_state_clone();
    assert!(
        state.winner.is_none(),
        "Bulbasaur should survive: Korrina does not boost damage against non-ex"
    );
}

// --- Cabbie Tests ---

#[test]
fn test_cabbie_fetches_stadium_from_deck() {
    // Cabbie: "Put a random Stadium card from your deck into your hand."
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let cabbie = make_trainer_card(CardId::B3150Cabbie);
    let peculiar_plaza = get_card_by_enum(CardId::B2155PeculiarPlaza);
    state.hands[0] = vec![Card::Trainer(cabbie.clone())];
    state.decks[0].cards = vec![peculiar_plaza.clone()];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: cabbie,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let has_stadium_in_hand = state.hands[0].iter().any(
        |card| matches!(card, Card::Trainer(tc) if tc.trainer_card_type == TrainerType::Stadium),
    );
    assert!(
        has_stadium_in_hand,
        "Peculiar Plaza (stadium) should be in hand after playing Cabbie"
    );
}

#[test]
fn test_cabbie_no_stadium_in_deck_does_nothing() {
    // If no Stadium is in the deck Cabbie should not crash (just shuffle).
    let mut game = get_initialized_game(1);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let cabbie = make_trainer_card(CardId::B3150Cabbie);
    // Only non-stadium cards in deck
    state.hands[0] = vec![Card::Trainer(cabbie.clone())];
    state.decks[0].cards = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: cabbie,
        },
        is_stack: false,
    };
    // Should not panic
    game.apply_action(&play_action);

    let state = game.get_state_clone();
    let has_stadium_in_hand = state.hands[0].iter().any(
        |card| matches!(card, Card::Trainer(tc) if tc.trainer_card_type == TrainerType::Stadium),
    );
    assert!(
        !has_stadium_in_hand,
        "No stadium should be in hand when deck had none"
    );
}

// --- Parasol Lady Tests ---

#[test]
fn test_parasol_lady_returns_water_non_ex_to_hand() {
    // Parasol Lady: "Put 1 of your [W] Pokémon in play, except any Pokémon ex, into your hand."
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Squirtle (Water, non-ex) on bench; Bulbasaur as active (not water)
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1053Squirtle),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let parasol_lady = make_trainer_card(CardId::B3152ParasolLady);
    state.hands[0] = vec![Card::Trainer(parasol_lady.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: parasol_lady,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Player must choose which Water non-ex pokemon to return
    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    let return_action = choices
        .iter()
        .find(|a| matches!(a.action, SimpleAction::ReturnPokemonToHand { .. }))
        .expect("Should have a ReturnPokemonToHand choice for Squirtle");
    game.apply_action(return_action);

    let state = game.get_state_clone();
    let squirtle_card = get_card_by_enum(CardId::A1053Squirtle);
    assert!(
        state.hands[0].contains(&squirtle_card),
        "Squirtle should be in hand after Parasol Lady"
    );
}

#[test]
fn test_parasol_lady_cannot_target_ex_pokemon() {
    // Parasol Lady should not allow targeting Water ex pokemon.
    // StarmieEx is Water but ex — it should not be a valid target.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1076StarmieEx),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let parasol_lady = make_trainer_card(CardId::B3152ParasolLady);
    state.hands[0] = vec![Card::Trainer(parasol_lady.clone())];
    game.set_state(state);

    // Parasol Lady should not be playable when the only Water pokemon in play is ex
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let can_play = actions.iter().any(|a| {
        matches!(&a.action, SimpleAction::Play { trainer_card } if trainer_card.name == "Parasol Lady")
    });
    assert!(
        !can_play,
        "Parasol Lady should not be playable when the only Water pokemon in play is ex"
    );
}

#[test]
fn test_parasol_lady_last_pokemon_loses_game() {
    // If the only pokemon the player has in play is a Water non-ex and they Parasol Lady it
    // back to hand, they should lose immediately (no pokemon in play).
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Squirtle is the only pokemon in play for player 0
    state.set_board(
        vec![PlayedCard::from_id(CardId::A1053Squirtle)],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let parasol_lady = make_trainer_card(CardId::B3152ParasolLady);
    state.hands[0] = vec![Card::Trainer(parasol_lady.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: parasol_lady,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // The only choice is to return Squirtle (the active)
    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    assert!(!choices.is_empty());
    game.apply_action(&choices[0]);

    assert_eq!(
        game.get_state_clone().winner,
        Some(GameOutcome::Win(1)),
        "Player 1 should win when player 0 Parasol Ladies away their last pokemon"
    );
}

#[test]
fn test_parasol_lady_active_triggers_promotion() {
    // If Parasol Lady returns the active pokemon and a bench pokemon is available,
    // the player must immediately promote a benched pokemon to the active spot.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    // Squirtle is the active (Water, non-ex); Charmander sits on bench
    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
    );

    let parasol_lady = make_trainer_card(CardId::B3152ParasolLady);
    state.hands[0] = vec![Card::Trainer(parasol_lady.clone())];
    game.set_state(state);

    let play_action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: parasol_lady,
        },
        is_stack: false,
    };
    game.apply_action(&play_action);

    // Choose to return Squirtle (the active, in_play_idx 0)
    let (_actor, choices) = game.get_state_clone().generate_possible_actions();
    let return_squirtle = choices
        .iter()
        .find(|a| {
            matches!(
                a.action,
                SimpleAction::ReturnPokemonToHand { in_play_idx: 0 }
            )
        })
        .expect("Should be able to return the active Squirtle");
    game.apply_action(return_squirtle);

    // Next action should be player 0 promoting one of their bench pokemon
    let (actor, promotion_choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0, "Player 0 must act next (promote)");
    assert!(
        promotion_choices
            .iter()
            .all(|a| matches!(a.action, SimpleAction::Activate { player: 0, .. })),
        "All choices should be Activate actions for player 0 to promote Charmander"
    );
}

use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, StatusCondition, TrainerType},
    test_support::{get_test_game_with_board, play_trainer, trainer_from_id},
    Game,
};

fn give_and_play(game: &mut Game<'static>, card_id: CardId) {
    let trainer = trainer_from_id(card_id);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer.clone())];
    game.set_state(state);
    play_trainer(game, 0, trainer);
}

/// Sophocles: "During this turn, attacks used by your Alolan Golem, Vikavolt, or Togedemaru do
/// +30 damage to your opponent's Active Pokémon."
#[test]
fn test_sophocles_boosts_togedemaru_damage() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3065Vikavolt)
            .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );
    let baseline = {
        let mut plain = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A3065Vikavolt)
                .with_energy(vec![EnergyType::Lightning, EnergyType::Lightning])],
            vec![PlayedCard::from_id(CardId::A1202Chansey)],
        );
        attack_with_first(&mut plain, CardId::A3065Vikavolt);
        plain.get_state_clone().get_remaining_hp(1, 0)
    };

    give_and_play(&mut game, CardId::A3153Sophocles);
    attack_with_first(&mut game, CardId::A3065Vikavolt);

    assert_eq!(
        game.get_state_clone().get_remaining_hp(1, 0),
        baseline - 30,
        "Sophocles should add +30 to Togedemaru's attack"
    );
}

fn attack_with_first(game: &mut Game<'static>, card_id: CardId) {
    game.apply_action(&deckgym::actions::Action {
        actor: 0,
        action: deckgym::test_support::attack_action(card_id, 0),
        is_stack: false,
    });
}

/// Team Galactic Grunt: "Put 1 random Glameow, Stunky, or Croagunk from your deck into your hand."
#[test]
fn test_team_galactic_grunt_pulls_a_named_basic_from_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A2139Glameow),
        get_card_by_enum(CardId::A1001Bulbasaur),
    ];
    game.set_state(state);

    give_and_play(&mut game, CardId::A2151TeamGalacticGrunt);

    let state = game.get_state_clone();
    assert!(
        state.hands[0].iter().any(|c| c.get_name() == "Glameow"),
        "Team Galactic Grunt should put Glameow into hand"
    );
    assert_eq!(state.decks[0].cards.len(), 1);
}

/// Traveling Merchant: "Look at the top 4 cards of your deck. Put all Pokémon Tool cards you find
/// there into your hand. Shuffle the other cards back into your deck."
#[test]
fn test_traveling_merchant_takes_only_tools() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.decks[0].cards = vec![
        get_card_by_enum(CardId::A2147GiantCape),
        get_card_by_enum(CardId::A2148RockyHelmet),
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::PA001Potion),
    ];
    game.set_state(state);

    give_and_play(&mut game, CardId::A4a070TravelingMerchant);

    let state = game.get_state_clone();
    let tools_in_hand = state.hands[0]
        .iter()
        .filter(|c| matches!(c, Card::Trainer(tc) if tc.trainer_card_type == TrainerType::Tool))
        .count();
    assert_eq!(tools_in_hand, 2, "Both Tools should end up in hand");
    assert_eq!(
        state.decks[0].cards.len(),
        2,
        "The two non-Tool cards should stay in the deck"
    );
    assert!(!state.hands[0].iter().any(|c| c.get_name() == "Potion"));
}

/// Whitney: "Heal 60 damage from 1 of your Miltank, and it recovers from being Asleep, Paralyzed,
/// and Confused." Notably it does NOT cure Poisoned.
#[test]
fn test_whitney_heals_miltank_and_cures_only_the_named_conditions() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a062Miltank).with_damage(60)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Confused);
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    game.set_state(state);

    give_and_play(&mut game, CardId::A4a069Whitney);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let heal = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::HealAndCureConditions { .. }))
        .expect("Whitney should offer to heal Miltank")
        .clone();
    game.apply_action(&heal);

    let state = game.get_state_clone();
    let miltank = state.get_active(0);
    assert_eq!(
        miltank.get_remaining_hp(),
        110,
        "Whitney should heal all 60 damage off the 110 HP Miltank"
    );
    assert!(!miltank.is_confused(), "Whitney should cure Confused");
    assert!(
        miltank.is_poisoned(),
        "Whitney should NOT cure Poisoned — it only names Asleep, Paralyzed and Confused"
    );
}

/// Whitney is unplayable without a Miltank in play.
#[test]
fn test_whitney_requires_a_miltank() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let whitney = trainer_from_id(CardId::A4a069Whitney);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(whitney.clone())];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.id == whitney.id
        )),
        "Whitney should not be playable without a Miltank"
    );
}

/// Budding Expeditioner: "Put your Mew ex in the Active Spot into your hand."
#[test]
fn test_budding_expeditioner_returns_active_mew_ex_to_hand() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1a032MewEx).with_energy(vec![EnergyType::Psychic]),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );

    give_and_play(&mut game, CardId::A1a066BuddingExpeditioner);

    // Returning the Active triggers a promotion choice from the bench.
    let (actor, actions) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    let promote = actions
        .iter()
        .find(|a| matches!(a.action, SimpleAction::Activate { player: 0, .. }))
        .expect("Player should promote after Mew ex leaves the Active Spot")
        .clone();
    game.apply_action(&promote);

    let state = game.get_state_clone();
    assert!(
        state.hands[0].iter().any(|c| c.get_name() == "Mew ex"),
        "Mew ex should be back in hand"
    );
    assert_eq!(state.get_active(0).get_name(), "Bulbasaur");
    assert_eq!(
        state.points[1], 0,
        "Returning your own Pokémon should not award the opponent a point"
    );
}

/// Budding Expeditioner is unplayable unless Mew ex is the Active Pokémon.
#[test]
fn test_budding_expeditioner_requires_active_mew_ex() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1a032MewEx),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let card = trainer_from_id(CardId::A1a066BuddingExpeditioner);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(card.clone())];
    game.set_state(state);

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    assert!(
        !actions.iter().any(|a| matches!(
            &a.action,
            SimpleAction::Play { trainer_card } if trainer_card.id == card.id
        )),
        "Budding Expeditioner needs Mew ex in the Active Spot, not on the Bench"
    );
}

/// Iono (B2a reprint): "Each player shuffles the cards in their hand into their deck, then draws
/// that many cards." Same text as the already-implemented A2b Iono.
#[test]
fn test_iono_reprint_refreshes_both_hands() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let iono = trainer_from_id(CardId::B2a089Iono);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![
        Card::Trainer(iono.clone()),
        get_card_by_enum(CardId::PA001Potion),
        get_card_by_enum(CardId::PA005PokeBall),
    ];
    state.hands[1] = vec![get_card_by_enum(CardId::PA006RedCard)];
    game.set_state(state);

    play_trainer(&mut game, 0, iono);

    let state = game.get_state_clone();
    // Iono itself was discarded when played, leaving 2 cards to shuffle and redraw.
    assert_eq!(state.hands[0].len(), 2);
    assert_eq!(state.hands[1].len(), 1);
    assert_eq!(state.discard_piles[0].len(), 1);
}

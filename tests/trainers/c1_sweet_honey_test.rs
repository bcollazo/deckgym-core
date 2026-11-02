use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{Card, PlayedCard},
    test_support::{
        get_initialized_game_with_board, get_test_game_with_board, play_trainer, trainer_from_id,
    },
    Game,
};

/// Player 0: a damaged Bulbasaur (10 HP left) Active, an undamaged Charmander and a damaged
/// Squirtle (10 HP left) on the Bench, Sweet Honey in hand.
fn honey_game(seed: u64, honey: CardId) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_remaining_hp(10),
            PlayedCard::from_id(CardId::A1033Charmander),
            PlayedCard::from_id(CardId::A1053Squirtle).with_remaining_hp(10),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer_from_id(honey))];
    game.set_state(state);
    game
}

/// The choices pending right after Sweet Honey is played.
fn play_honey(game: &mut Game<'static>, honey: CardId) -> Vec<Action> {
    play_trainer(game, 0, trainer_from_id(honey));
    game.get_state_clone().generate_possible_actions().1
}

fn hp(game: &Game<'static>, in_play_idx: usize) -> u32 {
    game.get_state_clone().in_play_pokemon[0][in_play_idx]
        .as_ref()
        .unwrap()
        .get_remaining_hp()
}

/// Sweet Honey: "Choose 1 of your Pokémon, and then flip a coin until you get tails. For each
/// heads, heal 20 damage from that Pokémon." The Pokémon is chosen first; the coins come after.
#[test]
fn test_sweet_honey_heals_20_per_heads_until_tails() {
    let mut healed_amounts = std::collections::BTreeSet::new();
    for seed in 0..60u64 {
        let mut game = honey_game(seed, CardId::C1119SweetHoney);
        let choices = play_honey(&mut game, CardId::C1119SweetHoney);
        assert!(
            game.get_state_clone().hands[0].is_empty(),
            "Sweet Honey leaves the hand"
        );
        // The first choice is the Active Bulbasaur.
        game.apply_action(&choices[0]);
        let healed = hp(&game, 0) - 10;
        assert_eq!(healed % 20, 0);
        assert_eq!(hp(&game, 2), 10, "only the chosen Pokémon is healed");
        healed_amounts.insert(healed);
    }
    assert!(
        healed_amounts.contains(&0),
        "tails first should heal nothing"
    );
    assert!(healed_amounts.contains(&20));
    assert!(healed_amounts.contains(&40));
}

/// Right after Sweet Honey is played, the only pending choices are its damaged targets (the
/// Pokémon Potion could heal), the same in every game, and nothing is healed yet: no coin has
/// been flipped before the choice.
#[test]
fn test_sweet_honey_chooses_a_damaged_pokemon_before_flipping() {
    for honey in [CardId::C1119SweetHoney, CardId::C1152SweetHoney] {
        let mut first: Option<Vec<Action>> = None;
        for seed in 0..40u64 {
            let mut game = honey_game(seed, honey);
            let choices = play_honey(&mut game, honey);
            assert_eq!(
                choices.len(),
                2,
                "{honey:?}: the damaged Bulbasaur and Squirtle, not the undamaged Charmander"
            );
            assert!(choices.iter().all(|a| a.actor == 0));
            assert_eq!(
                (hp(&game, 0), hp(&game, 1), hp(&game, 2)),
                (10, 60, 10),
                "{honey:?}: nothing healed before the choice"
            );
            match &first {
                None => first = Some(choices),
                Some(f) => assert_eq!(
                    format!("{f:?}"),
                    format!("{choices:?}"),
                    "{honey:?}, seed {seed}: the choice can't depend on a coin result"
                ),
            }
        }
    }
}

/// After the choice, the heal is 0, 20, 40, ... on the chosen Pokémon only.
#[test]
fn test_sweet_honey_heals_the_chosen_benched_pokemon() {
    let mut healed_amounts = std::collections::BTreeSet::new();
    for seed in 0..60u64 {
        let mut game = honey_game(seed, CardId::C1119SweetHoney);
        let choices = play_honey(&mut game, CardId::C1119SweetHoney);
        // The second choice is the Benched Squirtle (in play index 2).
        game.apply_action(&choices[1]);
        let healed = hp(&game, 2) - 10;
        // 20 per heads, up to Squirtle's full 60 HP (a heal of 60 or more leaves it at 60).
        assert!(healed % 20 == 0 || hp(&game, 2) == 60, "healed {healed}");
        assert_eq!(hp(&game, 0), 10, "the Active isn't healed");
        healed_amounts.insert(healed);
    }
    assert!(healed_amounts.contains(&0));
    assert!(healed_amounts.contains(&20));
    assert!(healed_amounts.contains(&40));
}

/// Like Potion, Sweet Honey needs a damaged Pokémon to target.
#[test]
fn test_sweet_honey_needs_a_damaged_pokemon() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer_from_id(CardId::C1119SweetHoney))];
    game.set_state(state);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));

    // A damaged Benched Pokémon is enough to play it.
    let mut state = game.get_state_clone();
    state.in_play_pokemon[0][1] =
        Some(PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10));
    game.set_state(state);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));
}

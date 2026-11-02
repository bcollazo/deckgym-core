use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{get_initialized_game_with_board, play_trainer, trainer_from_id},
    Game,
};

/// Player 0: Squirtle Active, Bulbasaur and Staryu on the Bench, Misty in hand.
fn misty_game(seed: u64, misty: CardId) -> Game<'static> {
    let mut game = get_initialized_game_with_board(
        seed,
        0,
        3,
        vec![
            PlayedCard::from_id(CardId::A1053Squirtle),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1074Staryu),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer_from_id(misty))];
    game.set_state(state);
    game
}

/// The choices pending right after Misty is played.
fn play_misty(game: &mut Game<'static>, misty: CardId) -> Vec<Action> {
    play_trainer(game, 0, trainer_from_id(misty));
    game.get_state_clone().generate_possible_actions().1
}

fn energy(game: &Game<'static>, in_play_idx: usize) -> Vec<EnergyType> {
    game.get_state_clone().in_play_pokemon[0][in_play_idx]
        .as_ref()
        .unwrap()
        .attached_energy
        .clone()
}

/// Misty: "Choose 1 of your [W] Pokémon, and flip a coin until you get tails. For each heads,
/// take a [W] Energy from your Energy Zone and attach it to that Pokémon." The Pokémon is
/// chosen first; the coins come after. Right after Misty is played, the only pending choices
/// are the [W] Pokémon, the same in every game, and no Energy is attached yet.
#[test]
fn test_misty_chooses_a_water_pokemon_before_flipping() {
    for misty in [CardId::A1220Misty, CardId::A1267Misty] {
        let mut first: Option<Vec<Action>> = None;
        for seed in 0..40u64 {
            let mut game = misty_game(seed, misty);
            let choices = play_misty(&mut game, misty);
            assert_eq!(
                choices.len(),
                2,
                "{misty:?}: Squirtle and Staryu, not Bulbasaur"
            );
            assert!(choices.iter().all(|a| a.actor == 0));
            for idx in 0..3 {
                assert!(
                    energy(&game, idx).is_empty(),
                    "{misty:?}: nothing attached before the choice"
                );
            }
            match &first {
                None => first = Some(choices),
                Some(f) => assert_eq!(
                    format!("{f:?}"),
                    format!("{choices:?}"),
                    "{misty:?}, seed {seed}: the choice can't depend on a coin result"
                ),
            }
        }
    }
}

/// After the choice, 0, 1, 2, ... [W] Energy (at most 5) go to the chosen Pokémon only.
#[test]
fn test_misty_attaches_water_to_the_chosen_pokemon_per_heads() {
    let mut counts = std::collections::BTreeSet::new();
    for seed in 0..80u64 {
        let mut game = misty_game(seed, CardId::A1220Misty);
        let choices = play_misty(&mut game, CardId::A1220Misty);
        // The second choice is the Benched Staryu (in play index 2).
        game.apply_action(&choices[1]);
        let attached = energy(&game, 2);
        assert!(attached.iter().all(|e| *e == EnergyType::Water));
        assert!(attached.len() <= 5);
        assert!(energy(&game, 0).is_empty(), "the Active gets nothing");
        assert!(energy(&game, 1).is_empty(), "Bulbasaur gets nothing");
        assert!(
            game.get_state_clone().hands[0].is_empty(),
            "Misty leaves the hand"
        );
        counts.insert(attached.len());
    }
    assert!(counts.contains(&0), "tails first attaches nothing");
    assert!(counts.contains(&1));
    assert!(counts.contains(&2));
}

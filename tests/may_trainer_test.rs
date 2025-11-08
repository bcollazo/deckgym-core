use deckgym::actions::{may_effect, Action, SimpleAction};
use deckgym::card_ids::CardId;
use deckgym::database::get_card_by_enum;
use deckgym::models::Card;
use deckgym::{Deck, State};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn test_may_generates_all_combinations() {
    // Setup: Player has 1 Pokemon (Pikachu) in hand initially
    // Deck has 2 Pokemon (Charmander, Squirtle)
    // After May draws 2 Pokemon from deck, player will have 3 Pokemon in hand
    // Player must choose 2 Pokemon to shuffle back
    // Possible combinations: (Pikachu, Charmander), (Pikachu, Squirtle), (Charmander, Squirtle)

    let pikachu = get_card_by_enum(CardId::A1a025Pikachu);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let squirtle = get_card_by_enum(CardId::A1053Squirtle);

    // Create state
    let mut state = State::new(&Deck::default(), &Deck::default());

    // Setup player's hand with Pikachu
    state.hands[0] = vec![pikachu.clone()];

    // Setup player's deck with Charmander and Squirtle
    state.decks[0].cards = vec![charmander.clone(), squirtle.clone()];
    state.current_player = 0;

    // Call May effect
    let (probabilities, mutations) = may_effect(0, &state);

    // Should have 1 outcome (drawing {Charmander, Squirtle} - order doesn't matter)
    assert_eq!(probabilities.len(), 1);
    assert_eq!(mutations.len(), 1);
    assert_eq!(probabilities[0], 1.0); // Only one possible combination, so 100% probability

    // Execute the mutation (draw Charmander and Squirtle)
    let mut state_copy = state.clone();
    let action = Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: get_card_by_enum(CardId::B1223May).as_trainer().clone(),
        },
        is_stack: false,
    };

    // Use a dummy RNG
    let mut rng = StdRng::seed_from_u64(42);
    let mut mutations_mut = mutations;
    mutations_mut.swap_remove(0)(&mut rng, &mut state_copy, &action);

    // After mutation, player should have 3 Pokemon in hand
    let hand_pokemon: Vec<&Card> = state_copy.iter_hand_pokemon(0).collect();
    assert_eq!(hand_pokemon.len(), 3);

    // Should have move generation stacked with shuffle choices
    assert_eq!(state_copy.move_generation_stack.len(), 1);
    let (actor, choices) = &state_copy.move_generation_stack[0];
    assert_eq!(*actor, 0);

    // Should have 3 choices (all 2-combinations of 3 Pokemon)
    // C(3,2) = 3 combinations
    assert_eq!(
        choices.len(),
        3,
        "Should have 3 shuffle combinations: (Pikachu,Charmander), (Pikachu,Squirtle), (Charmander,Squirtle)"
    );

    // Verify each choice is a ShufflePokemonIntoDeck with 2 cards
    for choice in choices {
        match choice {
            SimpleAction::ShufflePokemonIntoDeck { hand_pokemon } => {
                assert_eq!(
                    hand_pokemon.len(),
                    2,
                    "Each shuffle choice should have exactly 2 Pokemon"
                );
            }
            _ => panic!("Expected ShufflePokemonIntoDeck action"),
        }
    }
}

#[test]
fn test_may_probabilities_with_multiple_pokemon() {
    // Setup: Deck has 4 Pokemon (Pikachu, Charmander, Squirtle, Bulbasaur)
    // Drawing 2 from 4 should give C(4,2) = 6 possible outcomes
    // Each outcome should have probability 1/6

    let pikachu = get_card_by_enum(CardId::A1a025Pikachu);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let squirtle = get_card_by_enum(CardId::A1053Squirtle);
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);

    // Create state
    let mut state = State::new(&Deck::default(), &Deck::default());

    // Setup player's deck with 4 Pokemon
    state.decks[0].cards = vec![
        pikachu.clone(),
        charmander.clone(),
        squirtle.clone(),
        bulbasaur.clone(),
    ];
    state.current_player = 0;

    // Call May effect
    let (probabilities, mutations) = may_effect(0, &state);

    // Should have C(4,2) = 6 outcomes
    assert_eq!(
        probabilities.len(),
        6,
        "Should have 6 combinations when drawing 2 from 4 Pokemon"
    );
    assert_eq!(mutations.len(), 6);

    // Each outcome should have equal probability of 1/6
    for prob in &probabilities {
        assert!(
            (*prob - 1.0 / 6.0).abs() < 0.0001,
            "Each outcome should have probability 1/6"
        );
    }
}

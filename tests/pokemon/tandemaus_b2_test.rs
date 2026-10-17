use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

#[test]
fn test_flock_puts_tandemaus_and_maushold_from_deck_to_bench() {
    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();

        state.current_player = 0;
        state.turn_count = 1;
        state.set_board(
            vec![PlayedCard::from_id(CardId::B2142Tandemaus)
                .with_energy(vec![EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::A1211Snorlax)],
        );
        // Deck with 3 eligible cards (2 Tandemaus + 1 Maushold) + 2 fillers.
        // The 3 eligible cards fit exactly in the 3 free bench slots.
        state.decks[0].cards = vec![
            get_card_by_enum(CardId::B2142Tandemaus).clone(),
            get_card_by_enum(CardId::B2142Tandemaus).clone(),
            get_card_by_enum(CardId::B2a082Maushold).clone(),
            get_card_by_enum(CardId::A1001Bulbasaur).clone(),
            get_card_by_enum(CardId::A1001Bulbasaur).clone(),
        ];
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::B2142Tandemaus, 0),
            is_stack: false,
        });
        game.play_until_stable();

        let state = game.get_state_clone();

        // Flock deals no damage
        assert_eq!(state.get_active(1).get_remaining_hp(), 150, "seed {seed}");

        // All 3 eligible cards moved from the deck to the bench
        assert_eq!(state.enumerate_bench_pokemon(0).count(), 3, "seed {seed}");

        // Every benched card is a Tandemaus or a Maushold
        for (_, pokemon) in state.enumerate_bench_pokemon(0) {
            let name = pokemon.get_name();
            assert!(
                name == "Tandemaus" || name == "Maushold",
                "seed {seed}: Flock may only put Tandemaus/Maushold on the bench, got {name}"
            );
        }

        // Conservation: no Tandemaus/Maushold was left behind in the deck
        let leftover_in_deck = state.decks[0]
            .cards
            .iter()
            .filter(|card| {
                let name = card.get_name();
                name == "Tandemaus" || name == "Maushold"
            })
            .count();
        assert_eq!(leftover_in_deck, 0, "seed {seed}");
    }
}

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
};

fn attack_choices(game: &deckgym::Game) -> Vec<String> {
    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    actions
        .iter()
        .filter_map(|action| match &action.action {
            SimpleAction::Attack(attack) => Some(attack.title.clone()),
            _ => None,
        })
        .collect()
}

/// Mimikyu - Try to Imitate: "Flip a coin. If heads, choose 1 of your opponent's Active
/// Pokémon's attacks and use it as this attack." On tails nothing at all happens.
#[test]
fn test_try_to_imitate_offers_defender_attacks_only_on_heads() {
    let mut saw_heads = false;
    let mut saw_tails = false;

    for seed in 0..50 {
        let mut game = get_initialized_game(seed);
        let mut state = game.get_state_clone();
        state.current_player = 0;
        state.turn_count = 3;
        state.set_board(
            vec![PlayedCard::from_id(CardId::A3b035Mimikyu)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
            vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        );
        game.set_state(state);

        game.apply_action(&Action {
            actor: 0,
            action: attack_action(CardId::A3b035Mimikyu, 0),
            is_stack: false,
        });

        let titles = attack_choices(&game);
        if titles.contains(&"Vine Whip".to_string()) {
            saw_heads = true;
            // Bulbasaur is untouched until the copied attack is actually used.
            assert_eq!(game.get_state_clone().get_active(1).get_remaining_hp(), 70);
            game.play_until_stable();
            assert_eq!(
                game.get_state_clone().get_active(1).get_remaining_hp(),
                30,
                "seed {seed}: the copied Vine Whip should deal its own 40 damage"
            );
        } else {
            saw_tails = true;
            game.play_until_stable();
            assert_eq!(
                game.get_state_clone().get_active(1).get_remaining_hp(),
                70,
                "seed {seed}: on tails Try to Imitate does nothing"
            );
        }
    }

    assert!(
        saw_heads && saw_tails,
        "both coin results should occur across 50 seeds"
    );
}

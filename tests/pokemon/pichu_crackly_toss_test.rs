use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Crackly Toss: "Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched
/// Basic Pokémon." An evolved Benched Pokémon is not a valid target.
#[test]
fn test_pichu_crackly_toss_only_offers_benched_basic_pokemon() {
    // Bench: Raichu (Stage 1, idx 1), Pikachu (Basic, idx 2).
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A4066Pichu),
            PlayedCard::from_id(CardId::A1095Raichu),
            PlayedCard::from_id(CardId::A1094Pikachu),
        ],
        vec![PlayedCard::from_id(CardId::A1202Chansey)],
    );

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4066Pichu, 0),
        is_stack: false,
    });

    let (_actor, actions) = game.get_state_clone().generate_possible_actions();
    let targets: Vec<usize> = actions
        .iter()
        .filter_map(|a| match &a.action {
            SimpleAction::Attach { attachments, .. } => Some(attachments[0].2),
            _ => None,
        })
        .collect();
    assert_eq!(
        targets,
        vec![2],
        "only the Benched Basic Pikachu is a target"
    );

    game.apply_action(&actions[0]);
    let state = game.get_state_clone();
    let pikachu = state.in_play_pokemon[0][2].as_ref().unwrap();
    assert_eq!(pikachu.attached_energy, vec![EnergyType::Lightning]);
    let raichu = state.in_play_pokemon[0][1].as_ref().unwrap();
    assert!(raichu.attached_energy.is_empty());
}

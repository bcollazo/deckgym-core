use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board},
};

/// Interaction: Hoopa ex (holding Deceptive Needle) attacks a Caterpie (B3b 001) with Quick
/// Growth for 30 damage via Shadow Bullet. Caterpie has 40 HP, so it survives at 10 remaining.
/// At the end of Hoopa ex's turn, Deceptive Needle does an additional 10 damage to Caterpie,
/// exactly enough to knock it out (30 + 10 == 40). However, Quick Growth also triggers "at the
/// end of the opponent's turn" and evolves Caterpie into Metapod before the knockout is final,
/// so Caterpie should survive as a Metapod (80 max HP) carrying the 40 damage over, i.e. left
/// with 40 remaining HP, instead of being knocked out.
#[test]
fn test_deceptive_needle_lethal_damage_does_not_prevent_quick_growth_evolution() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4103HoopaEx)
            .with_energy(vec![EnergyType::Darkness])
            .with_tool(get_card_by_enum(CardId::B4148DeceptiveNeedle))],
        vec![
            PlayedCard::from_id(CardId::B3b001Caterpie),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    let mut state = game.get_state_clone();
    // Put exactly one Metapod (evolves from Caterpie) in player 1's deck.
    state.decks[1].cards = vec![get_card_by_enum(CardId::B3b002Metapod)];
    game.set_state(state);

    // Hoopa ex attacks with Shadow Bullet (30 damage to active, 20 to a Benched Pokemon).
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4103HoopaEx, 0),
        is_stack: false,
    });
    game.play_until_stable();

    let caterpie_hp_after_attack = game.get_state_clone().get_remaining_hp(1, 0);
    assert_eq!(
        caterpie_hp_after_attack, 10,
        "Shadow Bullet should leave Caterpie (40 HP) at 10 remaining HP"
    );

    // End Hoopa ex's turn: Deceptive Needle does 10 more damage (30 + 10 == 40, lethal), but
    // Quick Growth should evolve Caterpie into Metapod before the knockout is finalized.
    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    game.play_until_stable();

    let state = game.get_state_clone();
    let active = state.get_active(1);
    assert!(
        matches!(&active.card, Card::Pokemon(p) if p.name == "Metapod"),
        "Caterpie should have evolved into Metapod via Quick Growth despite lethal damage; got {:?}",
        active.card.get_name()
    );
    assert_eq!(
        active.get_remaining_hp(),
        40,
        "Metapod (80 HP) should carry over the 40 damage already dealt to Caterpie, leaving 40 HP"
    );
    assert!(
        state.decks[1].cards.is_empty(),
        "Metapod should have been removed from the deck"
    );
}

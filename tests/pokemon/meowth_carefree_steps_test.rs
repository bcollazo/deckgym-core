use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    models::{EnergyType, PlayedCard},
    test_support::get_initialized_game_with_board,
};

/// Meowth's "Carefree Steps": "If any damage is done to this Pokémon by attacks, flip a coin. If
/// heads, prevent that damage."
///
/// The key behavior (and the bug this fixes): on a heads flip only the *damage* to Meowth is
/// prevented — the rest of the attack (here, the Poison from Grimer's Poison Gas) still happens.
/// Previously the heads branch dropped the whole attack, including its effects.
///
/// We drive the attack across many RNG seeds and assert:
/// - Meowth is ALWAYS poisoned (the secondary effect resolves regardless of the coin), and
/// - Meowth's remaining HP is sometimes full (heads, damage prevented) and sometimes reduced
///   (tails, 10 damage), proving the prevention coin actually fires.
#[test]
fn test_meowth_carefree_steps_prevents_only_damage_not_effects() {
    let mut saw_prevented = false;
    let mut saw_damaged = false;

    for seed in 0..40u64 {
        // Grimer (attacker) vs Meowth with Carefree Steps (defender).
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A1174Grimer).with_energy(vec![EnergyType::Darkness])],
            vec![PlayedCard::from_id(CardId::B2124Meowth)],
        );

        // Poison Gas: 10 damage and the opponent's Active Pokémon is now Poisoned.
        game.apply_action(&Action {
            actor: 0,
            action: SimpleAction::Attack(0),
            is_stack: false,
        });

        let state = game.get_state_clone();
        let meowth = state.get_active(1);

        // The Poison effect must always land, even on the heads (damage-prevented) branch.
        assert!(
            meowth.is_poisoned(),
            "seed {seed}: Meowth should be Poisoned even when its damage is prevented"
        );

        match meowth.get_remaining_hp() {
            50 => saw_prevented = true, // Heads: damage to Meowth prevented.
            40 => saw_damaged = true,   // Tails: 10 damage dealt.
            other => panic!("seed {seed}: unexpected Meowth HP {other}"),
        }
    }

    assert!(
        saw_prevented,
        "expected at least one seed where Carefree Steps prevented the damage"
    );
    assert!(
        saw_damaged,
        "expected at least one seed where the damage went through"
    );
}

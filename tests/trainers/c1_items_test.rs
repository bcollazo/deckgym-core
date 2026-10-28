use deckgym::{
    actions::SimpleAction,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard, StatusCondition},
    test_support::{get_test_game_with_board, play_trainer, trainer_from_id},
};

fn give_card(game: &mut deckgym::Game<'static>, card_id: CardId) {
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(trainer_from_id(card_id))];
    game.set_state(state);
}

/// Super Potion: "Heal 40 damage from 1 of your Pokémon that has any Energy attached. If you
/// healed any damage in this way, discard a random Energy from that Pokémon."
#[test]
fn test_super_potion_heals_40_and_discards_an_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_remaining_hp(20)
                .with_energy(vec![EnergyType::Grass, EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    give_card(&mut game, CardId::C1120SuperPotion);
    play_trainer(&mut game, 0, trainer_from_id(CardId::C1120SuperPotion));

    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert_eq!(actions.len(), 1, "only the Pokémon with Energy is a target");
    game.apply_action(&actions[0]);

    let state = game.get_state_clone();
    assert_eq!(state.get_active(0).get_remaining_hp(), 60);
    assert_eq!(state.get_active(0).attached_energy.len(), 1);
    assert_eq!(state.discard_energies[0].len(), 1);
    assert!(state.hands[0].is_empty());
}

/// No Energy discard when no damage was actually healed.
#[test]
fn test_super_potion_needs_a_damaged_pokemon_with_energy() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            PlayedCard::from_id(CardId::A1033Charmander).with_remaining_hp(10),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    give_card(&mut game, CardId::C1120SuperPotion);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));
}

/// Max Repel: "Switch out your opponent's Active Stage 2 Pokémon to the Bench."
#[test]
fn test_max_repel_switches_out_a_stage_2_active() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
        vec![
            PlayedCard::from_id(CardId::A1003Venusaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );
    give_card(&mut game, CardId::C1121MaxRepel);
    play_trainer(&mut game, 0, trainer_from_id(CardId::C1121MaxRepel));

    let state = game.get_state_clone();
    let (actor, actions) = state.generate_possible_actions();
    assert_eq!(actor, 1, "the opponent chooses the new Active");
    game.apply_action(&actions[0]);
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_name(), "Bulbasaur");
}

#[test]
fn test_max_repel_not_playable_against_basic_active_or_empty_bench() {
    for opp in [
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
        vec![PlayedCard::from_id(CardId::A1003Venusaur)],
    ] {
        let mut game =
            get_test_game_with_board(vec![PlayedCard::from_id(CardId::A1033Charmander)], opp);
        give_card(&mut game, CardId::C1121MaxRepel);
        let state = game.get_state_clone();
        let (_, actions) = state.generate_possible_actions();
        assert!(!actions
            .iter()
            .any(|a| matches!(a.action, SimpleAction::Play { .. })));
    }
}

/// Full Heal: "Your Active Pokémon recovers from all Special Conditions."
#[test]
fn test_full_heal_cures_all_special_conditions_on_the_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1001Bulbasaur),
            PlayedCard::from_id(CardId::A1033Charmander),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    let mut state = game.get_state_clone();
    state.apply_status_condition(0, 0, StatusCondition::Poisoned);
    state.apply_status_condition(0, 0, StatusCondition::Burned);
    state.apply_status_condition(0, 0, StatusCondition::Asleep);
    game.set_state(state);
    give_card(&mut game, CardId::C1122FullHeal);
    play_trainer(&mut game, 0, trainer_from_id(CardId::C1122FullHeal));

    let state = game.get_state_clone();
    let active = state.get_active(0);
    assert!(!active.is_poisoned() && !active.is_burned() && !active.is_asleep());
    assert!(state.hands[0].is_empty());
}

#[test]
fn test_full_heal_not_playable_without_a_special_condition() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A1001Bulbasaur)],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    give_card(&mut game, CardId::C1122FullHeal);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));
}

/// Genome Converter Y: "Move all [P] Energy from 1 of your Benched [P] Pokémon to your Mega
/// Mewtwo Y ex in the Active Spot."
#[test]
fn test_genome_converter_y_moves_all_psychic_energy_to_mega_mewtwo() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::PC002MegaMewtwoYEx),
            PlayedCard::from_id(CardId::A1115Abra).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Fire,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    give_card(&mut game, CardId::PC009GenomeConverterY);
    play_trainer(&mut game, 0, trainer_from_id(CardId::PC009GenomeConverterY));

    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert_eq!(actions.len(), 1);
    game.apply_action(&actions[0]);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Psychic; 2]
    );
    assert_eq!(
        state.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .attached_energy,
        vec![EnergyType::Fire]
    );
}

#[test]
fn test_genome_converter_y_requires_mega_mewtwo_y_ex_active() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1115Abra),
            PlayedCard::from_id(CardId::A1115Abra).with_energy(vec![EnergyType::Psychic]),
        ],
        vec![PlayedCard::from_id(CardId::A1033Charmander)],
    );
    give_card(&mut game, CardId::PC009GenomeConverterY);
    let state = game.get_state_clone();
    let (_, actions) = state.generate_possible_actions();
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Play { .. })));
}

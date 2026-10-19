use deckgym::{
    actions::Action,
    card_ids::CardId,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_test_game_with_board, play_trainer, trainer_from_id},
    Game,
};

/// Heliolisk (B4b 299) Electrispark: 40 damage, "This attack also does 10 damage to each of your
/// opponent's Benched Pokémon."
fn electrispark_game() -> Game<'static> {
    get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4b299Heliolisk).with_energy(vec![EnergyType::Lightning])],
        vec![
            PlayedCard::from_id(CardId::A1202Chansey),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    )
}

fn play_backpack(game: &mut Game<'static>) {
    let backpack = trainer_from_id(CardId::B1a066ClemontsBackpack);
    let mut state = game.get_state_clone();
    state.hands[0] = vec![Card::Trainer(backpack.clone())];
    game.set_state(state);
    play_trainer(game, 0, backpack);
}

fn electrispark(game: &mut Game<'static>) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::B4b299Heliolisk, 0),
        is_stack: false,
    });
}

fn damage_taken(game: &Game<'static>, idx: usize) -> u32 {
    let state = game.get_state_clone();
    let card = state.in_play_pokemon[1][idx].as_ref().unwrap();
    PlayedCard::from_card(&card.card).get_remaining_hp() - card.get_remaining_hp()
}

/// Clemont's Backpack: "During this turn, attacks used by your Magneton or Heliolisk do +20
/// damage to your opponent's Pokémon." It says Pokémon, not Active Pokémon, so it also reaches
/// the Benched Pokémon hit by Electrispark.
#[test]
fn test_clemonts_backpack_adds_20_to_benched_damage() {
    let mut game = electrispark_game();
    play_backpack(&mut game);
    electrispark(&mut game);

    assert_eq!(damage_taken(&game, 0), 40 + 20, "Active gets +20 once");
    assert_eq!(damage_taken(&game, 1), 10 + 20, "Benched gets +20 too");
}

#[test]
fn test_electrispark_without_backpack_is_unchanged() {
    let mut game = electrispark_game();
    electrispark(&mut game);

    assert_eq!(damage_taken(&game, 0), 40);
    assert_eq!(damage_taken(&game, 1), 10);
}

use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, PlayedCard},
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};

// Extra HP isolates the attack's damage from knockouts and promotion choices.
fn durable_of(id: CardId) -> PlayedCard {
    PlayedCard::new(get_card_by_enum(id), 0, 1000, vec![], false, vec![])
}
// Snorlax is weak to [F]; Charmander is not weak to anything these tests attack with.
fn durable() -> PlayedCard {
    durable_of(CardId::A1211Snorlax)
}
fn ready(id: CardId, index: usize) -> PlayedCard {
    let Card::Pokemon(p) = get_card_by_enum(id) else {
        panic!("Pokémon required")
    };
    PlayedCard::from_id(id).with_energy(p.attacks[index].energy_required.clone())
}
fn board(ours: Vec<PlayedCard>, theirs: Vec<PlayedCard>) -> Game<'static> {
    get_initialized_game_with_board(0, 0, 3, ours, theirs)
}
fn attack(game: &mut Game<'static>, id: CardId, index: usize) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(id, index),
        is_stack: false,
    });
}
fn defender_damage(game: &Game<'static>) -> u32 {
    game.get_state_clone().get_active(1).public_details().damage
}

mod alakazam;

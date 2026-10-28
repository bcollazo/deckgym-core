use deckgym::{
    actions::Action,
    card_ids::CardId,
    database::get_card_by_enum,
    models::PlayedCard,
    test_support::{attack_action, get_initialized_game_with_board},
    Game,
};
// Extra HP isolates random damage branches from knockout and promotion choices.
fn durable() -> PlayedCard {
    PlayedCard::new(
        get_card_by_enum(CardId::A1211Snorlax),
        0,
        1000,
        vec![],
        false,
        vec![],
    )
}
fn ready(id: CardId, index: usize) -> PlayedCard {
    let deckgym::models::Card::Pokemon(p) = get_card_by_enum(id) else {
        panic!("Pokémon required")
    };
    PlayedCard::from_id(id).with_energy(p.attacks[index].energy_required.clone())
}
fn board(seed: u64, ours: Vec<PlayedCard>, theirs: Vec<PlayedCard>) -> Game<'static> {
    get_initialized_game_with_board(seed, 0, 3, ours, theirs)
}
fn attack(game: &mut Game<'static>, id: CardId, index: usize) {
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(id, index),
        is_stack: false,
    });
}

mod alolan_exeggutor;

mod fuecoco;

mod lycanroc;

mod mega_dragonite;

mod mega_mewtwo_y;

mod mega_skarmory;

mod mega_tyranitar;

mod mew;

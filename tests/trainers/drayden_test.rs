use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard},
    test_support::{attack_action, get_initialized_game},
    Game,
};

fn make_trainer_card(card_id: CardId) -> deckgym::models::TrainerCard {
    get_card_by_enum(card_id).as_trainer()
}

/// Rayquaza ex's Draco Meteor against a lone Wailord ex: every hit lands on the same target,
/// so the total damage is fully determined by the number of times a Pokémon is chosen.
fn game_with_rayquaza_against_wailord() -> Game<'static> {
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![PlayedCard::from_id(CardId::PA064RayquazaEx)
            .with_energy(vec![EnergyType::Colorless; 4])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    state.hands[0] = vec![Card::Trainer(make_trainer_card(CardId::B4151Drayden))];
    game.set_state(state);
    game
}

#[test]
fn test_draco_meteor_without_drayden_hits_four_times() {
    let mut game = game_with_rayquaza_against_wailord();

    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::PA064RayquazaEx, 0),
        is_stack: false,
    });

    // 4 chosen times x 40 damage = 160.
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 160);
}

/// Drayden: "During this turn, 1 of your opponent's Pokémon is chosen 1 more time for the
/// Draco Meteor attack used by your Pokémon."
#[test]
fn test_drayden_adds_one_draco_meteor_hit() {
    let mut game = game_with_rayquaza_against_wailord();

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: make_trainer_card(CardId::B4151Drayden),
        },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::PA064RayquazaEx, 0),
        is_stack: false,
    });

    // 5 chosen times x 40 damage = 200.
    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 200);
}

#[test]
fn test_drayden_does_not_boost_other_random_spread_attacks() {
    // Magcargo's Spurt Fire also chooses its targets at random, but Drayden only names the
    // Draco Meteor attack, so its total damage must stay at 3 x 50.
    let mut game = get_initialized_game(0);
    let mut state = game.get_state_clone();
    state.current_player = 0;
    state.turn_count = 3;

    state.set_board(
        vec![
            PlayedCard::from_id(CardId::A4031Magcargo).with_energy(vec![EnergyType::Fire; 2]),
            PlayedCard::from_id(CardId::B4037WailordEx),
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    state.hands[0] = vec![Card::Trainer(make_trainer_card(CardId::B4151Drayden))];
    game.set_state(state);

    game.apply_action(&Action {
        actor: 0,
        action: SimpleAction::Play {
            trainer_card: make_trainer_card(CardId::B4151Drayden),
        },
        is_stack: false,
    });
    game.apply_action(&Action {
        actor: 0,
        action: attack_action(CardId::A4031Magcargo, 0),
        is_stack: false,
    });

    // Spurt Fire spreads 3 x 50 between the opponent's Wailord ex and Magcargo's own benched
    // Wailord ex; neither can be Knocked Out by 150 damage, so the totals are readable.
    let state = game.get_state_clone();
    let opponent_damage = 250 - state.get_active(1).get_remaining_hp();
    let own_bench_damage = 250
        - state.in_play_pokemon[0][1]
            .as_ref()
            .expect("Benched Wailord ex should survive")
            .get_remaining_hp();
    assert_eq!(opponent_damage + own_bench_damage, 150);
}

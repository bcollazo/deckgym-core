//! Attacks from the attacks-a coverage batch that needed a new `Mechanic`.

use deckgym::{
    actions::{Action, SimpleAction},
    card_ids::CardId,
    database::get_card_by_enum,
    models::{Card, EnergyType, PlayedCard, TrainerType},
    test_support::{attack_action, get_initialized_game_with_board, get_test_game_with_board},
    Game,
};

fn attack(game: &mut Game<'static>, actor: usize, card_id: CardId, index: usize) {
    game.apply_action(&Action {
        actor,
        action: attack_action(card_id, index),
        is_stack: false,
    });
}

fn end_turn(game: &mut Game<'static>, actor: usize) {
    game.apply_action(&Action {
        actor,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
}

fn set_hand(game: &mut Game<'static>, player: usize, cards: Vec<Card>) {
    let mut state = game.get_state_clone();
    state.hands[player] = cards;
    game.set_state(state);
}

fn is_trainer_of_type(card: &Card, wanted: TrainerType) -> bool {
    matches!(card, Card::Trainer(trainer) if trainer.trainer_card_type == wanted)
}

// -------------------------------------------------------------------------------------------
// Discarding cards from the opponent's hand
// -------------------------------------------------------------------------------------------

/// Houndoom's Diving Swipe discards one random card from the opponent's hand, whatever it is.
#[test]
fn test_houndoom_diving_swipe_discards_one_random_opponent_hand_card() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4118Houndoom).with_energy(vec![
            EnergyType::Darkness,
            EnergyType::Darkness,
            EnergyType::Darkness,
        ])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    set_hand(
        &mut game,
        1,
        vec![
            get_card_by_enum(CardId::A1001Bulbasaur),
            get_card_by_enum(CardId::A2148RockyHelmet),
        ],
    );

    attack(&mut game, 0, CardId::A4118Houndoom, 0);

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), 1, "exactly one card leaves the hand");
    assert_eq!(
        state.discard_piles[1].len(),
        1,
        "and it lands in the opponent's discard pile"
    );
}

/// Alolan Raticate's Scrounge-and-Scarf only takes Item cards, ignoring Pokémon and Tools.
#[test]
fn test_alolan_raticate_only_discards_an_item_from_opponent_hand() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A3107AlolanRaticate)
            .with_energy(vec![EnergyType::Darkness, EnergyType::Darkness])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    set_hand(
        &mut game,
        1,
        vec![
            get_card_by_enum(CardId::A1001Bulbasaur),
            get_card_by_enum(CardId::A2148RockyHelmet), // a Tool, not an Item
            get_card_by_enum(CardId::PA001Potion),      // the only Item
        ],
    );

    attack(&mut game, 0, CardId::A3107AlolanRaticate, 0);

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), 2);
    assert!(
        !state.hands[1]
            .iter()
            .any(|card| is_trainer_of_type(card, TrainerType::Item)),
        "the Item is the card that must go"
    );
    assert!(
        state.hands[1]
            .iter()
            .any(|card| is_trainer_of_type(card, TrainerType::Tool)),
        "the Tool is not an Item and must stay"
    );
}

/// Alolan Meowth's Meddle is the same effect narrowed to Pokémon Tool cards, and does nothing
/// when the opponent holds none.
#[test]
fn test_alolan_meowth_meddle_takes_a_tool_and_no_ops_without_one() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3a037AlolanMeowth).with_energy(vec![EnergyType::Darkness])
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    set_hand(
        &mut game,
        1,
        vec![
            get_card_by_enum(CardId::PA001Potion),
            get_card_by_enum(CardId::A2148RockyHelmet),
        ],
    );

    attack(&mut game, 0, CardId::A3a037AlolanMeowth, 0);

    let state = game.get_state_clone();
    assert_eq!(state.hands[1].len(), 1);
    assert!(is_trainer_of_type(&state.hands[1][0], TrainerType::Item));

    // A hand with no Tool is left alone.
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3a037AlolanMeowth).with_energy(vec![EnergyType::Darkness])
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    set_hand(&mut game, 1, vec![get_card_by_enum(CardId::PA001Potion)]);

    attack(&mut game, 0, CardId::A3a037AlolanMeowth, 0);

    assert_eq!(game.get_state_clone().hands[1].len(), 1);
}

// -------------------------------------------------------------------------------------------
// Tools, Stadiums and Energy on the board
// -------------------------------------------------------------------------------------------

/// Hoopa's Mischievous Ring shuffles the Tools off EVERY one of the opponent's Pokémon (Active
/// and Bench) back into their deck rather than into the discard pile.
#[test]
fn test_hoopa_mischievous_ring_shuffles_all_opponent_tools_into_deck() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4077Hoopa).with_energy(vec![EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx)
                .with_tool(get_card_by_enum(CardId::A2148RockyHelmet)),
            PlayedCard::from_id(CardId::A1001Bulbasaur)
                .with_tool(get_card_by_enum(CardId::A2147GiantCape)),
        ],
    );
    let before = game.get_state_clone();
    let deck_before = before.decks[1].cards.len();
    let discard_before = before.discard_piles[1].len();

    attack(&mut game, 0, CardId::B4077Hoopa, 0);

    let state = game.get_state_clone();
    assert!(state.get_active(1).attached_tool.is_none());
    assert!(state.in_play_pokemon[1][1]
        .as_ref()
        .unwrap()
        .attached_tool
        .is_none());
    assert_eq!(
        state.decks[1].cards.len(),
        deck_before + 2,
        "both Tools go back into the deck"
    );
    assert_eq!(
        state.discard_piles[1].len(),
        discard_before,
        "and none of them are discarded"
    );
}

/// Machop's Shatter discards whatever Stadium is in play.
#[test]
fn test_machop_shatter_discards_the_stadium_in_play() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B2079Machop).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    let mut state = game.get_state_clone();
    state.set_active_stadium(get_card_by_enum(CardId::B2153TrainingArea));
    game.set_state(state);

    attack(&mut game, 0, CardId::B2079Machop, 0);

    assert!(
        game.get_state_clone().active_stadium.is_none(),
        "Shatter should clear the Stadium"
    );
}

/// Smeargle's Splatter Coating re-rolls one of the defender's Energy into a random basic type.
/// Over many seeds the result must vary and must always be one of the 8 Energy Zone types.
#[test]
fn test_smeargle_splatter_coating_randomizes_one_defender_energy() {
    let mut seen_types = std::collections::HashSet::new();

    for seed in 0..25 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A4148Smeargle)
                .with_energy(vec![EnergyType::Colorless, EnergyType::Colorless])],
            vec![PlayedCard::from_id(CardId::B4037WailordEx).with_energy(vec![EnergyType::Water])],
        );

        attack(&mut game, 0, CardId::A4148Smeargle, 0);

        let energy = game.get_state_clone().get_active(1).attached_energy.clone();
        assert_eq!(energy.len(), 1, "the Energy is retyped, not removed");
        assert!(
            EnergyType::SELECTABLE.contains(&energy[0]),
            "the new type must be one of the 8 Energy Zone types, got {:?}",
            energy[0]
        );
        seen_types.insert(energy[0]);
    }

    assert!(
        seen_types.len() > 1,
        "the retyped Energy should vary across seeds, saw only {seen_types:?}"
    );
}

/// Groudon's Gaia Blast discards 2 random Energy from among the attacker's OWN Pokémon, never
/// from the opponent's.
#[test]
fn test_groudon_gaia_blast_discards_two_of_your_own_energy() {
    for seed in 0..10 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B2b035Groudon).with_energy(vec![
                    EnergyType::Fighting,
                    EnergyType::Fighting,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
                PlayedCard::from_id(CardId::A1001Bulbasaur).with_energy(vec![EnergyType::Grass]),
            ],
            vec![PlayedCard::from_id(CardId::B4037WailordEx)
                .with_energy(vec![EnergyType::Water, EnergyType::Water])],
        );

        attack(&mut game, 0, CardId::B2b035Groudon, 0);

        let state = game.get_state_clone();
        let own_energy: usize = state
            .enumerate_in_play_pokemon(0)
            .map(|(_, pokemon)| pokemon.attached_energy.len())
            .sum();
        assert_eq!(own_energy, 5 - 2, "2 of your own 5 Energy are discarded");
        assert_eq!(
            state.get_active(1).attached_energy.len(),
            2,
            "the opponent's Energy is untouched"
        );
        assert_eq!(state.discard_energies[0].len(), 2);
    }
}

// -------------------------------------------------------------------------------------------
// Self-Energy discards paired with damage
// -------------------------------------------------------------------------------------------

/// Volcarona's Volcanic Ash discards 2 [R] and lets the attacker pick any one of the opponent's
/// Pokémon (Active or Benched) to take the 80.
#[test]
fn test_volcarona_volcanic_ash_discards_fire_and_snipes_a_chosen_target() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A1a014Volcarona).with_energy(vec![
                EnergyType::Fire,
                EnergyType::Fire,
                EnergyType::Colorless,
            ]),
        ],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx),
            PlayedCard::from_id(CardId::A1056BlastoiseEx),
        ],
    );

    attack(&mut game, 0, CardId::A1a014Volcarona, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Colorless],
        "both [R] Energy are discarded"
    );

    // The attacker now picks a target; take the Benched one to prove it can be sniped.
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    assert_eq!(choices.len(), 2, "Active and Bench are both legal targets");
    let bench_choice = choices
        .iter()
        .find(|choice| {
            matches!(&choice.action, SimpleAction::ApplyDamage { targets, .. }
                if targets.iter().any(|(_, _, idx)| *idx == 1))
        })
        .expect("the Benched Pokemon should be targetable")
        .clone();
    game.apply_action(&bench_choice);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        250,
        "Active is untouched"
    );
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        180 - 80
    );
}

/// Kyogre's Tidal Blast discards 3 [W] and hits every one of the opponent's Pokémon.
#[test]
fn test_kyogre_tidal_blast_discards_water_and_hits_the_whole_board() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4041Kyogre).with_energy(vec![
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
            EnergyType::Water,
        ])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx),
            PlayedCard::from_id(CardId::A1056BlastoiseEx),
        ],
    );

    attack(&mut game, 0, CardId::B4041Kyogre, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Water],
        "3 of the 4 [W] Energy are discarded"
    );
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 50);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        180 - 50
    );
}

/// Rapid Strike Urshifu's Tornado Shot discards a [W], hits the Active for 40, and snipes a
/// chosen Benched Pokémon for another 40.
#[test]
fn test_urshifu_tornado_shot_discards_water_and_hits_active_plus_bench() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B3051RapidStrikeUrshifu)
            .with_energy(vec![EnergyType::Water, EnergyType::Colorless])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx),
            PlayedCard::from_id(CardId::A1056BlastoiseEx),
        ],
    );

    attack(&mut game, 0, CardId::B3051RapidStrikeUrshifu, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(0).attached_energy,
        vec![EnergyType::Colorless],
        "the [W] Energy is discarded"
    );

    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    game.apply_action(&choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(state.get_active(1).get_remaining_hp(), 250 - 40);
    assert_eq!(
        state.in_play_pokemon[1][1]
            .as_ref()
            .unwrap()
            .get_remaining_hp(),
        180 - 40
    );
}

/// Galvantula's Electric Shock empties its own Energy and paralyzes the defender.
#[test]
fn test_galvantula_electric_shock_dumps_all_energy_and_paralyzes() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::A3b027Galvantula).with_energy(vec![
                EnergyType::Lightning,
                EnergyType::Lightning,
                EnergyType::Colorless,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    attack(&mut game, 0, CardId::A3b027Galvantula, 0);

    let state = game.get_state_clone();
    assert!(
        state.get_active(0).attached_energy.is_empty(),
        "every Energy is discarded, not just the [L] ones"
    );
    assert_eq!(state.discard_energies[0].len(), 3);
    assert!(state.get_active(1).is_paralyzed());
}

// -------------------------------------------------------------------------------------------
// Decks and hands
// -------------------------------------------------------------------------------------------

/// Ultra Necrozma ex's Shoegaze mills 5 cards off BOTH decks.
#[test]
fn test_ultra_necrozma_shoegaze_mills_both_decks() {
    let mut game = get_test_game_with_board(
        vec![
            PlayedCard::from_id(CardId::PA081UltraNecrozmaEx).with_energy(vec![
                EnergyType::Psychic,
                EnergyType::Psychic,
                EnergyType::Metal,
                EnergyType::Metal,
            ]),
        ],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    let before = game.get_state_clone();
    let decks_before = [before.decks[0].cards.len(), before.decks[1].cards.len()];
    let discards_before = [before.discard_piles[0].len(), before.discard_piles[1].len()];

    // Shoegaze is Ultra Necrozma ex's second attack.
    attack(&mut game, 0, CardId::PA081UltraNecrozmaEx, 1);

    let state = game.get_state_clone();
    for player in 0..2 {
        assert_eq!(
            state.decks[player].cards.len(),
            decks_before[player] - 5,
            "player {player}'s deck should lose 5 cards"
        );
        assert_eq!(
            state.discard_piles[player].len(),
            discards_before[player] + 5
        );
    }
}

/// Pachirisu's Crackling Snap does 30, or 50 when the milled card is an Item. Stacking the deck
/// makes each branch deterministic.
#[test]
fn test_pachirisu_crackling_snap_bonus_depends_on_the_milled_card() {
    for (top_card, expected_damage) in [
        (CardId::PA001Potion, 30 + 20), // Potion is an Item
        (CardId::A1001Bulbasaur, 30),   // a Pokemon is not
    ] {
        // Mega Venusaur ex is Weak to [R], so Pachirisu's [L] attack gets no Weakness bonus.
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::B4054Pachirisu)
                .with_energy(vec![EnergyType::Lightning])],
            vec![PlayedCard::from_id(CardId::B1a004MegaVenusaurEx)],
        );

        let mut state = game.get_state_clone();
        state.decks[0].cards = vec![get_card_by_enum(top_card)];
        game.set_state(state);

        attack(&mut game, 0, CardId::B4054Pachirisu, 0);

        let state = game.get_state_clone();
        assert_eq!(
            state.get_active(1).get_remaining_hp(),
            240 - expected_damage,
            "milling {top_card:?} should give {expected_damage} damage"
        );
        assert_eq!(state.decks[0].cards.len(), 0, "the top card is milled");
        assert_eq!(state.discard_piles[0].len(), 1);
    }
}

/// Dugtrio's Cliff Crumbler is the same shape, keyed on a [F] Pokémon instead of an Item.
#[test]
fn test_dugtrio_cliff_crumbler_bonus_depends_on_the_milled_card() {
    for (top_card, expected_damage) in [
        (CardId::B2079Machop, 40 + 60), // Machop is a [F] Pokemon
        (CardId::A1001Bulbasaur, 40),   // Bulbasaur is [G]
    ] {
        let mut game = get_test_game_with_board(
            vec![PlayedCard::from_id(CardId::A4a041Dugtrio).with_energy(vec![EnergyType::Fighting])],
            vec![PlayedCard::from_id(CardId::B4037WailordEx)],
        );

        let mut state = game.get_state_clone();
        state.decks[0].cards = vec![get_card_by_enum(top_card)];
        game.set_state(state);

        attack(&mut game, 0, CardId::A4a041Dugtrio, 0);

        assert_eq!(
            game.get_state_clone().get_active(1).get_remaining_hp(),
            250 - expected_damage,
            "milling {top_card:?} should give {expected_damage} damage"
        );
    }
}

/// Slowking's Litter offers every "up to 2 Tools" choice, and discarding 2 deals 100.
#[test]
fn test_slowking_litter_scales_with_the_tools_discarded() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4a018Slowking).with_energy(vec![EnergyType::Water])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );
    set_hand(
        &mut game,
        0,
        vec![
            get_card_by_enum(CardId::A2148RockyHelmet),
            get_card_by_enum(CardId::A2147GiantCape),
            get_card_by_enum(CardId::PA001Potion), // an Item: not discardable by Litter
        ],
    );

    attack(&mut game, 0, CardId::A4a018Slowking, 0);

    let (actor, choices) = game.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 0);
    // Noop (discard nothing) + 2 single-Tool choices + 1 both-Tools choice.
    assert_eq!(choices.len(), 4, "got {choices:?}");

    let both = choices
        .iter()
        .find(|choice| {
            matches!(&choice.action, SimpleAction::DiscardOwnCardsThenDamage { cards, .. }
                if cards.len() == 2)
        })
        .expect("discarding both Tools should be offered")
        .clone();
    game.apply_action(&both);

    // The composite action queues the damage; resolve it.
    let (_, damage_choices) = game.get_state_clone().generate_possible_actions();
    game.apply_action(&damage_choices[0].clone());

    let state = game.get_state_clone();
    assert_eq!(
        state.get_active(1).get_remaining_hp(),
        250 - 100,
        "2 Tools discarded means 2 x 50 damage"
    );
    assert_eq!(state.hands[0].len(), 1, "only the Item is left in hand");
    assert_eq!(state.discard_piles[0].len(), 2);
}

/// Aipom's Imitate draws until its owner's hand matches the opponent's.
#[test]
fn test_aipom_imitate_draws_up_to_the_opponents_hand_size() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4142Aipom).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A1001Bulbasaur)];
    state.hands[1] = vec![get_card_by_enum(CardId::A1001Bulbasaur); 5];
    game.set_state(state);

    attack(&mut game, 0, CardId::A4142Aipom, 0);

    let state = game.get_state_clone();
    assert_eq!(
        state.hands[0].len(),
        5,
        "Imitate should draw up to the opponent's 5 cards"
    );
    assert_eq!(state.hands[1].len(), 5, "the opponent's hand is untouched");
}

/// Imitate draws nothing when the attacker already holds at least as many cards.
#[test]
fn test_aipom_imitate_draws_nothing_when_already_ahead() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::A4142Aipom).with_energy(vec![EnergyType::Colorless])],
        vec![PlayedCard::from_id(CardId::B4037WailordEx)],
    );

    let mut state = game.get_state_clone();
    state.hands[0] = vec![get_card_by_enum(CardId::A1001Bulbasaur); 6];
    state.hands[1] = vec![get_card_by_enum(CardId::A1001Bulbasaur); 2];
    game.set_state(state);

    attack(&mut game, 0, CardId::A4142Aipom, 0);

    assert_eq!(game.get_state_clone().hands[0].len(), 6);
}

// -------------------------------------------------------------------------------------------
// Coin-flip removals
// -------------------------------------------------------------------------------------------

/// Bewear's Superpowered Hug Knocks Out the defender on double heads, scoring the point.
#[test]
fn test_bewear_superpowered_hug_knocks_out_on_double_heads() {
    let mut saw_ko = false;
    let mut saw_nothing = false;

    for seed in 0..20 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::A3a058Bewear).with_energy(vec![
                EnergyType::Colorless,
                EnergyType::Colorless,
                EnergyType::Colorless,
            ])],
            vec![
                PlayedCard::from_id(CardId::B4037WailordEx),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );

        attack(&mut game, 0, CardId::A3a058Bewear, 0);

        let state = game.get_state_clone();
        let wailord_gone = state.in_play_pokemon[1][0]
            .as_ref()
            .is_none_or(|pokemon| pokemon.get_name() != "Wailord ex");
        if wailord_gone {
            saw_ko = true;
            assert_eq!(
                state.points[0], 2,
                "knocking out a Pokemon ex scores 2 points"
            );
        } else {
            saw_nothing = true;
            assert_eq!(state.get_active(1).get_remaining_hp(), 250);
            assert_eq!(state.points[0], 0);
        }
    }

    assert!(
        saw_ko,
        "double heads should sometimes knock the defender out"
    );
    assert!(saw_nothing, "any tails should leave the defender alone");
}

/// Scream Tail's Shooing Shout discards the defender on double heads — it leaves play, but the
/// attacker scores NO point.
#[test]
fn test_scream_tail_shooing_shout_discards_without_scoring() {
    let mut saw_discard = false;
    let mut saw_nothing = false;

    for seed in 0..20 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![PlayedCard::from_id(CardId::B3a025ScreamTail)
                .with_energy(vec![EnergyType::Psychic, EnergyType::Psychic])],
            vec![
                PlayedCard::from_id(CardId::B4037WailordEx),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
        );
        let discard_before = game.get_state_clone().discard_piles[1].len();

        attack(&mut game, 0, CardId::B3a025ScreamTail, 0);

        let state = game.get_state_clone();
        if state.discard_piles[1].len() > discard_before {
            saw_discard = true;
            assert_eq!(
                state.points[0], 0,
                "a discarded Pokemon is not a knockout and scores nothing"
            );
            // Player 1 must promote a replacement.
            let (actor, choices) = state.generate_possible_actions();
            assert_eq!(actor, 1);
            assert!(choices
                .iter()
                .all(|choice| matches!(choice.action, SimpleAction::Activate { player: 1, .. })));
        } else {
            saw_nothing = true;
            assert_eq!(state.get_active(1).get_remaining_hp(), 250);
        }
    }

    assert!(saw_discard, "double heads should sometimes discard");
    assert!(saw_nothing, "any tails should leave the defender alone");
}

/// Druddigon's Giga Claw does its full 120 unless BOTH coins come up tails, in which case it
/// does nothing at all.
#[test]
fn test_druddigon_giga_claw_does_nothing_on_double_tails() {
    let mut saw_zero = false;
    let mut saw_full = false;

    for seed in 0..20 {
        let mut game = get_initialized_game_with_board(
            seed,
            0,
            3,
            vec![
                PlayedCard::from_id(CardId::B1176Druddigon).with_energy(vec![
                    EnergyType::Fire,
                    EnergyType::Water,
                    EnergyType::Colorless,
                    EnergyType::Colorless,
                ]),
            ],
            vec![PlayedCard::from_id(CardId::B4037WailordEx)],
        );

        attack(&mut game, 0, CardId::B1176Druddigon, 0);

        let damage = 250 - game.get_state_clone().get_active(1).get_remaining_hp();
        assert!(
            damage == 0 || damage == 120,
            "Giga Claw is all-or-nothing, got {damage}"
        );
        if damage == 0 {
            saw_zero = true;
        } else {
            saw_full = true;
        }
    }

    assert!(saw_zero, "double tails should sometimes fizzle");
    assert!(saw_full, "any heads should deal the full 120");
}

// -------------------------------------------------------------------------------------------
// Delayed knock out
// -------------------------------------------------------------------------------------------

/// Armaldo's Abyssal Drop empties its own Energy and marks a spot; whatever is standing there at
/// the end of the opponent's next turn is Knocked Out, no matter how much HP it has left.
#[test]
fn test_armaldo_abyssal_drop_knocks_out_the_chosen_spot() {
    let mut game = get_test_game_with_board(
        vec![PlayedCard::from_id(CardId::B4082Armaldo).with_energy(vec![
            EnergyType::Fighting,
            EnergyType::Colorless,
            EnergyType::Colorless,
        ])],
        vec![
            PlayedCard::from_id(CardId::B4037WailordEx),
            PlayedCard::from_id(CardId::A1001Bulbasaur),
        ],
    );

    attack(&mut game, 0, CardId::B4082Armaldo, 0);

    let state = game.get_state_clone();
    assert!(
        state.get_active(0).attached_energy.is_empty(),
        "Abyssal Drop discards all of Armaldo's Energy"
    );

    // Choose the (untouched, 250 HP) Active Spot.
    let (actor, choices) = state.generate_possible_actions();
    assert_eq!(actor, 0);
    let active_spot = choices
        .iter()
        .find(|choice| {
            matches!(
                choice.action,
                SimpleAction::ScheduleDelayedSpotKnockOut {
                    target_in_play_idx: 0,
                    ..
                }
            )
        })
        .expect("the Active Spot should be choosable")
        .clone();
    game.apply_action(&active_spot);

    assert_eq!(
        game.get_state_clone().get_active(1).get_remaining_hp(),
        250,
        "nothing happens right away"
    );

    end_turn(&mut game, 0);
    end_turn(&mut game, 1);

    let state = game.get_state_clone();
    assert_eq!(
        state.points[0], 2,
        "the delayed knockout of a Pokemon ex scores 2 points"
    );
    assert!(state
        .maybe_get_active(1)
        .is_none_or(|pokemon| pokemon.get_name() != "Wailord ex"));
}

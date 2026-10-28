use super::*;
use deckgym::{
    actions::SimpleAction,
    card_validation::{get_implementation_status, ImplementationStatus},
    models::{Card, EnergyType as E},
    test_support::{play_trainer, trainer_from_id},
};
use strum::IntoEnumIterator;

#[test]
fn every_new_ex_printing_has_complete_attack_and_ability_coverage() {
    let mut count = 0;
    for id in CardId::iter() {
        let card = get_card_by_enum(id);
        if (card.get_id().starts_with("C1 ") || card.get_id().starts_with("P-C "))
            && card.get_name().ends_with(" ex")
        {
            count += 1;
            assert_eq!(
                get_implementation_status(id),
                ImplementationStatus::Complete,
                "{id:?}"
            );
        }
    }
    assert_eq!(count, 50);
}
fn id(s: &str) -> CardId {
    CardId::from_card_id(s).unwrap()
}
fn play(game: &mut Game<'static>, id: CardId) {
    let t = trainer_from_id(id);
    let mut s = game.get_state_clone();
    s.hands[0] = vec![Card::Trainer(t.clone())];
    game.set_state(s);
    play_trainer(game, 0, t);
}
#[test]
fn bewear_hyper_lariat_has_only_the_single_and_double_head_totals() {
    let id = id("C1 118");
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        let mut g = board(seed, vec![ready(id, 0)], vec![durable()]);
        attack(&mut g, id, 0);
        seen.insert(g.get_state_clone().get_active(1).public_details().damage);
    }
    assert_eq!(seen, std::collections::BTreeSet::from([100, 200]));
}
#[test]
fn giratina_dragon_impact_discards_two_energy_after_damage() {
    let id = id("C1 101");
    let mut g = board(0, vec![ready(id, 1)], vec![durable()]);
    attack(&mut g, id, 1);
    let s = g.get_state_clone();
    assert_eq!(s.get_active(1).public_details().damage, 140);
    assert_eq!(s.get_active(0).attached_energy.len(), 1);
    assert_eq!(s.discard_energies[0].len(), 2);
}
#[test]
fn hisuian_zoroark_counts_pokemon_but_not_trainers_in_discard() {
    let id = id("C1 213");
    let mut g = board(0, vec![ready(id, 0)], vec![durable()]);
    let mut s = g.get_state_clone();
    s.discard_piles[0] = vec![
        get_card_by_enum(CardId::A1001Bulbasaur),
        get_card_by_enum(CardId::A1033Charmander),
        get_card_by_enum(CardId::PA001Potion),
    ];
    g.set_state(s);
    attack(&mut g, id, 0);
    assert_eq!(
        g.get_state_clone().get_active(1).public_details().damage,
        120
    );
}
#[test]
fn mega_audino_heals_active_and_bench_without_exceeding_full_hp() {
    let id = id("C1 212");
    let mut g = board(
        0,
        vec![ready(id, 0).with_damage(40), durable().with_damage(20)],
        vec![durable()],
    );
    attack(&mut g, id, 0);
    let s = g.get_state_clone();
    assert_eq!(s.get_active(1).public_details().damage, 90);
    assert_eq!(s.get_active(0).public_details().damage, 10);
    assert_eq!(
        s.in_play_pokemon[0][1]
            .as_ref()
            .unwrap()
            .public_details()
            .damage,
        0
    );
}
#[test]
fn mega_gallade_bonus_depends_on_actually_playing_a_supporter() {
    let id = id("C1 209");
    for supporter in [false, true] {
        let mut g = board(0, vec![ready(id, 0)], vec![durable()]);
        if supporter {
            play(&mut g, CardId::PA007ProfessorsResearch);
        }
        attack(&mut g, id, 0);
        // Snorlax is weak to Gallade's Fighting type.
        assert_eq!(
            g.get_state_clone().get_active(1).public_details().damage,
            if supporter { 170 } else { 120 }
        );
    }
}
#[test]
fn mega_greninja_prevents_the_defender_retreating() {
    let id = id("C1 042");
    let mut g = board(
        0,
        vec![ready(id, 0)],
        vec![durable().with_energy(vec![E::Water; 4]), durable()],
    );
    assert!(
        g.get_state_clone().in_play_pokemon[1][0]
            .as_ref()
            .unwrap()
            .attached_energy
            .len()
            >= 4
    );
    attack(&mut g, id, 0);
    g.apply_action(&Action {
        actor: 0,
        action: SimpleAction::EndTurn,
        is_stack: false,
    });
    g.play_until_stable();
    let (actor, actions) = g.get_state_clone().generate_possible_actions();
    assert_eq!(actor, 1);
    assert!(!actions
        .iter()
        .any(|a| matches!(a.action, SimpleAction::Retreat(_))));
    assert_eq!(
        g.get_state_clone().get_active(1).public_details().damage,
        100
    );
}
#[test]
fn mega_mewtwo_x_checks_stadium_and_applies_secondary_fighting_weakness() {
    let id = id("C1 056");
    for stadium in [false, true] {
        let mut g = board(0, vec![ready(id, 0)], vec![durable()]);
        if stadium {
            let mut s = g.get_state_clone();
            s.active_stadium = Some(get_card_by_enum(CardId::B2a093Mesagoza));
            g.set_state(s);
        }
        attack(&mut g, id, 0);
        assert_eq!(
            g.get_state_clone().get_active(1).public_details().damage,
            if stadium { 160 } else { 120 }
        );
    }
}
#[test]
fn mega_sceptile_discards_grass_and_poisons_after_damage() {
    let id = id("C1 207");
    let mut g = board(0, vec![ready(id, 0)], vec![durable()]);
    attack(&mut g, id, 0);
    let s = g.get_state_clone();
    assert_eq!(s.get_active(0).attached_energy, vec![E::Grass]);
    assert!(s.get_active(1).is_poisoned());
    assert_eq!(s.get_active(1).public_details().damage, 130);
}
#[test]
fn pikachu_thundertail_discards_only_two_lightning_energy() {
    let id = id("C1 043");
    let mut g = board(
        0,
        vec![PlayedCard::from_id(id).with_energy(vec![E::Lightning, E::Lightning, E::Water])],
        vec![durable()],
    );
    attack(&mut g, id, 1);
    let s = g.get_state_clone();
    assert_eq!(s.get_active(1).public_details().damage, 130);
    assert_eq!(s.get_active(0).attached_energy, vec![E::Water]);
}
#[test]
fn typhlosion_inferno_discards_per_heads_and_handles_empty_energy() {
    let id = id("C1 208");
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        for energy in [vec![], vec![E::Water, E::Fire]] {
            let initial = energy.len();
            let mut g = board(
                seed,
                vec![ready(id, 0)],
                vec![durable().with_energy(energy)],
            );
            attack(&mut g, id, 0);
            let s = g.get_state_clone();
            assert_eq!(s.get_active(1).public_details().damage, 110);
            assert!(s.get_active(1).attached_energy.len() <= initial);
            if initial > 0 {
                seen.insert(s.get_active(1).attached_energy.len());
            }
        }
    }
    assert_eq!(seen, std::collections::BTreeSet::from([0, 1, 2]));
}
#[test]
fn generalized_inferno_dance_still_attaches_fire_only_to_fire_bench_and_honors_will() {
    let id = CardId::A1047MoltresEx;
    for seed in 0..20 {
        let mut g = board(
            seed,
            vec![
                ready(id, 0),
                PlayedCard::from_id(CardId::A1033Charmander),
                PlayedCard::from_id(CardId::A1001Bulbasaur),
            ],
            vec![durable()],
        );
        play(&mut g, CardId::A4156Will);
        attack(&mut g, id, 0);
        let (_, actions) = g.get_state_clone().generate_possible_actions();
        let a = actions
            .into_iter()
            .find(|a| {
                matches!(
                    a.action,
                    SimpleAction::Attach {
                        is_turn_energy: false,
                        ..
                    }
                )
            })
            .expect("Will forces at least one heads");
        g.apply_action(&a);
        let s = g.get_state_clone();
        let energy = &s.in_play_pokemon[0][1].as_ref().unwrap().attached_energy;
        assert!((1..=3).contains(&energy.len()) && energy.iter().all(|e| *e == E::Fire));
        assert!(s.in_play_pokemon[0][2]
            .as_ref()
            .unwrap()
            .attached_energy
            .is_empty());
    }
}

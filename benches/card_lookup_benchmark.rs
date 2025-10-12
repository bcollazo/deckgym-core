use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deckgym::card_ids::CardId;
use deckgym::models::{Attack, Card, EnergyType, PokemonCard};
use lazy_static::lazy_static;

fn num_match_approach(id: u16) -> Option<CardId> {
    match id {
        1 => Some(CardId::A1001Bulbasaur),
        2 => Some(CardId::A1002Ivysaur),
        3 => Some(CardId::A1003Venusaur),
        _ => None,
    }
}

fn match_approach(id: &str) -> Option<CardId> {
    match id {
        "A1 001" => Some(CardId::A1001Bulbasaur),
        "A1 002" => Some(CardId::A1002Ivysaur),
        "A1 003" => Some(CardId::A1003Venusaur),
        _ => None,
    }
}

lazy_static! {
    static ref CARD_ID_MAP: HashMap<&'static str, CardId> = {
        let mut m = HashMap::new();
        m.insert("001", CardId::A1001Bulbasaur);
        m.insert("002", CardId::A1002Ivysaur);
        m.insert("003", CardId::A1003Venusaur);
        m
    };
}

fn lazy_static_map_approach(id: &str) -> Option<CardId> {
    CARD_ID_MAP.get(id).cloned()
}

lazy_static! {
    static ref BULBASAUR: Card = Card::Pokemon(PokemonCard {
        id: "A1 001".to_string(),
        name: "Bulbasaur".to_string(),
        stage: 0,
        evolves_from: None,
        hp: 70,
        energy_type: EnergyType::Grass,
        ability: None,
        attacks: vec![Attack {
            energy_required: vec![EnergyType::Grass, EnergyType::Colorless],
            title: "Vine Whip".to_string(),
            fixed_damage: 40,
            effect: None,
        }],
        weakness: Some(EnergyType::Fire),
        retreat_cost: vec![EnergyType::Colorless],
        rarity: "◇".to_string(),
        booster_pack: "Genetic Apex (A1) Mewtwo".to_string(),
    });
}

pub fn get_card_lazy_map(id: CardId) -> &'static Card {
    match id {
        CardId::A1001Bulbasaur => &BULBASAUR,
        _ => panic!("Card not found"),
    }
}

pub fn get_card_create(id: CardId) -> Card {
    match id {
        CardId::A1001Bulbasaur => Card::Pokemon(PokemonCard {
            id: "A1 001".to_string(),
            name: "Bulbasaur".to_string(),
            stage: 0,
            evolves_from: None,
            hp: 70,
            energy_type: EnergyType::Grass,
            ability: None,
            attacks: vec![Attack {
                energy_required: vec![EnergyType::Grass, EnergyType::Colorless],
                title: "Vine Whip".to_string(),
                fixed_damage: 40,
                effect: None,
            }],
            weakness: Some(EnergyType::Fire),
            retreat_cost: vec![EnergyType::Colorless],
            rarity: "◇".to_string(),
            booster_pack: "Genetic Apex (A1) Mewtwo".to_string(),
        }),
        _ => panic!("Card not found"),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("num_match_approach", |b| {
        b.iter(|| num_match_approach(black_box(1)))
    });
    c.bench_function("match_approach", |b| {
        b.iter(|| match_approach(black_box("A1 001")))
    });
    c.bench_function("lazy_static_map_approach", |b| {
        b.iter(|| lazy_static_map_approach(black_box("A1 001")))
    });

    c.bench_function("get_card_create", |b| {
        b.iter(|| get_card_create(black_box(CardId::A1001Bulbasaur)))
    });
    c.bench_function("get_card_lazy_map", |b| {
        b.iter(|| get_card_lazy_map(black_box(CardId::A1001Bulbasaur)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

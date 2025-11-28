use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deckgym::{
    card_ids::CardId, card_logic::can_rare_candy_evolve, database::get_card_by_enum,
    to_playable_card,
};

fn benchmark_rare_candy_evolution_check(c: &mut Criterion) {
    // Setup: Get common evolution lines
    let venusaur = get_card_by_enum(CardId::A1003Venusaur);
    let bulbasaur = get_card_by_enum(CardId::A1001Bulbasaur);
    let bulbasaur_played = to_playable_card(&bulbasaur, false);

    let charizard = get_card_by_enum(CardId::A1035Charizard);
    let charmander = get_card_by_enum(CardId::A1033Charmander);
    let charmander_played = to_playable_card(&charmander, false);

    c.bench_function("rare_candy_valid_evolution", |b| {
        b.iter(|| black_box(can_rare_candy_evolve(&venusaur, &bulbasaur_played)))
    });

    c.bench_function("rare_candy_invalid_evolution", |b| {
        b.iter(|| {
            // Charizard cannot evolve from Bulbasaur
            black_box(can_rare_candy_evolve(&charizard, &bulbasaur_played))
        })
    });

    // Benchmark multiple checks (simulates checking entire hand)
    c.bench_function("rare_candy_check_10_cards", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(can_rare_candy_evolve(&venusaur, &bulbasaur_played));
                black_box(can_rare_candy_evolve(&charizard, &charmander_played));
            }
        })
    });
}

criterion_group!(benches, benchmark_rare_candy_evolution_check);
criterion_main!(benches);

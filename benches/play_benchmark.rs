mod common;

use common::load_test_decks;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deckgym::players::{Player, RandomPlayer};

fn play_random_game(seed: u64) {
    let (deck_a, deck_b) = load_test_decks();
    let player_a = Box::new(RandomPlayer { deck: deck_a });
    let player_b = Box::new(RandomPlayer { deck: deck_b });
    let players: Vec<Box<dyn Player>> = vec![player_a, player_b];
    let mut game = deckgym::Game::new(players, seed);
    game.play();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("play random game", |b| {
        b.iter(|| play_random_game(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

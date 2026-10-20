/// First-turn Basic availability lookup table.
///
/// For a target Basic species with `c` copies in a 20-card deck that also has `m` other Basics,
/// `p` Poké Ball, `r` Professor's Research, `cc` Copycat and `t` Tools (rest inert), measures
/// how often at least one / both copies of the target are in play (Active or Bench) by the end
/// of the player's own first turn, going first and going second.
///
/// Each game is a mirror match (both players use the cell deck and the same policy) that stops
/// as soon as both players have finished their first turn, so every game yields one
/// "going first" and one "going second" sample.
///
/// Input is a JSONL plan (one cell per line: {"c","m","p","r","cc","t","games"}); output is an
/// append-only JSONL checkpoint with one line per finished (cell, policy). Cells already present
/// in the output with at least the requested games are skipped, so the run is resumable.
///
/// Run with:
///   cargo run --release --example first_turn_basics -- \
///     --plan first-turn-basics/plan.jsonl --out first-turn-basics/results.jsonl --seed 20261020
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

use clap::Parser;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};

use deckgym::{
    actions::{Action, SimpleAction},
    deck::Deck,
    models::{Card, TrainerType},
    players::Player,
    Game, State,
};

const TARGET: &str = "Bulbasaur";
const TARGET_LINE: &str = "Bulbasaur A1 001";
/// Other Basic species (no abilities), up to 2 copies each.
const OTHER_BASICS: [&str; 6] = [
    "Paras A1 014",
    "Tangela A1 024",
    "Pinsir A1 026",
    "Scyther A1 025",
    "Cottonee A1 027",
    "Skiddo A1 031",
];
/// Stage 1 Pokémon: can't be played on either player's first turn, so they are inert.
const INERT: [&str; 11] = [
    "Ivysaur A1 002",
    "Metapod A1 006",
    "Kakuna A1 009",
    "Gloom A1 012",
    "Parasect A1 015",
    "Venomoth A1 017",
    "Weepinbell A1 019",
    "Exeggutor A1 022",
    "Whimsicott A1 028",
    "Lilligant A1 030",
    "Gogoat A1 032",
];
const TOOLS: [&str; 2] = ["Giant Cape A2 147", "Rocky Helmet A2 148"];
const POKE_BALL: &str = "Poké Ball P-A 005";
const RESEARCH: &str = "Professor's Research P-A 007";
const COPYCAT: &str = "Copycat B1 225";

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    plan: String,
    #[arg(long)]
    out: String,
    #[arg(long, default_value_t = 20261020)]
    seed: u64,
    /// greedy | random | everything
    #[arg(long, default_value = "greedy")]
    policy: String,
    /// Only run the first N cells of the plan (for benchmarking)
    #[arg(long)]
    limit: Option<usize>,
    /// Print every action of N games of the first plan cell and exit
    #[arg(long)]
    trace: Option<u64>,
    /// Non-mirror mode: the opponent plays the cell deck but with this many Tools.
    #[arg(long)]
    opp_t: Option<u32>,
    /// Non-mirror mode: the opponent plays this fixed cell, given as "c,m,p,r,cc,t".
    #[arg(long)]
    opp_cell: Option<String>,
}

/// Who the opponent is: the same cell (mirror) or an override.
#[derive(Clone, Copy, Debug)]
enum Opponent {
    Mirror,
    Tools(u32),
    Fixed(Cell),
}

impl Opponent {
    fn deck_cell(&self, cell: &Cell) -> Cell {
        match self {
            Opponent::Mirror => *cell,
            Opponent::Tools(t) => Cell { t: *t, ..*cell },
            Opponent::Fixed(c) => *c,
        }
    }

    fn label(&self) -> String {
        match self {
            Opponent::Mirror => "mirror".into(),
            Opponent::Tools(t) => format!("opp_t={t}"),
            Opponent::Fixed(c) => format!("opp={},{},{},{},{},{}", c.c, c.m, c.p, c.r, c.cc, c.t),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Policy {
    Greedy,
    Random,
    Everything,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    c: u32,
    m: u32,
    p: u32,
    r: u32,
    cc: u32,
    t: u32,
}

impl Cell {
    fn key(&self) -> u64 {
        ((self.c as u64) << 40)
            | ((self.m as u64) << 32)
            | ((self.p as u64) << 24)
            | ((self.r as u64) << 16)
            | ((self.cc as u64) << 8)
            | self.t as u64
    }

    fn deck_string(&self) -> Result<String, String> {
        let mut lines = vec!["Energy: Grass".to_string()];
        lines.push(format!("{} {TARGET_LINE}", self.c));
        let mut push_spread = |n: u32, names: &[&str], what: &str| -> Result<(), String> {
            let mut left = n;
            for name in names {
                if left == 0 {
                    break;
                }
                let k = left.min(2);
                lines.push(format!("{k} {name}"));
                left -= k;
            }
            if left > 0 {
                return Err(format!("not enough distinct {what} names for {n} cards"));
            }
            Ok(())
        };
        push_spread(self.m, &OTHER_BASICS, "basic")?;
        push_spread(self.t, &TOOLS, "tool")?;
        let used = self.c + self.m + self.p + self.r + self.cc + self.t;
        if used > 20 {
            return Err(format!("cell uses {used} cards"));
        }
        push_spread(20 - used, &INERT, "inert")?;
        for (n, name) in [(self.p, POKE_BALL), (self.r, RESEARCH), (self.cc, COPYCAT)] {
            if n > 0 {
                lines.push(format!("{n} {name}"));
            }
        }
        Ok(lines.join("\n"))
    }
}

fn is_target(card: &Card) -> bool {
    card.get_name() == TARGET
}

fn targets_in_play(state: &State, player: usize) -> u32 {
    state.in_play_pokemon[player]
        .iter()
        .flatten()
        .filter(|p| p.get_name() == TARGET)
        .count() as u32
}

fn targets_in_hand(state: &State, player: usize) -> u32 {
    state.hands[player].iter().filter(|c| is_target(c)).count() as u32
}

fn free_bench_slots(state: &State, player: usize) -> u32 {
    state.in_play_pokemon[player][1..]
        .iter()
        .filter(|x| x.is_none())
        .count() as u32
}

fn trainer_of(action: &Action) -> Option<&deckgym::models::TrainerCard> {
    match &action.action {
        SimpleAction::Play { trainer_card } => Some(trainer_card),
        _ => None,
    }
}

/// Bot that maximises the number of target copies in play by the end of its first turn.
struct FirstTurnBot {
    deck: Deck,
    copies: u32,
    policy: Policy,
}

impl Debug for FirstTurnBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FirstTurnBot({:?})", self.policy)
    }
}

impl FirstTurnBot {
    fn find<'a>(actions: &'a [Action], pred: impl Fn(&Action) -> bool) -> Option<&'a Action> {
        actions.iter().find(|a| pred(a))
    }

    fn greedy(&self, state: &State, actions: &[Action]) -> Action {
        let me = actions[0].actor;
        let opp = (me + 1) % 2;
        // Forced choices pushed by an earlier action (e.g. where to attach a Tool): any is fine.
        if actions[0].is_stack {
            return actions[0].clone();
        }
        let in_play = targets_in_play(state, me);
        let missing = self.copies.saturating_sub(in_play);

        // 1. Put the target into play: Active if the Active Spot is open, else the Bench.
        if let Some(a) = Self::find(actions, |a| matches!(&a.action, SimpleAction::Place(c, 0) if is_target(c)))
            .or_else(|| Self::find(actions, |a| matches!(&a.action, SimpleAction::Place(c, _) if is_target(c))))
        {
            return a.clone();
        }
        let hand_targets = targets_in_hand(state, me);
        // Setup with no target in hand: any other Basic goes Active.
        if let Some(a) = Self::find(actions, |a| matches!(&a.action, SimpleAction::Place(_, 0))) {
            return a.clone();
        }

        if state.turn_count > 0 {
            // 2. Tools as early as possible (attaching one never costs a target copy).
            if let Some(a) = Self::find(actions, |a| {
                trainer_of(a).is_some_and(|t| t.trainer_card_type == TrainerType::Tool)
            }) {
                return a.clone();
            }
            // 3. Poké Ball while a copy is missing and none is in hand.
            if missing > 0 && hand_targets == 0 {
                if let Some(a) = Self::find(actions, |a| trainer_of(a).is_some_and(|t| t.name == "Poké Ball")) {
                    return a.clone();
                }
            }
        }

        // 4. Bench other Basics, keeping a Bench slot free for every missing target copy.
        if free_bench_slots(state, me) > missing {
            if let Some(a) = Self::find(actions, |a| matches!(&a.action, SimpleAction::Place(c, i) if *i != 0 && !is_target(c))) {
                return a.clone();
            }
        }

        // 5. One Supporter, whichever draws more right now. Never Copycat away a held target.
        if state.turn_count > 0 && missing > 0 {
            let research = Self::find(actions, |a| {
                trainer_of(a).is_some_and(|t| t.name == "Professor's Research")
            });
            let copycat = Self::find(actions, |a| trainer_of(a).is_some_and(|t| t.name == "Copycat"))
                .filter(|_| hand_targets == 0 && !state.hands[opp].is_empty());
            let pick = match (research, copycat) {
                (Some(r), Some(c)) => Some(if state.hands[opp].len() > 2 { c } else { r }),
                (r, c) => r.or(c),
            };
            if let Some(a) = pick {
                return a.clone();
            }
        }

        Self::find(actions, |a| matches!(a.action, SimpleAction::EndTurn))
            .unwrap_or(&actions[0])
            .clone()
    }

    fn everything(&self, actions: &[Action]) -> Action {
        if actions[0].is_stack {
            return actions[0].clone();
        }
        let order: [&dyn Fn(&Action) -> bool; 5] = [
            &|a| matches!(&a.action, SimpleAction::Place(c, _) if is_target(c)),
            &|a| matches!(&a.action, SimpleAction::Place(_, _)),
            &|a| trainer_of(a).is_some_and(|t| t.trainer_card_type == TrainerType::Tool),
            &|a| trainer_of(a).is_some_and(|t| t.name == "Poké Ball"),
            &|a| {
                trainer_of(a).is_some_and(|t| t.name == "Professor's Research" || t.name == "Copycat")
            },
        ];
        for pred in order {
            if let Some(a) = Self::find(actions, pred) {
                return a.clone();
            }
        }
        Self::find(actions, |a| matches!(a.action, SimpleAction::EndTurn))
            .unwrap_or(&actions[0])
            .clone()
    }
}

impl Player for FirstTurnBot {
    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }

    fn decision_fn(&mut self, rng: &mut StdRng, state: &State, actions: &[Action]) -> Action {
        match self.policy {
            Policy::Greedy => self.greedy(state, actions),
            Policy::Everything => self.everything(actions),
            Policy::Random => actions.choose(rng).expect("must have action").clone(),
        }
    }
}

/// Per-seat tallies. Index 0 = going first, 1 = going second.
#[derive(Default, Clone, Copy)]
struct Tally {
    n: [u64; 2],
    opening: [u64; 2],
    in1: [u64; 2],
    in2: [u64; 2],
    copycats: [u64; 2],
    copycat_draws: [u64; 2],
}

impl Tally {
    fn add(mut self, o: Tally) -> Tally {
        for s in 0..2 {
            self.n[s] += o.n[s];
            self.opening[s] += o.opening[s];
            self.in1[s] += o.in1[s];
            self.in2[s] += o.in2[s];
            self.copycats[s] += o.copycats[s];
            self.copycat_draws[s] += o.copycat_draws[s];
        }
        self
    }
}

fn game_seed(base: u64, cell: &Cell, policy: Policy, i: u64) -> u64 {
    // splitmix64 over (base, cell, policy, game index)
    let mut z = base
        ^ cell.key().wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (policy as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ i.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Plays one game with `decks.0` as the measured deck. In a mirror (`decks.1` is None) both
/// players are measured; otherwise only player 0 is.
fn play_one(
    decks: (&Deck, Option<&Deck>),
    cells: (&Cell, &Cell),
    policy: Policy,
    seed: u64,
    trace: bool,
) -> Tally {
    let mirror = decks.1.is_none();
    let opp_deck = decks.1.unwrap_or(decks.0);
    let mut rng = StdRng::seed_from_u64(seed);
    let state = State::initialize(decks.0, opp_deck, &mut rng);
    let first = state.current_player;
    let measured = |player: usize| mirror || player == 0;
    let mut tally = Tally::default();
    for player in 0..2 {
        let seat = if player == first { 0 } else { 1 };
        if measured(player) {
            tally.n[seat] = 1;
            if targets_in_hand(&state, player) > 0 {
                tally.opening[seat] = 1;
            }
        }
    }
    let players: Vec<Box<dyn Player>> = [(decks.0, cells.0), (opp_deck, cells.1)]
        .into_iter()
        .map(|(deck, cell)| {
            Box::new(FirstTurnBot {
                deck: deck.clone(),
                copies: cell.c,
                policy,
            }) as Box<dyn Player>
        })
        .collect();
    let mut game = Game::from_state(state, players, seed.wrapping_add(1));
    loop {
        let before = game.state().turn_count;
        let action = game.play_tick();
        if trace {
            let s = game.state();
            eprintln!(
                "  t{} p{} {:?} | hand={:?} | in_play={}",
                s.turn_count,
                action.actor,
                action.action,
                s.hands[action.actor].iter().map(|c| c.get_name()).collect::<Vec<_>>(),
                targets_in_play(s, action.actor)
            );
        }
        if let Some(t) = trainer_of(&action) {
            if t.name == "Copycat" && measured(action.actor) {
                let seat = if action.actor == first { 0 } else { 1 };
                tally.copycats[seat] += 1;
                tally.copycat_draws[seat] += game.state().hands[action.actor].len() as u64;
            }
        }
        let after = game.state().turn_count;
        if after != before && (before == 1 || before == 2) {
            // `before`'s player just ended their first turn.
            let seat = (before - 1) as usize;
            let player = if seat == 0 { first } else { (first + 1) % 2 };
            let n = targets_in_play(game.state(), player);
            if measured(player) && n >= 1 {
                tally.in1[seat] = 1;
            }
            if measured(player) && n >= 2 {
                tally.in2[seat] = 1;
            }
            if seat == 1 {
                break;
            }
        }
        if game.is_game_over() {
            panic!("game ended before both first turns finished (seed {seed})");
        }
    }
    tally
}

fn build_deck(cell: &Cell) -> Result<Deck, String> {
    let deck = Deck::from_string(&cell.deck_string()?)?;
    if deck.cards.len() != 20 {
        return Err(format!("deck has {} cards", deck.cards.len()));
    }
    Ok(deck)
}

fn run_cell(
    cell: &Cell,
    games: u64,
    policy: Policy,
    opponent: Opponent,
    base_seed: u64,
) -> Result<Tally, String> {
    let deck = build_deck(cell)?;
    let opp_cell = opponent.deck_cell(cell);
    let opp_deck = match opponent {
        Opponent::Mirror => None,
        _ => Some(build_deck(&opp_cell)?),
    };
    const CHUNK: u64 = 500;
    let chunks = games.div_ceil(CHUNK);
    let tally = (0..chunks)
        .into_par_iter()
        .map(|k| {
            let lo = k * CHUNK;
            let hi = (lo + CHUNK).min(games);
            (lo..hi).fold(Tally::default(), |acc, i| {
                acc.add(play_one(
                    (&deck, opp_deck.as_ref()),
                    (cell, &opp_cell),
                    policy,
                    game_seed(base_seed, cell, policy, i),
                    false,
                ))
            })
        })
        .reduce(Tally::default, Tally::add);
    Ok(tally)
}

fn parse_cell(v: &Value) -> (Cell, u64) {
    let g = |k: &str| v[k].as_u64().unwrap_or_else(|| panic!("missing {k} in {v}")) as u32;
    (
        Cell { c: g("c"), m: g("m"), p: g("p"), r: g("r"), cc: g("cc"), t: g("t") },
        v["games"].as_u64().expect("games"),
    )
}

fn done_keys(out: &str, policy: &str, opponent: &str, seed: u64) -> HashSet<(u64, u64)> {
    let mut done = HashSet::new();
    if let Ok(f) = File::open(out) {
        for line in BufReader::new(f).lines().map_while(Result::ok) {
            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                let opp = v["opponent"].as_str().unwrap_or("mirror");
                if v["policy"] == policy && opp == opponent && v["seed"] == seed {
                    let (cell, games) = parse_cell(&v);
                    done.insert((cell.key(), games));
                }
            }
        }
    }
    done
}

fn engine_commit() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn main() {
    let args = Args::parse();
    let policy = match args.policy.as_str() {
        "greedy" => Policy::Greedy,
        "random" => Policy::Random,
        "everything" => Policy::Everything,
        other => panic!("unknown policy {other}"),
    };
    let plan: Vec<(Cell, u64)> = BufReader::new(File::open(&args.plan).expect("plan file"))
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .map(|l| parse_cell(&serde_json::from_str(&l).expect("plan json")))
        .take(args.limit.unwrap_or(usize::MAX))
        .collect();
    let opponent = match (args.opp_t, &args.opp_cell) {
        (Some(t), _) => Opponent::Tools(t),
        (None, Some(spec)) => {
            let v: Vec<u32> = spec.split(',').map(|x| x.trim().parse().expect("opp_cell")).collect();
            assert_eq!(v.len(), 6, "--opp-cell takes c,m,p,r,cc,t");
            Opponent::Fixed(Cell { c: v[0], m: v[1], p: v[2], r: v[3], cc: v[4], t: v[5] })
        }
        (None, None) => Opponent::Mirror,
    };
    if let Some(n) = args.trace {
        let cell = plan[0].0;
        let opp_cell = opponent.deck_cell(&cell);
        let deck = build_deck(&cell).expect("deck");
        let opp_deck = match opponent {
            Opponent::Mirror => None,
            _ => Some(build_deck(&opp_cell).expect("deck")),
        };
        for i in 0..n {
            eprintln!("game {i} {cell:?} vs {}", opponent.label());
            let seed = game_seed(args.seed, &cell, policy, i);
            play_one((&deck, opp_deck.as_ref()), (&cell, &opp_cell), policy, seed, true);
        }
        return;
    }
    let done = done_keys(&args.out, &args.policy, &opponent.label(), args.seed);
    let commit = engine_commit();
    let mut out = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args.out)
        .expect("open out");

    let total_games: u64 = plan.iter().map(|(_, g)| g).sum();
    let mut games_done = 0u64;
    let start = Instant::now();
    for (idx, (cell, games)) in plan.iter().enumerate() {
        if done.contains(&(cell.key(), *games)) {
            games_done += games;
            continue;
        }
        let t0 = Instant::now();
        let tally = match run_cell(cell, *games, policy, opponent, args.seed) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("skip {cell:?}: {e}");
                continue;
            }
        };
        let secs = t0.elapsed().as_secs_f64();
        let seat = |s: usize| {
            json!({
                "n": tally.n[s],
                "opening": tally.opening[s], "in1": tally.in1[s], "in2": tally.in2[s],
                "copycats": tally.copycats[s], "copycatDraws": tally.copycat_draws[s],
            })
        };
        let line = json!({
            "c": cell.c, "m": cell.m, "p": cell.p, "r": cell.r, "cc": cell.cc, "t": cell.t,
            "games": games, "policy": args.policy, "opponent": opponent.label(),
            "seed": args.seed, "commit": commit,
            "first": seat(0), "second": seat(1), "secs": secs,
        });
        writeln!(out, "{line}").expect("write");
        out.flush().expect("flush");
        games_done += games;
        let elapsed = start.elapsed().as_secs_f64();
        eprintln!(
            "[{}/{}] {:?} {:.0} games/s  first={:.4} second={:.4}  ({:.0}s elapsed)",
            idx + 1,
            plan.len(),
            cell,
            *games as f64 / secs,
            tally.in1[0] as f64 / tally.n[0].max(1) as f64,
            tally.in1[1] as f64 / tally.n[1].max(1) as f64,
            elapsed
        );
    }
    eprintln!("done: {games_done}/{total_games} games in {:.0}s", start.elapsed().as_secs_f64());
}

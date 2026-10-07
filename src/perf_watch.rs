//! Opt-in detection of combinatorial blowups in the engine.
//!
//! Some effects enumerate possibilities: every outcome of a random effect (so search
//! players can weigh them) or every legal choice ("discard any 2 cards"). When the count
//! depends on the game state, for example one random hit per attached Energy, a rare
//! board can make a single action take minutes or never finish. This module reports
//! such operations with the full state so they can be reproduced and fixed.
//!
//! Disabled by default; then each watched operation costs one atomic load. Enable it with
//! the `DECKGYM_WATCH_DIR` environment variable (incidents are written there), tuned by
//! `DECKGYM_WATCH_SLOW_MS` (default 500), `DECKGYM_WATCH_BRANCHES` (default 1000) and
//! `DECKGYM_WATCH_RUNNING_SECS` (default 10), or call [`enable`].
//!
//! Each incident is one JSON file: the operation, why it was reported (`slow`,
//! `branches`, or `running` for an operation still going past the running deadline,
//! written by a monitor thread so operations that never finish are captured too), the
//! elapsed time, the branch count, and the action and state before the operation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::ThreadId;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::actions::Action;
use crate::State;

/// Thresholds and output folder for watched operations.
#[derive(Clone, Debug)]
pub struct WatchConfig {
    pub dir: PathBuf,
    /// Report a finished operation that took at least this long.
    pub slow: Duration,
    /// Report an operation that produced more than this many outcomes or legal actions.
    pub max_branches: usize,
    /// Report an operation still running after this long (once, while it runs).
    pub running: Duration,
}

impl WatchConfig {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        WatchConfig {
            dir: dir.into(),
            slow: Duration::from_millis(500),
            max_branches: 1000,
            running: Duration::from_secs(10),
        }
    }

    fn from_env() -> Option<Self> {
        let dir = std::env::var_os("DECKGYM_WATCH_DIR")?;
        let mut config = WatchConfig::new(dir);
        let number = |name: &str| std::env::var(name).ok().and_then(|v| v.parse::<u64>().ok());
        if let Some(ms) = number("DECKGYM_WATCH_SLOW_MS") {
            config.slow = Duration::from_millis(ms);
        }
        if let Some(branches) = number("DECKGYM_WATCH_BRANCHES") {
            config.max_branches = branches as usize;
        }
        if let Some(secs) = number("DECKGYM_WATCH_RUNNING_SECS") {
            config.running = Duration::from_secs(secs);
        }
        Some(config)
    }
}

static ENABLED: AtomicBool = AtomicBool::new(false);
static ENV_CHECKED: AtomicBool = AtomicBool::new(false);
static CONFIG: Mutex<Option<WatchConfig>> = Mutex::new(None);
static RUNNING: OnceLock<Mutex<HashMap<ThreadId, Running>>> = OnceLock::new();
static MONITOR: OnceLock<()> = OnceLock::new();
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

thread_local! {
    static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

struct Running {
    op: &'static str,
    started: Instant,
    state: State,
    action: Option<Action>,
    reported: bool,
}

/// Turn watching on (overrides the environment) and start the monitor thread.
pub fn enable(config: WatchConfig) {
    std::fs::create_dir_all(&config.dir).ok();
    *CONFIG.lock().unwrap() = Some(config);
    ENV_CHECKED.store(true, Ordering::SeqCst);
    ENABLED.store(true, Ordering::SeqCst);
    start_monitor();
}

/// Turn watching off. Operations already running finish unreported.
pub fn disable() {
    ENABLED.store(false, Ordering::SeqCst);
}

fn enabled() -> bool {
    if !ENV_CHECKED.load(Ordering::Relaxed) {
        ENV_CHECKED.store(true, Ordering::SeqCst);
        if let Some(config) = WatchConfig::from_env() {
            enable(config);
        }
    }
    ENABLED.load(Ordering::Relaxed)
}

fn config() -> Option<WatchConfig> {
    CONFIG.lock().unwrap().clone()
}

fn running() -> &'static Mutex<HashMap<ThreadId, Running>> {
    RUNNING.get_or_init(|| Mutex::new(HashMap::new()))
}

fn start_monitor() {
    MONITOR.get_or_init(|| {
        std::thread::Builder::new()
            .name("deckgym-perf-watch".into())
            .spawn(|| loop {
                std::thread::sleep(Duration::from_millis(250));
                if !ENABLED.load(Ordering::Relaxed) {
                    continue;
                }
                let Some(config) = config() else { continue };
                let mut overdue = Vec::new();
                for entry in running().lock().unwrap().values_mut() {
                    if !entry.reported && entry.started.elapsed() >= config.running {
                        entry.reported = true;
                        overdue.push((
                            entry.op,
                            entry.started.elapsed(),
                            entry.state.clone(),
                            entry.action.clone(),
                        ));
                    }
                }
                for (op, elapsed, state, action) in overdue {
                    write_incident(
                        &config,
                        op,
                        "running",
                        elapsed,
                        None,
                        &state,
                        action.as_ref(),
                    );
                }
            })
            .ok();
    });
}

/// Guard for one watched operation; reports it when dropped if it was slow or too wide.
pub struct Watch {
    inner: Option<WatchInner>,
}

struct WatchInner {
    op: &'static str,
    started: Instant,
    outermost: bool,
    branches: Option<usize>,
    snapshot: Option<(State, Option<Action>)>,
}

/// Start watching an operation on `state` (and `action`). Keep the guard alive for the
/// operation's duration and call [`Watch::branches`] with the size of its result.
pub fn watch(op: &'static str, state: &State, action: Option<&Action>) -> Watch {
    if !enabled() {
        return Watch { inner: None };
    }
    let outermost = DEPTH.with(|d| {
        let depth = d.get();
        d.set(depth + 1);
        depth == 0
    });
    let started = Instant::now();
    let snapshot = (state.clone(), action.cloned());
    if outermost {
        running().lock().unwrap().insert(
            std::thread::current().id(),
            Running {
                op,
                started,
                state: snapshot.0.clone(),
                action: snapshot.1.clone(),
                reported: false,
            },
        );
    }
    Watch {
        inner: Some(WatchInner {
            op,
            started,
            outermost,
            branches: None,
            snapshot: Some(snapshot),
        }),
    }
}

impl Watch {
    /// Record how many outcomes or legal actions the operation produced.
    pub fn branches(&mut self, count: usize) {
        if let Some(inner) = &mut self.inner {
            inner.branches = Some(count);
        }
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        let Some(mut inner) = self.inner.take() else {
            return;
        };
        DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
        let elapsed = inner.started.elapsed();
        let already_reported = if inner.outermost {
            running()
                .lock()
                .unwrap()
                .remove(&std::thread::current().id())
                .is_some_and(|r| r.reported)
        } else {
            false
        };
        let Some(config) = config() else { return };
        let too_wide = inner.branches.is_some_and(|b| b > config.max_branches);
        let too_slow = inner.outermost && elapsed >= config.slow;
        // A slow operation the monitor already reported as running is not reported twice.
        if !(too_wide || (too_slow && !already_reported)) {
            return;
        }
        let kind = if too_wide { "branches" } else { "slow" };
        let (state, action) = inner.snapshot.take().unwrap();
        write_incident(
            &config,
            inner.op,
            kind,
            elapsed,
            inner.branches,
            &state,
            action.as_ref(),
        );
    }
}

fn write_incident(
    config: &WatchConfig,
    op: &str,
    kind: &str,
    elapsed: Duration,
    branches: Option<usize>,
    state: &State,
    action: Option<&Action>,
) {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let path = config.dir.join(format!(
        "{stamp}-{}-{sequence:08}-{op}-{kind}.json",
        std::process::id()
    ));
    let incident = serde_json::json!({
        "op": op,
        "kind": kind,
        "elapsed_ms": elapsed.as_millis() as u64,
        "branches": branches,
        "action": action,
        "action_debug": action.map(|a| format!("{a:?}")),
        "state": state,
    });
    if let Ok(bytes) = serde_json::to_vec(&incident) {
        let _ = std::fs::write(&path, bytes);
    }
}

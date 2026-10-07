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

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock, Weak};
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
static CONFIG: RwLock<Option<Arc<WatchConfig>>> = RwLock::new(None);
/// Every thread's slot, so the monitor can see operations still running.
static SLOTS: Mutex<Vec<Weak<Mutex<Option<Running>>>>> = Mutex::new(Vec::new());
static MONITOR: OnceLock<()> = OnceLock::new();
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

type Snapshot = Arc<(State, Option<Action>)>;

thread_local! {
    static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// This thread's outermost running operation. Only the owning thread and the
    /// monitor lock it, so watching scales across threads.
    static SLOT: Arc<Mutex<Option<Running>>> = {
        let slot = Arc::new(Mutex::new(None));
        SLOTS.lock().unwrap().push(Arc::downgrade(&slot));
        slot
    };
}

struct Running {
    op: &'static str,
    started: Instant,
    snapshot: Snapshot,
    reported: bool,
}

/// Turn watching on (overrides the environment) and start the monitor thread.
pub fn enable(config: WatchConfig) {
    std::fs::create_dir_all(&config.dir).ok();
    *CONFIG.write().unwrap() = Some(Arc::new(config));
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

fn config() -> Option<Arc<WatchConfig>> {
    CONFIG.read().unwrap().clone()
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
                let slots: Vec<_> = {
                    let mut slots = SLOTS.lock().unwrap();
                    slots.retain(|slot| slot.strong_count() > 0);
                    slots.iter().filter_map(Weak::upgrade).collect()
                };
                for slot in slots {
                    let overdue = {
                        let mut running = slot.lock().unwrap();
                        match running.as_mut() {
                            Some(r) if !r.reported && r.started.elapsed() >= config.running => {
                                r.reported = true;
                                Some((r.op, r.started.elapsed(), r.snapshot.clone()))
                            }
                            _ => None,
                        }
                    };
                    if let Some((op, elapsed, snapshot)) = overdue {
                        write_incident(&config, op, "running", elapsed, None, &snapshot);
                    }
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
    snapshot: Snapshot,
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
    // A nested operation (the forecast inside an apply) runs on the same state as the
    // operation around it, so it reuses that snapshot instead of copying the state again.
    let snapshot = SLOT.with(|slot| {
        let mut running = slot.lock().unwrap();
        match running.as_ref() {
            Some(outer) if !outermost => outer.snapshot.clone(),
            _ => {
                let snapshot: Snapshot = Arc::new((state.clone(), action.cloned()));
                *running = Some(Running {
                    op,
                    started,
                    snapshot: snapshot.clone(),
                    reported: false,
                });
                snapshot
            }
        }
    });
    Watch {
        inner: Some(WatchInner {
            op,
            started,
            outermost,
            branches: None,
            snapshot,
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
        let Some(inner) = self.inner.take() else {
            return;
        };
        DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
        let elapsed = inner.started.elapsed();
        let already_reported = inner.outermost
            && SLOT.with(|slot| slot.lock().unwrap().take().is_some_and(|r| r.reported));
        let Some(config) = config() else { return };
        let too_wide = inner.branches.is_some_and(|b| b > config.max_branches);
        let too_slow = inner.outermost && elapsed >= config.slow;
        // A slow operation the monitor already reported as running is not reported twice.
        if !(too_wide || (too_slow && !already_reported)) {
            return;
        }
        let kind = if too_wide { "branches" } else { "slow" };
        write_incident(
            &config,
            inner.op,
            kind,
            elapsed,
            inner.branches,
            &inner.snapshot,
        );
    }
}

fn write_incident(
    config: &WatchConfig,
    op: &str,
    kind: &str,
    elapsed: Duration,
    branches: Option<usize>,
    snapshot: &Snapshot,
) {
    let (state, action) = (&snapshot.0, snapshot.1.as_ref());
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

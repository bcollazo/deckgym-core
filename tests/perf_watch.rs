//! perf_watch is process-wide configuration, so this runs in its own test binary: other
//! engine tests would add incidents while watching is enabled.
use std::time::{Duration, Instant};

use deckgym::perf_watch::{disable, enable, watch, WatchConfig};
use deckgym::State;

fn incidents(dir: &std::path::Path) -> Vec<serde_json::Value> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .map(|entries| entries.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    files.sort();
    files
        .iter()
        .map(|path| serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap())
        .collect()
}

#[test]
fn reports_slow_wide_and_still_running_operations_only_when_enabled() {
    let dir = std::env::temp_dir().join(format!("deckgym-perf-watch-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let state = State::default();

    // Disabled (no environment variable in tests): nothing is recorded.
    disable();
    {
        let mut w = watch("forecast_action", &state, None);
        w.branches(1_000_000);
    }
    assert!(incidents(&dir).is_empty());

    let mut config = WatchConfig::new(&dir);
    config.slow = Duration::from_millis(30);
    config.max_branches = 10;
    config.running = Duration::from_millis(300);
    enable(config);

    // Fast and narrow: not reported.
    watch("generate_possible_actions", &state, None).branches(3);
    assert!(incidents(&dir).is_empty());

    // Too many branches.
    watch("forecast_action", &state, None).branches(11);
    // Slow.
    {
        let _w = watch("apply_action", &state, None);
        std::thread::sleep(Duration::from_millis(40));
    }
    let found = incidents(&dir);
    assert_eq!(found.len(), 2);
    assert_eq!(
        (found[0]["kind"].as_str(), found[0]["branches"].as_u64()),
        (Some("branches"), Some(11))
    );
    assert_eq!(
        (found[1]["op"].as_str(), found[1]["kind"].as_str()),
        (Some("apply_action"), Some("slow"))
    );
    assert!(found[1]["state"].is_object());

    // Still running past the deadline: reported by the monitor before it finishes,
    // and not again when it finishes.
    {
        let _w = watch("apply_action", &state, None);
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline && !incidents(&dir).iter().any(|i| i["kind"] == "running") {
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    let found = incidents(&dir);
    assert_eq!(found.iter().filter(|i| i["kind"] == "running").count(), 1);
    assert_eq!(found.len(), 3);

    // Nested operations: only the outermost is timed, but any level can be too wide.
    {
        let _outer = watch("apply_action", &state, None);
        watch("forecast_action", &state, None).branches(50);
    }
    let found = incidents(&dir);
    assert_eq!(found.len(), 4);
    assert_eq!(found[3]["op"].as_str(), Some("forecast_action"));

    disable();
    let _ = std::fs::remove_dir_all(&dir);
}

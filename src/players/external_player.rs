//! A `Player` that delegates every decision to an external process over JSON lines on
//! stdin/stdout. See `docs/bot-protocol.md` for the wire protocol this implements.

use std::{
    fmt,
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::Duration,
};

use log::warn;
use rand::rngs::StdRng;
use serde_json::{json, Value};
use uuid::Uuid;

use super::Player;
use crate::{actions::Action, replay::ViewState, Deck, State};

const PROTOCOL_VERSION: u32 = 1;

struct SpawnedBot {
    child: Child,
    stdin: ChildStdin,
    stdout_lines: Receiver<String>,
    /// The name the bot reported in its `hello` reply, if any (used for `Debug`/replay display).
    name: Option<String>,
    agent_snapshot: bool,
}

/// Runs a command once (lazily, on the first decision) and keeps talking to it for the lifetime of
/// this `ExternalPlayer` — in practice, one game (see the "Deviations" note in
/// `docs/replay-viewer-plan.md`: the CLI constructs a fresh set of `Player`s per game, so one
/// `ExternalPlayer` instance's lifetime already matches one game's). On any protocol failure
/// (spawn failure, timeout, invalid JSON, an out-of-range index, or the process dying) this logs a
/// warning and falls back to the first legal action rather than panicking the simulation.
pub struct ExternalPlayer {
    deck: Deck,
    command: String,
    timeout: Duration,
    bot: Option<SpawnedBot>,
    ply: u32,
    last_note: Option<String>,
}

impl ExternalPlayer {
    pub fn new(deck: Deck, command: String, timeout: Duration) -> Self {
        Self {
            deck,
            command,
            timeout,
            bot: None,
            ply: 0,
            last_note: None,
        }
    }

    fn ensure_spawned(&mut self, you: usize) -> Result<(), String> {
        if self.bot.is_some() {
            return Ok(());
        }

        #[cfg(windows)]
        let mut command = {
            use std::os::windows::process::CommandExt;
            let mut command = Command::new("cmd");
            command.args(["/D", "/S", "/C"]).creation_flags(0x08000000);
            command
        };
        #[cfg(not(windows))]
        let mut command = {
            let mut command = Command::new("sh");
            command.arg("-c");
            command
        };
        let mut child = command
            .arg(&self.command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // Bot stderr is passed through for debugging rather than captured.
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("failed to spawn bot command {:?}: {e}", self.command))?;

        let mut stdin = child.stdin.take().expect("piped stdin");
        let stdout = child.stdout.take().expect("piped stdout");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // EOF: the bot process exited.
                    Ok(_) => {
                        if tx.send(line).is_err() {
                            break; // Receiver dropped (ExternalPlayer was dropped).
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        write_line(
            &mut stdin,
            &json!({"type": "hello", "protocol": PROTOCOL_VERSION}),
        )
        .map_err(|e| format!("failed writing hello to bot stdin: {e}"))?;
        let hello_reply = rx
            .recv_timeout(self.timeout)
            .map_err(|_| "bot did not reply to hello in time".to_string())?;
        let name = serde_json::from_str::<Value>(&hello_reply)
            .ok()
            .and_then(|v| v.get("name").and_then(Value::as_str).map(str::to_string));

        let agent_snapshot = serde_json::from_str::<Value>(&hello_reply)
            .ok()
            .and_then(|v| v.get("agent_snapshot").and_then(Value::as_bool))
            .unwrap_or(false);
        let deck_ids: Vec<String> = self.deck.cards.iter().map(|c| c.get_id()).collect();
        write_line(
            &mut stdin,
            &json!({
                "type": "new_game",
                "game_id": Uuid::new_v4().to_string(),
                "you": you,
                "deck": deck_ids,
                "deck_definition": if agent_snapshot { Some(&self.deck) } else { None },
            }),
        )
        .map_err(|e| format!("failed writing new_game to bot stdin: {e}"))?;

        self.bot = Some(SpawnedBot {
            child,
            stdin,
            stdout_lines: rx,
            agent_snapshot,
            name,
        });
        Ok(())
    }

    /// Sends a `decide` message and waits (up to `self.timeout`) for the bot's reply. Returns the
    /// chosen index and optional note, or an error describing why it couldn't get one.
    fn ask_bot(
        &mut self,
        actor: usize,
        state: &State,
        possible_actions: &[Action],
    ) -> Result<(usize, Option<String>), String> {
        self.ply += 1;
        let bot = self.bot.as_mut().expect("ensure_spawned called first");

        let view = ViewState::for_player(state, actor);
        let actions: Vec<Value> = possible_actions
            .iter()
            .enumerate()
            .map(|(i, a)| json!({"i": i, "text": a.action.to_string(), "action": a.action}))
            .collect();
        write_line(
            &mut bot.stdin,
            &json!({"type": "decide", "ply": self.ply, "state": view, "actions": actions,
                "actor": actor,
                "agent_snapshot": if bot.agent_snapshot { Some(state.agent_snapshot(actor, possible_actions)) } else { None },
                "legal_actions": if bot.agent_snapshot { Some(possible_actions) } else { None },
            }),
        )
        .map_err(|e| format!("failed writing decide to bot stdin: {e}"))?;

        let reply = bot
            .stdout_lines
            .recv_timeout(self.timeout)
            .map_err(|e| match e {
                RecvTimeoutError::Timeout => "bot timed out".to_string(),
                RecvTimeoutError::Disconnected => "bot process exited unexpectedly".to_string(),
            })?;
        let value: Value =
            serde_json::from_str(&reply).map_err(|e| format!("invalid JSON from bot: {e}"))?;
        let i = value
            .get("i")
            .and_then(Value::as_u64)
            .ok_or("bot reply is missing a numeric 'i' field")? as usize;
        let note = value
            .get("note")
            .and_then(Value::as_str)
            .map(str::to_string);
        Ok((i, note))
    }
}

fn write_line(stdin: &mut impl Write, value: &Value) -> std::io::Result<()> {
    // Serialize before writing: formatting a Value directly into an unbuffered
    // pipe can issue a system call for every JSON token.
    let mut line = serde_json::to_vec(value)?;
    line.push(b'\n');
    stdin.write_all(&line)?;
    stdin.flush()
}

impl Player for ExternalPlayer {
    fn get_deck(&self) -> Deck {
        self.deck.clone()
    }

    fn decision_fn(
        &mut self,
        _rng: &mut StdRng,
        state: &State,
        possible_actions: &[Action],
    ) -> Action {
        self.last_note = None;
        // Every element of `possible_actions` is for the same decision, so shares one actor.
        let actor = possible_actions
            .first()
            .map(|a| a.actor)
            .expect("decision_fn is only called with at least one possible action");

        let outcome = self
            .ensure_spawned(actor)
            .and_then(|()| self.ask_bot(actor, state, possible_actions));

        match outcome {
            Ok((i, note)) if i < possible_actions.len() => {
                self.last_note = note;
                possible_actions[i].clone()
            }
            Ok((i, _)) => {
                let msg = format!(
                    "chose out-of-range index {i} (only {} legal actions)",
                    possible_actions.len()
                );
                warn!(
                    "ExternalPlayer({}): {msg}; falling back to the first legal action",
                    self.command
                );
                self.last_note = Some(format!("[bot error: {msg}]"));
                possible_actions[0].clone()
            }
            Err(e) => {
                warn!(
                    "ExternalPlayer({}): {e}; falling back to the first legal action",
                    self.command
                );
                self.last_note = Some(format!("[bot error: {e}]"));
                possible_actions[0].clone()
            }
        }
    }

    fn last_note(&self) -> Option<String> {
        self.last_note.clone()
    }
}

impl fmt::Debug for ExternalPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.bot.as_ref().and_then(|b| b.name.clone()) {
            Some(name) => write!(f, "ExternalPlayer({name})"),
            None => write!(f, "ExternalPlayer({})", self.command),
        }
    }
}

impl Drop for ExternalPlayer {
    fn drop(&mut self) {
        if let Some(mut bot) = self.bot.take() {
            // Close stdin first so a well-behaved bot sees EOF and can exit on its own; then make
            // sure it's actually gone so simulations don't leak subprocesses.
            drop(bot.stdin);
            let _ = bot.child.kill();
            let _ = bot.child.wait();
        }
    }
}

#[cfg(test)]
mod pipe_write_tests {
    use super::*;
    #[derive(Default)]
    struct CountingWriter {
        bytes: Vec<u8>,
        writes: usize,
        flushes: usize,
    }
    impl Write for CountingWriter {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.writes += 1;
            self.bytes.extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }
    #[test]
    fn protocol_message_is_one_buffered_json_line() {
        let message = json!({"type":"decide", "text":"line one\nline two", "values":[1,2,3]});
        let mut output = CountingWriter::default();
        write_line(&mut output, &message).unwrap();
        assert_eq!(output.writes, 1);
        assert_eq!(output.flushes, 1);
        assert_eq!(output.bytes.iter().filter(|&&b| b == b'\n').count(), 1);
        assert_eq!(
            serde_json::from_slice::<Value>(&output.bytes).unwrap(),
            message
        );
    }
}

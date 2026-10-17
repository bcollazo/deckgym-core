#!/usr/bin/env python3
"""A minimal deckgym external bot: picks a uniformly random legal action every turn.

Stdlib-only (no dependencies), so it runs anywhere `python3` runs. See
docs/bot-protocol.md for the wire protocol this implements.

Usage (from the repo root):
    cargo run simulate example_decks/venusaur-exeggutor.txt example_decks/weezing-arbok.txt \
        -n 1 --players r,x --bot-b "python3 examples/bots/random_bot.py"
"""

import json
import random
import sys


def send(message: dict) -> None:
    """Writes one JSON line to stdout and flushes it immediately (the engine reads line by line
    and blocks waiting for a reply, so buffered-but-unflushed output would hang it)."""
    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


def log(message: str) -> None:
    """Bot stderr is passed through by the engine, so this is safe to use for debugging."""
    print(message, file=sys.stderr)


def main() -> None:
    rng = random.Random()
    you = None

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        message = json.loads(line)
        msg_type = message.get("type")

        if msg_type == "hello":
            send({"type": "hello", "name": "random_bot.py"})
        elif msg_type == "new_game":
            you = message["you"]
            log(f"new_game: playing as player {you}, deck has {len(message['deck'])} cards")
        elif msg_type == "decide":
            actions = message["actions"]
            choice = rng.choice(actions)
            # `scores` is optional: one number per action, in order. The viewer shows them next to
            # each option (probabilities are shown as percentages).
            scores = [1.0 / len(actions)] * len(actions)
            send({"i": choice["i"], "note": f"random pick: {choice['text']}", "scores": scores})
        else:
            log(f"unrecognized message type: {msg_type!r}")


if __name__ == "__main__":
    try:
        main()
    except (EOFError, KeyboardInterrupt, BrokenPipeError):
        pass

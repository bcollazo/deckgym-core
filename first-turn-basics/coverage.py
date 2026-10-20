"""Real-deck coverage for the first-turn Basic availability grid.

Pulls the top archetypes from DeckGym's public meta analytics API (Limitless
tournament data), expands every exact deck variant weighted by its deck count,
and reports which (deck, Basic species) pairs fall inside the simulation grid.

Usage (from deckgym-core root):
    python first-turn-basics/coverage.py [--period MONTHLY] [--limit 100] [--refresh]
"""

import argparse
import json
import os
import urllib.parse
import urllib.request
from collections import Counter

API = "https://www.deckgym.com/api/analytics"
HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, "data")

GRID = {
    "c": range(1, 3),
    "m": range(0, 13),
    "p": range(0, 3),
    "r": range(0, 3),
    "cc": range(0, 3),
    "t": range(0, 5),
}


def fetch_json(url, cache_name, refresh):
    path = os.path.join(CACHE, cache_name)
    if os.path.exists(path) and not refresh:
        with open(path, encoding="utf-8") as f:
            return json.load(f)
    with urllib.request.urlopen(url, timeout=120) as resp:
        data = json.load(resp)
    os.makedirs(CACHE, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f)
    return data


def load_cards():
    with open(os.path.join(HERE, "..", "database.json"), encoding="utf-8") as f:
        db = json.load(f)
    cards = {}
    for entry in db:
        (kind, card), = entry.items()
        cards[card["id"]] = (kind, card)
    return cards


def classify(card_id, cards):
    kind, card = cards[card_id]
    name = card["name"]
    if kind == "Pokemon":
        return ("basic" if card["stage"] == 0 else "pokemon"), name
    ttype = card["trainer_card_type"]
    if name == "Poké Ball":
        return "p", name
    if name == "Professor's Research":
        return "r", name
    if name == "Copycat":
        return "cc", name
    if ttype == "Tool":
        return "t", name
    return "trainer", name


def deck_tuples(card_ids, cards):
    """Yields (c, m, p, r, cc, t, species) for each Basic species in the deck."""
    counts = Counter()
    species = Counter()
    trainers = Counter()
    for cid in card_ids:
        cls, name = classify(cid, cards)
        counts[cls] += 1
        if cls == "basic":
            species[name] += 1
        elif cls == "trainer":
            trainers[name] += 1
    total_basics = counts["basic"]
    for name, c in species.items():
        yield (c, total_basics - c, counts["p"], counts["r"], counts["cc"], counts["t"], name), trainers


def in_grid(key):
    c, m, p, r, cc, t, _ = key
    return (
        c in GRID["c"] and m in GRID["m"] and p in GRID["p"] and r in GRID["r"]
        and cc in GRID["cc"] and t in GRID["t"]
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--period", default="MONTHLY")
    ap.add_argument("--format", default="STANDARD")
    ap.add_argument("--limit", type=int, default=100)
    ap.add_argument("--refresh", action="store_true")
    ap.add_argument("--out", default=os.path.join(HERE, "coverage.json"))
    args = ap.parse_args()

    cards = load_cards()
    q = urllib.parse.urlencode(
        {"periodType": args.period, "format": args.format, "limit": args.limit}
    )
    share = fetch_json(f"{API}/meta-share?{q}", f"meta-share-{args.period}.json", args.refresh)
    archetypes = share["archetypes"]

    pair_weight = Counter()  # cell key (without species) -> weighted (deck, species) pairs
    pair_total = 0
    pairs_in = 0
    deck_total = 0
    decks_covered_fully = 0
    trainer_decks = Counter()  # trainer name -> decks containing it
    dims = {k: Counter() for k in ["c", "m", "p", "r", "cc", "t"]}
    total_decks_reported = 0
    for arch in archetypes:
        aid = arch["archetypeId"]
        q = urllib.parse.urlencode(
            {"archetypeId": aid, "periodType": args.period, "format": args.format}
        )
        safe = "".join(ch if ch.isalnum() else "_" for ch in aid)
        detail = fetch_json(
            f"{API}/meta-deck-detail?{q}", f"detail-{args.period}-{safe}.json", args.refresh
        )
        total_decks_reported += detail["totalDecks"]
        for variant in detail["topVariants"]:
            w = variant["deckCount"]
            ids = variant["cardIds"]
            if len(ids) != 20:
                continue
            deck_total += w
            all_in = True
            trainers_seen = None
            for key, trainers in deck_tuples(ids, cards):
                trainers_seen = trainers
                pair_total += w
                cell = key[:6]
                pair_weight[cell] += w
                for k, v in zip(["c", "m", "p", "r", "cc", "t"], cell):
                    dims[k][v] += w
                if in_grid(key):
                    pairs_in += w
                else:
                    all_in = False
            if all_in:
                decks_covered_fully += w
            for name in (trainers_seen or {}):
                trainer_decks[name] += w

    report = {
        "period": args.period,
        "format": args.format,
        "archetypes": len(archetypes),
        "decks_in_variants": deck_total,
        "decks_reported_by_archetypes": total_decks_reported,
        "pairs": pair_total,
        "pairs_in_grid": pairs_in,
        "pair_coverage": pairs_in / pair_total,
        "deck_coverage": decks_covered_fully / deck_total,
        "dims": {k: dict(sorted(v.items())) for k, v in dims.items()},
        "top_other_trainers": [
            {"name": n, "decks": w, "share": w / deck_total}
            for n, w in trainer_decks.most_common(25)
        ],
        "cells": [
            {"c": k[0], "m": k[1], "p": k[2], "r": k[3], "cc": k[4], "t": k[5], "weight": w}
            for k, w in pair_weight.most_common()
        ],
    }
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=1)

    print(f"archetypes={len(archetypes)} decks(variants)={deck_total} "
          f"decks(reported)={total_decks_reported} pairs={pair_total}")
    print(f"pair coverage={report['pair_coverage']:.4f} deck coverage={report['deck_coverage']:.4f}")
    for k, v in report["dims"].items():
        print(k, v)
    print("distinct cells used:", len(pair_weight))
    print("top other trainers:")
    for row in report["top_other_trainers"]:
        print(f"  {row['name']:<28} {row['share']:.3f}")


if __name__ == "__main__":
    main()

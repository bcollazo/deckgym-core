"""Writes the simulation plan (one JSON cell per line).

Every legal cell of the grid gets BASE_GAMES; cells that hold >= HOT_SHARE of the real-deck
(deck, Basic species) pairs in coverage.json get HOT_GAMES for a tighter interval.

Usage: python first-turn-basics/make_plan.py > first-turn-basics/plan.jsonl
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BASE_GAMES = 40_000
HOT_GAMES = 160_000
HOT_SHARE = 0.005

with open(os.path.join(HERE, "coverage.json"), encoding="utf-8") as f:
    coverage = json.load(f)
total = coverage["pairs"]
hot = {
    (x["c"], x["m"], x["p"], x["r"], x["cc"], x["t"])
    for x in coverage["cells"]
    if x["weight"] / total >= HOT_SHARE
}

rows = []
for c in (1, 2):
    for m in range(13):
        for p in range(3):
            for r in range(3):
                for cc in range(3):
                    for t in range(5):
                        if c + m + p + r + cc + t > 20:
                            continue
                        key = (c, m, p, r, cc, t)
                        games = HOT_GAMES if key in hot else BASE_GAMES
                        rows.append(dict(c=c, m=m, p=p, r=r, cc=cc, t=t, games=games))
# Hot cells first so the most useful data lands early.
rows.sort(key=lambda x: -x["games"])
for row in rows:
    sys.stdout.write(json.dumps(row) + "\n")
print(f"{len(rows)} cells, {sum(x['games'] for x in rows):,} games, {len(hot)} hot", file=sys.stderr)

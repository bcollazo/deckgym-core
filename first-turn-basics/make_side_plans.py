"""Plans for the side experiments (Tools isolation, policy and opponent sensitivity).

Usage: python first-turn-basics/make_side_plans.py
Writes plan_tools.jsonl, plan_sensitivity.jsonl and plan_opponent.jsonl next to this script.
"""

import json
import os
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "coverage.json"), encoding="utf-8") as f:
    coverage = json.load(f)


def write(name, rows):
    with open(os.path.join(HERE, name), "w", encoding="utf-8") as f:
        for row in rows:
            f.write(json.dumps(row) + "\n")
    print(f"{name}: {len(rows)} cells, {sum(r['games'] for r in rows):,} games")


# Most common (c, m, p, r, cc) families in real decks, ignoring t.
families = Counter()
for x in coverage["cells"]:
    if x["c"] <= 2 and x["m"] <= 12 and x["p"] <= 2 and x["r"] <= 2 and x["cc"] <= 2:
        families[(x["c"], x["m"], x["p"], x["r"], x["cc"])] += x["weight"]

# Tools isolation: opponent keeps 0 Tools while ours vary. Non-mirror games measure one
# player, so 80k games give ~40k samples per seat.
tools = []
for (c, m, p, r, cc), _ in [kv for kv in families.most_common() if kv[0][4] > 0][:30]:
    for t in range(5):
        if c + m + p + r + cc + t <= 20:
            tools.append(dict(c=c, m=m, p=p, r=r, cc=cc, t=t, games=80_000))
write("plan_tools.jsonl", tools)

# Policy sensitivity: 10 representative cells.
sens = [
    (2, 4, 2, 2, 0, 0), (2, 4, 2, 2, 2, 0), (2, 3, 2, 2, 1, 1), (1, 4, 2, 2, 2, 2),
    (1, 6, 2, 2, 0, 0), (2, 0, 0, 0, 0, 0), (2, 8, 1, 1, 1, 0), (1, 2, 0, 2, 2, 0),
    (2, 6, 2, 2, 2, 4), (1, 10, 2, 2, 2, 0),
]
write("plan_sensitivity.jsonl",
      [dict(c=c, m=m, p=p, r=r, cc=cc, t=t, games=40_000) for c, m, p, r, cc, t in sens])

# Opponent sensitivity: the common real-deck cells against one fixed typical opponent.
total = coverage["pairs"]
hot = [x for x in coverage["cells"] if x["weight"] / total >= 0.005 and x["t"] <= 4]
write("plan_opponent.jsonl",
      [dict(c=x["c"], m=x["m"], p=x["p"], r=x["r"], cc=x["cc"], t=x["t"], games=80_000)
       for x in hot])

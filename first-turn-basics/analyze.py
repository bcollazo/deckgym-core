"""Validates the simulation results and builds the shipped lookup table.

Usage (from deckgym-core root):
    python first-turn-basics/analyze.py [--results first-turn-basics/results.jsonl]

Writes table.json, table.csv and analysis.json next to this script, and prints the
validation / Tools / sensitivity report used in METHODS.md.
"""

import argparse
import csv
import datetime
import json
import math
import os
from collections import defaultdict
from math import comb

HERE = os.path.dirname(os.path.abspath(__file__))
Z = 1.959964
SEED = 20261020


def wilson_half(k, n):
    if n == 0:
        return None
    p = k / n
    denom = 1 + Z * Z / n
    return Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n)) / denom


def load(path, policy="greedy"):
    rows = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            v = json.loads(line)
            if v["policy"] != policy or v["seed"] != SEED:
                continue
            key = (v["c"], v["m"], v["p"], v["r"], v["cc"], v["t"])
            rows[key] = v  # last write wins
    return rows


# ---------- exact engine opening-hand model: 1 random Basic + 4 of the other 19 ----------
def exact_opening(c, m):
    b = c + m
    return c / b + (m / b) * (1 - comb(19 - c, 4) / comb(19, 4))


def exact_top6(c, m):
    """P(>= 1 target among the opening 5 + the turn draw), no search/draw cards."""
    b = c + m
    return c / b + (m / b) * (1 - comb(19 - c, 5) / comb(19, 5))


def exact_top6_both(m):
    """c = 2: P(both targets among the first 6 cards)."""
    b = 2 + m
    return (2 / b) * (5 / 19) + (m / b) * (comb(17, 3) / comb(19, 5))


def exact_conditional_uniform_opening(c, m):
    """Alternative model: uniform over 5-card hands that contain >= 1 Basic."""
    b = c + m
    with_basic = comb(20, 5) - comb(20 - b, 5)
    with_target = comb(20, 5) - comb(20 - c, 5)
    return with_target / with_basic


def seat_rates(v, seat):
    s = v[seat]
    n = v["games"]
    return {
        "n": n,
        "opening": s["opening"] / n,
        "in1": s["in1"] / n,
        "in2": s["in2"] / n,
        "k_open": s["opening"],
        "k1": s["in1"],
        "k2": s["in2"],
    }


def zdiff(k_a, n_a, k_b, n_b):
    pa, pb = k_a / n_a, k_b / n_b
    pooled = (k_a + k_b) / (n_a + n_b)
    se = math.sqrt(max(pooled * (1 - pooled), 1e-12) * (1 / n_a + 1 / n_b))
    return (pa - pb) / se, pa - pb


def validate_opening(rows):
    out = []
    worst = 0
    for key, v in sorted(rows.items()):
        c, m, p, r, cc, t = key
        if (p, r, cc, t) != (0, 0, 0, 0):
            continue
        n = 2 * v["games"]
        k = v["first"]["opening"] + v["second"]["opening"]
        exact = exact_opening(c, m)
        exact6 = exact_top6(c, m)
        k6 = v["first"]["in1"] + v["second"]["in1"]
        z_open = (k / n - exact) / math.sqrt(exact * (1 - exact) / n) if 0 < exact < 1 else 0
        z6 = (k6 / n - exact6) / math.sqrt(exact6 * (1 - exact6) / n) if 0 < exact6 < 1 else 0
        row = {
            "c": c, "m": m, "sim_opening": k / n, "exact_opening": exact, "z_opening": z_open,
            "sim_inPlay1": k6 / n, "exact_inPlay1": exact6, "z_inPlay1": z6,
            "conditional_uniform_opening": exact_conditional_uniform_opening(c, m),
        }
        if c == 2:
            e2 = exact_top6_both(m)
            k2 = v["first"]["in2"] + v["second"]["in2"]
            row.update(sim_inPlay2=k2 / n, exact_inPlay2=e2,
                       z_inPlay2=(k2 / n - e2) / math.sqrt(e2 * (1 - e2) / n))
        worst = max(worst, abs(z_open), abs(z6), abs(row.get("z_inPlay2", 0)))
        out.append(row)
    return out, worst


def tools_effect(rows):
    """For each (c,m,p,r,cc) compare every t>0 against t=0, per seat and metric."""
    groups = defaultdict(lambda: {"diffs": [], "z": []})
    for key, v in rows.items():
        c, m, p, r, cc, t = key
        if t == 0:
            continue
        base = rows.get((c, m, p, r, cc, 0))
        if base is None:
            continue
        for seat in ("first", "second"):
            metrics = [("in1", "in1")] + ([("in2", "in2")] if c == 2 else [])
            for name, field in metrics:
                z, d = zdiff(v[seat][field], v["games"], base[seat][field], base["games"])
                g = groups[(cc > 0, seat, name, t)]
                g["diffs"].append(d)
                g["z"].append(z)
    summary = []
    for (has_cc, seat, metric, t), g in sorted(groups.items()):
        zs = g["z"]
        n = len(zs)
        summary.append({
            "cc>0": has_cc, "seat": seat, "metric": metric, "t": t, "cells": n,
            "mean_diff_pp": 100 * sum(g["diffs"]) / n,
            "max_abs_diff_pp": 100 * max(abs(x) for x in g["diffs"]),
            # Stouffer combination: under "no effect" this is ~N(0,1).
            "stouffer_z": sum(zs) / math.sqrt(n),
            "share_|z|>2": sum(abs(x) > 2 for x in zs) / n,
        })
    return summary


def monotonicity(rows):
    """Count neighbour pairs that move the wrong way by more than 2 / 3 standard errors."""
    checks = {"p": 2, "r": 3, "cc": 4, "m": 1}
    report = {}
    examples = []
    for dim, idx in checks.items():
        total = v2 = v3 = 0
        for key, v in rows.items():
            nxt = list(key)
            nxt[idx] += 1
            w = rows.get(tuple(nxt))
            if w is None:
                continue
            for seat in ("first", "second"):
                for field in ("in1",) + (("in2",) if key[0] == 2 else ()):
                    z, d = zdiff(w[seat][field], w["games"], v[seat][field], v["games"])
                    if dim == "m":
                        z, d = -z, -d  # more other Basics should not help
                    total += 1
                    if z < -2:
                        v2 += 1
                    if z < -3:
                        v3 += 1
                        examples.append({"dim": dim, "from": key, "seat": seat, "metric": field,
                                         "diff_pp": 100 * d, "z": z})
        report[dim] = {"comparisons": total, "violations_2se": v2, "violations_3se": v3,
                       "expected_2se_by_chance": round(total * 0.02275, 1)}
    return report, sorted(examples, key=lambda x: x["z"])[:20]


def zeraora(rows):
    blog = [100, 93.9, 88.2, 87.5, 83.8, 80.5, 77.7]
    out = []
    for i, m in enumerate(range(0, 7)):
        row = {"basics": m + 2, "blog_pct": blog[i]}
        for t in (0, 4):
            v = rows.get((2, m, 2, 2, 0, t))
            if v:
                f, s = seat_rates(v, "first"), seat_rates(v, "second")
                row[f"t{t}_first_pct"] = 100 * f["in1"]
                row[f"t{t}_second_pct"] = 100 * s["in1"]
                row[f"t{t}_avg_pct"] = 50 * (f["in1"] + s["in1"])
        out.append(row)
    return out


def table_cells(rows, drop_t):
    """When t is dropped, ship the t = 0 cells. Pooling over t would mix in the mirror-match
    artifact (the opponent's own Tools shrink the hand our Copycat copies)."""
    return {k: (v["games"], v["first"], v["second"])
            for k, v in rows.items() if not drop_t or k[5] == 0}


def build_table(rows, drop_t, commit):
    cells = []
    for key, (n, f, s) in sorted(table_cells(rows, drop_t).items()):
        c, m, p, r, cc, t = key

        def seat(x):
            d = {"opening": round(x["opening"] / n, 3), "inPlay1": round(x["in1"] / n, 3)}
            if c == 2:
                d["inPlay2"] = round(x["in2"] / n, 3)
            d["ci"] = round(wilson_half(x["in1"], n), 4)
            return d

        avg_k = {x: f[x] + s[x] for x in ("opening", "in1", "in2")}
        pf, ps = f["in1"] / n, s["in1"] / n
        avg = {"opening": round(avg_k["opening"] / (2 * n), 3),
               "inPlay1": round(avg_k["in1"] / (2 * n), 3)}
        if c == 2:
            avg["inPlay2"] = round(avg_k["in2"] / (2 * n), 3)
        avg["ci"] = round(Z * math.sqrt((pf * (1 - pf) + ps * (1 - ps)) / (4 * n)), 4)
        cell = {"c": c, "m": m, "p": p, "r": r, "cc": cc}
        if not drop_t:
            cell["t"] = t
        cell.update(games=n, first=seat(f), second=seat(s), avg=avg)
        cells.append(cell)

    grid = {"c": [1, 2], "m": list(range(13)), "p": [0, 1, 2], "r": [0, 1, 2], "cc": [0, 1, 2]}
    if not drop_t:
        grid["t"] = [0, 1, 2, 3, 4]
    meta = {
        "engine_commit": commit,
        "engine_repo": "https://github.com/bcollazo/deckgym-core (branch first-turn-basics)",
        "seed": SEED,
        "date": datetime.date.today().isoformat(),
        "policy": "greedy (see METHODS.md)",
        "assumptions": [
            "20-card deck; target species with c copies, m other Basic Pokemon cards, p Poke Ball, "
            "r Professor's Research, cc Copycat; every other card is inert for turn 1.",
            "Opening hand follows deckgym-core: 1 uniformly random Basic card + 4 uniformly from "
            "the other 19 cards; no mulligans.",
            "Both players draw 1 card at the start of their first turn (deckgym-core queues a draw "
            "for the first player on turn 1 as well).",
            "Mirror match: the opponent plays the same deck and policy. Copycat draws as many cards "
            "as the opponent's hand at that moment, so cc > 0 values depend on this assumption.",
            "inPlay counts the target as Active or Benched at the end of the player's own first "
            "turn (turn 1 going first, turn 2 going second). Bench limit 3.",
            "Fossils are not Basic Pokemon in the engine and are not counted in m.",
        ] + (["Tools (t) are not a key: values are the t = 0 cells. Own Tools played before "
              "Copycat add only +0.1..+0.3 pp (see METHODS.md); look up any deck by ignoring t."]
             if drop_t else []),
    }
    return {"meta": meta, "grid": grid, "cells": cells}


def write_csv(table, path):
    has_t = "t" in table["grid"]
    head = ["c", "m", "p", "r", "cc"] + (["t"] if has_t else []) + ["games"]
    for seat in ("first", "second", "avg"):
        head += [f"{seat}_opening", f"{seat}_inPlay1", f"{seat}_inPlay2", f"{seat}_ci"]
    with open(path, "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(head)
        for cell in table["cells"]:
            row = [cell[k] for k in head[: len(head) - 12]]
            for seat in ("first", "second", "avg"):
                s = cell[seat]
                row += [s["opening"], s["inPlay1"], s.get("inPlay2", ""), s["ci"]]
            w.writerow(row)


def sensitivity(path):
    by_policy = {p: load(path, p) for p in ("greedy", "random", "everything")}
    out = []
    for key in sorted(by_policy["random"]):
        row = {"cell": dict(zip(["c", "m", "p", "r", "cc", "t"], key))}
        for pol, rows in by_policy.items():
            v = rows.get(key)
            if v:
                row[pol] = {
                    "first": round(v["first"]["in1"] / v["games"], 4),
                    "second": round(v["second"]["in1"] / v["games"], 4),
                }
        out.append(row)
    return out


def load_side(path, opponent):
    rows = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            v = json.loads(line)
            if v.get("opponent") == opponent and v["seed"] == SEED:
                rows[(v["c"], v["m"], v["p"], v["r"], v["cc"], v["t"])] = v
    return rows


def tools_isolated(side):
    """Our Tools vary while the opponent keeps 0 Tools: the own-side effect alone."""
    rows = load_side(side, "opp_t=0")
    acc = defaultdict(lambda: {"d": [], "z": []})
    for key, v in rows.items():
        base = rows.get(key[:5] + (0,))
        if key[5] == 0 or base is None:
            continue
        for seat in ("first", "second"):
            z, d = zdiff(v[seat]["in1"], v[seat]["n"], base[seat]["in1"], base[seat]["n"])
            acc[(seat, key[5])]["d"].append(d)
            acc[(seat, key[5])]["z"].append(z)
    return [{"seat": s, "t": t, "cells": len(g["z"]),
             "mean_diff_pp": 100 * sum(g["d"]) / len(g["d"]),
             "stouffer_z": sum(g["z"]) / math.sqrt(len(g["z"]))}
            for (s, t), g in sorted(acc.items())]


def opponent_sensitivity(side, mirror_rows, label="opp=2,4,2,2,1,1"):
    rows = load_side(side, label)
    out = []
    for key, v in sorted(rows.items()):
        m = mirror_rows.get(key)
        if not m:
            continue
        row = {"cell": key}
        for seat in ("first", "second"):
            row[seat] = {
                "mirror": m[seat]["in1"] / m["games"],
                "fixed": v[seat]["in1"] / v[seat]["n"],
            }
        out.append(row)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--results", default=os.path.join(HERE, "results.jsonl"))
    ap.add_argument("--sensitivity", default=os.path.join(HERE, "sensitivity.jsonl"))
    ap.add_argument("--keep-t", action="store_true", help="ship t as a table dimension")
    args = ap.parse_args()

    rows = load(args.results)
    commit = next(iter(rows.values()))["commit"]
    print(f"cells loaded: {len(rows)}  games: {sum(v['games'] for v in rows.values()):,}")

    opening, worst = validate_opening(rows)
    print(f"\n[1] opening/top-6 vs exact enumeration: worst |z| = {worst:.2f}")
    for r in opening:
        line = (f"  c={r['c']} m={r['m']:>2} open sim {r['sim_opening']:.4f} exact "
                f"{r['exact_opening']:.4f} (cond-uniform {r['conditional_uniform_opening']:.4f}) | "
                f"in1 sim {r['sim_inPlay1']:.4f} exact {r['exact_inPlay1']:.4f}")
        if "exact_inPlay2" in r:
            line += f" | in2 sim {r['sim_inPlay2']:.4f} exact {r['exact_inPlay2']:.4f}"
        print(line)

    z = zeraora(rows)
    print("\n[2] Zeraora comparison (c=2, p=2, r=2, cc=0)")
    for r in z:
        print("  ", {k: (round(v, 1) if isinstance(v, float) else v) for k, v in r.items()})

    mono, mono_examples = monotonicity(rows)
    print("\n[3] monotonicity:", json.dumps(mono, indent=1))
    for e in mono_examples[:8]:
        print("  ", e)

    tools = tools_effect(rows)
    print("\n[T] Tools effect (t vs t=0), Stouffer z across cells:")
    for r in tools:
        print(f"  cc>0={r['cc>0']!s:<5} {r['seat']:<6} {r['metric']} t={r['t']} cells={r['cells']:>4} "
              f"mean={r['mean_diff_pp']:+.3f}pp max|d|={r['max_abs_diff_pp']:.2f}pp "
              f"z={r['stouffer_z']:+.2f} |z|>2: {r['share_|z|>2']:.3f}")

    side = os.path.join(HERE, "side.jsonl")
    iso, opp = [], []
    if os.path.exists(side):
        iso = tools_isolated(side)
        print("\n[T2] Tools with the opponent fixed at 0 Tools (own-side effect only):")
        for r in iso:
            print(f"  {r['seat']:<6} t={r['t']} cells={r['cells']:>3} "
                  f"mean={r['mean_diff_pp']:+.3f}pp z={r['stouffer_z']:+.2f}")
        opp = opponent_sensitivity(side, rows)
        print("\n[O] mirror vs fixed opponent (2,4,2,2,1,1), inPlay1, pp difference fixed - mirror:")
        for r in opp:
            d = [100 * (r[s]["fixed"] - r[s]["mirror"]) for s in ("first", "second")]
            print(f"  {r['cell']}  first {r['first']['mirror']:.3f} {d[0]:+.2f}pp   "
                  f"second {r['second']['mirror']:.3f} {d[1]:+.2f}pp")

    sens = sensitivity(args.sensitivity) if os.path.exists(args.sensitivity) else []
    if sens:
        print("\n[S] policy sensitivity (inPlay1)")
        for r in sens:
            print("  ", r)

    table = build_table(rows, drop_t=not args.keep_t, commit=commit)
    with open(os.path.join(HERE, "table.json"), "w", encoding="utf-8") as f:
        json.dump(table, f, separators=(",", ":"))
    write_csv(table, os.path.join(HERE, "table.csv"))
    size = os.path.getsize(os.path.join(HERE, "table.json"))
    print(f"\ntable.json: {len(table['cells'])} cells, {size / 1024:.1f} KB")

    with open(os.path.join(HERE, "analysis.json"), "w", encoding="utf-8") as f:
        json.dump({"opening": opening, "zeraora": z, "monotonicity": mono,
                   "monotonicity_worst": mono_examples, "tools": tools,
                   "tools_isolated": iso, "opponent_sensitivity": opp, "sensitivity": sens},
                  f, indent=1, default=str)


if __name__ == "__main__":
    main()

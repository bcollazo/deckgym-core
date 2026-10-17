import { describe, expect, it } from "vitest";
import { describeScores } from "./scores";

describe("describeScores", () => {
  it("shows probabilities as percentages and orders by preference", () => {
    const view = describeScores([0.05, 0.9, 0.05], 3);
    expect(view?.probabilities).toBe(true);
    expect(view?.entries.map((e) => e.label)).toEqual(["5%", "90%", "5%"]);
    expect(view?.entries[1].fraction).toBeCloseTo(0.9);
    expect(view?.order).toEqual([1, 0, 2]);
  });

  it("rounds tiny probabilities to <1% and zero to 0%", () => {
    const view = describeScores([0.999, 0.001, 0], 3);
    expect(view?.entries.map((e) => e.label)).toEqual(["100%", "<1%", "0%"]);
  });

  it("shows other values raw with a bar scaled to their range", () => {
    const view = describeScores([-1, 3, 1], 3);
    expect(view?.probabilities).toBe(false);
    expect(view?.entries.map((e) => e.label)).toEqual(["-1.00", "3.00", "1.00"]);
    expect(view?.entries.map((e) => e.fraction)).toEqual([0, 1, 0.5]);
    expect(view?.order).toEqual([1, 2, 0]);
  });

  it("gives equal scores a full bar and keeps their original order", () => {
    const view = describeScores([2, 2, 2], 3);
    expect(view?.entries.map((e) => e.fraction)).toEqual([1, 1, 1]);
    expect(view?.order).toEqual([0, 1, 2]);
  });

  it("ignores missing, mismatched or non-finite scores", () => {
    expect(describeScores(undefined, 2)).toBeNull();
    expect(describeScores(null, 2)).toBeNull();
    expect(describeScores([0.5], 2)).toBeNull();
    expect(describeScores([0.5, Number.NaN], 2)).toBeNull();
    expect(describeScores([], 0)).toBeNull();
  });
});

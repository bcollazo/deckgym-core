/** Display model for the optional per-option `scores` a bot attaches to a decision. */
export interface ScoreView {
  /** Whether the values are probabilities (non-negative, summing to 1) and shown as percentages. */
  probabilities: boolean;
  /** One entry per option, in the original order. */
  entries: { label: string; fraction: number }[];
  /** Option indices from most to least preferred (ties keep their original order). */
  order: number[];
}

const PROBABILITY_TOLERANCE = 0.01;

/** Returns null when there are no usable scores (absent, wrong length, or non-finite). */
export function describeScores(scores: number[] | null | undefined, optionCount: number): ScoreView | null {
  if (!scores || scores.length !== optionCount || optionCount === 0) return null;
  if (!scores.every((s) => Number.isFinite(s))) return null;

  const sum = scores.reduce((a, b) => a + b, 0);
  const probabilities = scores.every((s) => s >= 0) && Math.abs(sum - 1) <= PROBABILITY_TOLERANCE;
  const min = Math.min(...scores);
  const max = Math.max(...scores);

  const entries = scores.map((s) => {
    if (probabilities) return { label: percentLabel(s), fraction: s };
    return { label: s.toFixed(2), fraction: max === min ? 1 : (s - min) / (max - min) };
  });
  const order = scores.map((_, i) => i).sort((a, b) => scores[b] - scores[a] || a - b);
  return { probabilities, entries, order };
}

function percentLabel(p: number): string {
  if (p <= 0) return "0%";
  const pct = p * 100;
  return pct < 1 ? "<1%" : `${Math.round(pct)}%`;
}

import { formatAction } from "../anim/formatAction";
import type { Replay } from "../types/replay";
import { describeScores } from "./scores";

interface OptionsPanelProps {
  replay: Replay;
  currentIndex: number;
}

/** Every legal action available at the *current* rest-snapshot (i.e. the step about to be taken,
 * `steps[currentIndex]`), with the one that was actually chosen highlighted. When the acting bot
 * supplied `scores`, each option shows its score (a percentage for probabilities) with a bar, and
 * the list is ordered from most to least preferred. Empty once the replay reaches its final
 * snapshot (there's no next step). */
export function OptionsPanel({ replay, currentIndex }: OptionsPanelProps) {
  const step = replay.steps[currentIndex];
  const scoreView = step ? describeScores(step.scores, step.options.length) : null;
  const order = scoreView?.order ?? step?.options.map((_, i) => i) ?? [];

  return (
    <div className="sidebar-section">
      <h2>Options at this step</h2>
      <div className="options-list">
        {!step && <div className="option-row">Game over — no further actions.</div>}
        {step &&
          order.map((i) => {
            const option = step.options[i];
            const entry = scoreView?.entries[i];
            return (
              <div key={i} className={`option-row${i === step.chosen ? " chosen" : ""}${entry ? " scored" : ""}`}>
                {entry && <span className="score-bar" style={{ width: `${Math.round(entry.fraction * 100)}%` }} />}
                <span className="dot" />
                <span className="option-text">
                  {formatAction(option.action, option.text, { cards: replay.cards, beforeState: step.state, actor: step.actor })}
                </span>
                {entry && <span className="score-label">{entry.label}</span>}
              </div>
            );
          })}
      </div>
      {step?.note && <div className="bot-note">"{step.note}"</div>}
    </div>
  );
}

import type { Replay } from "../types/replay";

interface OptionsPanelProps {
  replay: Replay;
  currentIndex: number;
}

/** Every legal action available at the *current* rest-snapshot (i.e. the step about to be taken,
 * `steps[currentIndex]`), with the one that was actually chosen highlighted. Empty once the replay
 * reaches its final snapshot (there's no next step). */
export function OptionsPanel({ replay, currentIndex }: OptionsPanelProps) {
  const step = replay.steps[currentIndex];

  return (
    <div className="sidebar-section" style={{ flex: "0 1 40%" }}>
      <h2>Options at this step</h2>
      <div className="options-list">
        {!step && <div className="option-row">Game over — no further actions.</div>}
        {step?.options.map((option, i) => (
          <div key={i} className={`option-row${i === step.chosen ? " chosen" : ""}`}>
            <span className="dot" />
            <span>{option.text}</span>
          </div>
        ))}
      </div>
      {step?.note && <div className="bot-note">"{step.note}"</div>}
    </div>
  );
}

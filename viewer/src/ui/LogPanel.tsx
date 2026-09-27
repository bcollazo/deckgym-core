import { useEffect, useRef } from "react";
import { formatAction } from "../anim/formatAction";
import type { Replay } from "../types/replay";

interface LogPanelProps {
  replay: Replay;
  currentIndex: number;
  onSeek: (index: number) => void;
}

/** One row per decision point ("step"): who acted and what they chose. Clicking a row seeks the
 * board to the rest-snapshot right after that action (i.e. index = ply + 1). */
export function LogPanel({ replay, currentIndex, onSeek }: LogPanelProps) {
  const currentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    currentRef.current?.scrollIntoView({ block: "nearest" });
  }, [currentIndex]);

  return (
    <div className="sidebar-section">
      <h2>Log</h2>
      <div className="sidebar-list">
        {replay.steps.map((step, i) => {
          const isCurrent = i + 1 === currentIndex;
          const playerName = replay.players[step.actor]?.name ?? `Player ${step.actor}`;
          const chosen = step.options[step.chosen];
          const chosenText = chosen
            ? formatAction(chosen.action, chosen.text, { cards: replay.cards, beforeState: step.state, actor: step.actor })
            : "?";
          return (
            <div
              key={step.ply}
              ref={isCurrent ? currentRef : undefined}
              className={`log-row player-${step.actor}${isCurrent ? " current" : ""}`}
              onClick={() => onSeek(i + 1)}
            >
              <span className="ply">T{step.turn}</span>
              <span className="who">{playerName}</span>
              <span>{chosenText}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

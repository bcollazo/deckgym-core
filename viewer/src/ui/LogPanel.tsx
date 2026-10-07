import { useEffect, useRef } from "react";
import { formatAction } from "../anim/formatAction";
import type { Replay } from "../types/replay";

interface LogPanelProps {
  replay: Replay;
  currentIndex: number;
  onSeek: (index: number) => void;
}

/** One row per decision point ("step") taken so far: who acted and what they chose. The log grows
 * as playback advances and stays pinned to the latest action. Clicking a row seeks the board to
 * the rest-snapshot right after that action (i.e. index = ply + 1). */
export function LogPanel({ replay, currentIndex, onSeek }: LogPanelProps) {
  const listRef = useRef<HTMLDivElement>(null);

  // Scroll only the list itself (scrollIntoView would also scroll the page), so the latest action
  // is always the visible bottom row.
  useEffect(() => {
    const list = listRef.current;
    if (list) list.scrollTop = list.scrollHeight;
  }, [currentIndex]);

  return (
    <div className="sidebar-section">
      <h2>Log</h2>
      <div className="sidebar-list" ref={listRef}>
        {replay.steps.slice(0, currentIndex).map((step, i) => {
          const isCurrent = i + 1 === currentIndex;
          const playerName = replay.players[step.actor]?.name ?? `Player ${step.actor}`;
          const chosen = step.options[step.chosen];
          const chosenText = chosen
            ? formatAction(chosen.action, chosen.text, { cards: replay.cards, beforeState: step.state, actor: step.actor })
            : "?";
          return (
            <div
              key={step.ply}
              className={`log-row player-${step.actor}${isCurrent ? " current" : ""}`}
              onClick={() => onSeek(i + 1)}
            >
              <span className="ply">T{step.turn}</span>
              <span className="who" title={playerName}>
                P{step.actor}
              </span>
              <span className="log-text">{chosenText}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

import { useMemo, useRef } from "react";
import { snapshotAt, snapshotCount, type Replay } from "../types/replay";
import type { PlaybackController } from "../store/playbackController";

interface ControlsProps {
  replay: Replay;
  controller: PlaybackController;
  index: number;
  playing: boolean;
  animating: boolean;
  speed: number;
}

const SPEEDS = [0.5, 1, 2, 4];

export function Controls({ replay, controller, index, playing, animating, speed }: ControlsProps) {
  const trackRef = useRef<HTMLDivElement>(null);
  const total = snapshotCount(replay);

  const { turnTicks, koTicks } = useMemo(() => {
    const turns: number[] = [];
    const kos: number[] = [];
    let lastTurn = snapshotAt(replay, 0).turn;
    let lastPoints = snapshotAt(replay, 0).points;
    for (let i = 1; i < total; i++) {
      const snap = snapshotAt(replay, i);
      if (snap.turn !== lastTurn) {
        turns.push(i);
        lastTurn = snap.turn;
      }
      if (snap.points[0] > lastPoints[0] || snap.points[1] > lastPoints[1]) {
        kos.push(i);
      }
      lastPoints = snap.points;
    }
    return { turnTicks: turns, koTicks: kos };
  }, [replay, total]);

  const current = snapshotAt(replay, index);
  const fraction = total > 1 ? index / (total - 1) : 0;

  const seekFromClientX = (clientX: number) => {
    const track = trackRef.current;
    if (!track) return;
    const rect = track.getBoundingClientRect();
    const ratio = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
    controller.jumpTo(Math.round(ratio * (total - 1)));
  };

  return (
    <div className="controls">
      <div className="controls-row">
        <button className="icon-btn" title="Start (Home)" onClick={() => controller.jumpTo(0)} disabled={index === 0}>
          {"⏮"}
        </button>
        <button
          className="icon-btn"
          title="Step back (←)"
          onClick={() => controller.stepBackward()}
          disabled={!controller.canStepBackward() || animating}
        >
          {"◀"}
        </button>
        <button
          className={`icon-btn${playing ? " playing" : ""}`}
          title="Play/Pause (Space)"
          onClick={() => controller.togglePlay()}
        >
          {playing ? "⏸" : "▶"}
        </button>
        <button
          className="icon-btn"
          title="Step forward (→)"
          onClick={() => controller.stepForward()}
          disabled={!controller.canStepForward() || animating}
        >
          {"▶"}
        </button>
        <button
          className="icon-btn"
          title="End (End)"
          onClick={() => controller.jumpTo(total - 1)}
          disabled={index === total - 1}
        >
          {"⏭"}
        </button>

        <select
          className="speed-select"
          value={speed}
          onChange={(e) => controller.setSpeed(Number(e.target.value))}
        >
          {SPEEDS.map((s) => (
            <option key={s} value={s}>
              {s}x
            </option>
          ))}
        </select>

        <div className="scrubber-wrap">
          <div
            ref={trackRef}
            className="scrubber-track"
            onPointerDown={(e) => {
              (e.target as HTMLElement).setPointerCapture(e.pointerId);
              seekFromClientX(e.clientX);
            }}
            onPointerMove={(e) => {
              if (e.buttons === 1) seekFromClientX(e.clientX);
            }}
          >
            <div className="scrubber-fill" style={{ width: `${fraction * 100}%` }} />
            {turnTicks.map((t) => (
              <div key={`t${t}`} className="scrubber-tick" style={{ left: `${(t / (total - 1)) * 100}%` }} />
            ))}
            {koTicks.map((t) => (
              <div key={`k${t}`} className="scrubber-tick ko" style={{ left: `${(t / (total - 1)) * 100}%` }} />
            ))}
            <div className="scrubber-thumb" style={{ left: `${fraction * 100}%` }} />
          </div>
        </div>

        <div className="ply-label">
          ply {index}/{total - 1} &middot; turn {current.turn}
        </div>
      </div>
    </div>
  );
}

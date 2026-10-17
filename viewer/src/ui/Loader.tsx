import { useCallback, useRef, useState } from "react";
import { loadReplayFromFile, ReplayLoadError } from "../store/loadReplay";
import type { Replay } from "../types/replay";

interface LoaderProps {
  onLoaded: (replay: Replay) => void;
  onError: (message: string) => void;
  error: string | null;
}

export function Loader({ onLoaded, onError, error }: LoaderProps) {
  const [dragActive, setDragActive] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFile = useCallback(
    async (file: File) => {
      try {
        onLoaded(await loadReplayFromFile(file));
      } catch (e) {
        onError(e instanceof ReplayLoadError ? e.message : `Couldn't read ${file.name}.`);
      }
    },
    [onLoaded, onError],
  );

  return (
    <div className="loader-overlay">
      <div
        className={`loader-card${dragActive ? " drag-active" : ""}`}
        onDragOver={(e) => {
          e.preventDefault();
          setDragActive(true);
        }}
        onDragLeave={() => setDragActive(false)}
        onDrop={(e) => {
          e.preventDefault();
          setDragActive(false);
          const file = e.dataTransfer.files[0];
          if (file) void handleFile(file);
        }}
      >
        <h2>Load a replay</h2>
        <p>Drag a deckgym replay .json file here, or pick one from disk.</p>
        <button type="button" onClick={() => inputRef.current?.click()}>
          Choose file…
        </button>
        <input
          ref={inputRef}
          type="file"
          accept="application/json,.json"
          style={{ display: "none" }}
          onChange={(e) => {
            const file = e.target.files?.[0];
            if (file) void handleFile(file);
            e.target.value = "";
          }}
        />
        {error && <p className="loader-error">{error}</p>}
      </div>
    </div>
  );
}

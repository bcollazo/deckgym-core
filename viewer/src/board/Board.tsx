import { Application } from "@pixi/react";
import { useRef } from "react";
import { BOARD_HEIGHT, BOARD_WIDTH } from "./layout";
import { createBoardScene, type BoardScene } from "./scene";

interface BoardProps {
  onReady: (scene: BoardScene) => void;
}

/**
 * Hosts the Pixi canvas via `@pixi/react`'s `<Application>`, then builds and manages the entire
 * scene graph imperatively (see `scene.ts`) rather than declaring it as nested `@pixi/react` JSX.
 * The board is a persistent, GSAP-driven scene that outlives React re-renders; the declarative
 * reconciler is a poor fit for "mutate these objects' properties over time" and fights GSAP for
 * ownership of node state, so this uses `@pixi/react` for what it's good at (mounting/sizing/
 * disposing the canvas within React's lifecycle) and drives everything inside it directly.
 */
export function Board({ onReady }: BoardProps) {
  const sceneRef = useRef<BoardScene | null>(null);

  return (
    <div className="board-canvas-wrap">
      <Application
        width={BOARD_WIDTH}
        height={BOARD_HEIGHT}
        background={0x0e1018}
        resolution={Math.min(window.devicePixelRatio || 1, 2)}
        autoDensity
        antialias
        onInit={(app) => {
          if (sceneRef.current) return;
          const scene = createBoardScene(app);
          sceneRef.current = scene;
          onReady(scene);
        }}
      />
    </div>
  );
}

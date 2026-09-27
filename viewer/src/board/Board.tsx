import { Application } from "@pixi/react";
import { useRef } from "react";
import type { CardImageStore } from "./cardImages";
import { BOARD_HEIGHT, BOARD_WIDTH } from "./layout";
import { createBoardScene, type BoardScene } from "./scene";

interface BoardProps {
  onReady: (scene: BoardScene) => void;
  images: CardImageStore;
}

/**
 * Hosts the Pixi canvas via `@pixi/react`'s `<Application>`, then builds and manages the entire
 * scene graph imperatively (see `scene.ts`) rather than declaring it as nested `@pixi/react` JSX.
 * The board is a persistent, GSAP-driven scene that outlives React re-renders; the declarative
 * reconciler is a poor fit for "mutate these objects' properties over time" and fights GSAP for
 * ownership of node state, so this uses `@pixi/react` for what it's good at (mounting/sizing/
 * disposing the canvas within React's lifecycle) and drives everything inside it directly.
 */
export function Board({ onReady, images }: BoardProps) {
  const sceneRef = useRef<BoardScene | null>(null);

  return (
    <div className="board-canvas-wrap">
      {/* `autoDensity` (below) makes Pixi set the canvas's CSS width/height as fixed inline pixel
       * values (`style.width = "640px"`), which beats any external stylesheet rule short of
       * `!important` and defeats a plain `width:auto`/`max-width` responsive rule: if the wrap's
       * available width and height don't happen to be in the board's own ratio, one axis gets
       * clamped by max-width/max-height without the other following, stretching everything inside
       * non-uniformly (this was the real cause of Round 3's "images look stretched" bug — see the
       * plan doc). Fixing it needs a two-level box: this frame carries the *correct* aspect ratio
       * (computed from the real BOARD_WIDTH/BOARD_HEIGHT, so it can't drift out of sync) and is
       * itself constrained to fit the wrap; the canvas then fills 100% of the already-correctly-
       * shaped frame via a `!important` stylesheet rule (index.css), which is the only thing that
       * reliably wins against Pixi's inline style. */}
      <div className="board-canvas-frame" style={{ aspectRatio: `${BOARD_WIDTH} / ${BOARD_HEIGHT}` }}>
        <Application
          width={BOARD_WIDTH}
          height={BOARD_HEIGHT}
          background={0x07090d}
          resolution={Math.min(window.devicePixelRatio || 1, 2)}
          autoDensity
          antialias
          onInit={(app) => {
            if (sceneRef.current) return;
            const scene = createBoardScene(app, images);
            sceneRef.current = scene;
            onReady(scene);
          }}
        />
      </div>
    </div>
  );
}

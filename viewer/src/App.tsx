import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { Group, Panel, Separator, useDefaultLayout } from "react-resizable-panels";
import { Board } from "./board/Board";
import { createCardImageStore, preloadCardImages, resolveCardImageConfig } from "./board/cardImages";
import type { BoardScene } from "./board/scene";
import { PlaybackController } from "./store/playbackController";
import { loadReplayFromUrl } from "./store/loadReplay";
import type { Replay } from "./types/replay";
import { Loader } from "./ui/Loader";
import { LogPanel } from "./ui/LogPanel";
import { OptionsPanel } from "./ui/OptionsPanel";
import { Controls } from "./ui/Controls";
import { SettingsPopover } from "./ui/SettingsPopover";

const NO_CONTROLLER_SUBSCRIBE = () => () => {};

/** Panel sizes persist in localStorage when it's available; the viewer works the same without it. */
const layoutStorage = {
  getItem(key: string): string | null {
    try {
      return window.localStorage.getItem(key);
    } catch {
      return null;
    }
  },
  setItem(key: string, value: string): void {
    try {
      window.localStorage.setItem(key, value);
    } catch {
      // Not persisting is fine.
    }
  },
};

const NARROW_QUERY = "(max-width: 900px)";

/** True on narrow screens, where the board and the sidebar stack instead of sitting side by side. */
function useIsNarrow(): boolean {
  return useSyncExternalStore(
    (onChange) => {
      const query = window.matchMedia(NARROW_QUERY);
      query.addEventListener("change", onChange);
      return () => query.removeEventListener("change", onChange);
    },
    () => window.matchMedia(NARROW_QUERY).matches,
    () => false,
  );
}

function useControllerState(controller: PlaybackController | null) {
  return useSyncExternalStore(
    useCallback((onChange) => (controller ? controller.subscribe(onChange) : NO_CONTROLLER_SUBSCRIBE()), [controller]),
    () => (controller ? controller.getSnapshot() : null),
  );
}

export default function App() {
  const [replay, setReplay] = useState<Replay | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [controller, setController] = useState<PlaybackController | null>(null);
  // State, not a ref: the replay fetch and Pixi's async canvas init race each other on load, and
  // the controller effect below must re-run when whichever of the two finishes last arrives.
  const [scene, setScene] = useState<BoardScene | null>(null);
  const triedUrlParam = useRef(false);
  const narrow = useIsNarrow();
  const mainLayout = useDefaultLayout({ id: `viewer-main-${narrow ? "stacked" : "side"}`, storage: layoutStorage });
  const sidebarLayout = useDefaultLayout({ id: "viewer-sidebar", storage: layoutStorage });
  // Resolved once at startup (URL params > localStorage > VITE_CARD_IMAGE_URL); changing it in the
  // settings popover saves and reloads the page rather than reconfiguring this live (see
  // board/cardImages.ts).
  const images = useMemo(() => createCardImageStore(resolveCardImageConfig()), []);

  const state = useControllerState(controller);

  const handleReady = useCallback((s: BoardScene) => setScene(s), []);

  // Build (or rebuild) the PlaybackController once both a replay and the scene exist, in either order.
  useEffect(() => {
    if (!replay || !scene) return;
    const c = new PlaybackController(scene, replay);
    setController(c);
    return () => c.destroy();
  }, [replay, scene]);

  // Kick off loading every card image the replay could show, up front, so most are ready before
  // the viewer ever needs to draw them (a no-op when images are off).
  useEffect(() => {
    if (!replay) return;
    preloadCardImages(images, Object.keys(replay.cards));
  }, [replay, images]);

  // `?url=<replay.json>` support, falling back to the bundled sample replay so the app demos out
  // of the box. Runs once; a file dropped/picked before either resolves just wins the race.
  useEffect(() => {
    if (triedUrlParam.current) return;
    triedUrlParam.current = true;
    const explicitUrl = new URLSearchParams(window.location.search).get("url");
    loadReplayFromUrl(explicitUrl ?? "sample-replay.json")
      .then((r) => setReplay((current) => current ?? r))
      .catch((e: unknown) => {
        // A missing bundled sample fails silently (the loader overlay just stays up); a
        // user-provided `?url=` that fails is worth surfacing.
        if (explicitUrl) setLoadError((current) => current ?? (e instanceof Error ? e.message : String(e)));
      });
  }, []);

  useEffect(() => {
    if (!controller) return;
    function onKeyDown(e: KeyboardEvent) {
      if (!controller) return;
      switch (e.key) {
        case "ArrowLeft":
          e.preventDefault();
          if (e.shiftKey) controller.jumpTo(controller.turnBoundaryIndex(-1));
          else controller.stepBackward();
          break;
        case "ArrowRight":
          e.preventDefault();
          if (e.shiftKey) controller.jumpTo(controller.turnBoundaryIndex(1));
          else controller.stepForward();
          break;
        case "Home":
          e.preventDefault();
          controller.jumpTo(0);
          break;
        case "End":
          e.preventDefault();
          controller.jumpTo(controller.maxIndex);
          break;
        case " ":
          e.preventDefault();
          controller.togglePlay();
          break;
        default:
          break;
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [controller]);

  return (
    <div className="app">
      <header className="app-header">
        <div>
          <h1>deckgym replay viewer</h1>
          {replay && (
            <span className="subtitle">
              {replay.outcome === null ? "In progress" : replay.outcome === "Tie" ? "Tie" : `P${replay.outcome.Win} wins`}
            </span>
          )}
        </div>
        <div className="header-actions">
          <SettingsPopover />
          {replay && (
            <button type="button" onClick={() => setReplay(null)}>
              Load another replay
            </button>
          )}
        </div>
      </header>

      <div className="app-body">
        <Group
          key={narrow ? "stacked" : "side"}
          id={`viewer-main-${narrow ? "stacked" : "side"}`}
          className="panel-group"
          orientation={narrow ? "vertical" : "horizontal"}
          defaultLayout={mainLayout.defaultLayout}
          onLayoutChanged={mainLayout.onLayoutChanged}
        >
          <Panel id="board" defaultSize={narrow ? "60%" : "74%"} minSize="25%">
            <div className="main-column">
              <div className="app-shell">
                <Board onReady={handleReady} images={images} />
                {!replay && (
                  <Loader
                    error={loadError}
                    onError={setLoadError}
                    onLoaded={(r) => {
                      setLoadError(null);
                      setReplay(r);
                    }}
                  />
                )}
              </div>
              {replay && controller && state && (
                <Controls
                  replay={replay}
                  controller={controller}
                  index={state.index}
                  playing={state.playing}
                  animating={state.animating}
                  speed={state.speed}
                />
              )}
            </div>
          </Panel>
          <Separator className={`resize-handle ${narrow ? "stacked" : "side"}`} />
          <Panel id="sidebar" defaultSize={narrow ? "40%" : "26%"} minSize="12%">
            <aside className="sidebar">
              {replay && state && (
                <Group
                  id="viewer-sidebar"
                  className="panel-group"
                  orientation="vertical"
                  defaultLayout={sidebarLayout.defaultLayout}
                  onLayoutChanged={sidebarLayout.onLayoutChanged}
                >
                  <Panel id="log" defaultSize="62%" minSize="15%">
                    <LogPanel replay={replay} currentIndex={state.index} onSeek={(i) => controller?.jumpTo(i)} />
                  </Panel>
                  <Separator className="resize-handle stacked" />
                  <Panel id="options" defaultSize="38%" minSize="15%">
                    <OptionsPanel replay={replay} currentIndex={state.index} />
                  </Panel>
                </Group>
              )}
            </aside>
          </Panel>
        </Group>
      </div>
    </div>
  );
}

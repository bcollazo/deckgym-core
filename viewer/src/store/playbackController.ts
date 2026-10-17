// Owns playback over a loaded Replay against a BoardScene: which rest-snapshot the board is on,
// stepping forward/backward with an animated transition, and jumping straight to a snapshot (the
// scrubber). Deliberately plain TS (no React) — App wraps it in a small hook so Pixi/GSAP objects
// never become React state.

import { buildTimeline } from "../anim/buildTimeline";
import { gsap } from "../anim/gsap";
import type { BoardScene } from "../board/scene";
import type { Replay, SimpleAction } from "../types/replay";
import { snapshotAt, snapshotCount } from "../types/replay";

export interface PlaybackSnapshot {
  index: number;
  playing: boolean;
  animating: boolean;
  speed: number;
}

export class PlaybackController {
  readonly replay: Replay;
  private scene: BoardScene;
  private timelines = new Map<number, gsap.core.Timeline>();
  private listeners = new Set<() => void>();
  index = 0;
  playing = false;
  speed = 1;
  /** Set while an animated step is in flight, so callers can disable step buttons. */
  animating = false;
  // Cached so `getSnapshot()` returns a referentially-stable object between `emit()`s, which
  // `useSyncExternalStore` requires to avoid re-rendering (or looping) on every render.
  private snapshot: PlaybackSnapshot;

  constructor(scene: BoardScene, replay: Replay) {
    this.scene = scene;
    this.replay = replay;
    this.snapshot = this.computeSnapshot();
    this.scene.setPlayerNames(replay.players);
    this.scene.clearFx();
    this.scene.reconcile(snapshotAt(replay, 0), replay.cards);
  }

  subscribe(fn: () => void): () => void {
    this.listeners.add(fn);
    return () => this.listeners.delete(fn);
  }

  private computeSnapshot(): PlaybackSnapshot {
    return { index: this.index, playing: this.playing, animating: this.animating, speed: this.speed };
  }

  getSnapshot(): PlaybackSnapshot {
    return this.snapshot;
  }

  private emit() {
    this.snapshot = this.computeSnapshot();
    for (const fn of this.listeners) fn();
  }

  get maxIndex(): number {
    return snapshotCount(this.replay) - 1;
  }

  private chosenActionFor(transitionIndex: number): SimpleAction {
    const step = this.replay.steps[transitionIndex];
    return step.options[step.chosen]?.action ?? "Noop";
  }

  /** Builds (and caches) the timeline for transition `i` (rest-snapshot i -> i+1). If it has never
   * been built before, this temporarily reconciles the board to snapshot `i` first so every tween's
   * implicit "from" value is correct, then restores whatever the board should currently show — all
   * synchronously, so nothing is visibly painted in between. See `anim/buildTimeline.ts`'s header. */
  private timelineFor(i: number): gsap.core.Timeline {
    const cached = this.timelines.get(i);
    if (cached) return cached;

    const needsPriming = this.index !== i;
    if (needsPriming) {
      this.scene.reconcile(snapshotAt(this.replay, i), this.replay.cards);
    }
    const prev = snapshotAt(this.replay, i);
    const next = snapshotAt(this.replay, i + 1);
    const step = this.replay.steps[i];
    const tl = buildTimeline(prev, next, this.chosenActionFor(i), { scene: this.scene, cards: this.replay.cards, actor: step.actor });
    this.timelines.set(i, tl);
    if (needsPriming) {
      // Restore the board to what it should currently show (snapshot i+1) without a visible flash:
      // this renders every tween at its end state and runs the forward `tl.call`s once (content
      // swaps apply `next`'s state, since `tl.reversed()` is still false here), synchronously,
      // before the browser gets a chance to paint the primed (snapshot i) frame set up above. A
      // decorative flourish (particles, screen shake) can still fire for real during this jump —
      // a harmless quirk the first time a step is played backward without playing it forward first.
      tl.progress(1);
    }
    return tl;
  }

  private gotoRest(index: number) {
    this.index = index;
    this.animating = false;
    this.emit();
  }

  /** Hard-jumps to `index` with no animation (the scrubber dragged more than one step, or the
   * replay just loaded). */
  jumpTo(index: number) {
    const clamped = Math.max(0, Math.min(this.maxIndex, Math.round(index)));
    for (const tl of this.timelines.values()) tl.kill();
    this.timelines.clear();
    this.scene.clearFx();
    this.scene.reconcile(snapshotAt(this.replay, clamped), this.replay.cards);
    this.gotoRest(clamped);
  }

  canStepForward(): boolean {
    return this.index < this.maxIndex;
  }

  canStepBackward(): boolean {
    return this.index > 0;
  }

  stepForward(onDone?: () => void) {
    if (!this.canStepForward() || this.animating) return;
    const i = this.index;
    this.scene.clearFx();
    // Playback speed is applied once, globally (see `setSpeed`), since every timeline this
    // controller creates is (by default) a child of `gsap.globalTimeline`.
    const tl = this.timelineFor(i);
    if (tl.totalDuration() === 0) {
      // No visual diff at all (e.g. an EndTurn during setup, or a DrawCard — hand/deck counts
      // aren't animated, see buildTimeline.ts). GSAP's own onComplete for a childless, zero-
      // duration timeline isn't reliable to await, so this resolves the step immediately instead.
      this.scene.reconcile(snapshotAt(this.replay, i + 1), this.replay.cards);
      this.gotoRest(i + 1);
      onDone?.();
      return;
    }
    this.animating = true;
    this.emit();
    tl.eventCallback("onComplete", () => {
      this.scene.reconcile(snapshotAt(this.replay, i + 1), this.replay.cards);
      this.gotoRest(i + 1);
      onDone?.();
    });
    tl.eventCallback("onReverseComplete", null);
    tl.play(0);
  }

  stepBackward(onDone?: () => void) {
    if (!this.canStepBackward() || this.animating) return;
    const i = this.index - 1;
    this.scene.clearFx();
    const tl = this.timelineFor(i);
    if (tl.totalDuration() === 0) {
      this.scene.reconcile(snapshotAt(this.replay, i), this.replay.cards);
      this.gotoRest(i);
      onDone?.();
      return;
    }
    this.animating = true;
    this.emit();
    tl.eventCallback("onReverseComplete", () => {
      this.scene.reconcile(snapshotAt(this.replay, i), this.replay.cards);
      this.gotoRest(i);
      onDone?.();
    });
    tl.eventCallback("onComplete", null);
    tl.reverse();
  }

  setSpeed(speed: number) {
    this.speed = speed;
    gsap.globalTimeline.timeScale(speed);
    this.emit();
  }

  /** Chains `stepForward` calls until the end of the replay or `pause()` is called. */
  play() {
    if (this.playing) return;
    this.playing = true;
    this.emit();
    const advance = () => {
      if (!this.playing) return;
      if (!this.canStepForward()) {
        this.pause();
        return;
      }
      this.stepForward(advance);
    };
    advance();
  }

  pause() {
    if (!this.playing) return;
    this.playing = false;
    this.emit();
  }

  togglePlay() {
    if (this.playing) this.pause();
    else this.play();
  }

  /** The next (or previous) rest-snapshot index where the turn number differs from the current
   * one, clamped to the replay's bounds. Used for Shift+Left/Right ("step a whole turn"). */
  turnBoundaryIndex(direction: 1 | -1): number {
    const currentTurn = snapshotAt(this.replay, this.index).turn;
    let i = this.index;
    while (i + direction >= 0 && i + direction <= this.maxIndex) {
      i += direction;
      if (snapshotAt(this.replay, i).turn !== currentTurn) return i;
    }
    return i;
  }

  destroy() {
    for (const tl of this.timelines.values()) tl.kill();
    this.timelines.clear();
    this.listeners.clear();
  }
}

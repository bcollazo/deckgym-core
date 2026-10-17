import { gsap } from "gsap";
import { CustomEase } from "gsap/CustomEase";
import { MotionPathPlugin } from "gsap/MotionPathPlugin";
import { PixiPlugin } from "gsap/PixiPlugin";
import * as PIXI from "pixi.js";

let registered = false;

/** Registers the GSAP plugins the plan calls for, once. Safe to call repeatedly. */
export function setupGsap(): typeof gsap {
  if (!registered) {
    PixiPlugin.registerPIXI(PIXI);
    gsap.registerPlugin(PixiPlugin, MotionPathPlugin, CustomEase);
    CustomEase.create("attackLunge", "M0,0 C0.2,0 0.2,1.4 0.4,1.4 0.7,1.4 0.6,1 1,1");
    CustomEase.create("popOvershoot", "M0,0 C0.34,1.56 0.64,1 1,1");
    registered = true;
  }
  return gsap;
}

/** Whether the viewer should skip motion-heavy tweens (`prefers-reduced-motion`). Re-checked on
 * every call rather than cached, since the setting can change while the page is open. */
export function prefersReducedMotion(): boolean {
  return typeof window !== "undefined" && window.matchMedia?.("(prefers-reduced-motion: reduce)").matches;
}

export { gsap };

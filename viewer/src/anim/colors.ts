import type { EnergyType } from "../types/replay";

/** Type-colored frames/accents, shared by card rendering and energy-attach particle colors. */
export const ENERGY_COLORS: Record<EnergyType, number> = {
  Grass: 0x4caf50,
  Fire: 0xff5722,
  Water: 0x2196f3,
  Lightning: 0xffc107,
  Psychic: 0xe040fb,
  Fighting: 0xb85c38,
  Darkness: 0x4a4a5a,
  Metal: 0x9aa5b1,
  Dragon: 0xff9800,
  Colorless: 0xd8d3c5,
};

export const PLAYER_COLORS: [number, number] = [0x4f8cff, 0xff6b6b];

export function energyColor(type: EnergyType | null | undefined): number {
  return type ? ENERGY_COLORS[type] : 0x888888;
}

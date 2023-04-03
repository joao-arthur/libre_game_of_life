import { stillLife } from "./stillLife/mod.ts";
import { oscilators } from "./oscilators/mod.ts";
import { spaceShips } from "./spaceShips/mod.ts";

export const presets = {
    stillLife,
    oscilators,
    spaceShips,
} as const;

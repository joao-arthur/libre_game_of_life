import { boat } from "./stillLife/boat/boat.ts";
import { block } from "./stillLife/general/block.ts";

import { blinker } from "./oscillators/general/blinker.ts";

import { rPentomino } from "./methuselahs/general/rPentomino.ts";

import { glider } from "./spaceships/general/glider.ts";

import { gosperGliderGun } from "./gliderGun/general/gosperGliderGun.ts";

import { puffer1 } from "./puffer/general/puffer1.ts";

export const presets = [
    boat,
    block,
    blinker,
    rPentomino,
    glider,
    gosperGliderGun,
    puffer1,
] as const;

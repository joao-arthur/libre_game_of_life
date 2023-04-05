import { boat } from "./stillLife/boat/boat.js";
import { block } from "./stillLife/general/block.js";

import { blinker } from "./oscillators/general/blinker.js";

import { rPentomino } from "./methuselahs/general/rPentomino.js";

import { glider } from "./spaceships/general/glider.js";

import { gosperGliderGun } from "./gliderGun/general/gosperGliderGun.js";

import { puffer1 } from "./puffer/general/puffer1.js";

export const presets = [
    boat,
    block,
    blinker,
    rPentomino,
    glider,
    gosperGliderGun,
    puffer1,
] as const;

import { presetGroupType } from "./presetGroup.js";
import { stillLife } from "./stillLife/mod.js";
import { oscillators } from "./oscillators/mod.js";
import { methuselahs } from "./methuselahs/mod.js";
import { spaceships } from "./spaceships/mod.js";
import { gliderGun } from "./gliderGun/mod.js";
import { puffer } from "./puffer/mod.js";

export const presets: readonly presetGroupType[] = [
    stillLife,
    oscillators,
    methuselahs,
    spaceships,
    gliderGun,
    puffer,
];

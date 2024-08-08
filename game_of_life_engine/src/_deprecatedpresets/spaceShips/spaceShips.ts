import { glider } from "./glider.js";
import { hwss } from "./hwss.js";
import { lwss } from "./lwss.js";
import { mwss } from "./mwss.js";

export const spaceShips = {
    glider,
    hwss,
    lwss,
    mwss,
} as const;

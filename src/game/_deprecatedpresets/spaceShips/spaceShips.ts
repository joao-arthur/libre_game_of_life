import { glider } from "./glider.ts";
import { hwss } from "./hwss.ts";
import { lwss } from "./lwss.ts";
import { mwss } from "./mwss.ts";

export const spaceShips = {
    glider,
    hwss,
    lwss,
    mwss,
} as const;

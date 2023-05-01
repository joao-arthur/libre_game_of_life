import { PresetGroup } from "../presetGroup.js";
import { ship } from "./ship/mod.js";
import { boat } from "./boat/mod.js";
import { loaf } from "./loaf/mod.js";
import { general } from "./general/mod.js";

export const stillLife: PresetGroup = {
    group: { name: "StillLife", id: "stillLife" },
    subGroups: [
        ship,
        boat,
        loaf,
        general,
    ],
};

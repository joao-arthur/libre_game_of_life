import { PresetGroup } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const spaceships: PresetGroup = {
    group: { name: "Spaceships", id: "spaceships" },
    subGroups: [
        general,
    ],
};

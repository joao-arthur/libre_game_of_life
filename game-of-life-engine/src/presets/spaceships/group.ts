import { presetGroupType } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const spaceships: presetGroupType = {
    group: { name: "Spaceships", id: "spaceships" },
    subGroups: [
        general,
    ],
};

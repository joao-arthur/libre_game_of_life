import { presetGroupType } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const gliderGun: presetGroupType = {
    group: { name: "Glider gun", id: "gliderGun" },
    subGroups: [
        general,
    ],
};

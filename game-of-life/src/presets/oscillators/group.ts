import { presetGroupType } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const oscillators: presetGroupType = {
    group: { name: "Oscillators", id: "oscillators" },
    subGroups: [
        general,
    ],
};

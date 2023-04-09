import { presetGroupType } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const puffer: presetGroupType = {
    group: { name: "Puffer", id: "puffer" },
    subGroups: [
        general,
    ],
};

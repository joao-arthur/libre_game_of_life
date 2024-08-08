import { PresetGroup } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const puffer: PresetGroup = {
    group: { name: "Puffer", id: "puffer" },
    subGroups: [
        general,
    ],
};

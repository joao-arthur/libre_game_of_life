import { PresetGroup } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const oscillators: PresetGroup = {
    group: { name: "Oscillators", id: "oscillators" },
    subGroups: [
        general,
    ],
};

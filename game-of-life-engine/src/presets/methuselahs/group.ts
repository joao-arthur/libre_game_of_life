import { PresetGroup } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const methuselahs: PresetGroup = {
    group: { name: "Methuselahs", id: "methuselahs" },
    subGroups: [
        general,
    ],
};

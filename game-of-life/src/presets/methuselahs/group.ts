import { presetGroupType } from "../presetGroup.js";
import { general } from "./general/mod.js";

export const methuselahs: presetGroupType = {
    group: { name: "Methuselahs", id: "methuselahs" },
    subGroups: [
        general,
    ],
};

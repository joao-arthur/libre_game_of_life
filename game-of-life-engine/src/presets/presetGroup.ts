import { PresetSubGroup } from "./presetSubGroup.js";

export type PresetGroup = {
    readonly group: {
        readonly name: string;
        readonly id: string;
    };
    readonly subGroups: readonly PresetSubGroup[];
};

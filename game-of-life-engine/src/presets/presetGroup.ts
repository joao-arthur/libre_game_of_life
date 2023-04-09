import { presetSubGroupType } from "./presetSubGroup.js";

export type presetGroupType = {
    readonly group: {
        readonly name: string;
        readonly id: string;
    };
    readonly subGroups: readonly presetSubGroupType[];
};

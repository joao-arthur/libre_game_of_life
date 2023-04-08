import { presetType } from "./preset.js";

export type presetSubGroupType = {
    readonly name: string;
    readonly id: string;
    readonly items: readonly presetType[];
};

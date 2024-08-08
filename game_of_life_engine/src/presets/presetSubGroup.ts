import { Preset } from "./preset.js";

export type PresetSubGroup = {
    readonly name: string;
    readonly id: string;
    readonly items: readonly Preset[];
};

import { arrays } from "funis";
import { presets } from "game-of-life";

type presetOptionsType = {
    readonly label: string;
    readonly value: string;
    readonly options: {
        readonly label: string;
        readonly value: string;
    }[];
}[];

export function buildPresetsOptions(): presetOptionsType {
    return arrays
        .groupToEntries(presets, (preset) => preset.group.name)
        .map(([groupName, groupItems]) => ({
            label: groupName,
            value: groupName,
            options: groupItems.map((item) => ({
                label: item.name,
                value: item.id,
            })),
        }));
}

import { presetGroups } from "game_of_life_engine";

type presetOptionsType = {
    readonly label: string;
    readonly value: string;
    readonly options: {
        readonly label: string;
        readonly value: string;
    }[];
}[];

export function buildPresetsOptions(): presetOptionsType {
    return presetGroups.map((group) => ({
        label: group.group.name,
        value: group.group.id,
        options: group.subGroups.flatMap((subGroup) => subGroup.items)
            .map((item) => ({
                label: item.name,
                value: item.id,
            })),
    }));
}

import { presetGroups, presetType } from "game-of-life-engine";

export function buildPresetsItems(): presetType[] {
    return presetGroups.flatMap((group) =>
        group.subGroups.flatMap((subGroup) => subGroup.items)
    );
}

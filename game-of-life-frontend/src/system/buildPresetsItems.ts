import { Preset, presetGroups } from "game-of-life-engine";

export function buildPresetsItems(): Preset[] {
    return presetGroups.flatMap((group) =>
        group.subGroups.flatMap((subGroup) => subGroup.items)
    );
}

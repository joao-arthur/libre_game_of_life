import { Preset, presetGroups } from "game_of_life_engine";

export function buildPresetsItems(): Preset[] {
    return presetGroups.flatMap((group) =>
        group.subGroups.flatMap((subGroup) => subGroup.items)
    );
}

import { modelFns, modelType } from "game-of-life-engine";

export function getUnitSize(
    model: modelType,
    totalSize: number,
): number {
    return totalSize / modelFns.getSize(model);
}

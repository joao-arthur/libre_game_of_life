import { modelType } from "../model.js";

export function getSize(model: modelType): number {
    const deltaX = model.position.x2 - model.position.x1;

    return deltaX + 1;
}

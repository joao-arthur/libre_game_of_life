import { modelType } from "../model.js";

export function iterate(
    model: modelType,
): modelType {
    return {
        value: new Map(),
        iteration: model.iteration + 1,
        position: model.position,
    };
}

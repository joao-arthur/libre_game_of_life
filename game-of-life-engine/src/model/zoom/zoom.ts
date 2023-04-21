import { modelType } from "../model.js";

export function zoom(model: modelType, newSize: number): modelType {
    const halfNewSize = newSize / 2;
    const halfX = (model.position.x1 + model.position.x2) / 2;
    const halfY = (model.position.y1 + model.position.y2) / 2;

    const x1 = Math.ceil(halfX - halfNewSize);
    const y1 = Math.ceil(halfY - halfNewSize);
    const x2 = x1 + newSize - 1;
    const y2 = y1 + newSize - 1;

    return {
        value: model.value,
        iteration: model.iteration,
        position: {
            x1: x1 === -0 ? 0 : x1,
            y1: y1 === -0 ? 0 : y1,
            x2: x2 === -0 ? 0 : x2,
            y2: y2 === -0 ? 0 : y2,
        },
    };
}

import { modelType } from "../model.js";

export function zoom(model: modelType, newSize: number): modelType {
    const halfX = Math.floor(
        (model.position.x1 + model.position.x2) / 2,
    );
    const halfY = Math.floor(
        (model.position.y1 + model.position.y2) / 2,
    );

    const x1 = Math.ceil(halfX - newSize / 2 + 1);
    const x2 = Math.ceil(halfX + newSize / 2);
    const y1 = Math.ceil(halfY - newSize / 2 + 1);
    const y2 = Math.ceil(halfY + newSize / 2);

    return {
        value: model.value,
        iteration: model.iteration,
        position: { x1, y1, x2, y2 },
    };
}

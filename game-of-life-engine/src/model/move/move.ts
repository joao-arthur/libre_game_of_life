import { modelType } from "../model.js";

type changeType = {
    readonly deltaX: number;
    readonly deltaY: number;
};

export function move(
    model: modelType,
    change: changeType,
): modelType {
    return {
        value: model.value,
        iteration: model.iteration,
        position: {
            x1: model.position.x1 + change.deltaX,
            y1: model.position.y1 + change.deltaY,
            x2: model.position.x2 + change.deltaX,
            y2: model.position.y2 + change.deltaY,
        },
    };
}

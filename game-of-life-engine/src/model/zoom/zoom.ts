import { num } from "funis";
import { Model } from "../model.js";

export function zoom(model: Model, newSize: number): Model {
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
            x1: num.normalizeZero(x1),
            y1: num.normalizeZero(y1),
            x2: num.normalizeZero(x2),
            y2: num.normalizeZero(y2),
        },
    };
}

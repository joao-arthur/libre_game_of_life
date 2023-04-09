import { cartesianPointType } from "../../cartesianPlane/mod.js";
import { modelType } from "../model.js";

export function move(
    model: modelType,
    delta: cartesianPointType,
): modelType {
    return {
        value: model.value,
        iteration: model.iteration,
        position: {
            x1: model.position.x1 + delta.x,
            y1: model.position.y1 + delta.y,
            x2: model.position.x2 + delta.x,
            y2: model.position.y2 + delta.y,
        },
    };
}

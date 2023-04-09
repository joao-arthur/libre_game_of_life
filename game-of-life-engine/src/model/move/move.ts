import { cartesianPointType } from "../../cartesianPlane/mod.js";
import { modelType } from "../model.js";

export function move(
    model: modelType,
    change: cartesianPointType,
): modelType {
    return {
        value: model.value,
        iteration: model.iteration,
        position: {
            x1: model.position.x1 + change.x,
            y1: model.position.y1 + change.y,
            x2: model.position.x2 + change.x,
            y2: model.position.y2 + change.y,
        },
    };
}

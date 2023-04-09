import { cartesianPointType } from "../../core/cartesianPlane/cartesianPoint.js";
import { modelFns, modelType } from "../../model/mod.js";
import { neighborsType } from "../neighbors.js";

export function fromModel(
    model: modelType,
    point: cartesianPointType,
): neighborsType {
    const { getValue } = modelFns;

    return [
        getValue(model, { x: point.x - 1, y: point.y + 1 }),
        getValue(model, { x: point.x, y: point.y + 1 }),
        getValue(model, { x: point.x + 1, y: point.y + 1 }),

        getValue(model, { x: point.x - 1, y: point.y }),
        //getValue(model, { x: point.x, y: point.y }),
        getValue(model, { x: point.x + 1, y: point.y }),

        getValue(model, { x: point.x - 1, y: point.y - 1 }),
        getValue(model, { x: point.x, y: point.y - 1 }),
        getValue(model, { x: point.x + 1, y: point.y - 1 }),
    ];
}

import { CartesianPoint } from "../../cartesianPlane/mod.js";
import { Model, modelFns } from "../../model/mod.js";
import { Neighbors } from "../neighbors.js";

export function fromModel(
    model: Model,
    point: CartesianPoint,
): Neighbors {
    const { getValue } = modelFns;

    return [
        getValue(model, { x: point.x - 1, y: point.y + 1 }),
        getValue(model, { x: point.x, y: point.y + 1 }),
        getValue(model, { x: point.x + 1, y: point.y + 1 }),

        getValue(model, { x: point.x - 1, y: point.y }),
        getValue(model, { x: point.x + 1, y: point.y }),

        getValue(model, { x: point.x - 1, y: point.y - 1 }),
        getValue(model, { x: point.x, y: point.y - 1 }),
        getValue(model, { x: point.x + 1, y: point.y - 1 }),
    ];
}

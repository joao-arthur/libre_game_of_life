import { pipe } from "funis";
import { cartesianPointType } from "../../core/cartesianPlane/cartesianPoint.js";
import { aliveNeighborsType } from "../aliveNeighbors.js";
import { fromModel } from "../fromModel/mod.js";
import { modelType } from "../../model/mod.js";
import { numberOfAlive } from "../numberOfAlive/mod.js";

export function aliveFromModel(
    model: modelType,
    point: cartesianPointType,
): aliveNeighborsType {
    return pipe(
        () => fromModel(model, point),
        (neighbors) => numberOfAlive(neighbors),
    )(undefined);
}

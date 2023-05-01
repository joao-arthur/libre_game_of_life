import { pipe } from "funis";
import { CartesianPoint } from "../../cartesianPlane/mod.js";
import { AliveNeighbors } from "../aliveNeighbors.js";
import { fromModel } from "../fromModel/mod.js";
import { Model } from "../../model/mod.js";
import { numberOfAlive } from "../numberOfAlive/mod.js";

export function aliveFromModel(
    model: Model,
    point: CartesianPoint,
): AliveNeighbors {
    return pipe(
        () => fromModel(model, point),
        (neighbors) => numberOfAlive(neighbors),
    )(undefined);
}

import { pipe } from "https://deno.land/x/funis@v1.0.2/mod.js";
import { aliveNeighborsType } from "../aliveNeighbors.js";
import { fromModel } from "../fromModel/mod.js";
import { modelType } from "../../model/mod.js";
import { numberOfAlive } from "../numberOfAlive/mod.js";
import { positionType } from "../../model/position.js";

export function aliveFromModel(
    model: modelType,
    position: positionType,
): aliveNeighborsType {
    return pipe(
        () => fromModel(model, position),
        (neighbors) => numberOfAlive(neighbors),
    )(undefined);
}

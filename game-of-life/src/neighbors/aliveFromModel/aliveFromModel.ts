import { pipe } from "https://deno.land/x/funis@v1.0.2/mod.ts";
import { aliveNeighborsType } from "../aliveNeighbors.ts";
import { fromModel } from "../fromModel/mod.ts";
import { modelType } from "../../model/mod.ts";
import { numberOfAlive } from "../numberOfAlive/mod.ts";
import { positionType } from "../../model/position.ts";

export function aliveFromModel(
    model: modelType,
    position: positionType,
): aliveNeighborsType {
    return pipe(
        () => fromModel(model, position),
        (neighbors) => numberOfAlive(neighbors),
    )(undefined);
}

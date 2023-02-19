import { aliveNeighborsType } from "../aliveNeighbors.ts";
import { fromModel } from "../fromModel/mod.ts";
import { modelType } from "../../model/mod.ts";
import { numberOfAlive } from "../numberOfAlive/mod.ts";
import { positionType } from "../../model/position.ts";

export function aliveFromModel(
    model: modelType,
    position: positionType,
): aliveNeighborsType {
    const neighbors = fromModel(model, position);
    const numberOfAliveNeighbors = numberOfAlive(neighbors);
    return numberOfAliveNeighbors;
}

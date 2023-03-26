import { stateType } from "../../cell/mod.ts";
import { neighborsType } from "../neighbors.ts";
import { aliveNeighborsType } from "../aliveNeighbors.ts";

export function numberOfAlive(
    neighbors: neighborsType,
): aliveNeighborsType {
    return neighbors
        .filter((neighbor) => neighbor === stateType.ALIVE)
        .length as aliveNeighborsType;
}

import { stateType } from "../../cell/mod.ts";
import { neighborsType } from "../neighbors.ts";
import { aliveNeighborsType } from "../aliveNeighbors.ts";

export function numberOfAlive(
    neighbors: neighborsType,
): aliveNeighborsType {
    return neighbors
        .filter((neighbor) => neighbor === stateType.ALIVE)
        .length as 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
}

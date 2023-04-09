import { stateType } from "../../cell/mod.js";
import { neighborsType } from "../neighbors.js";
import { aliveNeighborsType } from "../aliveNeighbors.js";

export function numberOfAlive(
    neighbors: neighborsType,
): aliveNeighborsType {
    return neighbors
        .filter((neighbor) => neighbor === stateType.ALIVE)
        .length as aliveNeighborsType;
}

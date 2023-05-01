import { State } from "../../cell/mod.js";
import { Neighbors } from "../neighbors.js";
import { AliveNeighbors } from "../aliveNeighbors.js";

export function numberOfAlive(
    neighbors: Neighbors,
): AliveNeighbors {
    return neighbors
        .filter((neighbor) => neighbor === State.ALIVE)
        .length as AliveNeighbors;
}

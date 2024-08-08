import { cb } from "funis";
import { State } from "../../cell/mod.js";
import { Neighbor, Neighbors } from "../neighbors.js";
import { AliveNeighbors } from "../aliveNeighbors.js";

export function numberOfAlive(
    neighbors: Neighbors,
): AliveNeighbors {
    return neighbors
        .filter(cb.eq(State.ALIVE as Neighbor))
        .length as AliveNeighbors;
}

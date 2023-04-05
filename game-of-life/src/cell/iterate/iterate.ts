import { aliveNeighborsType } from "../../neighbors/aliveNeighbors.js";
import { stateType } from "../state.js";

export function iterate(
    state: stateType,
    numberOfAliveNeighbors: aliveNeighborsType,
): stateType {
    switch (state) {
        case stateType.ALIVE:
            return [2, 3].includes(numberOfAliveNeighbors)
                ? stateType.ALIVE
                : stateType.DEAD;
        case stateType.DEAD:
            return [3].includes(numberOfAliveNeighbors)
                ? stateType.ALIVE
                : stateType.DEAD;
    }
}

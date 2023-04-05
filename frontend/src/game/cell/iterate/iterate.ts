import { aliveNeighborsType } from "../../neighbors/aliveNeighbors.ts";
import { stateType } from "../state.ts";

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

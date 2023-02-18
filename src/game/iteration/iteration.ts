import { stateType } from "../state.ts";

export function iteration(
    state: stateType,
    numberOfAliveNeighbors: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8,
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

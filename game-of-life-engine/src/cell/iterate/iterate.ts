import { AliveNeighbors } from "../../neighbors/aliveNeighbors.js";
import { State } from "../state.js";

export function iterate(
    state: State,
    numberOfAliveNeighbors: AliveNeighbors,
): State {
    switch (state) {
        case State.ALIVE:
            return [2, 3].includes(numberOfAliveNeighbors)
                ? State.ALIVE
                : State.DEAD;
        case State.DEAD:
            return [3].includes(numberOfAliveNeighbors)
                ? State.ALIVE
                : State.DEAD;
    }
}

import type { AliveNeighbors } from "./neighbors.js";

export enum State {
    DEAD,
    ALIVE,
}

export function iterateCell(state: State, numberOfAliveNeighbors: AliveNeighbors): State {
    switch (state) {
        case State.ALIVE:
            return [2, 3].includes(numberOfAliveNeighbors) ? State.ALIVE : State.DEAD;
        case State.DEAD:
            return [3].includes(numberOfAliveNeighbors) ? State.ALIVE : State.DEAD;
    }
}

export function toggleCell(state: State): State {
    switch (state) {
        case State.ALIVE:
            return State.DEAD;
        case State.DEAD:
            return State.ALIVE;
    }
}

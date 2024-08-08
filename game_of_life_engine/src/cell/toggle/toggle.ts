import { State } from "../state.js";

export function toggle(
    state: State,
): State {
    switch (state) {
        case State.ALIVE:
            return State.DEAD;
        case State.DEAD:
            return State.ALIVE;
    }
}

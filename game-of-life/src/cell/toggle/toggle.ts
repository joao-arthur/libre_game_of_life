import { stateType } from "../state.ts";

export function toggle(
    state: stateType,
): stateType {
    switch (state) {
        case stateType.ALIVE:
            return stateType.DEAD;
        case stateType.DEAD:
            return stateType.ALIVE;
    }
}

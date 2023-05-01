import { expect, it } from "vitest";
import { State } from "../../cell/state.js";
import { numberOfAlive } from "./numberOfAlive.js";

it("numberOfAlive", () => {
    expect(
        numberOfAlive([
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
        ]),
    ).toBe(0);
    expect(
        numberOfAlive([
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
        ]),
    ).toBe(0);
    expect(
        numberOfAlive([
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
        ]),
    ).toBe(8);
    expect(
        numberOfAlive([
            State.ALIVE,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            undefined,
        ]),
    ).toBe(1);
});

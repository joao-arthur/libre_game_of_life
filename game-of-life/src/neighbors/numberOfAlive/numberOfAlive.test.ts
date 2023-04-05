import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../../cell/state.js";
import { numberOfAlive } from "./numberOfAlive.js";

Deno.test("Should return the number of alive neighbors", () => {
    assertEquals(
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
        0,
    );
    assertEquals(
        numberOfAlive([
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
        ]),
        0,
    );
    assertEquals(
        numberOfAlive([
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
            stateType.ALIVE,
        ]),
        8,
    );
    assertEquals(
        numberOfAlive([
            stateType.ALIVE,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,
            undefined,
        ]),
        1,
    );
});

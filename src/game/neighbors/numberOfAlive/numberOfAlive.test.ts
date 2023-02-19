import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/state.ts";
import { numberOfAlive } from "./numberOfAlive.ts";

Deno.test("numberOfAlive", () => {
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

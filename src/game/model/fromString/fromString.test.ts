import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { fromString } from "./fromString.ts";

Deno.test("fromString", () => {
    assertEquals(
        fromString(["⬛"]),
        {
            size: 1,
            value: [[stateType.DEAD]],
            iteration: 0,
        },
    );
    assertEquals(
        fromString(["⬜"]),
        {
            size: 1,
            value: [[stateType.ALIVE]],
            iteration: 0,
        },
    );
    assertEquals(
        fromString([
            "⬜⬜",
            "⬜⬛",
        ]),
        {
            size: 2,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
            ],
            iteration: 0,
        },
    );
});

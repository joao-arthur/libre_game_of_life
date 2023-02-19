import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { fromString } from "./fromString.ts";

Deno.test("fromString", () => {
    assertEquals(
        fromString(["⬛"]),
        { width: 1, height: 1, value: [[stateType.DEAD]] },
    );
    assertEquals(
        fromString(["⬜"]),
        { width: 1, height: 1, value: [[stateType.ALIVE]] },
    );
    assertEquals(
        fromString([
            "⬜⬜",
            "⬜⬛",
            "⬛⬜",
            "⬛⬛",
        ]),
        {
            width: 2,
            height: 4,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD],
            ],
        },
    );
});

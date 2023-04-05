import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../../model/fromString/mod.js";
import { fromModel } from "./fromModel.js";

Deno.test("Should return the neighbors of the cell", () => {
    assertEquals(
        fromModel(
            fromString([
                "⬛⬜",
                "⬛⬜",
            ]),
            { column: 0, row: 0 },
        ),
        [
            undefined,
            undefined,
            undefined,

            undefined,
            stateType.ALIVE,

            undefined,
            stateType.DEAD,
            stateType.ALIVE,
        ],
    );
    assertEquals(
        fromModel(
            fromString([
                "⬛⬜⬜",
                "⬛⬜⬜",
                "⬛⬜⬛",
            ]),
            { column: 1, row: 1 },
        ),
        [
            stateType.DEAD,
            stateType.ALIVE,
            stateType.ALIVE,

            stateType.DEAD,
            stateType.ALIVE,

            stateType.DEAD,
            stateType.ALIVE,
            stateType.DEAD,
        ],
    );
});

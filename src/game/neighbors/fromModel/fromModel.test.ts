import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { fromModel } from "./fromModel.ts";
import { fromString } from "../../model/mod.ts";

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

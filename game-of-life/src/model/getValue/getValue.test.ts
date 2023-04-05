import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { getValue } from "./getValue.js";

Deno.test("Value out of range", () => {
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: -1, row: 0 },
        ),
        undefined,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: -1, row: 0 },
        ),
        undefined,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 0, row: -1 },
        ),
        undefined,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 2, row: 0 },
        ),
        undefined,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 0, row: 2 },
        ),
        undefined,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 2, row: 2 },
        ),
        undefined,
    );
});

Deno.test("Value in range", () => {
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 0, row: 0 },
        ),
        stateType.DEAD,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 0, row: 1 },
        ),
        stateType.ALIVE,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 1, row: 0 },
        ),
        stateType.DEAD,
    );
    assertEquals(
        getValue(
            fromString([
                "⬛⬛",
                "⬜⬜",
            ]),
            { column: 1, row: 1 },
        ),
        stateType.ALIVE,
    );
});

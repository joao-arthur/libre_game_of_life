import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { map } from "./map.js";

Deno.test("map", () => {
    assertEquals(
        map(
            fromString([
                "⬛⬜",
                "⬜⬛",
                "⬛⬜",
            ]),
            ({ row }) => row > 0 ? stateType.DEAD : stateType.ALIVE,
        ),
        fromString([
            "⬜⬜",
            "⬛⬛",
            "⬛⬛",
        ]),
    );
    assertEquals(
        map(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
            ]),
            ({ column }) =>
                column > 0 ? stateType.DEAD : stateType.ALIVE,
        ),
        fromString([
            "⬜⬛⬛",
            "⬜⬛⬛",
        ]),
    );
    assertEquals(
        map(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
                "⬛⬜⬛",
            ]),
            ({ row, column }) =>
                row > 1 || column > 1
                    ? stateType.DEAD
                    : stateType.ALIVE,
        ),
        fromString([
            "⬜⬜⬛",
            "⬜⬜⬛",
            "⬛⬛⬛",
        ]),
    );
});

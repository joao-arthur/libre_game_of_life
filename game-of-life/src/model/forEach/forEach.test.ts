import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { forEach } from "./forEach.js";

Deno.test("forEach", () => {
    const values1: stateType[][] = [[]];
    forEach(
        fromString([
            "⬛⬜",
            "⬜⬛",
            "⬛⬜",
        ]),
        (_, state) => values1[0].push(state),
    );
    assertEquals(
        values1,
        fromString([
            "⬛⬜⬜⬛⬛⬜",
        ]).value,
    );

    const values2: stateType[][] = [[]];
    forEach(
        fromString([
            "⬛⬜⬛",
            "⬜⬛⬜",
        ]),
        (_, state) => values2[0].push(state),
    );
    assertEquals(
        values2,
        fromString(["⬛⬜⬛⬜⬛⬜"]).value,
    );

    const values3: stateType[][] = [[]];
    forEach(
        fromString([
            "⬛⬜⬛",
            "⬜⬛⬜",
            "⬛⬜⬛",
        ]),
        (_, state) => values3[0].push(state),
    );
    assertEquals(
        values3,
        fromString(["⬛⬜⬛⬜⬛⬜⬛⬜⬛"]).value,
    );
});

import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { fromString } from "../fromString/mod.js";
import { iterate } from "./iterate.js";

Deno.test("iterate", () => {
    assertEquals(
        iterate(
            fromString(["⬜"]),
        ),
        {
            ...fromString(["⬛"]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬜⬜",
                "⬜⬜",
            ]),
        ),
        {
            ...fromString([
                "⬜⬜",
                "⬜⬜",
            ]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬛⬜⬛",
                "⬛⬜⬛",
                "⬛⬜⬛",
            ]),
        ),
        {
            ...fromString([
                "⬛⬛⬛",
                "⬜⬜⬜",
                "⬛⬛⬛",
            ]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬛⬛⬛",
                "⬜⬜⬜",
                "⬛⬛⬛",
            ]),
        ),
        {
            ...fromString([
                "⬛⬜⬛",
                "⬛⬜⬛",
                "⬛⬜⬛",
            ]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬛⬛⬜",
                "⬜⬜⬜",
                "⬛⬛⬛",
            ]),
        ),
        {
            ...fromString([
                "⬛⬛⬜",
                "⬛⬜⬜",
                "⬛⬜⬛",
            ]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬛⬛⬜",
                "⬛⬜⬜",
                "⬛⬜⬛",
            ]),
        ),
        {
            ...fromString([
                "⬛⬜⬜",
                "⬛⬜⬜",
                "⬛⬜⬜",
            ]),
            iteration: 1,
        },
    );
    assertEquals(
        iterate(
            fromString([
                "⬜⬜⬛",
                "⬜⬜⬜",
                "⬛⬜⬛",
            ]),
        ),
        {
            ...fromString([
                "⬜⬛⬜",
                "⬛⬛⬜",
                "⬜⬜⬜",
            ]),
            iteration: 1,
        },
    );
});

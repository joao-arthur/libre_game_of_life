import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../fromString/mod.ts";
import { iterate } from "./iterate.ts";

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

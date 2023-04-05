import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../fromString/mod.ts";
import { zoom } from "./zoom.ts";

const model = fromString([
    "⬛⬜⬛",
    "⬜⬜⬜",
    "⬜⬛⬜",
]);

Deno.test("zoom", () => {
    assertEquals(
        zoom(model, 1),
        fromString([
            "⬜",
        ]),
    );
    assertEquals(
        zoom(model, 2),
        fromString([
            "⬜",
        ]),
    );
    assertEquals(
        zoom(model, 3),
        fromString([
            "⬛⬜⬛",
            "⬜⬜⬜",
            "⬜⬛⬜",
        ]),
    );
    assertEquals(
        zoom(model, 4),
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬜⬛⬜⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
    assertEquals(
        zoom(model, 5),
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬜⬛⬜⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
});

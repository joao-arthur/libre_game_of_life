import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { fromString } from "../fromString/mod.js";
import { zoom } from "./zoom.js";

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

import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../fromString/mod.ts";
import { zoomIn } from "./zoomIn.ts";

const model = fromString([
    "⬛⬛⬛⬛⬛⬛⬛",
    "⬛⬜⬜⬜⬜⬜⬛",
    "⬛⬜⬜⬛⬛⬜⬛",
    "⬛⬜⬛⬜⬛⬜⬛",
    "⬛⬜⬛⬜⬛⬜⬛",
    "⬛⬜⬜⬜⬜⬜⬛",
    "⬛⬛⬛⬛⬛⬛⬛",
]);

Deno.test("zoomIn", () => {
    assertEquals(
        zoomIn(model, 1),
        fromString([
            "⬜⬜⬜⬜⬜",
            "⬜⬜⬛⬛⬜",
            "⬜⬛⬜⬛⬜",
            "⬜⬛⬜⬛⬜",
            "⬜⬜⬜⬜⬜",
        ]),
    );
    assertEquals(
        zoomIn(model, 2),
        fromString([
            "⬜⬛⬛",
            "⬛⬜⬛",
            "⬛⬜⬛",
        ]),
    );
    assertEquals(
        zoomIn(model, 3),
        fromString([
            "⬜",
        ]),
    );
    assertEquals(
        zoomIn(model, 4),
        fromString([""]),
    );
    assertEquals(
        zoomIn(
            fromString([
                "⬛⬛",
                "⬛⬛",
            ]),
            1,
        ),
        fromString([""]),
    );
});

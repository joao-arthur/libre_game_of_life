import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { fromString } from "../fromString/mod.js";
import { zoomOut } from "./zoomOut.js";

const model = fromString(["⬜"]);

Deno.test("zoomOut", () => {
    assertEquals(
        zoomOut(model, 1),
        fromString([
            "⬛⬛⬛",
            "⬛⬜⬛",
            "⬛⬛⬛",
        ]),
    );
    assertEquals(
        zoomOut(model, 2),
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
    assertEquals(
        zoomOut(model, 3),
        fromString([
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬜⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
        ]),
    );
    assertEquals(
        zoomOut(fromString([""]), 2),
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
        ]),
    );
    assertEquals(
        zoomOut(
            fromString([
                "⬜⬜",
                "⬛⬜",
            ]),
            1,
        ),
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬜⬜⬛",
            "⬛⬛⬜⬛",
            "⬛⬛⬛⬛",
        ]),
    );
});

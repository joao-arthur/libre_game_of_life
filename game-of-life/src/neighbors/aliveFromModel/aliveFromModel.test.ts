import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { fromString } from "../../model/fromString/mod.js";
import { aliveFromModel } from "./aliveFromModel.js";

Deno.test("Should return the number of alive neighbors from the model", () => {
    assertEquals(
        aliveFromModel(
            fromString([
                "⬛⬜",
                "⬛⬜",
            ]),
            { column: 0, row: 0 },
        ),
        2,
    );
});

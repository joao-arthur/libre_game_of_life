import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../../model/mod.ts";
import { aliveFromModel } from "./aliveFromModel.ts";

Deno.test("aliveFromModel", () => {
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

import { describe, expect, it } from "vitest";
import { fromString } from "../../model/fromString/mod.js";
import { aliveFromModel } from "./aliveFromModel.js";

it("aliveFromModel", () => {
    expect(
        aliveFromModel(
            fromString([
                "⬛⬜",
                "⬛⬜",
            ]),
            { column: 0, row: 0 },
        ),
    ).toBe(2);
});

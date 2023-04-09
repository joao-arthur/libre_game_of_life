import { expect, it } from "vitest";
import { fromString } from "../../model/fromString/mod.js";
import { aliveFromModel } from "./aliveFromModel.js";

it("aliveFromModel", () => {
    expect(
        aliveFromModel(
            fromString([
                "⬛⬜",
                "⬛⬜",
            ]),
            { x: -1, y: 1 },
        ),
    ).toBe(2);
});

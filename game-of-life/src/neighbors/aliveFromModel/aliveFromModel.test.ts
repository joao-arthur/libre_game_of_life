import { describe, expect, it } from "vitest";
import { fromString } from "../../model/fromString/mod.js";
import { aliveFromModel } from "./aliveFromModel.js";

describe("aliveFromModel", ()=> {
    it("Should return the number of alive neighbors from the model", () => {
        expect(
            aliveFromModel(
                fromString([
                    "⬛⬜",
                    "⬛⬜",
                ]),
                { column: 0, row: 0 },
            )).toBe(
            2,
        );
    });
});

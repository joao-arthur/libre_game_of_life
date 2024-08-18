import { assert, it } from "vitest";
import { arrPosFrom } from "./array.js";

it("arrPosFrom", () => {
    assert.deepStrictEqual(arrPosFrom(20, -13), { row: 20, col: -13 });
});

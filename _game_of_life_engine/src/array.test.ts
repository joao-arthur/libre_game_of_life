import { assert, test } from "vitest";
import { arrPosFrom } from "./array.js";

test("arrPosFrom", () => {
    assert.deepStrictEqual(arrPosFrom(20, -13), { row: 20, col: -13 });
});

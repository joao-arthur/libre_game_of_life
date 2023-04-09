import { expect, it } from "vitest";
import { deserializePoint } from "./deserializePoint.js";

it("deserializePoint", () => {
    expect(deserializePoint("(x: 0, y: 0)")).toEqual({ x: 0, y: 0 });
    expect(deserializePoint("(x: -1, y: -1)")).toEqual({
        x: -1,
        y: -1,
    });
    expect(deserializePoint("(x: 1, y: 1)")).toEqual({ x: 1, y: 1 });
    expect(deserializePoint("(x: 3, y: 6)")).toEqual({ x: 3, y: 6 });
});

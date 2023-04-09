import { expect, it } from "vitest";
import { getQuadrant } from "./getQuadrant.js";

it("getQuadrant", () => {
    expect(getQuadrant({ position: { row: 0, col: 1 }, length: 2 }))
        .toBe("1");
    expect(getQuadrant({ position: { row: 0, col: 0 }, length: 2 }))
        .toBe("2");
    expect(getQuadrant({ position: { row: 1, col: 0 }, length: 2 }))
        .toBe("3");
    expect(getQuadrant({ position: { row: 1, col: 1 }, length: 2 }))
        .toBe("4");
});

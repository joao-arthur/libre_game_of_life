import { expect, it } from "vitest";
import { Model } from "../model.js";
import { getCellSize } from "./getCellSize.js";

const model: Model = {
    value: new Map(),
    iteration: 0,
    position: {
        x1: 1,
        y1: 1,
        x2: 10,
        y2: 10,
    },
};

it("getCellSize", () => {
    expect(getCellSize(model, 100)).toBe(10);
    expect(getCellSize(model, 90)).toBe(9);
    expect(getCellSize(model, 50)).toBe(5);
    expect(getCellSize(model, 10)).toBe(1);
});

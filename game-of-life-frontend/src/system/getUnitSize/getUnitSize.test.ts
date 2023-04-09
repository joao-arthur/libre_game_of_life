import { expect, it } from "vitest";
import { modelType } from "game-of-life-engine";
import { getUnitSize } from "./getUnitSize";

const model: modelType = {
    value: new Map(),
    iteration: 0,
    position: {
        x1: 1,
        y1: 1,
        x2: 10,
        y2: 10,
    },
};

it("getUnitSize", () => {
    expect(getUnitSize(model, 100)).toBe(10);
    expect(getUnitSize(model, 90)).toBe(9);
    expect(getUnitSize(model, 50)).toBe(5);
    expect(getUnitSize(model, 10)).toBe(1);
});

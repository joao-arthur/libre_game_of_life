import { expect, it } from "vitest";
import { modelType } from "../model.js";
import { getSize } from "./getSize.js";

function build(position: modelType["position"]): modelType {
    return { value: new Map(), iteration: 0, position };
}

it("getSize", () => {
    expect(getSize(build({ x1: -10, y1: -10, x2: 10, y2: 10 }))).toBe(
        21,
    );
    expect(getSize(build({ x1: 1, y1: 1, x2: 10, y2: 10 }))).toBe(10);
    expect(getSize(build({ x1: 4, y1: 4, x2: 5, y2: 5 }))).toBe(2);
    expect(getSize(build({ x1: 5, y1: 5, x2: 5, y2: 5 }))).toBe(1);
});

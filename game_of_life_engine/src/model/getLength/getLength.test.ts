import { expect, it } from "vitest";
import { Model } from "../model.js";
import { getLength } from "./getLength.js";

function build(position: Model["position"]): Model {
    return { value: new Map(), iteration: 0, position };
}

it("getLength", () => {
    expect(
        getLength(build({ x1: -10, y1: -10, x2: 10, y2: 10 })),
    ).toBe(21);
    expect(
        getLength(build({ x1: 1, y1: 1, x2: 10, y2: 10 })),
    ).toBe(10);
    expect(
        getLength(build({ x1: 4, y1: 4, x2: 5, y2: 5 })),
    ).toBe(2);
    expect(
        getLength(build({ x1: 5, y1: 5, x2: 5, y2: 5 })),
    ).toBe(1);
});

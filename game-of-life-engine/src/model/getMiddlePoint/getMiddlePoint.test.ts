import { expect, it } from "vitest";
import { Model } from "../model.js";
import { getMiddlePoint } from "./getMiddlePoint.js";

function build(position: Model["position"]): Model {
    return { value: new Map(), iteration: 0, position };
}

it("getMiddlePoint", () => {
    expect(
        getMiddlePoint(build({ x1: -10, y1: -10, x2: 10, y2: 10 })),
    ).toEqual({ x: 0, y: 0 });
    expect(
        getMiddlePoint(build({ x1: 1, y1: 1, x2: 10, y2: 10 })),
    ).toEqual({ x: 5.5, y: 5.5 });
    expect(
        getMiddlePoint(build({ x1: 4, y1: 4, x2: 5, y2: 5 })),
    ).toEqual({ x: 4.5, y: 4.5 });
    expect(
        getMiddlePoint(build({ x1: 5, y1: 5, x2: 5, y2: 5 })),
    ).toEqual({ x: 5, y: 5 });
});

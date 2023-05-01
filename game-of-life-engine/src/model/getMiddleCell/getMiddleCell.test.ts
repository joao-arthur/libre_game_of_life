import { expect, it } from "vitest";
import { Model } from "../model.js";
import { getMiddleCell } from "./getMiddleCell.js";

function build(position: Model["position"]): Model {
    return { value: new Map(), iteration: 0, position };
}

it("getMiddleCell", () => {
    expect(
        getMiddleCell(
            build({ x1: -10, y1: -10, x2: 10, y2: 10 }),
            100,
        ),
    ).toEqual({ x: 0, y: 0 });
    expect(
        getMiddleCell(build({ x1: 1, y1: 1, x2: 10, y2: 10 }), 50),
    ).toEqual({ x: 27.5, y: 27.5 });
    expect(
        getMiddleCell(build({ x1: 4, y1: 4, x2: 5, y2: 5 }), 10),
    ).toEqual({ x: 22.5, y: 22.5 });
    expect(
        getMiddleCell(build({ x1: 5, y1: 5, x2: 5, y2: 5 }), 1),
    ).toEqual({ x: 5, y: 5 });
});

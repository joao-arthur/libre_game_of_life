import { expect, it } from "vitest";
import { modelType } from "../model.js";
import { move } from "./move.js";

function build(position: modelType["position"]): modelType {
    return { value: new Map(), iteration: 0, position };
}

it("move", () => {
    const model = build({ x1: -10, y1: -10, x2: 10, y2: 10 });

    expect(
        move(model, { x: 1, y: 0 }),
    ).toEqual(build({ x1: -9, y1: -10, x2: 11, y2: 10 }));
    expect(
        move(model, { x: -1, y: 0 }),
    ).toEqual(build({ x1: -11, y1: -10, x2: 9, y2: 10 }));
    expect(
        move(model, { x: 0, y: 1 }),
    ).toEqual(build({ x1: -10, y1: -9, x2: 10, y2: 11 }));
    expect(
        move(model, { x: 0, y: -1 }),
    ).toEqual(build({ x1: -10, y1: -11, x2: 10, y2: 9 }));

    expect(
        move(model, { x: 11, y: 0 }),
    ).toEqual(build({ x1: 1, y1: -10, x2: 21, y2: 10 }));
    expect(
        move(model, { x: -11, y: 0 }),
    ).toEqual(build({ x1: -21, y1: -10, x2: -1, y2: 10 }));
    expect(
        move(model, { x: 0, y: 11 }),
    ).toEqual(build({ x1: -10, y1: 1, x2: 10, y2: 21 }));
    expect(
        move(model, { x: 0, y: -11 }),
    ).toEqual(build({ x1: -10, y1: -21, x2: 10, y2: -1 }));
});

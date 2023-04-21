import { describe, expect, it } from "vitest";
import { zoom } from "./zoom.js";
import { modelType } from "../model.js";

function build(position: modelType["position"]): modelType {
    return { value: new Map(), iteration: 0, position };
}

describe("zoom", () => {
    it("odd size", () => {
        const model = build({ x1: -10, y1: -10, x2: 10, y2: 10 });

        expect(
            zoom(model, 1),
        ).toEqual(
            build({ x1: 0, y1: 0, x2: 0, y2: 0 }),
        );
        expect(
            zoom(model, 2),
        ).toEqual(
            build({ x1: -1, y1: -1, x2: 0, y2: 0 }),
        );
        expect(
            zoom(model, 3),
        ).toEqual(
            build({ x1: -1, y1: -1, x2: 1, y2: 1 }),
        );

        expect(
            zoom(model, 19),
        ).toEqual(
            build({ x1: -9, y1: -9, x2: 9, y2: 9 }),
        );
        expect(
            zoom(model, 21),
        ).toEqual(
            build({ x1: -10, y1: -10, x2: 10, y2: 10 }),
        );
        expect(
            zoom(model, 23),
        ).toEqual(
            build({ x1: -11, y1: -11, x2: 11, y2: 11 }),
        );
    });

    it("even size", () => {
        const model = build({ x1: 10, y1: 10, x2: 19, y2: 19 });

        expect(
            zoom(model, 1),
        ).toEqual(
            build({ x1: 14, y1: 14, x2: 14, y2: 14 }),
        );
        expect(
            zoom(model, 2),
        ).toEqual(
            build({ x1: 14, y1: 14, x2: 15, y2: 15 }),
        );
        expect(
            zoom(model, 3),
        ).toEqual(
            build({ x1: 13, y1: 13, x2: 15, y2: 15 }),
        );

        expect(
            zoom(model, 8),
        ).toEqual(
            build({ x1: 11, y1: 11, x2: 18, y2: 18 }),
        );
        expect(
            zoom(model, 10),
        ).toEqual(
            build({ x1: 10, y1: 10, x2: 19, y2: 19 }),
        );
        expect(
            zoom(model, 12),
        ).toEqual(
            build({ x1: 9, y1: 9, x2: 20, y2: 20 }),
        );
    });
});

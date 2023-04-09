import { describe, expect, it } from "vitest";
import { zoom } from "./zoom.js";
import { modelType } from "../model.js";

function build(position: modelType["position"]): modelType {
    return { value: new Map(), iteration: 0, position };
}

describe("zoom", () => {
    it("all quadrants zoom", () => {
        const model = build({ x1: -10, y1: -10, x2: 10, y2: 10 });

        expect(
            zoom(model, 23),
        ).toEqual(build({ x1: -10, y1: -10, x2: 12, y2: 12 }));
        expect(
            zoom(model, 22),
        ).toEqual(build({ x1: -10, y1: -10, x2: 11, y2: 11 }));
        expect(
            zoom(model, 21),
        ).toEqual(build({ x1: -9, y1: -9, x2: 11, y2: 11 }));
        expect(
            zoom(model, 20),
        ).toEqual(build({ x1: -9, y1: -9, x2: 10, y2: 10 }));
        expect(
            zoom(model, 19),
        ).toEqual(build({ x1: -8, y1: -8, x2: 10, y2: 10 }));
        expect(
            zoom(model, 18),
        ).toEqual(build({ x1: -8, y1: -8, x2: 9, y2: 9 }));
        expect(
            zoom(model, 17),
        ).toEqual(build({ x1: -7, y1: -7, x2: 9, y2: 9 }));
        expect(
            zoom(model, 16),
        ).toEqual(build({ x1: -7, y1: -7, x2: 8, y2: 8 }));
        expect(
            zoom(model, 15),
        ).toEqual(build({ x1: -6, y1: -6, x2: 8, y2: 8 }));
        expect(
            zoom(model, 14),
        ).toEqual(build({ x1: -6, y1: -6, x2: 7, y2: 7 }));
        expect(
            zoom(model, 13),
        ).toEqual(build({ x1: -5, y1: -5, x2: 7, y2: 7 }));
        expect(
            zoom(model, 12),
        ).toEqual(build({ x1: -5, y1: -5, x2: 6, y2: 6 }));
        expect(
            zoom(model, 11),
        ).toEqual(build({ x1: -4, y1: -4, x2: 6, y2: 6 }));
        expect(
            zoom(model, 10),
        ).toEqual(build({ x1: -4, y1: -4, x2: 5, y2: 5 }));
        expect(
            zoom(model, 9),
        ).toEqual(build({ x1: -3, y1: -3, x2: 5, y2: 5 }));
        expect(
            zoom(model, 8),
        ).toEqual(build({ x1: -3, y1: -3, x2: 4, y2: 4 }));
        expect(
            zoom(model, 7),
        ).toEqual(build({ x1: -2, y1: -2, x2: 4, y2: 4 }));
        expect(
            zoom(model, 6),
        ).toEqual(build({ x1: -2, y1: -2, x2: 3, y2: 3 }));
        expect(
            zoom(model, 5),
        ).toEqual(build({ x1: -1, y1: -1, x2: 3, y2: 3 }));
        expect(
            zoom(model, 4),
        ).toEqual(build({ x1: -1, y1: -1, x2: 2, y2: 2 }));
        expect(
            zoom(model, 3),
        ).toEqual(build({ x1: -0, y1: -0, x2: 2, y2: 2 }));
        expect(
            zoom(model, 2),
        ).toEqual(build({ x1: 0, y1: 0, x2: 1, y2: 1 }));
        expect(
            zoom(model, 1),
        ).toEqual(build({ x1: 1, y1: 1, x2: 1, y2: 1 }));
    });

    it("quadrant 1 zoom", () => {
        const model = build({ x1: 10, y1: 10, x2: 19, y2: 19 });

        expect(
            zoom(model, 36),
        ).toEqual(build({ x1: -3, y1: -3, x2: 32, y2: 32 }));
        expect(
            zoom(model, 35),
        ).toEqual(build({ x1: -2, y1: -2, x2: 32, y2: 32 }));
        expect(
            zoom(model, 34),
        ).toEqual(build({ x1: -2, y1: -2, x2: 31, y2: 31 }));
        expect(
            zoom(model, 33),
        ).toEqual(build({ x1: -1, y1: -1, x2: 31, y2: 31 }));
        expect(
            zoom(model, 32),
        ).toEqual(build({ x1: -1, y1: -1, x2: 30, y2: 30 }));
        expect(
            zoom(model, 31),
        ).toEqual(build({ x1: -0, y1: -0, x2: 30, y2: 30 }));
        expect(
            zoom(model, 30),
        ).toEqual(build({ x1: 0, y1: 0, x2: 29, y2: 29 }));
        expect(
            zoom(model, 29),
        ).toEqual(build({ x1: 1, y1: 1, x2: 29, y2: 29 }));
        expect(
            zoom(model, 28),
        ).toEqual(build({ x1: 1, y1: 1, x2: 28, y2: 28 }));
        expect(
            zoom(model, 27),
        ).toEqual(build({ x1: 2, y1: 2, x2: 28, y2: 28 }));
        expect(
            zoom(model, 26),
        ).toEqual(build({ x1: 2, y1: 2, x2: 27, y2: 27 }));
        expect(
            zoom(model, 25),
        ).toEqual(build({ x1: 3, y1: 3, x2: 27, y2: 27 }));
        expect(
            zoom(model, 24),
        ).toEqual(build({ x1: 3, y1: 3, x2: 26, y2: 26 }));
        expect(
            zoom(model, 23),
        ).toEqual(build({ x1: 4, y1: 4, x2: 26, y2: 26 }));
        expect(
            zoom(model, 22),
        ).toEqual(build({ x1: 4, y1: 4, x2: 25, y2: 25 }));
        expect(
            zoom(model, 21),
        ).toEqual(build({ x1: 5, y1: 5, x2: 25, y2: 25 }));
        expect(
            zoom(model, 20),
        ).toEqual(build({ x1: 5, y1: 5, x2: 24, y2: 24 }));
        expect(
            zoom(model, 19),
        ).toEqual(build({ x1: 6, y1: 6, x2: 24, y2: 24 }));
        expect(
            zoom(model, 18),
        ).toEqual(build({ x1: 6, y1: 6, x2: 23, y2: 23 }));
        expect(
            zoom(model, 17),
        ).toEqual(build({ x1: 7, y1: 7, x2: 23, y2: 23 }));
        expect(
            zoom(model, 16),
        ).toEqual(build({ x1: 7, y1: 7, x2: 22, y2: 22 }));
        expect(
            zoom(model, 15),
        ).toEqual(build({ x1: 8, y1: 8, x2: 22, y2: 22 }));
        expect(
            zoom(model, 14),
        ).toEqual(build({ x1: 8, y1: 8, x2: 21, y2: 21 }));
        expect(
            zoom(model, 13),
        ).toEqual(build({ x1: 9, y1: 9, x2: 21, y2: 21 }));
        expect(
            zoom(model, 12),
        ).toEqual(build({ x1: 9, y1: 9, x2: 20, y2: 20 }));
        expect(
            zoom(model, 11),
        ).toEqual(build({ x1: 10, y1: 10, x2: 20, y2: 20 }));
        expect(
            zoom(model, 10),
        ).toEqual(build({ x1: 10, y1: 10, x2: 19, y2: 19 }));
        expect(
            zoom(model, 9),
        ).toEqual(build({ x1: 11, y1: 11, x2: 19, y2: 19 }));
        expect(
            zoom(model, 8),
        ).toEqual(build({ x1: 11, y1: 11, x2: 18, y2: 18 }));
        expect(
            zoom(model, 7),
        ).toEqual(build({ x1: 12, y1: 12, x2: 18, y2: 18 }));
        expect(
            zoom(model, 6),
        ).toEqual(build({ x1: 12, y1: 12, x2: 17, y2: 17 }));
        expect(
            zoom(model, 5),
        ).toEqual(build({ x1: 13, y1: 13, x2: 17, y2: 17 }));
        expect(
            zoom(model, 4),
        ).toEqual(build({ x1: 13, y1: 13, x2: 16, y2: 16 }));
        expect(
            zoom(model, 3),
        ).toEqual(build({ x1: 14, y1: 14, x2: 16, y2: 16 }));
        expect(
            zoom(model, 2),
        ).toEqual(build({ x1: 14, y1: 14, x2: 15, y2: 15 }));
        expect(
            zoom(model, 1),
        ).toEqual(build({ x1: 15, y1: 15, x2: 15, y2: 15 }));
    });

    it("quadrant 2 zoom", () => {
        const model = build({ x1: -19, y1: 10, x2: -10, y2: 19 });

        expect(
            zoom(model, 32),
        ).toEqual(build({ x1: -30, y1: -1, x2: 1, y2: 30 }));
        expect(
            zoom(model, 31),
        ).toEqual(build({ x1: -29, y1: -0, x2: 1, y2: 30 }));
        expect(
            zoom(model, 30),
        ).toEqual(build({ x1: -29, y1: 0, x2: 0, y2: 29 }));
        expect(
            zoom(model, 29),
        ).toEqual(build({ x1: -28, y1: 1, x2: -0, y2: 29 }));
        expect(
            zoom(model, 28),
        ).toEqual(build({ x1: -28, y1: 1, x2: -1, y2: 28 }));
        expect(
            zoom(model, 27),
        ).toEqual(build({ x1: -27, y1: 2, x2: -1, y2: 28 }));
        expect(
            zoom(model, 26),
        ).toEqual(build({ x1: -27, y1: 2, x2: -2, y2: 27 }));
        expect(
            zoom(model, 25),
        ).toEqual(build({ x1: -26, y1: 3, x2: -2, y2: 27 }));
        expect(
            zoom(model, 24),
        ).toEqual(build({ x1: -26, y1: 3, x2: -3, y2: 26 }));
        expect(
            zoom(model, 23),
        ).toEqual(build({ x1: -25, y1: 4, x2: -3, y2: 26 }));
        expect(
            zoom(model, 22),
        ).toEqual(build({ x1: -25, y1: 4, x2: -4, y2: 25 }));
        expect(
            zoom(model, 21),
        ).toEqual(build({ x1: -24, y1: 5, x2: -4, y2: 25 }));
        expect(
            zoom(model, 20),
        ).toEqual(build({ x1: -24, y1: 5, x2: -5, y2: 24 }));
        expect(
            zoom(model, 19),
        ).toEqual(build({ x1: -23, y1: 6, x2: -5, y2: 24 }));
        expect(
            zoom(model, 18),
        ).toEqual(build({ x1: -23, y1: 6, x2: -6, y2: 23 }));
        expect(
            zoom(model, 17),
        ).toEqual(build({ x1: -22, y1: 7, x2: -6, y2: 23 }));
        expect(
            zoom(model, 16),
        ).toEqual(build({ x1: -22, y1: 7, x2: -7, y2: 22 }));
        expect(
            zoom(model, 15),
        ).toEqual(build({ x1: -21, y1: 8, x2: -7, y2: 22 }));
        expect(
            zoom(model, 14),
        ).toEqual(build({ x1: -21, y1: 8, x2: -8, y2: 21 }));
        expect(
            zoom(model, 13),
        ).toEqual(build({ x1: -20, y1: 9, x2: -8, y2: 21 }));
        expect(
            zoom(model, 12),
        ).toEqual(build({ x1: -20, y1: 9, x2: -9, y2: 20 }));
        expect(
            zoom(model, 11),
        ).toEqual(build({ x1: -19, y1: 10, x2: -9, y2: 20 }));
        expect(
            zoom(model, 10),
        ).toEqual(build({ x1: -19, y1: 10, x2: -10, y2: 19 }));
        expect(
            zoom(model, 9),
        ).toEqual(build({ x1: -18, y1: 11, x2: -10, y2: 19 }));
        expect(
            zoom(model, 8),
        ).toEqual(build({ x1: -18, y1: 11, x2: -11, y2: 18 }));
        expect(
            zoom(model, 7),
        ).toEqual(build({ x1: -17, y1: 12, x2: -11, y2: 18 }));
        expect(
            zoom(model, 6),
        ).toEqual(build({ x1: -17, y1: 12, x2: -12, y2: 17 }));
        expect(
            zoom(model, 5),
        ).toEqual(build({ x1: -16, y1: 13, x2: -12, y2: 17 }));
        expect(
            zoom(model, 4),
        ).toEqual(build({ x1: -16, y1: 13, x2: -13, y2: 16 }));
        expect(
            zoom(model, 3),
        ).toEqual(build({ x1: -15, y1: 14, x2: -13, y2: 16 }));
        expect(
            zoom(model, 2),
        ).toEqual(build({ x1: -15, y1: 14, x2: -14, y2: 15 }));
        expect(
            zoom(model, 1),
        ).toEqual(build({ x1: -14, y1: 15, x2: -14, y2: 15 }));
    });

    it("quadrant 3 zoom", () => {
        const model = build({ x1: -19, y1: -19, x2: -10, y2: -10 });

        expect(
            zoom(model, 32),
        ).toEqual(build({ x1: -30, y1: -30, x2: 1, y2: 1 }));
        expect(
            zoom(model, 31),
        ).toEqual(build({ x1: -29, y1: -29, x2: 1, y2: 1 }));
        expect(
            zoom(model, 30),
        ).toEqual(build({ x1: -29, y1: -29, x2: 0, y2: 0 }));
        expect(
            zoom(model, 29),
        ).toEqual(build({ x1: -28, y1: -28, x2: -0, y2: -0 }));
        expect(
            zoom(model, 28),
        ).toEqual(build({ x1: -28, y1: -28, x2: -1, y2: -1 }));
        expect(
            zoom(model, 27),
        ).toEqual(build({ x1: -27, y1: -27, x2: -1, y2: -1 }));
        expect(
            zoom(model, 26),
        ).toEqual(build({ x1: -27, y1: -27, x2: -2, y2: -2 }));
        expect(
            zoom(model, 25),
        ).toEqual(build({ x1: -26, y1: -26, x2: -2, y2: -2 }));
        expect(
            zoom(model, 24),
        ).toEqual(build({ x1: -26, y1: -26, x2: -3, y2: -3 }));
        expect(
            zoom(model, 23),
        ).toEqual(build({ x1: -25, y1: -25, x2: -3, y2: -3 }));
        expect(
            zoom(model, 22),
        ).toEqual(build({ x1: -25, y1: -25, x2: -4, y2: -4 }));
        expect(
            zoom(model, 21),
        ).toEqual(build({ x1: -24, y1: -24, x2: -4, y2: -4 }));
        expect(
            zoom(model, 20),
        ).toEqual(build({ x1: -24, y1: -24, x2: -5, y2: -5 }));
        expect(
            zoom(model, 19),
        ).toEqual(build({ x1: -23, y1: -23, x2: -5, y2: -5 }));
        expect(
            zoom(model, 18),
        ).toEqual(build({ x1: -23, y1: -23, x2: -6, y2: -6 }));
        expect(
            zoom(model, 17),
        ).toEqual(build({ x1: -22, y1: -22, x2: -6, y2: -6 }));
        expect(
            zoom(model, 16),
        ).toEqual(build({ x1: -22, y1: -22, x2: -7, y2: -7 }));
        expect(
            zoom(model, 15),
        ).toEqual(build({ x1: -21, y1: -21, x2: -7, y2: -7 }));
        expect(
            zoom(model, 14),
        ).toEqual(build({ x1: -21, y1: -21, x2: -8, y2: -8 }));
        expect(
            zoom(model, 13),
        ).toEqual(build({ x1: -20, y1: -20, x2: -8, y2: -8 }));
        expect(
            zoom(model, 12),
        ).toEqual(build({ x1: -20, y1: -20, x2: -9, y2: -9 }));
        expect(
            zoom(model, 11),
        ).toEqual(build({ x1: -19, y1: -19, x2: -9, y2: -9 }));
        expect(
            zoom(model, 10),
        ).toEqual(build({ x1: -19, y1: -19, x2: -10, y2: -10 }));
        expect(
            zoom(model, 9),
        ).toEqual(build({ x1: -18, y1: -18, x2: -10, y2: -10 }));
        expect(
            zoom(model, 8),
        ).toEqual(build({ x1: -18, y1: -18, x2: -11, y2: -11 }));
        expect(
            zoom(model, 7),
        ).toEqual(build({ x1: -17, y1: -17, x2: -11, y2: -11 }));
        expect(
            zoom(model, 6),
        ).toEqual(build({ x1: -17, y1: -17, x2: -12, y2: -12 }));
        expect(
            zoom(model, 5),
        ).toEqual(build({ x1: -16, y1: -16, x2: -12, y2: -12 }));
        expect(
            zoom(model, 4),
        ).toEqual(build({ x1: -16, y1: -16, x2: -13, y2: -13 }));
        expect(
            zoom(model, 3),
        ).toEqual(build({ x1: -15, y1: -15, x2: -13, y2: -13 }));
        expect(
            zoom(model, 2),
        ).toEqual(build({ x1: -15, y1: -15, x2: -14, y2: -14 }));
        expect(
            zoom(model, 1),
        ).toEqual(build({ x1: -14, y1: -14, x2: -14, y2: -14 }));
    });

    it("quadrant 4 zoom", () => {
        const model = build({ x1: 10, y1: -19, x2: 19, y2: -10 });

        expect(
            zoom(model, 36),
        ).toEqual(build({ x1: -3, y1: -32, x2: 32, y2: 3 }));
        expect(
            zoom(model, 35),
        ).toEqual(build({ x1: -2, y1: -31, x2: 32, y2: 3 }));
        expect(
            zoom(model, 34),
        ).toEqual(build({ x1: -2, y1: -31, x2: 31, y2: 2 }));
        expect(
            zoom(model, 33),
        ).toEqual(build({ x1: -1, y1: -30, x2: 31, y2: 2 }));
        expect(
            zoom(model, 32),
        ).toEqual(build({ x1: -1, y1: -30, x2: 30, y2: 1 }));
        expect(
            zoom(model, 31),
        ).toEqual(build({ x1: -0, y1: -29, x2: 30, y2: 1 }));
        expect(
            zoom(model, 30),
        ).toEqual(build({ x1: 0, y1: -29, x2: 29, y2: 0 }));
        expect(
            zoom(model, 29),
        ).toEqual(build({ x1: 1, y1: -28, x2: 29, y2: -0 }));
        expect(
            zoom(model, 28),
        ).toEqual(build({ x1: 1, y1: -28, x2: 28, y2: -1 }));
        expect(
            zoom(model, 27),
        ).toEqual(build({ x1: 2, y1: -27, x2: 28, y2: -1 }));
        expect(
            zoom(model, 26),
        ).toEqual(build({ x1: 2, y1: -27, x2: 27, y2: -2 }));
        expect(
            zoom(model, 25),
        ).toEqual(build({ x1: 3, y1: -26, x2: 27, y2: -2 }));
        expect(
            zoom(model, 24),
        ).toEqual(build({ x1: 3, y1: -26, x2: 26, y2: -3 }));
        expect(
            zoom(model, 23),
        ).toEqual(build({ x1: 4, y1: -25, x2: 26, y2: -3 }));
        expect(
            zoom(model, 22),
        ).toEqual(build({ x1: 4, y1: -25, x2: 25, y2: -4 }));
        expect(
            zoom(model, 21),
        ).toEqual(build({ x1: 5, y1: -24, x2: 25, y2: -4 }));
        expect(
            zoom(model, 20),
        ).toEqual(build({ x1: 5, y1: -24, x2: 24, y2: -5 }));
        expect(
            zoom(model, 19),
        ).toEqual(build({ x1: 6, y1: -23, x2: 24, y2: -5 }));
        expect(
            zoom(model, 18),
        ).toEqual(build({ x1: 6, y1: -23, x2: 23, y2: -6 }));
        expect(
            zoom(model, 17),
        ).toEqual(build({ x1: 7, y1: -22, x2: 23, y2: -6 }));
        expect(
            zoom(model, 16),
        ).toEqual(build({ x1: 7, y1: -22, x2: 22, y2: -7 }));
        expect(
            zoom(model, 15),
        ).toEqual(build({ x1: 8, y1: -21, x2: 22, y2: -7 }));
        expect(
            zoom(model, 14),
        ).toEqual(build({ x1: 8, y1: -21, x2: 21, y2: -8 }));
        expect(
            zoom(model, 13),
        ).toEqual(build({ x1: 9, y1: -20, x2: 21, y2: -8 }));
        expect(
            zoom(model, 12),
        ).toEqual(build({ x1: 9, y1: -20, x2: 20, y2: -9 }));
        expect(
            zoom(model, 11),
        ).toEqual(build({ x1: 10, y1: -19, x2: 20, y2: -9 }));
        expect(
            zoom(model, 10),
        ).toEqual(build({ x1: 10, y1: -19, x2: 19, y2: -10 }));
        expect(
            zoom(model, 9),
        ).toEqual(build({ x1: 11, y1: -18, x2: 19, y2: -10 }));
        expect(
            zoom(model, 8),
        ).toEqual(build({ x1: 11, y1: -18, x2: 18, y2: -11 }));
        expect(
            zoom(model, 7),
        ).toEqual(build({ x1: 12, y1: -17, x2: 18, y2: -11 }));
        expect(
            zoom(model, 6),
        ).toEqual(build({ x1: 12, y1: -17, x2: 17, y2: -12 }));
        expect(
            zoom(model, 5),
        ).toEqual(build({ x1: 13, y1: -16, x2: 17, y2: -12 }));
        expect(
            zoom(model, 4),
        ).toEqual(build({ x1: 13, y1: -16, x2: 16, y2: -13 }));
        expect(
            zoom(model, 3),
        ).toEqual(build({ x1: 14, y1: -15, x2: 16, y2: -13 }));
        expect(
            zoom(model, 2),
        ).toEqual(build({ x1: 14, y1: -15, x2: 15, y2: -14 }));
        expect(
            zoom(model, 1),
        ).toEqual(build({ x1: 15, y1: -14, x2: 15, y2: -14 }));
    });
});

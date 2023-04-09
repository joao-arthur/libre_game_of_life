import { describe, expect, it } from "vitest";
import { indexToPoint } from "./indexToPoint.js";

describe("indexToPoint", () => {
    it("1x1 grid", () => {
        expect(
            indexToPoint({ row: 0, col: 0 }, 1),
        ).toEqual({ x: 1, y: 1 });
    });

    it("3x3 grid", () => {
        expect(
            indexToPoint({ row: 0, col: 0 }, 3),
        ).toEqual({ x: -1, y: 2 });
        expect(
            indexToPoint({ row: 0, col: 1 }, 3),
        ).toEqual({ x: 1, y: 2 });
        expect(
            indexToPoint({ row: 0, col: 2 }, 3),
        ).toEqual({ x: 2, y: 2 });

        expect(
            indexToPoint({ row: 1, col: 0 }, 3),
        ).toEqual({ x: -1, y: 1 });
        expect(
            indexToPoint({ row: 1, col: 1 }, 3),
        ).toEqual({ x: 1, y: 1 });
        expect(
            indexToPoint({ row: 1, col: 2 }, 3),
        ).toEqual({ x: 2, y: 1 });

        expect(
            indexToPoint({ row: 2, col: 0 }, 3),
        ).toEqual({ x: -1, y: -1 });
        expect(
            indexToPoint({ row: 2, col: 1 }, 3),
        ).toEqual({ x: 1, y: -1 });
        expect(
            indexToPoint({ row: 2, col: 2 }, 3),
        ).toEqual({ x: 2, y: -1 });
    });

    it("2x2 grid", () => {
        expect(
            indexToPoint({ row: 0, col: 0 }, 2),
        ).toEqual({ x: -1, y: 1 });
        expect(
            indexToPoint({ row: 0, col: 1 }, 2),
        ).toEqual({ x: 1, y: 1 });

        expect(
            indexToPoint({ row: 1, col: 0 }, 2),
        ).toEqual({ x: -1, y: -1 });
        expect(
            indexToPoint({ row: 1, col: 1 }, 2),
        ).toEqual({ x: 1, y: -1 });
    });

    it("4x4 grid", () => {
        expect(
            indexToPoint({ row: 0, col: 0 }, 4),
        ).toEqual({ x: -2, y: 2 });
        expect(
            indexToPoint({ row: 0, col: 1 }, 4),
        ).toEqual({ x: -1, y: 2 });
        expect(
            indexToPoint({ row: 0, col: 2 }, 4),
        ).toEqual({ x: 1, y: 2 });
        expect(
            indexToPoint({ row: 0, col: 3 }, 4),
        ).toEqual({ x: 2, y: 2 });

        expect(
            indexToPoint({ row: 1, col: 0 }, 4),
        ).toEqual({ x: -2, y: 1 });
        expect(
            indexToPoint({ row: 1, col: 1 }, 4),
        ).toEqual({ x: -1, y: 1 });
        expect(
            indexToPoint({ row: 1, col: 2 }, 4),
        ).toEqual({ x: 1, y: 1 });
        expect(
            indexToPoint({ row: 1, col: 3 }, 4),
        ).toEqual({ x: 2, y: 1 });

        expect(
            indexToPoint({ row: 2, col: 0 }, 4),
        ).toEqual({ x: -2, y: -1 });
        expect(
            indexToPoint({ row: 2, col: 1 }, 4),
        ).toEqual({ x: -1, y: -1 });
        expect(
            indexToPoint({ row: 2, col: 2 }, 4),
        ).toEqual({ x: 1, y: -1 });
        expect(
            indexToPoint({ row: 2, col: 3 }, 4),
        ).toEqual({ x: 2, y: -1 });

        expect(
            indexToPoint({ row: 3, col: 0 }, 4),
        ).toEqual({ x: -2, y: -2 });
        expect(
            indexToPoint({ row: 3, col: 1 }, 4),
        ).toEqual({ x: -1, y: -2 });
        expect(
            indexToPoint({ row: 3, col: 2 }, 4),
        ).toEqual({ x: 1, y: -2 });
        expect(
            indexToPoint({ row: 3, col: 3 }, 4),
        ).toEqual({ x: 2, y: -2 });
    });
});

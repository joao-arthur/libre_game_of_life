import { describe, expect, it } from "vitest";
import { pointToIndex } from "./pointToIndex.js";

describe("pointToIndex", () => {
    it("1x1 grid", () => {
        expect(
            pointToIndex({ x: 0, y: 0 }, 1),
        ).toEqual({ row: 0, col: 0 });
    });

    it("3x3 grid", () => {
        expect(
            pointToIndex({ x: -1, y: 1 }, 3),
        ).toEqual({ row: 0, col: 0 });
        expect(
            pointToIndex({ x: 0, y: 1 }, 3),
        ).toEqual({ row: 0, col: 1 });
        expect(
            pointToIndex({ x: 1, y: 1 }, 3),
        ).toEqual({ row: 0, col: 2 });

        expect(
            pointToIndex({ x: -1, y: 0 }, 3),
        ).toEqual({ row: 1, col: 0 });
        expect(
            pointToIndex({ x: 0, y: 0 }, 3),
        ).toEqual({ row: 1, col: 1 });
        expect(
            pointToIndex({ x: 1, y: 0 }, 3),
        ).toEqual({ row: 1, col: 2 });

        expect(
            pointToIndex({ x: -1, y: -1 }, 3),
        ).toEqual({ row: 2, col: 0 });
        expect(
            pointToIndex({ x: 0, y: -1 }, 3),
        ).toEqual({ row: 2, col: 1 });
        expect(
            pointToIndex({ x: 1, y: -1 }, 3),
        ).toEqual({ row: 2, col: 2 });
    });

    it("2x2 grid", () => {
        expect(
            pointToIndex({ x: -1, y: 1 }, 2),
        ).toEqual({ row: 0, col: 0 });
        expect(
            pointToIndex({ x: 0, y: 1 }, 2),
        ).toEqual({ row: 0, col: 1 });

        expect(
            pointToIndex({ x: -1, y: 0 }, 2),
        ).toEqual({ row: 1, col: 0 });
        expect(
            pointToIndex({ x: 0, y: 0 }, 2),
        ).toEqual({ row: 1, col: 1 });
    });

    it("4x4 grid", () => {
        expect(
            pointToIndex({ x: -2, y: 2 }, 4),
        ).toEqual({ row: 0, col: 0 });
        expect(
            pointToIndex({ x: -1, y: 2 }, 4),
        ).toEqual({ row: 0, col: 1 });
        expect(
            pointToIndex({ x: 0, y: 2 }, 4),
        ).toEqual({ row: 0, col: 2 });
        expect(
            pointToIndex({ x: 1, y: 2 }, 4),
        ).toEqual({ row: 0, col: 3 });

        expect(
            pointToIndex({ x: -2, y: 1 }, 4),
        ).toEqual({ row: 1, col: 0 });
        expect(
            pointToIndex({ x: -1, y: 1 }, 4),
        ).toEqual({ row: 1, col: 1 });
        expect(
            pointToIndex({ x: 0, y: 1 }, 4),
        ).toEqual({ row: 1, col: 2 });
        expect(
            pointToIndex({ x: 1, y: 1 }, 4),
        ).toEqual({ row: 1, col: 3 });

        expect(
            pointToIndex({ x: -2, y: 0 }, 4),
        ).toEqual({ row: 2, col: 0 });
        expect(
            pointToIndex({ x: -1, y: 0 }, 4),
        ).toEqual({ row: 2, col: 1 });
        expect(
            pointToIndex({ x: 0, y: 0 }, 4),
        ).toEqual({ row: 2, col: 2 });
        expect(
            pointToIndex({ x: 1, y: 0 }, 4),
        ).toEqual({ row: 2, col: 3 });

        expect(
            pointToIndex({ x: -2, y: -1 }, 4),
        ).toEqual({ row: 3, col: 0 });
        expect(
            pointToIndex({ x: -1, y: -1 }, 4),
        ).toEqual({ row: 3, col: 1 });
        expect(
            pointToIndex({ x: 0, y: -1 }, 4),
        ).toEqual({ row: 3, col: 2 });
        expect(
            pointToIndex({ x: 1, y: -1 }, 4),
        ).toEqual({ row: 3, col: 3 });
    });
});

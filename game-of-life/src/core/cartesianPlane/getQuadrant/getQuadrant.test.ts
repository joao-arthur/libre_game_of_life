import { describe, expect, it } from "vitest";
import { getQuadrant } from "./getQuadrant.js";

describe("describe", () => {
    it("1x1 grid", () => {
        expect(getQuadrant({ row: 0, col: 0 }, 1)).toBe(1);
    });

    it("3x3 grid", () => {
        expect(getQuadrant({ row: 0, col: 0 }, 3)).toBe(2);
        expect(getQuadrant({ row: 0, col: 1 }, 3)).toBe(1);
        expect(getQuadrant({ row: 0, col: 2 }, 3)).toBe(1);

        expect(getQuadrant({ row: 1, col: 0 }, 3)).toBe(2);
        expect(getQuadrant({ row: 1, col: 1 }, 3)).toBe(1);
        expect(getQuadrant({ row: 1, col: 2 }, 3)).toBe(1);

        expect(getQuadrant({ row: 2, col: 0 }, 3)).toBe(3);
        expect(getQuadrant({ row: 2, col: 1 }, 3)).toBe(4);
        expect(getQuadrant({ row: 2, col: 2 }, 3)).toBe(4);
    });

    it("2x2 grid", () => {
        expect(getQuadrant({ row: 0, col: 1 }, 2)).toBe(1);
        expect(getQuadrant({ row: 0, col: 0 }, 2)).toBe(2);

        expect(getQuadrant({ row: 1, col: 0 }, 2)).toBe(3);
        expect(getQuadrant({ row: 1, col: 1 }, 2)).toBe(4);
    });

    it("4x4 grid", () => {
        expect(getQuadrant({ row: 0, col: 0 }, 4)).toBe(2);
        expect(getQuadrant({ row: 0, col: 1 }, 4)).toBe(2);
        expect(getQuadrant({ row: 0, col: 2 }, 4)).toBe(1);
        expect(getQuadrant({ row: 0, col: 3 }, 4)).toBe(1);

        expect(getQuadrant({ row: 1, col: 0 }, 4)).toBe(2);
        expect(getQuadrant({ row: 1, col: 1 }, 4)).toBe(2);
        expect(getQuadrant({ row: 1, col: 2 }, 4)).toBe(1);
        expect(getQuadrant({ row: 1, col: 3 }, 4)).toBe(1);

        expect(getQuadrant({ row: 2, col: 0 }, 4)).toBe(3);
        expect(getQuadrant({ row: 2, col: 1 }, 4)).toBe(3);
        expect(getQuadrant({ row: 2, col: 2 }, 4)).toBe(4);
        expect(getQuadrant({ row: 2, col: 3 }, 4)).toBe(4);

        expect(getQuadrant({ row: 3, col: 0 }, 4)).toBe(3);
        expect(getQuadrant({ row: 3, col: 1 }, 4)).toBe(3);
        expect(getQuadrant({ row: 3, col: 2 }, 4)).toBe(4);
        expect(getQuadrant({ row: 3, col: 3 }, 4)).toBe(4);
    });
});

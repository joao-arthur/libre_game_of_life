import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { getValue } from "./getValue.js";

describe("getValue", ()=> {
    it("Value out of range", () => {
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: -1, row: 0 },
            )).toBe(
            undefined,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: -1, row: 0 },
            )).toBe(
            undefined,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 0, row: -1 },
            )).toBe(
            undefined,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 2, row: 0 },
            )).toBe(
            undefined,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 0, row: 2 },
            )).toBe(
            undefined,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 2, row: 2 },
            )).toBe(
            undefined,
        );
    });
    
    it("Value in range", () => {
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 0, row: 0 },
            )).toBe(
            stateType.DEAD,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 0, row: 1 },
            )).toBe(
            stateType.ALIVE,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 1, row: 0 },
            )).toBe(
            stateType.DEAD,
        );
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { column: 1, row: 1 },
            )).toBe(
            stateType.ALIVE,
        );
    });
});

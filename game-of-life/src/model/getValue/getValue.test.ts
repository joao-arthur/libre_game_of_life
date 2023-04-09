import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { getValue } from "./getValue.js";

describe("getValue", () => {
    it("Value out of range", () => {
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -100, y: -100 },
            ),
        ).toBe(stateType.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -100, y: 100 },
            ),
        ).toBe(stateType.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 100, y: -100 },
            ),
        ).toBe(stateType.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 100, y: 100 },
            ),
        ).toBe(stateType.DEAD);
    });

    it("Value in range", () => {
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 0, y: 1 },
            ),
        ).toBe(stateType.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -1, y: 1 },
            ),
        ).toBe(stateType.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -1, y: 0 },
            ),
        ).toBe(stateType.ALIVE);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 0, y: 0 },
            ),
        ).toBe(stateType.ALIVE);
    });
});

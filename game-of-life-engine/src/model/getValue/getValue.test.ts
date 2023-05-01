import { describe, expect, it } from "vitest";
import { State } from "../../cell/mod.js";
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
        ).toBe(State.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -100, y: 100 },
            ),
        ).toBe(State.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 100, y: -100 },
            ),
        ).toBe(State.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 100, y: 100 },
            ),
        ).toBe(State.DEAD);
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
        ).toBe(State.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -1, y: 1 },
            ),
        ).toBe(State.DEAD);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: -1, y: 0 },
            ),
        ).toBe(State.ALIVE);
        expect(
            getValue(
                fromString([
                    "⬛⬛",
                    "⬜⬜",
                ]),
                { x: 0, y: 0 },
            ),
        ).toBe(State.ALIVE);
    });
});

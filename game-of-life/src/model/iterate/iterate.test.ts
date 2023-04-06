import { describe, expect, it } from "vitest";
import { fromString } from "../fromString/mod.js";
import { iterate } from "./iterate.js";

describe("iterate", ()=> {
    it("iterate", () => {
        expect(
            iterate(
                fromString(["⬜"]),
            )).toEqual(
            {
                ...fromString(["⬛"]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬜⬜",
                    "⬜⬜",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬜⬜",
                    "⬜⬜",
                ]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬛⬜⬛",
                    "⬛⬜⬛",
                    "⬛⬜⬛",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬛⬛⬛",
                    "⬜⬜⬜",
                    "⬛⬛⬛",
                ]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬛⬛⬛",
                    "⬜⬜⬜",
                    "⬛⬛⬛",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬛⬜⬛",
                    "⬛⬜⬛",
                    "⬛⬜⬛",
                ]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬛⬛⬜",
                    "⬜⬜⬜",
                    "⬛⬛⬛",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬛⬛⬜",
                    "⬛⬜⬜",
                    "⬛⬜⬛",
                ]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬛⬛⬜",
                    "⬛⬜⬜",
                    "⬛⬜⬛",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬛⬜⬜",
                    "⬛⬜⬜",
                    "⬛⬜⬜",
                ]),
                iteration: 1,
            },
        );
        expect(
            iterate(
                fromString([
                    "⬜⬜⬛",
                    "⬜⬜⬜",
                    "⬛⬜⬛",
                ]),
            )).toEqual(
            {
                ...fromString([
                    "⬜⬛⬜",
                    "⬛⬛⬜",
                    "⬜⬜⬜",
                ]),
                iteration: 1,
            },
        );
    });
        
});

import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { forEach } from "./forEach.js";

describe("forEach", ()=> {
    it("forEach", ()=> {
        const values1: stateType[][] = [[]];
        forEach(
            fromString([
                "⬛⬜",
                "⬜⬛",
                "⬛⬜",
            ]),
            (_, state) => values1[0].push(state),
        );
        expect(
            values1).toEqual(
            fromString([
                "⬛⬜⬜⬛⬛⬜",
            ]).value,
        );
    });

    it("forEach", ()=> {
        const values: stateType[][] = [[]];
        forEach(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
            ]),
            (_, state) => values[0].push(state),
        );
        expect(
            values).toEqual(
            fromString(["⬛⬜⬛⬜⬛⬜"]).value,
        );
    });

    it("forEach", ()=> {
        const values: stateType[][] = [[]];
        forEach(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
                "⬛⬜⬛",
            ]),
            (_, state) => values[0].push(state),
        );
        expect(
            values).toEqual(
            fromString(["⬛⬜⬛⬜⬛⬜⬛⬜⬛"]).value,
        );
    });

});

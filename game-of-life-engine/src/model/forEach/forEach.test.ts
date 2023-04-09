import { describe, expect, it } from "vitest";
import { cartesianPointType } from "../../core/cartesianPlane/cartesianPoint.js";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { forEach } from "./forEach.js";

describe("forEach", () => {
    it("forEach", () => {
        const values: [cartesianPointType, stateType.ALIVE][] = [];
        forEach(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬛",
                "⬛⬜⬛",
            ]),
            (point, state) => {
                if (state === stateType.ALIVE) {
                    values.push([point, state]);
                }
            },
        );
        expect(values).toEqual([
            [{ x: -1, y: 0 }, stateType.ALIVE],
            [{ x: 0, y: -1 }, stateType.ALIVE],
            [{ x: 0, y: 1 }, stateType.ALIVE],
        ]);
    });

    it("forEach", () => {
        const values: [cartesianPointType, stateType.ALIVE][] = [];
        forEach(
            fromString([
                "⬛⬜",
                "⬜⬛",
            ]),
            (point, state) => {
                if (state === stateType.ALIVE) {
                    values.push([point, state]);
                }
            },
        );
        expect(values).toEqual([
            [{ x: -1, y: 0 }, stateType.ALIVE],
            [{ x: 0, y: 1 }, stateType.ALIVE],
        ]);
    });

    it("forEach", () => {
        const values: [cartesianPointType, stateType.ALIVE][] = [];
        forEach(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
                "⬛⬜⬛",
            ]),
            (point, state) => {
                if (state === stateType.ALIVE) {
                    values.push([point, state]);
                }
            },
        );
        expect(values).toEqual([
            [{ x: -1, y: 0 }, stateType.ALIVE],
            [{ x: 0, y: -1 }, stateType.ALIVE],
            [{ x: 0, y: 1 }, stateType.ALIVE],
            [{ x: 1, y: 0 }, stateType.ALIVE],
        ]);
    });
});

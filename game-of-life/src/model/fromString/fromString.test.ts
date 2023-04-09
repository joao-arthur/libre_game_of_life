import { expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "./fromString.js";

it("fromString", () => {
    //    expect(
    //        fromString([""]),
    //    ).toEqual(
    //        {
    //            value: new Map(),
    //            iteration: 0,
    //            position: { x: 0, y: 0 },
    //            size: 20,
    //        },
    //    );
    //    expect(
    //        fromString(["⬛"]),
    //    ).toEqual(
    //        {
    //            value: new Map(),
    //            iteration: 0,
    //            position: { x: 0, y: 0 },
    //            size: 20,
    //        },
    //    );
    //    expect(
    //        fromString(["⬜"]),
    //    ).toEqual(
    //        {
    //            value: new Map([["1;1", stateType.ALIVE]]),
    //            iteration: 0,
    //            position: { x: 0, y: 0 },
    //            size: 20,
    //        },
    //    );
    expect(
        fromString([
            "⬛⬛⬛⬜",
            "⬜⬛⬛⬛",
            "⬛⬛⬜⬛",
            "⬛⬛⬛⬛",
        ]),
    ).toEqual(
        {
            value: new Map([
                ["(x: 2, y: 2)", stateType.ALIVE],
                ["(x: -2, y: 1)", stateType.ALIVE],
                ["(x: 1, y: -1)", stateType.ALIVE],
            ]),
            iteration: 0,
            position: { x: 0, y: 0 },
            size: 20,
        },
    );
    //    expect(
    //        fromString([
    //            "⬜ ⬜",
    //            "⬜ ⬛",
    //        ]),
    //    ).toEqual(
    //        {
    //            value: [
    //                [stateType.ALIVE, stateType.ALIVE],
    //                [stateType.ALIVE, stateType.DEAD],
    //            ],
    //            iteration: 0,
    //            position: { x: 0, y: 0 },
    //            size: 20,
    //        },
    //    );
});

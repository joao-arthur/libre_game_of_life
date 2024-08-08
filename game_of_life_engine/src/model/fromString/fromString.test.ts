import { expect, it } from "vitest";
import { State } from "../../cell/mod.js";
import { fromString } from "./fromString.js";

it("fromString", () => {
    expect(
        fromString([""]),
    ).toEqual({
        value: new Map(),
        iteration: 0,
        position: { x1: -10, y1: -10, x2: 10, y2: 10 },
    });
    expect(
        fromString(["⬛"]),
    ).toEqual({
        value: new Map(),
        iteration: 0,
        position: { x1: -10, y1: -10, x2: 10, y2: 10 },
    });
    expect(
        fromString(["⬜"]),
    ).toEqual({
        value: new Map([["(x: 0, y: 0)", State.ALIVE]]),
        iteration: 0,
        position: { x1: -10, y1: -10, x2: 10, y2: 10 },
    });
    expect(
        fromString([
            "⬛⬛⬛⬜",
            "⬜⬛⬛⬛",
            "⬛⬛⬜⬛",
            "⬛⬛⬛⬛",
        ]),
    ).toEqual({
        value: new Map([
            ["(x: 1, y: 2)", State.ALIVE],
            ["(x: -2, y: 1)", State.ALIVE],
            ["(x: 0, y: 0)", State.ALIVE],
        ]),
        iteration: 0,
        position: { x1: -10, y1: -10, x2: 10, y2: 10 },
    });
});

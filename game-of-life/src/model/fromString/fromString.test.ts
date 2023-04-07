import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "./fromString.js";

it("fromString", () => {
    expect(
        fromString([""]),
    ).toEqual(
        {
            size: 0,
            value: [[]],
            iteration: 0,
        },
    );
    expect(
        fromString(["⬛"]),
    ).toEqual(
        {
            size: 1,
            value: [[stateType.DEAD]],
            iteration: 0,
        },
    );
    expect(
        fromString(["⬜"]),
    ).toEqual(
        {
            size: 1,
            value: [[stateType.ALIVE]],
            iteration: 0,
        },
    );
    expect(
        fromString([
            "⬜⬜",
            "⬜⬛",
        ]),
    ).toEqual(
        {
            size: 2,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
            ],
            iteration: 0,
        },
    );
    expect(
        fromString([
            "⬜ ⬜",
            "⬜ ⬛",
        ]),
    ).toEqual(
        {
            size: 2,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
            ],
            iteration: 0,
        },
    );
});

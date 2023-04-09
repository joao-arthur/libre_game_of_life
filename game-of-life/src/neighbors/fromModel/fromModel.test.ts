import { expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../../model/fromString/mod.js";
import { fromModel } from "./fromModel.js";

it("fromModel", () => {
    expect(
        fromModel(
            fromString([
                "⬛⬜",
                "⬛⬜",
            ]),
            { x: -1, y: 1 },
        ),
    ).toEqual(
        [
            stateType.DEAD,
            stateType.DEAD,
            stateType.DEAD,

            stateType.DEAD,
            stateType.ALIVE,

            stateType.DEAD,
            stateType.DEAD,
            stateType.ALIVE,
        ],
    );
    expect(
        fromModel(
            fromString([
                "⬛⬜⬜",
                "⬛⬜⬜",
                "⬛⬜⬛",
            ]),
            { x: 0, y: 0 },
        ),
    ).toEqual(
        [
            stateType.DEAD,
            stateType.ALIVE,
            stateType.ALIVE,

            stateType.DEAD,
            stateType.ALIVE,

            stateType.DEAD,
            stateType.ALIVE,
            stateType.DEAD,
        ],
    );
});

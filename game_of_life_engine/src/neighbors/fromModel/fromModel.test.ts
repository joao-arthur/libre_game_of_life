import { expect, it } from "vitest";
import { State } from "../../cell/mod.js";
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
            State.DEAD,
            State.DEAD,
            State.DEAD,

            State.DEAD,
            State.ALIVE,

            State.DEAD,
            State.DEAD,
            State.ALIVE,
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
            State.DEAD,
            State.ALIVE,
            State.ALIVE,

            State.DEAD,
            State.ALIVE,

            State.DEAD,
            State.ALIVE,
            State.DEAD,
        ],
    );
});

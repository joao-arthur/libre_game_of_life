import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../fromString/mod.js";
import { map } from "./map.js";

it("map", () => {
    expect(
        map(
            fromString([
                "⬛⬜",
                "⬜⬛",
                "⬛⬜",
            ]),
            ({ row }) => row > 0 ? stateType.DEAD : stateType.ALIVE,
        ),
    ).toEqual(
        fromString([
            "⬜⬜",
            "⬛⬛",
            "⬛⬛",
        ]),
    );
    expect(
        map(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
            ]),
            ({ column }) =>
                column > 0 ? stateType.DEAD : stateType.ALIVE,
        ),
    ).toEqual(
        fromString([
            "⬜⬛⬛",
            "⬜⬛⬛",
        ]),
    );
    expect(
        map(
            fromString([
                "⬛⬜⬛",
                "⬜⬛⬜",
                "⬛⬜⬛",
            ]),
            ({ row, column }) =>
                row > 1 || column > 1
                    ? stateType.DEAD
                    : stateType.ALIVE,
        ),
    ).toEqual(
        fromString([
            "⬜⬜⬛",
            "⬜⬜⬛",
            "⬛⬛⬛",
        ]),
    );
});

import { expect, it } from "vitest";
import { fromString } from "../fromString/mod.js";
import { toggle } from "./toggle.js";

const model = fromString([
    "⬛⬛⬛⬛",
    "⬛⬛⬛⬛",
    "⬜⬜⬜⬜",
    "⬜⬜⬜⬜",
]);

it("toggle", () => {
    expect(
        toggle(model, { column: 0, row: 0 }),
    ).toEqual(
        fromString([
            "⬜⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(
        toggle(model, { column: 1, row: 1 }),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬜⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(
        toggle(model, { column: 2, row: 2 }),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬛⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(
        toggle(model, { column: 3, row: 3 }),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬛",
        ]),
    );
    expect(
        toggle(model, { column: 3, row: 0 }),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬜",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(
        toggle(model, { column: 0, row: 3 }),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬛⬜⬜⬜",
        ]),
    );
});

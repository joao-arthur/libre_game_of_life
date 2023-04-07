import { expect, it } from "vitest";
import { fromString } from "../fromString/mod.js";
import { zoom } from "./zoom.js";

const model = fromString([
    "⬛⬜⬛",
    "⬜⬜⬜",
    "⬜⬛⬜",
]);

it("zoom", () => {
    expect(
        zoom(model, 1),
    ).toEqual(
        fromString([
            "⬜",
        ]),
    );
    expect(
        zoom(model, 2),
    ).toEqual(
        fromString([
            "⬜",
        ]),
    );
    expect(
        zoom(model, 3),
    ).toEqual(
        fromString([
            "⬛⬜⬛",
            "⬜⬜⬜",
            "⬜⬛⬜",
        ]),
    );
    expect(
        zoom(model, 4),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬜⬛⬜⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
    expect(
        zoom(model, 5),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬜⬜⬜⬛",
            "⬛⬜⬛⬜⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
});

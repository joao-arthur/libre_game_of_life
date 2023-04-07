import { describe, expect, it } from "vitest";
import { fromString } from "../fromString/mod.js";
import { zoomIn } from "./zoomIn.js";

const model = fromString([
    "⬛⬛⬛⬛⬛⬛⬛",
    "⬛⬜⬜⬜⬜⬜⬛",
    "⬛⬜⬜⬛⬛⬜⬛",
    "⬛⬜⬛⬜⬛⬜⬛",
    "⬛⬜⬛⬜⬛⬜⬛",
    "⬛⬜⬜⬜⬜⬜⬛",
    "⬛⬛⬛⬛⬛⬛⬛",
]);

it("zoomIn", () => {
    expect(
        zoomIn(model, 1),
    ).toEqual(
        fromString([
            "⬜⬜⬜⬜⬜",
            "⬜⬜⬛⬛⬜",
            "⬜⬛⬜⬛⬜",
            "⬜⬛⬜⬛⬜",
            "⬜⬜⬜⬜⬜",
        ]),
    );
    expect(
        zoomIn(model, 2),
    ).toEqual(
        fromString([
            "⬜⬛⬛",
            "⬛⬜⬛",
            "⬛⬜⬛",
        ]),
    );
    expect(
        zoomIn(model, 3),
    ).toEqual(
        fromString([
            "⬜",
        ]),
    );
    expect(
        zoomIn(model, 4),
    ).toEqual(
        fromString([""]),
    );
    expect(
        zoomIn(
            fromString([
                "⬛⬛",
                "⬛⬛",
            ]),
            1,
        ),
    ).toEqual(
        fromString([""]),
    );
});

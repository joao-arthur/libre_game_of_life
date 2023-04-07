import { expect, it } from "vitest";
import { fromString } from "../fromString/mod.js";
import { zoomOut } from "./zoomOut.js";

const model = fromString(["⬜"]);

it("zoomOut", () => {
    expect(
        zoomOut(model, 1),
    ).toEqual(
        fromString([
            "⬛⬛⬛",
            "⬛⬜⬛",
            "⬛⬛⬛",
        ]),
    );
    expect(
        zoomOut(model, 2),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬜⬛⬛",
            "⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛",
        ]),
    );
    expect(
        zoomOut(model, 3),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬜⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
            "⬛⬛⬛⬛⬛⬛⬛",
        ]),
    );
    expect(
        zoomOut(fromString([""]), 2),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
        ]),
    );
    expect(
        zoomOut(
            fromString([
                "⬜⬜",
                "⬛⬜",
            ]),
            1,
        ),
    ).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬜⬜⬛",
            "⬛⬛⬜⬛",
            "⬛⬛⬛⬛",
        ]),
    );
});

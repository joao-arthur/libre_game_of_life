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
    expect(toggle(model, { x: -2, y: 2 })).toEqual(
        fromString([
            "⬜⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(toggle(model, { x: -1, y: 1 })).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬜⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(toggle(model, { x: 1, y: -1 })).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬛⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(toggle(model, { x: 2, y: -2 })).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬛",
        ]),
    );
    expect(toggle(model, { x: 2, y: 2 })).toEqual(
        fromString([
            "⬛⬛⬛⬜",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬜⬜⬜⬜",
        ]),
    );
    expect(toggle(model, { x: -2, y: -2 })).toEqual(
        fromString([
            "⬛⬛⬛⬛",
            "⬛⬛⬛⬛",
            "⬜⬜⬜⬜",
            "⬛⬜⬜⬜",
        ]),
    );
});

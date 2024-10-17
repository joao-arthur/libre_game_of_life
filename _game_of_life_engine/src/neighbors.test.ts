import { assert, test } from "vitest";
import { modelFromString } from "./model.js";
import { State } from "./cell.js";
import { pointFrom } from "./cartesianPlane.js";
import {
    aliveNeighborsFromModel,
    neighborsFromModel,
    numberOfAliveNeighbors,
} from "./neighbors.js";

test("aliveNeighborsFromModel", () => {
    const model2x2 = modelFromString([
        "⬛⬜",
        "⬛⬜",
    ]);
    assert.deepStrictEqual(aliveNeighborsFromModel(model2x2, pointFrom(-1, 1)), 2);
});

test("neighborsFromModel", () => {
    const model2x2 = modelFromString([
        "⬛⬜",
        "⬛⬜",
    ]);
    const model3x3 = modelFromString([
        "⬛⬜⬜",
        "⬛⬜⬜",
        "⬛⬜⬛",
    ]);

    assert.deepStrictEqual(
        neighborsFromModel(model2x2, pointFrom(-1, 1)),
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
    assert.deepStrictEqual(
        neighborsFromModel(model3x3, pointFrom(0, 0)),
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

test("numberOfAliveNeighbors", () => {
    assert.deepStrictEqual(
        numberOfAliveNeighbors([
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
            undefined,
        ]),
        0,
    );
    assert.deepStrictEqual(
        numberOfAliveNeighbors([
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
        ]),
        0,
    );
    assert.deepStrictEqual(
        numberOfAliveNeighbors([
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
            State.ALIVE,
        ]),
        8,
    );
    assert.deepStrictEqual(
        numberOfAliveNeighbors([
            State.ALIVE,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            State.DEAD,
            undefined,
        ]),
        1,
    );
});

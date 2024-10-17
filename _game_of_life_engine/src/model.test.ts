import { assert, test } from "vitest";
import { State } from "./cell.js";
import { pointFrom, rectFrom } from "./cartesianPlane.js";
import {
    getModelCellSize,
    getModelLength,
    getModelMiddleCell,
    getModelMiddlePoint,
    getModelValue,
    iterateModel,
    modelDefault,
    modelFromPos,
    modelFromString,
    moveModel,
    toggleModel,
    zoomModel,
} from "./model.js";

test("modelDefault", () => {
    assert.deepStrictEqual(
        modelDefault(),
        {
            value: new Map(),
            iter: 0,
            pos: rectFrom(0, 0, 0, 0),
        },
    );
});

test("modelFromPos", () => {
    assert.deepStrictEqual(
        modelFromPos(rectFrom(-30, 48, -2, 178)),
        {
            value: new Map(),
            iter: 0,
            pos: rectFrom(-30, 48, -2, 178),
        },
    );
});

test("modelFromString", () => {
    const modelEmpty = modelFromString([""]);
    const modelModel1x1Dead = modelFromString(["⬛"]);
    const modelModel1x1Alive = modelFromString(["⬜"]);
    const modelModel4x4 = modelFromString([
        "⬛⬛⬛⬜",
        "⬜⬛⬛⬛",
        "⬛⬛⬜⬛",
        "⬛⬛⬛⬛",
    ]);

    assert.deepStrictEqual(modelEmpty, {
        value: new Map(),
        iter: 0,
        pos: rectFrom(-10, -10, 10, 10),
    });
    assert.deepStrictEqual(modelModel1x1Dead, {
        value: new Map(),
        iter: 0,
        pos: rectFrom(-10, -10, 10, 10),
    });
    assert.deepStrictEqual(modelModel1x1Alive, {
        value: new Map([["(0, 0)", State.ALIVE]]),
        iter: 0,
        pos: rectFrom(-10, -10, 10, 10),
    });
    assert.deepStrictEqual(
        modelModel4x4,
        {
            value: new Map([
                ["(1, 2)", State.ALIVE],
                ["(-2, 1)", State.ALIVE],
                ["(0, 0)", State.ALIVE],
            ]),
            iter: 0,
            pos: rectFrom(-10, -10, 10, 10),
        },
    );
});

test("getModelCellSize", () => {
    const m = modelFromPos(rectFrom(1, 1, 10, 10));
    assert.deepStrictEqual(getModelCellSize(m, 100), 10);
    assert.deepStrictEqual(getModelCellSize(m, 90), 9);
    assert.deepStrictEqual(getModelCellSize(m, 50), 5);
    assert.deepStrictEqual(getModelCellSize(m, 10), 1);
});

test("getModelLength", () => {
    assert.deepStrictEqual(getModelLength(modelFromPos(rectFrom(-10, -10, 10, 10))), 21);
    assert.deepStrictEqual(getModelLength(modelFromPos(rectFrom(1, 1, 10, 10))), 10);
    assert.deepStrictEqual(getModelLength(modelFromPos(rectFrom(4, 4, 5, 5))), 2);
    assert.deepStrictEqual(getModelLength(modelFromPos(rectFrom(5, 5, 5, 5))), 1);
});

test("getModelMiddleCell", () => {
    assert.deepStrictEqual(
        getModelMiddleCell(modelFromPos(rectFrom(-10, -10, 10, 10)), 100),
        pointFrom(0, 0),
    );
    assert.deepStrictEqual(
        getModelMiddleCell(modelFromPos(rectFrom(1, 1, 10, 10)), 50),
        pointFrom(27.5, 27.5),
    );
    assert.deepStrictEqual(
        getModelMiddleCell(modelFromPos(rectFrom(4, 4, 5, 5)), 10),
        pointFrom(22.5, 22.5),
    );
    assert.deepStrictEqual(
        getModelMiddleCell(modelFromPos(rectFrom(5, 5, 5, 5)), 1),
        pointFrom(5, 5),
    );
});

test("getModelMiddlePoint", () => {
    assert.deepStrictEqual(
        getModelMiddlePoint(modelFromPos(rectFrom(-10, -10, 10, 10))),
        pointFrom(0, 0),
    );
    assert.deepStrictEqual(
        getModelMiddlePoint(modelFromPos(rectFrom(1, 1, 10, 10))),
        pointFrom(5.5, 5.5),
    );
    assert.deepStrictEqual(
        getModelMiddlePoint(modelFromPos(rectFrom(4, 4, 5, 5))),
        pointFrom(4.5, 4.5),
    );
    assert.deepStrictEqual(
        getModelMiddlePoint(modelFromPos(rectFrom(5, 5, 5, 5))),
        pointFrom(5, 5),
    );
});

test("getModelValue", () => {
    const m = modelFromString([
        "⬛⬛",
        "⬜⬜",
    ]);

    assert.deepStrictEqual(getModelValue(m, pointFrom(-2, -2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(-1, -2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(0, -2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(1, -2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(2, -2)), State.DEAD);

    assert.deepStrictEqual(getModelValue(m, pointFrom(-2, -1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(-1, -1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(0, -1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(1, -1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(2, -1)), State.DEAD);

    assert.deepStrictEqual(getModelValue(m, pointFrom(-2, 0)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(-1, 0)), State.ALIVE);
    assert.deepStrictEqual(getModelValue(m, pointFrom(0, 0)), State.ALIVE);
    assert.deepStrictEqual(getModelValue(m, pointFrom(1, 0)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(2, 0)), State.DEAD);

    assert.deepStrictEqual(getModelValue(m, pointFrom(-2, 1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(-1, 1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(0, 1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(1, 1)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(2, 1)), State.DEAD);

    assert.deepStrictEqual(getModelValue(m, pointFrom(-2, 2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(-1, 2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(0, 2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(1, 2)), State.DEAD);
    assert.deepStrictEqual(getModelValue(m, pointFrom(2, 2)), State.DEAD);
});

test("iterateModel", () => {
    const model1x1iter0 = modelFromString(["⬜"]);
    const model1x1iter1 = modelFromString(["⬛"]);

    const model2x2iter0 = modelFromString([
        "⬜⬜",
        "⬜⬜",
    ]);
    const model2x2iter1 = modelFromString([
        "⬜⬜",
        "⬜⬜",
    ]);

    const model3x3_1_iter0 = modelFromString([
        "⬛⬜⬛",
        "⬛⬜⬛",
        "⬛⬜⬛",
    ]);
    const model3x3_1_iter1 = modelFromString([
        "⬛⬛⬛",
        "⬜⬜⬜",
        "⬛⬛⬛",
    ]);

    const model3x3_2_iter0 = modelFromString([
        "⬛⬛⬛",
        "⬜⬜⬜",
        "⬛⬛⬛",
    ]);
    const model3x3_2_iter1 = modelFromString([
        "⬛⬜⬛",
        "⬛⬜⬛",
        "⬛⬜⬛",
    ]);

    const model3x3_3_iter0 = modelFromString([
        "⬛⬛⬜",
        "⬜⬜⬜",
        "⬛⬛⬛",
    ]);
    const model3x3_3_iter1 = modelFromString([
        "⬛⬛⬜",
        "⬛⬜⬜",
        "⬛⬜⬛",
    ]);

    const model3x3_4_iter0 = modelFromString([
        "⬛⬛⬜",
        "⬛⬜⬜",
        "⬛⬜⬛",
    ]);
    const model3x3_4_iter1 = modelFromString([
        "⬛⬜⬜",
        "⬛⬜⬜",
        "⬛⬜⬜",
    ]);

    const model3x3_5_iter0 = modelFromString([
        "⬜⬜⬛",
        "⬜⬜⬜",
        "⬛⬜⬛",
    ]);
    const model3x3_5_iter1 = modelFromString([
        "⬜⬛⬜",
        "⬛⬛⬜",
        "⬜⬜⬜",
    ]);

    assert.deepStrictEqual(iterateModel(model1x1iter0), { ...model1x1iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model2x2iter0), { ...model2x2iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model3x3_1_iter0), { ...model3x3_1_iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model3x3_2_iter0), { ...model3x3_2_iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model3x3_3_iter0), { ...model3x3_3_iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model3x3_4_iter0), { ...model3x3_4_iter1, iter: 1 });
    assert.deepStrictEqual(iterateModel(model3x3_5_iter0), { ...model3x3_5_iter1, iter: 1 });
});

test("moveModel", () => {
    const m = modelFromPos(rectFrom(-10, -10, 10, 10));
    assert.deepStrictEqual(moveModel(m, pointFrom(1, 0)), modelFromPos(rectFrom(-9, -10, 11, 10)));
    assert.deepStrictEqual(moveModel(m, pointFrom(-1, 0)), modelFromPos(rectFrom(-11, -10, 9, 10)));
    assert.deepStrictEqual(moveModel(m, pointFrom(0, 1)), modelFromPos(rectFrom(-10, -9, 10, 11)));
    assert.deepStrictEqual(moveModel(m, pointFrom(0, -1)), modelFromPos(rectFrom(-10, -11, 10, 9)));
    assert.deepStrictEqual(moveModel(m, pointFrom(11, 0)), modelFromPos(rectFrom(1, -10, 21, 10)));
    assert.deepStrictEqual(
        moveModel(m, pointFrom(-11, 0)),
        modelFromPos(rectFrom(-21, -10, -1, 10)),
    );
    assert.deepStrictEqual(moveModel(m, pointFrom(0, 11)), modelFromPos(rectFrom(-10, 1, 10, 21)));
    assert.deepStrictEqual(
        moveModel(m, pointFrom(0, -11)),
        modelFromPos(rectFrom(-10, -21, 10, -1)),
    );
});

test("toggleModel", () => {
    const model = modelFromString([
        "⬛⬛⬛⬛",
        "⬛⬛⬛⬛",
        "⬜⬜⬜⬜",
        "⬜⬜⬜⬜",
    ]);
    const modelToggle1 = modelFromString([
        "⬜⬛⬛⬛",
        "⬛⬛⬛⬛",
        "⬜⬜⬜⬜",
        "⬜⬜⬜⬜",
    ]);
    const modelToggle2 = modelFromString([
        "⬛⬛⬛⬛",
        "⬛⬜⬛⬛",
        "⬜⬜⬜⬜",
        "⬜⬜⬜⬜",
    ]);
    const modelToggle3 = modelFromString([
        "⬛⬛⬛⬛",
        "⬛⬛⬛⬛",
        "⬜⬜⬛⬜",
        "⬜⬜⬜⬜",
    ]);
    const modelToggle4 = modelFromString([
        "⬛⬛⬛⬛",
        "⬛⬛⬛⬛",
        "⬜⬜⬜⬜",
        "⬜⬜⬜⬛",
    ]);
    const modelToggle5 = modelFromString([
        "⬛⬛⬛⬜",
        "⬛⬛⬛⬛",
        "⬜⬜⬜⬜",
        "⬜⬜⬜⬜",
    ]);
    const modelToggle6 = modelFromString([
        "⬛⬛⬛⬛",
        "⬛⬛⬛⬛",
        "⬜⬜⬜⬜",
        "⬛⬜⬜⬜",
    ]);
    assert.deepStrictEqual(toggleModel(model, pointFrom(-2, 2)), modelToggle1);
    assert.deepStrictEqual(toggleModel(model, pointFrom(-1, 1)), modelToggle2);
    assert.deepStrictEqual(toggleModel(model, pointFrom(0, 0)), modelToggle3);
    assert.deepStrictEqual(toggleModel(model, pointFrom(1, -1)), modelToggle4);
    assert.deepStrictEqual(toggleModel(model, pointFrom(1, 2)), modelToggle5);
    assert.deepStrictEqual(toggleModel(model, pointFrom(-2, -1)), modelToggle6);
});

test("zoom odd size", () => {
    const m = modelFromPos(rectFrom(-10, -10, 10, 10));
    assert.deepStrictEqual(zoomModel(m, 1), modelFromPos(rectFrom(0, 0, 0, 0)));
    assert.deepStrictEqual(zoomModel(m, 2), modelFromPos(rectFrom(-1, -1, 0, 0)));
    assert.deepStrictEqual(zoomModel(m, 3), modelFromPos(rectFrom(-1, -1, 1, 1)));
    assert.deepStrictEqual(zoomModel(m, 19), modelFromPos(rectFrom(-9, -9, 9, 9)));
    assert.deepStrictEqual(zoomModel(m, 21), modelFromPos(rectFrom(-10, -10, 10, 10)));
    assert.deepStrictEqual(zoomModel(m, 23), modelFromPos(rectFrom(-11, -11, 11, 11)));
});

test("zoom even size", () => {
    const m = modelFromPos(rectFrom(10, 10, 19, 19));
    assert.deepStrictEqual(zoomModel(m, 1), modelFromPos(rectFrom(14, 14, 14, 14)));
    assert.deepStrictEqual(zoomModel(m, 2), modelFromPos(rectFrom(14, 14, 15, 15)));
    assert.deepStrictEqual(zoomModel(m, 3), modelFromPos(rectFrom(13, 13, 15, 15)));
    assert.deepStrictEqual(zoomModel(m, 8), modelFromPos(rectFrom(11, 11, 18, 18)));
    assert.deepStrictEqual(zoomModel(m, 10), modelFromPos(rectFrom(10, 10, 19, 19)));
    assert.deepStrictEqual(zoomModel(m, 12), modelFromPos(rectFrom(9, 9, 20, 20)));
});

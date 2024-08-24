import { assert, test } from "vitest";
import { arrPosFrom } from "./array.js";
import {
    absoluteToRelative,
    deserializePoint,
    indexToPoint,
    pointFrom,
    pointToIndex,
    rectFrom,
    relativeToAbsolute,
    serializePoint,
} from "./cartesianPlane.js";

test("rectFrom", () => {
    assert.deepStrictEqual(rectFrom(20, -13, 874, -174), { x1: 20, y1: -13, x2: 874, y2: -174 });
});

test("pointFrom", () => {
    assert.deepStrictEqual(pointFrom(20, -13), { x: 20, y: -13 });
});

test("deserializePoint", () => {
    assert.deepStrictEqual(deserializePoint("(0, 0)"), pointFrom(0, 0));
    assert.deepStrictEqual(deserializePoint("(-1, -1)"), pointFrom(-1, -1));
    assert.deepStrictEqual(deserializePoint("(1, 1)"), pointFrom(1, 1));
    assert.deepStrictEqual(deserializePoint("(3, 6)"), pointFrom(3, 6));
});

test("serializePoint", () => {
    assert.deepStrictEqual(serializePoint({ x: 0, y: 0 }), "(0, 0)");
    assert.deepStrictEqual(serializePoint({ x: -1, y: -1 }), "(-1, -1)");
    assert.deepStrictEqual(serializePoint({ x: 1, y: 1 }), "(1, 1)");
    assert.deepStrictEqual(serializePoint({ x: 3, y: 6 }), "(3, 6)");
});

test("indexToPoint 1x1 grid", () => {
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 0), 1), pointFrom(0, 0));
});

test("indexToPoint 2x2 grid", () => {
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 0), 2), pointFrom(-1, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 1), 2), pointFrom(0, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 0), 2), pointFrom(-1, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 1), 2), pointFrom(0, 0));
});

test("indexToPoint 3x3 grid", () => {
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 0), 3), pointFrom(-1, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 1), 3), pointFrom(0, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 2), 3), pointFrom(1, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 0), 3), pointFrom(-1, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 1), 3), pointFrom(0, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 2), 3), pointFrom(1, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 0), 3), pointFrom(-1, -1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 1), 3), pointFrom(0, -1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 2), 3), pointFrom(1, -1));
});

test("indexToPoint 4x4 grid", () => {
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 0), 4), pointFrom(-2, 2));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 1), 4), pointFrom(-1, 2));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 2), 4), pointFrom(0, 2));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(0, 3), 4), pointFrom(1, 2));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 0), 4), pointFrom(-2, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 1), 4), pointFrom(-1, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 2), 4), pointFrom(0, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(1, 3), 4), pointFrom(1, 1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 0), 4), pointFrom(-2, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 1), 4), pointFrom(-1, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 2), 4), pointFrom(0, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(2, 3), 4), pointFrom(1, 0));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(3, 0), 4), pointFrom(-2, -1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(3, 1), 4), pointFrom(-1, -1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(3, 2), 4), pointFrom(0, -1));
    assert.deepStrictEqual(indexToPoint(arrPosFrom(3, 3), 4), pointFrom(1, -1));
});

test("pointToIndex 1x1 grid", () => {
    assert.deepStrictEqual(pointToIndex({ x: 0, y: 0 }, 1), arrPosFrom(0, 0));
});

test("pointToIndex 2x2 grid", () => {
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 1), 2), arrPosFrom(0, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 1), 2), arrPosFrom(0, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 0), 2), arrPosFrom(1, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 0), 2), arrPosFrom(1, 1));
});

test("pointToIndex 3x3 grid", () => {
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 1), 3), arrPosFrom(0, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 1), 3), arrPosFrom(0, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, 1), 3), arrPosFrom(0, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 0), 3), arrPosFrom(1, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 0), 3), arrPosFrom(1, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, 0), 3), arrPosFrom(1, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, -1), 3), arrPosFrom(2, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, -1), 3), arrPosFrom(2, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, -1), 3), arrPosFrom(2, 2));
});

test("pointToIndex 4x4 grid", () => {
    assert.deepStrictEqual(pointToIndex(pointFrom(-2, 2), 4), arrPosFrom(0, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 2), 4), arrPosFrom(0, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 2), 4), arrPosFrom(0, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, 2), 4), arrPosFrom(0, 3));
    assert.deepStrictEqual(pointToIndex(pointFrom(-2, 1), 4), arrPosFrom(1, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 1), 4), arrPosFrom(1, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 1), 4), arrPosFrom(1, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, 1), 4), arrPosFrom(1, 3));
    assert.deepStrictEqual(pointToIndex(pointFrom(-2, 0), 4), arrPosFrom(2, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, 0), 4), arrPosFrom(2, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, 0), 4), arrPosFrom(2, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, 0), 4), arrPosFrom(2, 3));
    assert.deepStrictEqual(pointToIndex(pointFrom(-2, -1), 4), arrPosFrom(3, 0));
    assert.deepStrictEqual(pointToIndex(pointFrom(-1, -1), 4), arrPosFrom(3, 1));
    assert.deepStrictEqual(pointToIndex(pointFrom(0, -1), 4), arrPosFrom(3, 2));
    assert.deepStrictEqual(pointToIndex(pointFrom(1, -1), 4), arrPosFrom(3, 3));
});

test("absoluteToRelative", () => {
    assert.deepStrictEqual(absoluteToRelative(0, 30), 0);
    assert.deepStrictEqual(absoluteToRelative(10, 30), 0);
    assert.deepStrictEqual(absoluteToRelative(20, 30), 0);
    assert.deepStrictEqual(absoluteToRelative(29, 30), 0);
    assert.deepStrictEqual(absoluteToRelative(30, 30), 1);
    assert.deepStrictEqual(absoluteToRelative(300, 30), 10);
});

test("relativeToAbsolute", () => {
    assert.deepStrictEqual(relativeToAbsolute(0, 30), 0);
    assert.deepStrictEqual(relativeToAbsolute(1, 30), 30);
    assert.deepStrictEqual(relativeToAbsolute(10, 30), 300);
});

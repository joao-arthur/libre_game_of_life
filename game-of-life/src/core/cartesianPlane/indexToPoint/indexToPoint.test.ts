import { expect, it } from "vitest";
import { indexToPoint } from "./indexToPoint.js";

it("indexToPoint", () => {
    expect(
        indexToPoint({ position: { row: 0, col: 0 }, length: 4 }),
    ).toEqual({ x: -2, y: 2 });
    expect(
        indexToPoint({ position: { row: 0, col: 1 }, length: 4 }),
    ).toEqual({ x: -1, y: 2 });
    expect(
        indexToPoint({ position: { row: 0, col: 2 }, length: 4 }),
    ).toEqual({ x: 1, y: 2 });
    expect(
        indexToPoint({ position: { row: 0, col: 3 }, length: 4 }),
    ).toEqual({ x: 2, y: 2 });

    expect(
        indexToPoint({ position: { row: 1, col: 0 }, length: 4 }),
    ).toEqual({ x: -2, y: 1 });
    expect(
        indexToPoint({ position: { row: 1, col: 1 }, length: 4 }),
    ).toEqual({ x: -1, y: 1 });
    expect(
        indexToPoint({ position: { row: 1, col: 2 }, length: 4 }),
    ).toEqual({ x: 1, y: 1 });
    expect(
        indexToPoint({ position: { row: 1, col: 3 }, length: 4 }),
    ).toEqual({ x: 2, y: 1 });

    expect(
        indexToPoint({ position: { row: 2, col: 0 }, length: 4 }),
    ).toEqual({ x: -2, y: -1 });
    expect(
        indexToPoint({ position: { row: 2, col: 1 }, length: 4 }),
    ).toEqual({ x: -1, y: -1 });
    expect(
        indexToPoint({ position: { row: 2, col: 2 }, length: 4 }),
    ).toEqual({ x: 1, y: -1 });
    expect(
        indexToPoint({ position: { row: 2, col: 3 }, length: 4 }),
    ).toEqual({ x: 2, y: -1 });

    expect(
        indexToPoint({ position: { row: 3, col: 0 }, length: 4 }),
    ).toEqual({ x: -2, y: -2 });
    expect(
        indexToPoint({ position: { row: 3, col: 1 }, length: 4 }),
    ).toEqual({ x: -1, y: -2 });
    expect(
        indexToPoint({ position: { row: 3, col: 2 }, length: 4 }),
    ).toEqual({ x: 1, y: -2 });
    expect(
        indexToPoint({ position: { row: 3, col: 3 }, length: 4 }),
    ).toEqual({ x: 2, y: -2 });

    expect(
        indexToPoint({ position: { row: 0, col: 0 }, length: 2 }),
    ).toEqual({ x: -1, y: 1 });
    expect(
        indexToPoint({ position: { row: 0, col: 1 }, length: 2 }),
    ).toEqual({ x: 1, y: 1 });
    expect(
        indexToPoint({ position: { row: 1, col: 0 }, length: 2 }),
    ).toEqual({ x: -1, y: -1 });
    expect(
        indexToPoint({ position: { row: 1, col: 1 }, length: 2 }),
    ).toEqual({ x: 1, y: -1 });
});

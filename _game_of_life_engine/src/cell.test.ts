import { assert, test } from "vitest";
import { iterateCell, State, toggleCell } from "./cell.js";

test("toggleCell", () => {
    assert.deepStrictEqual(toggleCell(State.ALIVE), State.DEAD);
    assert.deepStrictEqual(toggleCell(State.DEAD), State.ALIVE);
});

test("iterate any live cell with fewer than two live neighbours dies", () => {
    assert.deepStrictEqual(iterateCell(State.ALIVE, 0), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 1), State.DEAD);
});

test("iterate any live cell with two or three live neighbours lives", () => {
    assert.deepStrictEqual(iterateCell(State.ALIVE, 2), State.ALIVE);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 3), State.ALIVE);
});

test("iterate any live cell with more than three live neighbours dies", () => {
    assert.deepStrictEqual(iterateCell(State.ALIVE, 4), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 5), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 6), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 7), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.ALIVE, 8), State.DEAD);
});

test("iterate any dead cell with exactly three live neighbours becomes a live cell", () => {
    assert.deepStrictEqual(iterateCell(State.DEAD, 0), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 1), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 2), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 3), State.ALIVE);
    assert.deepStrictEqual(iterateCell(State.DEAD, 4), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 5), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 6), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 7), State.DEAD);
    assert.deepStrictEqual(iterateCell(State.DEAD, 8), State.DEAD);
});

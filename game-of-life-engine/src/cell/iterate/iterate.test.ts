import { describe, expect, it } from "vitest";
import { State } from "../state.js";
import { iterate } from "./iterate.js";

describe("iterate", () => {
    it("Any live cell with fewer than two live neighbours dies", () => {
        expect(iterate(State.ALIVE, 0)).toBe(State.DEAD);
        expect(iterate(State.ALIVE, 1)).toBe(State.DEAD);
    });

    it("Any live cell with two or three live neighbours lives", () => {
        expect(iterate(State.ALIVE, 2)).toBe(State.ALIVE);
        expect(iterate(State.ALIVE, 3)).toBe(State.ALIVE);
    });

    it("Any live cell with more than three live neighbours dies", () => {
        expect(iterate(State.ALIVE, 4)).toBe(State.DEAD);
        expect(iterate(State.ALIVE, 5)).toBe(State.DEAD);
        expect(iterate(State.ALIVE, 6)).toBe(State.DEAD);
        expect(iterate(State.ALIVE, 7)).toBe(State.DEAD);
        expect(iterate(State.ALIVE, 8)).toBe(State.DEAD);
    });

    it("Any dead cell with exactly three live neighbours becomes a live cell", () => {
        expect(iterate(State.DEAD, 0)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 1)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 2)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 3)).toBe(State.ALIVE);
        expect(iterate(State.DEAD, 4)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 5)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 6)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 7)).toBe(State.DEAD);
        expect(iterate(State.DEAD, 8)).toBe(State.DEAD);
    });
});

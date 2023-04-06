import { describe, expect, it } from "vitest";
import { stateType } from "../state.js";
import { iterate } from "./iterate.js";

describe("iterate", () => {
    it("Any live cell with fewer than two live neighbours dies", () => {
        expect(iterate(stateType.ALIVE, 0)).toBe(stateType.DEAD);
        expect(iterate(stateType.ALIVE, 1)).toBe(stateType.DEAD);
    });

    it("Any live cell with two or three live neighbours lives", () => {
        expect(iterate(stateType.ALIVE, 2)).toBe(stateType.ALIVE);
        expect(iterate(stateType.ALIVE, 3)).toBe(stateType.ALIVE);
    });

    it("Any live cell with more than three live neighbours dies", () => {
        expect(iterate(stateType.ALIVE, 4)).toBe(stateType.DEAD);
        expect(iterate(stateType.ALIVE, 5)).toBe(stateType.DEAD);
        expect(iterate(stateType.ALIVE, 6)).toBe(stateType.DEAD);
        expect(iterate(stateType.ALIVE, 7)).toBe(stateType.DEAD);
        expect(iterate(stateType.ALIVE, 8)).toBe(stateType.DEAD);
    });

    it("Any dead cell with exactly three live neighbours becomes a live cell", () => {
        expect(iterate(stateType.DEAD, 0)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 1)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 2)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 3)).toBe(stateType.ALIVE);
        expect(iterate(stateType.DEAD, 4)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 5)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 6)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 7)).toBe(stateType.DEAD);
        expect(iterate(stateType.DEAD, 8)).toBe(stateType.DEAD);
    });
});

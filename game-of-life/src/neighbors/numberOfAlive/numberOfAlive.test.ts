import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/state.js";
import { numberOfAlive } from "./numberOfAlive.js";

describe("numberOfAlive", ()=>{
    it("Should return the number of alive neighbors", () => {
        expect(
            numberOfAlive([
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
                undefined,
            ]),
        ).toBe(
            0,
        );
        expect(
            numberOfAlive([
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
            ])).toBe(
            0,
        );
        expect(
            numberOfAlive([
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
                stateType.ALIVE,
            ])).toBe(
            8,
        );
        expect(
            numberOfAlive([
                stateType.ALIVE,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                stateType.DEAD,
                undefined,
            ])).toBe(
            1,
        );
    });
});

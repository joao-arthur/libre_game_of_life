import { describe, expect, it } from "vitest";
import { stateType } from "../../cell/mod.js";
import { fromString } from "../../model/fromString/mod.js";
import { fromModel } from "./fromModel.js";

describe("fromModel", ()=> {
    it("Should return the neighbors of the cell", () => {
        expect(
            fromModel(
                fromString([
                    "⬛⬜",
                    "⬛⬜",
                ]),
                { column: 0, row: 0 },
            )).toEqual(
            [
                undefined,
                undefined,
                undefined,
    
                undefined,
                stateType.ALIVE,
    
                undefined,
                stateType.DEAD,
                stateType.ALIVE,
            ],
        );
        expect(
            fromModel(
                fromString([
                    "⬛⬜⬜",
                    "⬛⬜⬜",
                    "⬛⬜⬛",
                ]),
                { column: 1, row: 1 },
            )).toEqual(
            [
                stateType.DEAD,
                stateType.ALIVE,
                stateType.ALIVE,
    
                stateType.DEAD,
                stateType.ALIVE,
    
                stateType.DEAD,
                stateType.ALIVE,
                stateType.DEAD,
            ],
        );
    });
});

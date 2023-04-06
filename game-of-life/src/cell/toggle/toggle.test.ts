import { describe, expect, it } from "vitest";
import { stateType } from "../state.js";
import { toggle } from "./toggle.js";

describe("toggle", ()=> {
    it("Should toggle cell", () => {
        expect(toggle(stateType.ALIVE)).toBe(stateType.DEAD);
        expect(toggle(stateType.DEAD)).toBe(stateType.ALIVE);
    });
});

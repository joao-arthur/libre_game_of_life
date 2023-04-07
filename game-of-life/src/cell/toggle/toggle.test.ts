import { describe, expect, it } from "vitest";
import { stateType } from "../state.js";
import { toggle } from "./toggle.js";

it("toggle", () => {
    expect(toggle(stateType.ALIVE)).toBe(stateType.DEAD);
    expect(toggle(stateType.DEAD)).toBe(stateType.ALIVE);
});

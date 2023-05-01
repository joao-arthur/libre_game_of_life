import { expect, it } from "vitest";
import { State } from "../state.js";
import { toggle } from "./toggle.js";

it("toggle", () => {
    expect(toggle(State.ALIVE)).toBe(State.DEAD);
    expect(toggle(State.DEAD)).toBe(State.ALIVE);
});

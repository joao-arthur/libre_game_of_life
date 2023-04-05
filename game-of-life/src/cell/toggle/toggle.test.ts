import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../state.js";
import { toggle } from "./toggle.js";

Deno.test("Should toggle cell", () => {
    assertEquals(toggle(stateType.ALIVE), stateType.DEAD);
    assertEquals(toggle(stateType.DEAD), stateType.ALIVE);
});

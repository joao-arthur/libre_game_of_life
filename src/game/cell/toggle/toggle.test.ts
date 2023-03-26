import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../state.ts";
import { toggle } from "./toggle.ts";

Deno.test("Should toggle cell", () => {
    assertEquals(toggle(stateType.ALIVE), stateType.DEAD);
    assertEquals(toggle(stateType.DEAD), stateType.ALIVE);
});

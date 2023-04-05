import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.js";
import { stateType } from "../state.js";
import { iterate } from "./iterate.js";

Deno.test("Any live cell with fewer than two live neighbours dies", () => {
    assertEquals(iterate(stateType.ALIVE, 0), stateType.DEAD);
    assertEquals(iterate(stateType.ALIVE, 1), stateType.DEAD);
});

Deno.test("Any live cell with two or three live neighbours lives", () => {
    assertEquals(iterate(stateType.ALIVE, 2), stateType.ALIVE);
    assertEquals(iterate(stateType.ALIVE, 3), stateType.ALIVE);
});

Deno.test("Any live cell with more than three live neighbours dies", () => {
    assertEquals(iterate(stateType.ALIVE, 4), stateType.DEAD);
    assertEquals(iterate(stateType.ALIVE, 5), stateType.DEAD);
    assertEquals(iterate(stateType.ALIVE, 6), stateType.DEAD);
    assertEquals(iterate(stateType.ALIVE, 7), stateType.DEAD);
    assertEquals(iterate(stateType.ALIVE, 8), stateType.DEAD);
});

Deno.test("Any dead cell with exactly three live neighbours becomes a live cell", () => {
    assertEquals(iterate(stateType.DEAD, 0), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 1), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 2), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 3), stateType.ALIVE);
    assertEquals(iterate(stateType.DEAD, 4), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 5), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 6), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 7), stateType.DEAD);
    assertEquals(iterate(stateType.DEAD, 8), stateType.DEAD);
});

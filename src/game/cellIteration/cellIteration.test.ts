import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../state.ts";
import { cellIteration } from "./cellIteration.ts";

Deno.test("Any live cell with fewer than two live neighbours dies", () => {
    assertEquals(cellIteration(stateType.ALIVE, 0), stateType.DEAD);
    assertEquals(cellIteration(stateType.ALIVE, 1), stateType.DEAD);
});

Deno.test("Any live cell with two or three live neighbours lives", () => {
    assertEquals(cellIteration(stateType.ALIVE, 2), stateType.ALIVE);
    assertEquals(cellIteration(stateType.ALIVE, 3), stateType.ALIVE);
});

Deno.test("Any live cell with more than three live neighbours dies", () => {
    assertEquals(cellIteration(stateType.ALIVE, 4), stateType.DEAD);
    assertEquals(cellIteration(stateType.ALIVE, 5), stateType.DEAD);
    assertEquals(cellIteration(stateType.ALIVE, 6), stateType.DEAD);
    assertEquals(cellIteration(stateType.ALIVE, 7), stateType.DEAD);
    assertEquals(cellIteration(stateType.ALIVE, 8), stateType.DEAD);
});

Deno.test("Any dead cell with exactly three live neighbours becomes a live cell", () => {
    assertEquals(cellIteration(stateType.DEAD, 0), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 1), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 2), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 3), stateType.ALIVE);
    assertEquals(cellIteration(stateType.DEAD, 4), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 5), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 6), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 7), stateType.DEAD);
    assertEquals(cellIteration(stateType.DEAD, 8), stateType.DEAD);
});

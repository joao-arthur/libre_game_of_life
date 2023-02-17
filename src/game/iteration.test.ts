import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { iteration } from "./iteration.ts";
import { status } from "./status.ts";

Deno.test("Any live cell with fewer than two live neighbours dies", () => {
    assertEquals(iteration(status.ALIVE, 0), status.DEAD);
    assertEquals(iteration(status.ALIVE, 1), status.DEAD);
});

Deno.test("Any live cell with two or three live neighbours lives", () => {
    assertEquals(iteration(status.ALIVE, 2), status.ALIVE);
    assertEquals(iteration(status.ALIVE, 3), status.ALIVE);
});

Deno.test("Any live cell with more than three live neighbours dies", () => {
    assertEquals(iteration(status.ALIVE, 4), status.DEAD);
    assertEquals(iteration(status.ALIVE, 5), status.DEAD);
    assertEquals(iteration(status.ALIVE, 6), status.DEAD);
    assertEquals(iteration(status.ALIVE, 7), status.DEAD);
    assertEquals(iteration(status.ALIVE, 8), status.DEAD);
});

Deno.test("Any dead cell with exactly three live neighbours becomes a live cell", () => {
    assertEquals(iteration(status.DEAD, 0), status.DEAD);
    assertEquals(iteration(status.DEAD, 1), status.DEAD);
    assertEquals(iteration(status.DEAD, 2), status.DEAD);
    assertEquals(iteration(status.DEAD, 3), status.ALIVE);
    assertEquals(iteration(status.DEAD, 4), status.DEAD);
    assertEquals(iteration(status.DEAD, 5), status.DEAD);
    assertEquals(iteration(status.DEAD, 6), status.DEAD);
    assertEquals(iteration(status.DEAD, 7), status.DEAD);
    assertEquals(iteration(status.DEAD, 8), status.DEAD);
});

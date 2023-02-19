import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../state.ts";
import { modelIteration } from "./modelIteration.ts";

Deno.test("mapModel", () => {
    assertEquals(
        modelIteration({
            width: 1,
            height: 1,
            values: [[stateType.ALIVE]],
        }),
        { width: 1, height: 1, values: [[stateType.DEAD]] },
    );
    assertEquals(
        modelIteration({
            width: 2,
            height: 2,
            values: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }),
        {
            width: 2,
            height: 2,
            values: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
    assertEquals(
        modelIteration({
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        modelIteration({
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        modelIteration({
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        modelIteration({
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
    assertEquals(
        modelIteration({
            width: 3,
            height: 3,
            values: [
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.ALIVE, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
});

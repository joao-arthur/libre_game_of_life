import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../state.ts";
import { getNeighbors } from "./getNeighbors.ts";

Deno.test("getNeighbors", () => {
    assertEquals(
        getNeighbors(
            {
                width: 2,
                height: 2,
                values: [
                    [stateType.DEAD, stateType.ALIVE],
                    [stateType.DEAD, stateType.ALIVE],
                ],
            },
            { column: 0, row: 0 },
        ),
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
    assertEquals(
        getNeighbors(
            {
                width: 3,
                height: 3,
                values: [
                    [
                        stateType.DEAD,
                        stateType.ALIVE,
                        stateType.ALIVE,
                    ],
                    [
                        stateType.DEAD,
                        stateType.ALIVE,
                        stateType.ALIVE,
                    ],
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                ],
            },
            { column: 1, row: 1 },
        ),
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

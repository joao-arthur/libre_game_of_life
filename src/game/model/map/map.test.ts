import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { map } from "./map.ts";

Deno.test("map", () => {
    assertEquals(
        map({
            width: 2,
            height: 3,
            value: [
                [stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE],
            ],
        }, ({ row }) => row > 0 ? stateType.DEAD : stateType.ALIVE),
        {
            width: 2,
            height: 3,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD],
                [stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        map(
            {
                width: 3,
                height: 2,
                value: [
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                    [
                        stateType.ALIVE,
                        stateType.DEAD,
                        stateType.ALIVE,
                    ],
                ],
            },
            ({ column }) =>
                column > 0 ? stateType.DEAD : stateType.ALIVE,
        ),
        {
            width: 3,
            height: 2,
            value: [
                [stateType.ALIVE, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        map(
            {
                width: 3,
                height: 3,
                value: [
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                    [
                        stateType.ALIVE,
                        stateType.DEAD,
                        stateType.ALIVE,
                    ],
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                ],
            },
            ({ row, column }) =>
                row > 1 || column > 1
                    ? stateType.DEAD
                    : stateType.ALIVE,
        ),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
});

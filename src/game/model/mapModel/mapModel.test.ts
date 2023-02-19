import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../state.ts";
import { mapModel } from "./mapModel.ts";

Deno.test("mapModel", () => {
    assertEquals(
        mapModel({
            width: 2,
            height: 3,
            values: [
                [stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE],
            ],
        }, ({ row }) => row > 0 ? stateType.DEAD : stateType.ALIVE),
        {
            width: 2,
            height: 3,
            values: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD],
                [stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        mapModel(
            {
                width: 3,
                height: 2,
                values: [
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
            values: [
                [stateType.ALIVE, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        mapModel(
            {
                width: 3,
                height: 3,
                values: [
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                    [
                        stateType.ALIVE,
                        stateType.DEAD,
                        stateType.ALIVE,
                    ],
                    [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                ],
            },
            ({ row, column }) => row > 1 || column > 1
                ? stateType.DEAD
                : stateType.ALIVE,
        ),
        {
            width: 3,
            height: 3,
            values: [
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
});

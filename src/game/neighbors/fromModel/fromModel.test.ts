import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { fromModel } from "./fromModel.ts";

Deno.test("fromModel", () => {
    assertEquals(
        fromModel(
            {
                width: 2,
                height: 2,
                value: [
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
        fromModel(
            {
                width: 3,
                height: 3,
                value: [
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

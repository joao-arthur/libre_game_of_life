import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { aliveFromModel } from "./aliveFromModel.ts";

Deno.test("aliveFromModel", () => {
    assertEquals(
        aliveFromModel(
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
        2,
    );
});

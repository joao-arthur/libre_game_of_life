import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { iterate } from "./iterate.ts";

Deno.test("iterate", () => {
    assertEquals(
        iterate({
            width: 1,
            height: 1,
            value: [[stateType.ALIVE]],
        }),
        { width: 1, height: 1, value: [[stateType.DEAD]] },
    );
    assertEquals(
        iterate({
            width: 2,
            height: 2,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }),
        {
            width: 2,
            height: 2,
            value: [
                [stateType.ALIVE, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
    assertEquals(
        iterate({
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        iterate({
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        iterate({
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        },
    );
    assertEquals(
        iterate({
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
    assertEquals(
        iterate({
            width: 3,
            height: 3,
            value: [
                [stateType.ALIVE, stateType.ALIVE, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
                [stateType.DEAD, stateType.ALIVE, stateType.DEAD],
            ],
        }),
        {
            width: 3,
            height: 3,
            value: [
                [stateType.ALIVE, stateType.DEAD, stateType.ALIVE],
                [stateType.DEAD, stateType.DEAD, stateType.ALIVE],
                [stateType.ALIVE, stateType.ALIVE, stateType.ALIVE],
            ],
        },
    );
});

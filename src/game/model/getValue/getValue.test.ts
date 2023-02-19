import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { stateType } from "../../cell/mod.ts";
import { getValue } from "./getValue.ts";

Deno.test("Value out of range", () => {
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: -1,
            row: 0,
        }),
        undefined,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: -1,
            row: 0,
        }),
        undefined,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 0,
            row: -1,
        }),
        undefined,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 2,
            row: 0,
        }),
        undefined,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 0,
            row: 2,
        }),
        undefined,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 2,
            row: 2,
        }),
        undefined,
    );
});

Deno.test("Value in range", () => {
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 0,
            row: 0,
        }),
        stateType.DEAD,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 0,
            row: 1,
        }),
        stateType.ALIVE,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 1,
            row: 0,
        }),
        stateType.DEAD,
    );
    assertEquals(
        getValue({
            width: 2,
            height: 2,
            value: [
                [stateType.DEAD, stateType.DEAD],
                [stateType.ALIVE, stateType.ALIVE],
            ],
        }, {
            column: 1,
            row: 1,
        }),
        stateType.ALIVE,
    );
});

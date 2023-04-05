import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { absoluteToRelative } from "./absoluteToRelative.ts";

Deno.test("Should transform the absolute value to relative", () => {
    assertEquals(absoluteToRelative(0, 30), 0);
    assertEquals(absoluteToRelative(10, 30), 0);
    assertEquals(absoluteToRelative(20, 30), 0);
    assertEquals(absoluteToRelative(29, 30), 0);
    assertEquals(absoluteToRelative(30, 30), 1);
    assertEquals(absoluteToRelative(300, 30), 10);
});

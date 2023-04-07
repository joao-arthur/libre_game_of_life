import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts";
import { fpsToMilis } from "./fpsToMilis";

Deno.test("Should convert fps to milisseconds", () => {
    assertEquals(fpsToMilis(0), Infinity);
    assertEquals(fpsToMilis(1), 1000);
    assertEquals(fpsToMilis(5), 200);
    assertEquals(fpsToMilis(10), 100);
    assertEquals(fpsToMilis(50), 20);
    assertEquals(fpsToMilis(100), 10);
    assertEquals(fpsToMilis(500), 2);
    assertEquals(fpsToMilis(1000), 1);
});

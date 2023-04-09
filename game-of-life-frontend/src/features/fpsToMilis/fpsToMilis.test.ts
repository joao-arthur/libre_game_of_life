import { expect, it } from "vitest";
import { fpsToMilis } from "./fpsToMilis";

it("fpsToMilis", () => {
    expect(fpsToMilis(0)).toBe(Infinity);
    expect(fpsToMilis(1)).toBe(1000);
    expect(fpsToMilis(5)).toBe(200);
    expect(fpsToMilis(10)).toBe(100);
    expect(fpsToMilis(50)).toBe(20);
    expect(fpsToMilis(100)).toBe(10);
    expect(fpsToMilis(500)).toBe(2);
    expect(fpsToMilis(1000)).toBe(1);
});

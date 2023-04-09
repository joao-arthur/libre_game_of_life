import { expect, it } from "vitest";
import { absoluteToRelative } from "./absoluteToRelative.js";

it("absoluteToRelative", () => {
    expect(absoluteToRelative(0, 30)).toBe(0);
    expect(absoluteToRelative(10, 30)).toBe(0);
    expect(absoluteToRelative(20, 30)).toBe(0);
    expect(absoluteToRelative(29, 30)).toBe(0);
    expect(absoluteToRelative(30, 30)).toBe(1);
    expect(absoluteToRelative(300, 30)).toBe(10);
});

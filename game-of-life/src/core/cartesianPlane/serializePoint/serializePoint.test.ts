import { describe, expect } from "vitest";
import { serializePoint } from "./serializePoint.js";

describe("serializePoint", () => {
    expect(serializePoint({ x: 0, y: 0 })).toBe("(x: 0, y: 0)");
    expect(serializePoint({ x: -1, y: -1 })).toBe("(x: -1, y: -1)");
    expect(serializePoint({ x: 1, y: 1 })).toBe("(x: 1, y: 1)");
    expect(serializePoint({ x: 3, y: 6 })).toBe("(x: 3, y: 6)");
});

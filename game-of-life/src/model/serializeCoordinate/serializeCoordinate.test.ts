import { describe, expect } from "vitest";
import { serializeCoordinate } from "./serializeCoordinate.js";

describe("serializeCoordinate", () => {
    expect(serializeCoordinate({ x: 0, y: 0 })).toBe("(x: 0, y: 0)");
    expect(serializeCoordinate({ x: -1, y: -1 })).toBe(
        "(x: -1, y: -1)",
    );
    expect(serializeCoordinate({ x: 1, y: 1 })).toBe("(x: 1, y: 1)");
    expect(serializeCoordinate({ x: 3, y: 6 })).toBe("(x: 3, y: 6)");
});

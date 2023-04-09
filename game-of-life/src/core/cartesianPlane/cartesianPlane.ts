import { indexToPoint } from "./indexToPoint/mod.js";
import { serializePoint } from "./serializePoint/mod.js";
import { deserializePoint } from "./deserializePoint/mod.js";

export const cartesianPlane = {
    indexToPoint,
    serializePoint,
    deserializePoint,
} as const;

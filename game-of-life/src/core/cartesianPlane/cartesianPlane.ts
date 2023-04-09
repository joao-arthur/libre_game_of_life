import { indexToPoint } from "./indexToPoint/mod.js";
import { serializePoint } from "./serializePoint/mod.js";

export const cartesianPlane = {
    indexToPoint,
    serializePoint,
} as const;

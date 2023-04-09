import { indexToPoint } from "./indexToPoint/mod.js";
import { pointToIndex } from "./pointToIndex/mod.js";
import { serializePoint } from "./serializePoint/mod.js";
import { deserializePoint } from "./deserializePoint/mod.js";
import { absoluteToRelative } from "./absoluteToRelative/mod.js";

export const cartesianPlane = {
    indexToPoint,
    pointToIndex,
    serializePoint,
    deserializePoint,
    absoluteToRelative,
} as const;

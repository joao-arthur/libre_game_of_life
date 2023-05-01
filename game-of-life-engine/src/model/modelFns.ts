import { fromString } from "./fromString/mod.js";
import { getValue } from "./getValue/mod.js";
import { getLength } from "./getLength/mod.js";
import { getCellSize } from "./getCellSize/mod.js";
import { getMiddlePoint } from "./getMiddlePoint/mod.js";
import { iterate } from "./iterate/mod.js";
import { toggle } from "./toggle/mod.js";
import { zoom } from "./zoom/mod.js";
import { move } from "./move/mod.js";

export const modelFns = {
    fromString,
    getValue,
    getLength,
    getCellSize,
    getMiddlePoint,
    iterate,
    toggle,
    zoom,
    move,
} as const;

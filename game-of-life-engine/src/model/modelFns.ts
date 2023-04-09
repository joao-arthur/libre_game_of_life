import { fromString } from "./fromString/mod.js";
import { getValue } from "./getValue/mod.js";
import { iterate } from "./iterate/mod.js";
import { toggle } from "./toggle/mod.js";
import { zoom } from "./zoom/mod.js";
import { getSize } from "./getSize/mod.js";
import { move } from "./move/mod.js";

export const modelFns = {
    fromString,
    getValue,
    iterate,
    toggle,
    zoom,
    getSize,
    move,
} as const;

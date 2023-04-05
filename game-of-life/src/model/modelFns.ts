import { fromString } from "./fromString/mod.js";
import { getValue } from "./getValue/mod.js";
import { map } from "./map/mod.js";
import { forEach } from "./forEach/mod.js";
import { iterate } from "./iterate/mod.js";
import { toggle } from "./toggle/mod.js";
import { zoom } from "./zoom/mod.js";

export const modelFns = {
    fromString,
    getValue,
    map,
    forEach,
    iterate,
    toggle,
    zoom,
} as const;

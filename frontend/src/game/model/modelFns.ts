import { fromString } from "./fromString/mod.ts";
import { getValue } from "./getValue/mod.ts";
import { map } from "./map/mod.ts";
import { forEach } from "./forEach/mod.ts";
import { iterate } from "./iterate/mod.ts";
import { toggle } from "./toggle/mod.ts";
import { zoom } from "./zoom/mod.ts";

export const modelFns = {
    fromString,
    getValue,
    map,
    forEach,
    iterate,
    toggle,
    zoom,
} as const;

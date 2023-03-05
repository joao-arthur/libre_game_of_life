import { getValue } from "./getValue/mod.ts";
import { map } from "./map/mod.ts";
import { forEach } from "./forEach/mod.ts";
import { iterate } from "./iterate/mod.ts";

export const modelFns = {
    getValue,
    map,
    forEach,
    iterate,
} as const;

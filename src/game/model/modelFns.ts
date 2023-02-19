import { getValue } from "./getValue/mod.ts";
import { map } from "./map/mod.ts";
import { iterate } from "./iterate/mod.ts";

export const modelFns = {
    getValue,
    map,
    iterate,
} as const;

import { getValue } from "./getValue/mod.ts";
import { mapModel } from "./mapModel/mod.ts";

export const modelFns = {
    getValue,
    mapModel,
} as const;

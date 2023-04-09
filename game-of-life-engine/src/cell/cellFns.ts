import { iterate } from "./iterate/mod.js";
import { toggle } from "./toggle/mod.js";

export const cellFns = {
    iterate,
    toggle,
} as const;

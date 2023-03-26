import { iterate } from "./iterate/mod.ts";
import { toggle } from "./toggle/mod.ts";

export const cellFns = {
    iterate,
    toggle,
} as const;

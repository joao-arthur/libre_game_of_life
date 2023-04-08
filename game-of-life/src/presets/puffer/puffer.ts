import { puffer1 } from "./general/puffer1.js";

export const puffer = {
    group: { name: "Puffer", id: "puffer" },
    subGroups: [
        {
            name: "General",
            id: "general",
            items: [puffer1],
        },
    ],
} as const;

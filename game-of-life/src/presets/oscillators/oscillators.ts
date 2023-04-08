import { blinker } from "./general/blinker.js";

export const oscillators = {
    group: { name: "Oscillators", id: "oscillators" },
    subGroups: [
        {
            name: "General",
            id: "general",
            items: [blinker],
        },
    ],
} as const;

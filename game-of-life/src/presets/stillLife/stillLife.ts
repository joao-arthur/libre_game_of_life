import { boat } from "./boat/boat.js";
import { block } from "./general/block.js";

export const stillLife = {
    group: { name: "StillLife", id: "stillLife" },
    subGroups: [
        {
            name: "Ship",
            id: "ship",
            items: [],
        },
        {
            name: "Boat",
            id: "boat",
            items: [boat],
        },
        {
            name: "Loaf",
            id: "loaf",
            items: [],
        },
        {
            name: "General",
            id: "general",
            items: [block],
        },
    ],
} as const;

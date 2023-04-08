import { rPentomino } from "./general/rPentomino.js";

export const methuselahs = {
    group: { name: "Methuselahs", id: "methuselahs" },
    subGroups: [
        {
            name: "General",
            id: "general",
            items: [rPentomino],
        },
    ],
} as const;

import { gosperGliderGun } from "./general/gosperGliderGun.js";

export const gliderGun = {
    group: { name: "Glider gun", id: "gliderGun" },
    subGroups: [
        {
            name: "General",
            id: "general",
            items: [gosperGliderGun],
        },
    ],
} as const;

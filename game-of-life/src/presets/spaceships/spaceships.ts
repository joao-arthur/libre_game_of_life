import { glider } from "./general/glider.js";

export const spaceships = {
    group: { name: "Spaceships", id: "spaceships" },
    subGroups: [
        {
            name: "General",
            id: "general",
            items: [glider],
        },
    ],
} as const;

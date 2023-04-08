import { modelType } from "../mod.js";
import { stillLife } from "./stillLife/mod.js";
import { oscillators } from "./oscillators/mod.js";
import { methuselahs } from "./methuselahs/mod.js";
import { spaceships } from "./spaceships/mod.js";
import { gliderGun } from "./gliderGun/mod.js";
import { puffer } from "./puffer/mod.js";

type presetsGroupsType = {
    readonly group: {
        readonly name: string;
        readonly id: string;
    };
    readonly subGroups: {
        readonly name: string;
        readonly id: string;
        readonly items: {
            readonly name: string;
            readonly id: string;
            readonly discover: {
                readonly name: string;
                readonly year: number;
            };
            readonly model: modelType;
        }[];
    }[];
}[];

export const presets: presetsGroupsType = [
    stillLife,
    oscillators,
    methuselahs,
    spaceships,
    gliderGun,
    puffer,
];

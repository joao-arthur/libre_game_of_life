import { modelType } from "../mod.js";

import { boat } from "./stillLife/boat/boat.js";
import { block } from "./stillLife/general/block.js";

import { blinker } from "./oscillators/general/blinker.js";

import { rPentomino } from "./methuselahs/general/rPentomino.js";

import { glider } from "./spaceships/general/glider.js";

import { gosperGliderGun } from "./gliderGun/general/gosperGliderGun.js";

import { puffer1 } from "./puffer/general/puffer1.js";

type presetsType = {
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

export const presets: presetsType = [
    {
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
    },
    {
        group: { name: "Oscillators", id: "oscillators" },
        subGroups: [
            {
                name: "General",
                id: "general",
                items: [blinker],
            },
        ],
    },
    {
        group: { name: "Methuselahs", id: "methuselahs" },
        subGroups: [
            {
                name: "General",
                id: "general",
                items: [rPentomino],
            },
        ],
    },
    {
        group: { name: "Spaceships", id: "spaceships" },
        subGroups: [
            {
                name: "General",
                id: "general",
                items: [glider],
            },
        ],
    },
    {
        group: { name: "Glider gun", id: "gliderGun" },
        subGroups: [
            {
                name: "General",
                id: "general",
                items: [gosperGliderGun],
            },
        ],
    },
    {
        group: { name: "Puffer", id: "puffer" },
        subGroups: [
            {
                name: "General",
                id: "general",
                items: [puffer1],
            },
        ],
    },
];

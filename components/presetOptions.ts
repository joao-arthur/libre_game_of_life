import { presetsKeys } from "../src/game/presets/presetsMap.ts";

type presetOptionsType = {
    readonly label: string;
    readonly options: {
        readonly label: string;
        readonly value: presetsKeys;
    }[];
}[];

export const presetOptions: presetOptionsType = [
    {
        label: "Still life",
        options: [
            { label: "Square", value: "square" },
            { label: "Bee-hive", value: "beeHive" },
            { label: "Loaf", value: "loaf" },
            { label: "Tub", value: "tub" },
            { label: "Boat", value: "boat" },
            { label: "Ship", value: "ship" },
        ],
    },
    {
        label: "Oscilators",
        options: [
            { label: "Airforce", value: "airforce" },
            { label: "AVerage", value: "aVerage" },
            { label: "101", value: "oneZeroOne" },
            { label: "Blinker", value: "blinker" },
            { label: "Toad", value: "toad" },
            { label: "Beacon", value: "beacon" },
            { label: "Pulsar", value: "pulsar" },
            { label: "Penta-decathlon", value: "pentaDecathlon" },
        ],
    },
    {
        label: "Spaceships",
        options: [
            { label: "Glider", value: "glider" },
            {
                label: "Light-weight spaceship (LWSS)",
                value: "lwss",
            },
            {
                label: "Middle-weight spaceship (MWSS)",
                value: "mwss",
            },
            {
                label: "Heavy-weight spaceship (HWSS)",
                value: "hwss",
            },
        ],
    },
    {
        label: "Machines",
        options: [
            {
                label: "Glider gun",
                value: "gliderGun",
            },
        ],
    },
    {
        label: "Long lasting",
        options: [
            { label: "Acorn", value: "acorn" },
            { label: "Diehard", value: "diehard" },
            { label: "F-Pentomino", value: "fPentomino" },
        ],
    },
];

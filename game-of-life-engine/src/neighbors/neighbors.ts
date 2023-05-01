import { State } from "../cell/mod.js";

export type Neighbors = readonly [
    State | undefined,
    State | undefined,
    State | undefined,
    State | undefined,
    State | undefined,
    State | undefined,
    State | undefined,
    State | undefined,
];

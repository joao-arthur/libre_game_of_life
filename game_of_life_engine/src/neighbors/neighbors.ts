import { State } from "../cell/mod.js";

export type Neighbor = State | undefined;

export type Neighbors = readonly [
    Neighbor,
    Neighbor,
    Neighbor,
    Neighbor,
    Neighbor,
    Neighbor,
    Neighbor,
    Neighbor,
];

import type { Model } from "./model.js";
import { type Point, pointFrom } from "./cartesianPlane.js";
import { cb, std } from "funis";
import { State } from "./cell.js";
import { getModelValue } from "./model.js";

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

export type AliveNeighbors = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;

export function neighborsFromModel(model: Model, point: Point): Neighbors {
    return [
        getModelValue(model, pointFrom(point.x - 1, point.y + 1)),
        getModelValue(model, pointFrom(point.x, point.y + 1)),
        getModelValue(model, pointFrom(point.x + 1, point.y + 1)),
        getModelValue(model, pointFrom(point.x - 1, point.y)),
        getModelValue(model, pointFrom(point.x + 1, point.y)),
        getModelValue(model, pointFrom(point.x - 1, point.y - 1)),
        getModelValue(model, pointFrom(point.x, point.y - 1)),
        getModelValue(model, pointFrom(point.x + 1, point.y - 1)),
    ];
}

export function numberOfAliveNeighbors(neighbors: Neighbors): AliveNeighbors {
    return neighbors
        .filter(cb.eq(State.ALIVE as Neighbor))
        .length as AliveNeighbors;
}

export function aliveNeighborsFromModel(model: Model, point: Point): AliveNeighbors {
    return std.pipe(
        () => neighborsFromModel(model, point),
        (neighbors) => numberOfAliveNeighbors(neighbors),
    )(undefined);
}

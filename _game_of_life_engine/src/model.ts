import type { Point, Rect } from "./cartesianPlane.js";
import { arr, map, num } from "funis";
import {
    deserializePoint,
    indexToPoint,
    pointFrom,
    rectFrom,
    serializePoint,
} from "./cartesianPlane.js";
import { aliveNeighborsFromModel } from "./neighbors.js";
import { iterateCell, State } from "./cell.js";

export type Model = {
    readonly value: Map<string, State.ALIVE>;
    readonly iter: number;
    readonly pos: Rect;
};

export function modelDefault(): Model {
    return {
        value: new Map(),
        iter: 0,
        pos: rectFrom(0, 0, 0, 0),
    };
}

export function modelFromPos(pos: Rect): Model {
    return { ...modelDefault(), pos };
}

export function modelFromString(stringValue: string[]): Model {
    const length = stringValue.length;
    const aliveCells = stringValue.flatMap(
        (rowValue, row) =>
            rowValue
                .split("")
                .map((colValue, col) =>
                    colValue === "⬜" ? indexToPoint({ row, col }, length) : undefined
                )
                .filter((entry) => entry !== undefined)
                .map((entry) => entry as Point),
    );
    const entries: [string, State.ALIVE][] = aliveCells.map(
        (aliveCell) => [
            serializePoint(aliveCell),
            State.ALIVE,
        ],
    );

    return {
        value: new Map(entries),
        iter: 0,
        pos: rectFrom(-10, -10, 10, 10),
    };
}

export function getModelLength(model: Model): number {
    return model.pos.x2 - model.pos.x1 + 1;
}

export function getModelCellSize(model: Model, totalSize: number): number {
    return totalSize / getModelLength(model);
}

export function getModelMiddlePoint(model: Model): Point {
    return {
        x: (model.pos.x1 + model.pos.x2) / 2,
        y: (model.pos.y1 + model.pos.y2) / 2,
    };
}

export function getModelMiddleCell(model: Model, totalSize: number): Point {
    const cellSize = getModelCellSize(model, totalSize);
    const middlePoint = getModelMiddlePoint(model);

    return {
        x: middlePoint.x * cellSize,
        y: middlePoint.y * cellSize,
    };
}

export function getModelValue(model: Model, point: Point): State {
    return model.value.has(serializePoint(point)) ? State.ALIVE : State.DEAD;
}

export function iterateModel(model: Model): Model {
    const iteratingPoints = map
        .keys(model.value)
        .map(deserializePoint)
        .flatMap((point) => [
            pointFrom(point.x - 1, point.y + 1),
            pointFrom(point.x, point.y + 1),
            pointFrom(point.x + 1, point.y + 1),
            pointFrom(point.x - 1, point.y),
            pointFrom(point.x, point.y),
            pointFrom(point.x + 1, point.y),
            pointFrom(point.x - 1, point.y - 1),
            pointFrom(point.x, point.y - 1),
            pointFrom(point.x + 1, point.y - 1),
        ]);
    const uniquePoints = arr
        .unique(iteratingPoints.map(serializePoint))
        .map(deserializePoint);
    const entries = uniquePoints
        .map((point) => {
            const state = getModelValue(model, point);
            const neighbors = aliveNeighborsFromModel(model, point);
            const newCell = iterateCell(state, neighbors);
            return newCell === State.ALIVE ? [serializePoint(point), newCell] : undefined;
        })
        .filter((value) => value !== undefined)
        .map((value) => value as [string, State.ALIVE]);

    return {
        value: new Map(entries),
        iter: model.iter + 1,
        pos: model.pos,
    };
}

export function moveModel(model: Model, delta: Point): Model {
    return {
        value: model.value,
        iter: model.iter,
        pos: {
            x1: model.pos.x1 + delta.x,
            y1: model.pos.y1 + delta.y,
            x2: model.pos.x2 + delta.x,
            y2: model.pos.y2 + delta.y,
        },
    };
}

export function toggleModel(model: Model, point: Point): Model {
    const key = serializePoint(point);
    const current = map.entries(model.value);

    const entries = model.value.has(key)
        ? current.filter(([valueKey]) => valueKey !== key)
        : current.concat([[key, State.ALIVE]]);

    return {
        value: new Map(entries),
        iter: model.iter,
        pos: model.pos,
    };
}

export function zoomModel(model: Model, newSize: number): Model {
    const halfNewSize = newSize / 2;
    const halfX = (model.pos.x1 + model.pos.x2) / 2;
    const halfY = (model.pos.y1 + model.pos.y2) / 2;

    const x1 = Math.ceil(halfX - halfNewSize);
    const y1 = Math.ceil(halfY - halfNewSize);
    const x2 = x1 + newSize - 1;
    const y2 = y1 + newSize - 1;

    return {
        value: model.value,
        iter: model.iter,
        pos: {
            x1: num.normalizeZero(x1),
            y1: num.normalizeZero(y1),
            x2: num.normalizeZero(x2),
            y2: num.normalizeZero(y2),
        },
    };
}

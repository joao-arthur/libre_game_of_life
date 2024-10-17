import type { ArrPos } from "./array.js";

export type Rect = {
    readonly x1: number;
    readonly y1: number;
    readonly x2: number;
    readonly y2: number;
};

export type Point = {
    readonly x: number;
    readonly y: number;
};

export function rectFrom(x1: number, y1: number, x2: number, y2: number): Rect {
    return { x1, y1, x2, y2 };
}

export function pointFrom(x: number, y: number): Point {
    return { x, y };
}

export function serializePoint(point: Point): string {
    return `(${point.x}, ${point.y})`;
}

export function deserializePoint(point: string): Point {
    const [x, y] = point
        .replace("(", "")
        .replace(")", "")
        .split(", ");

    return {
        x: Number(x),
        y: Number(y),
    };
}

export function pointToIndex(point: Point, length: number): ArrPos {
    const half = Math.floor(length / 2);

    return {
        col: half + point.x,
        row: half - point.y,
    };
}

export function indexToPoint(pos: ArrPos, length: number): Point {
    const half = Math.floor(length / 2);

    return {
        x: -half + pos.col,
        y: half - pos.row,
    };
}

export function absoluteToRelative(value: number, unitSize: number): number {
    return Math.trunc(value / unitSize);
}

export function relativeToAbsolute(value: number, unitSize: number): number {
    return value * unitSize;
}

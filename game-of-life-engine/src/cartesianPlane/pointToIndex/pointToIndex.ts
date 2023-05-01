import { ArrayPosition } from "../../array/arrayPosition.js";
import { CartesianPoint } from "../cartesianPoint.js";

export function pointToIndex(
    point: CartesianPoint,
    length: number,
): ArrayPosition {
    const half = Math.floor(length / 2);

    return {
        col: half + point.x,
        row: half - point.y,
    };
}

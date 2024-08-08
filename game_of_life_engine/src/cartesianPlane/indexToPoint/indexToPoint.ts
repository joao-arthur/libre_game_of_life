import { ArrayPosition } from "../../array/arrayPosition.js";
import { CartesianPoint } from "../cartesianPoint.js";

export function indexToPoint(
    position: ArrayPosition,
    length: number,
): CartesianPoint {
    const half = Math.floor(length / 2);

    return {
        x: -half + position.col,
        y: half - position.row,
    };
}

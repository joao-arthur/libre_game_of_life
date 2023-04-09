import { arrayPositionType } from "../../array/arrayPosition.js";
import { cartesianPointType } from "../cartesianPoint.js";

// [[0, 0]]

// [[-1, 1], [0, 1]]
// [[-1, 0], [0, 0]]

// [[-1,  1], [0,  1], [1,  1]]
// [[-1,  0], [0,  0], [1,  0]]
// [[-1, -1], [0, -1], [1, -1]]

// [[-2,  2], [-1,  2], [0,  2], [1,  2]]
// [[-2,  1], [-1,  1], [0,  1], [1,  1]]
// [[-2,  0], [-1,  0], [0,  0], [1,  0]]
// [[-2, -1], [-1, -1], [0, -1], [1, -1]]

export function indexToPoint(
    position: arrayPositionType,
    length: number,
): cartesianPointType {
    const half = length / 2;

    return {
        x: -Math.floor(half) + position.col,
        y: Math.floor(half) - position.row,
    };
}

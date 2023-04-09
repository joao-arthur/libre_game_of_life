import { arrayPositionType } from "../../array/arrayPosition.js";
import { cartesianPointType } from "../cartesianPoint.js";

export function indexToPoint(
    position: arrayPositionType,
    length: number,
): cartesianPointType {
    const half = Math.floor(length / 2);

    return {
        x: -half + position.col,
        y: half - position.row,
    };
}

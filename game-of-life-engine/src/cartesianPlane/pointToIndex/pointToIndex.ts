import { arrayPositionType } from "../../array/arrayPosition.js";
import { cartesianPointType } from "../cartesianPoint.js";

export function pointToIndex(
    point: cartesianPointType,
    length: number,
): arrayPositionType {
    const half = Math.floor(length / 2);

    return {
        col: half + point.x,
        row: half - point.y,
    };
}

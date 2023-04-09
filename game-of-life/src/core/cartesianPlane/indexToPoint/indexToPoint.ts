import { arrayPositionType } from "../../array/arrayPosition.js";
import { cartesianPointType } from "../cartesianPoint.js";
import { getQuadrant } from "../getQuadrant/getQuadrant.js";

export function indexToPoint(
    position: arrayPositionType,
    length: number,
): cartesianPointType {
    const half = length / 2;
    const col = position.col;
    const row = position.row;
    const quadrant = getQuadrant(position, length);

    const baseX = -Math.floor(half) + col;
    const baseY = Math.ceil(half) - row;

    switch (quadrant) {
        case 1:
            return {
                x: baseX + 1,
                y: baseY,
            };
        case 2:
            return {
                x: baseX,
                y: baseY,
            };
        case 3:
            return {
                x: baseX,
                y: baseY - 1,
            };
        case 4:
            return {
                x: baseX + 1,
                y: baseY - 1,
            };
    }
}

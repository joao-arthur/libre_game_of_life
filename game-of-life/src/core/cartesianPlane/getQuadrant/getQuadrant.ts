import { arrayPositionType } from "../../array/arrayPosition.js";

type quadrantType = 1 | 2 | 3 | 4;

export function getQuadrant(
    position: arrayPositionType,
    length: number,
): quadrantType {
    const half = length / 2;
    const rowAfterHalf = position.row >= half;
    const colAfterHalf = position.col >= Math.floor(half);

    if (rowAfterHalf) {
        if (colAfterHalf) {
            return 4;
        } else {
            return 3;
        }
    } else {
        if (colAfterHalf) {
            return 1;
        } else {
            return 2;
        }
    }
}

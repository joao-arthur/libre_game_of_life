import { arrayPositionType } from "../../array/arrayPosition.js";

type paramsType = {
    readonly position: arrayPositionType;
    readonly length: number;
};

type quadrantType = "1" | "2" | "3" | "4";

export function getQuadrant(
    { position: { row, col }, length }: paramsType,
): quadrantType {
    const halfLength = length / 2;

    if (row >= halfLength) {
        if (col >= halfLength) {
            return "4";
        } else {
            return "3";
        }
    } else {
        if (col >= halfLength) {
            return "1";
        } else {
            return "2";
        }
    }
}

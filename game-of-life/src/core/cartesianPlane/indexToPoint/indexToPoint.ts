import { arrayPositionType } from "../../array/arrayPosition.js";
import { cartesianPointType } from "../cartesianPoint.js";
import { getQuadrant } from "../getQuadrant/getQuadrant.js";

type paramsType = {
    readonly position: arrayPositionType;
    readonly length: number;
};

export function indexToPoint(
    params: paramsType,
): cartesianPointType {
    const halfLength = params.length / 2;
    const col = params.position.col;
    const row = params.position.row;

    switch (getQuadrant(params)) {
        case "1":
            return {
                x: -halfLength + col + 1,
                y: halfLength - row,
            };
        case "2":
            return {
                x: -halfLength + col,
                y: halfLength - row,
            };
        case "3":
            return {
                x: -halfLength + col,
                y: halfLength - row - 1,
            };
        case "4":
            return {
                x: -halfLength + col + 1,
                y: halfLength - row - 1,
            };
    }
}

import { cartesianPointType } from "../cartesianPoint.js";

export function deserializePoint(
    point: string,
): cartesianPointType {
    const [x, y] = point
        .replace("(", "")
        .replace("x: ", "")
        .replace("y: ", "")
        .replace(")", "")
        .split(",");
    return {
        x: Number(x),
        y: Number(y),
    };
}

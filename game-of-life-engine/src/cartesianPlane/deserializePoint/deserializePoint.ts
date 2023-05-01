import { CartesianPoint } from "../cartesianPoint.js";

export function deserializePoint(
    point: string,
): CartesianPoint {
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

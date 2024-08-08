import { CartesianPoint } from "../cartesianPoint.js";

export function serializePoint(
    point: CartesianPoint,
): string {
    return `(x: ${point.x}, y: ${point.y})`;
}

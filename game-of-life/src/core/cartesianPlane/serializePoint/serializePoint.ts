import { cartesianPointType } from "../cartesianPoint.js";

export function serializePoint(
    point: cartesianPointType,
): string {
    return `(x: ${point.x}, y: ${point.y})`;
}

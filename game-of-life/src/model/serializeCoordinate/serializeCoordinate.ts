import { cartesianPointType } from "../../core/cartesianPlane/mod.js";

export function serializeCoordinate(
    { x, y }: cartesianPointType,
): string {
    return `(x: ${x}, y: ${y})`;
}

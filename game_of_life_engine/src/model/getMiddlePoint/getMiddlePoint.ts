import { CartesianPoint } from "../../cartesianPlane/mod.js";
import { Model } from "../model.js";

export function getMiddlePoint(model: Model): CartesianPoint {
    const { x1, x2, y1, y2 } = model.position;

    return {
        x: (x1 + x2) / 2,
        y: (y1 + y2) / 2,
    };
}

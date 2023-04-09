import { numbers } from "funis";
import { cartesianPointType } from "../../core/cartesianPlane/mod.js";
import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { getValue } from "../getValue/mod.js";

type cbType = (
    point: cartesianPointType,
    state: stateType,
) => void;

export function forEach(
    model: modelType,
    cb: cbType,
): void {
    const { x1, y1, x2, y2 } = model.position;

    numbers.range(x1, x2).forEach((x) => {
        numbers.range(y1, y2).forEach((y) => {
            const point = { x, y };
            cb(point, getValue(model, point));
        });
    });
}

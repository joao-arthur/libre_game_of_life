import { maps } from "funis";
import {
    cartesianPlane,
    cartesianPointType,
} from "../../core/cartesianPlane/mod.js";
import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";

export function toggle(
    model: modelType,
    point: cartesianPointType,
): modelType {
    const key = cartesianPlane.serializePoint(point);
    const current = maps.entries(model.value);

    const entries = model.value.has(key)
        ? current.filter(([valueKey]) => valueKey !== key)
        : current.concat([[key, stateType.ALIVE]]);

    return {
        value: new Map(entries),
        iteration: model.iteration,
        position: model.position,
    };
}

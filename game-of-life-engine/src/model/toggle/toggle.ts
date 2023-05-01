import { maps } from "funis";
import {
    cartesianPlaneFns,
    CartesianPoint,
} from "../../cartesianPlane/mod.js";
import { State } from "../../cell/mod.js";
import { Model } from "../model.js";

export function toggle(
    model: Model,
    point: CartesianPoint,
): Model {
    const key = cartesianPlaneFns.serializePoint(point);
    const current = maps.entries(model.value);

    const entries = model.value.has(key)
        ? current.filter(([valueKey]) => valueKey !== key)
        : current.concat([[key, State.ALIVE]]);

    return {
        value: new Map(entries),
        iteration: model.iteration,
        position: model.position,
    };
}

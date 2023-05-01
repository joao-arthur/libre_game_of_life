import {
    cartesianPlaneFns,
    CartesianPoint,
} from "../../cartesianPlane/mod.js";
import { State } from "../../cell/mod.js";
import { Model } from "../model.js";

export function getValue(
    model: Model,
    point: CartesianPoint,
): State {
    return model.value.has(cartesianPlaneFns.serializePoint(point))
        ? State.ALIVE
        : State.DEAD;
}

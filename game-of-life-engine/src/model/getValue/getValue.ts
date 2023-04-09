import {
    cartesianPlane,
    cartesianPointType,
} from "../../core/cartesianPlane/mod.js";
import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";

export function getValue(
    model: modelType,
    point: cartesianPointType,
): stateType {
    return model.value.has(cartesianPlane.serializePoint(point))
        ? stateType.ALIVE
        : stateType.DEAD;
}

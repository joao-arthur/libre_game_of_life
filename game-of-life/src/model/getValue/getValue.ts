import { cartesianPointType } from "../../core/cartesianPlane/mod.js";
import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { serializeCoordinate } from "../serializeCoordinate/mod.js";

export function getValue(
    model: modelType,
    point: cartesianPointType,
): stateType {
    return model.value.has(serializeCoordinate(point))
        ? stateType.ALIVE
        : stateType.DEAD;
}

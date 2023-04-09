import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { positionType } from "../position.js";
import { serializeCoordinate } from "../serializeCoordinate/mod.js";

export function getValue(
    model: modelType,
    { column, row }: positionType,
): stateType | undefined {
    return model.value.get(
        serializeCoordinate({ x: column, y: row }),
    );
}

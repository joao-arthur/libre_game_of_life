import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { positionType } from "../position.js";

export function getValue(
    model: modelType,
    { column, row }: positionType,
): stateType | undefined {
    if (column < 0) {
        return undefined;
    }
    if (row < 0) {
        return undefined;
    }
    if (column >= model.size) {
        return undefined;
    }
    if (row >= model.size) {
        return undefined;
    }
    return model.value[row][column];
}

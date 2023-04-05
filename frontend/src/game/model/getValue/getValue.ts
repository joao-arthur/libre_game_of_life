import { stateType } from "../../cell/mod.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

export function getValue(
    model: modelType,
    { column, row }: positionType,
): stateType | undefined {
    if (column < 0) return undefined;
    if (row < 0) return undefined;
    if (column >= model.size) return undefined;
    if (row >= model.size) return undefined;

    return model.value[row][column];
}

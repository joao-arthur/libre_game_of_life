import { stateType } from "../../state.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

export function getValue(
    model: modelType,
    { column, row }: positionType,
): stateType | undefined {
    if (column < 0) return undefined;
    if (row < 0) return undefined;
    if (column >= model.width) return undefined;
    if (row >= model.height) return undefined;

    return model.values[row][column];
}

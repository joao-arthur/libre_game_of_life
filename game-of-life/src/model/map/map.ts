import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { positionType } from "../position.js";

type cbType = (position: positionType, state: stateType) => stateType;

export function map(
    model: modelType,
    cb: cbType,
): modelType {
    return {
        ...model,
        value: model.value.map((row, rowIndex) =>
            row.map((state, columnIndex) =>
                cb({ row: rowIndex, column: columnIndex }, state),
            ),
        ),
    };
}

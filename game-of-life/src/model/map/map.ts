import { stateType } from "../../cell/mod.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

type cbType = (position: positionType, state: stateType) => stateType;

export function map(
    model: modelType,
    cb: cbType,
): modelType {
    return {
        ...model,
        value: model.value.map((row, rowIndex) =>
            row.map((state, columnIndex) =>
                cb({ row: rowIndex, column: columnIndex }, state)
            )
        ),
    };
}

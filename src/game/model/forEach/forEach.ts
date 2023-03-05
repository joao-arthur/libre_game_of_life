import { stateType } from "../../cell/mod.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

type cbType = (position: positionType, state: stateType) => void;

export function forEach(
    model: modelType,
    cb: cbType,
): void {
    model.value.forEach((row, rowIndex) =>
        row.forEach((state, columnIndex) =>
            cb({ row: rowIndex, column: columnIndex }, state)
        )
    );
}

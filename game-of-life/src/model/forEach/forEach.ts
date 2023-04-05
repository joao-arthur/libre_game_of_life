import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";
import { positionType } from "../position.js";

type cbType = (position: positionType, state: stateType) => void;

export function forEach(
    model: modelType,
    cb: cbType,
): void {
    model.value.forEach((row, rowIndex) =>
        row.forEach((state, columnIndex) =>
            cb({ row: rowIndex, column: columnIndex }, state),
        ),
    );
}

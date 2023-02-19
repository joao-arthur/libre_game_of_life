import { stateType } from "../../state.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

type cbType = (position: positionType, state: stateType) => stateType;

export function mapModel(
    { width, height, values }: modelType,
    cb: cbType,
): modelType {
    return {
        width,
        height,
        values: values.map((row, rowIndex) =>
            row.map((state, columnIndex) =>
                cb({ row: rowIndex, column: columnIndex }, state)
            )
        ),
    };
}

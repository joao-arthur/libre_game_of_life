import { stateType } from "../../state.ts";
import { modelType } from "../model.ts";
import { positionType } from "../position.ts";

export function mapModel(
    { width, height, values }: modelType,
    cb: (position: positionType) => stateType,
): modelType {
    return {
        width,
        height,
        values: values.map((row, rowIndex) =>
            row.map((_, columnIndex) =>
                cb({ row: rowIndex, column: columnIndex })
            )
        ),
    };
}

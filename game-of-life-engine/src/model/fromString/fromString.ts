import {
    cartesianPlane,
    cartesianPointType,
} from "../../core/cartesianPlane/mod.js";
import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";

export function fromString(stringValue: string[]): modelType {
    const length = stringValue.length;

    const aliveCells = stringValue.flatMap(
        (rowValue, row) =>
            rowValue
                .split("")
                .map((colValue, col) =>
                    colValue === "⬜"
                        ? cartesianPlane.indexToPoint(
                            { row, col },
                            length,
                        )
                        : undefined
                )
                .filter((entry) => entry !== undefined)
                .map((entry) => entry as cartesianPointType),
    );

    const entries: [string, stateType.ALIVE][] = aliveCells.map(
        (aliveCell) => [
            cartesianPlane.serializePoint(aliveCell),
            stateType.ALIVE,
        ],
    );
    const value = new Map(entries);

    return {
        value,
        iteration: 0,
        position: {
            x1: -10,
            y1: -10,
            x2: 10,
            y2: 10,
        },
    };
}

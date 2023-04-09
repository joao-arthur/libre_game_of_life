import { stateType } from "../../cell/mod.js";
import {
    cartesianPlane,
    cartesianPointType,
} from "../../core/cartesianPlane/mod.js";
import { serializeCoordinate } from "../serializeCoordinate/mod.js";
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

    const entries: [string, stateType][] = aliveCells.map(
        (aliveCell) => [
            serializeCoordinate(aliveCell),
            stateType.ALIVE,
        ],
    );
    const value: Map<string, stateType> = new Map(entries);

    return {
        value,
        iteration: 0,
        position: { x: 0, y: 0 },
        size: 20,
    };
}

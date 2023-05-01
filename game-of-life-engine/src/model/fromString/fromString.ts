import {
    cartesianPlaneFns,
    CartesianPoint,
} from "../../cartesianPlane/mod.js";
import { State } from "../../cell/mod.js";
import { Model } from "../model.js";

export function fromString(stringValue: string[]): Model {
    const length = stringValue.length;

    const aliveCells = stringValue.flatMap(
        (rowValue, row) =>
            rowValue
                .split("")
                .map((colValue, col) =>
                    colValue === "⬜"
                        ? cartesianPlaneFns.indexToPoint(
                            { row, col },
                            length,
                        )
                        : undefined
                )
                .filter((entry) => entry !== undefined)
                .map((entry) => entry as CartesianPoint),
    );

    const entries: [string, State.ALIVE][] = aliveCells.map(
        (aliveCell) => [
            cartesianPlaneFns.serializePoint(aliveCell),
            State.ALIVE,
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

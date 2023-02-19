import { modelType } from "./model/model.ts";
import { modelFns } from "./model/modelFns.ts";
import { positionType } from "./model/position.ts";
import { stateType } from "./state.ts";

type returnType = readonly [
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
    stateType | undefined,
];

export function getNeighbors(
    model: modelType,
    { column, row }: positionType,
): returnType {
    const { getValue } = modelFns;

    return [
        getValue(model, { column: column - 1, row: row - 1 }),
        getValue(model, { column, row: row - 1 }),
        getValue(model, { column: column + 1, row: row - 1 }),
        getValue(model, { column: column - 1, row }),
        getValue(model, { column: column + 1, row }),
        getValue(model, { column: column - 1, row: row + 1 }),
        getValue(model, { column, row: row + 1 }),
        getValue(model, { column: column + 1, row: row + 1 }),
    ];
}

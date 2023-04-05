import {
    modelFns,
    modelType,
    positionType,
} from "../../model/mod.js";
import { neighborsType } from "../neighbors.js";

export function fromModel(
    model: modelType,
    { column, row }: positionType,
): neighborsType {
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

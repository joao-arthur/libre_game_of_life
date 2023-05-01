import { CartesianPoint } from "../../mod.js";
import { getCellSize } from "../getCellSize/mod.js";
import { getMiddlePoint } from "../getMiddlePoint/mod.js";
import { Model } from "../model.js";

export function getMiddleCell(
    model: Model,
    totalSize: number,
): CartesianPoint {
    const cellSize = getCellSize(model, totalSize);
    const middlePoint = getMiddlePoint(model);

    return {
        x: middlePoint.x * cellSize,
        y: middlePoint.y * cellSize,
    };
}

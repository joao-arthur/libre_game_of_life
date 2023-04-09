import { arrays, maps } from "funis";
import { cartesianPlane } from "../../core/cartesianPlane/cartesianPlane.js";
import { modelType } from "../model.js";
import { neighborsFns } from "../../neighbors/mod.js";
import { cellFns, stateType } from "../../cell/mod.js";
import { getValue } from "../getValue/mod.js";

export function iterate(
    model: modelType,
): modelType {
    const iteratingPoints = maps
        .keys(model.value)
        .map(cartesianPlane.deserializePoint)
        .flatMap((point) => [
            { x: point.x - 1, y: point.y + 1 },
            { x: point.x, y: point.y + 1 },
            { x: point.x + 1, y: point.y + 1 },
            { x: point.x - 1, y: point.y },
            { x: point.x, y: point.y },
            { x: point.x + 1, y: point.y },
            { x: point.x - 1, y: point.y - 1 },
            { x: point.x, y: point.y - 1 },
            { x: point.x + 1, y: point.y - 1 },
        ]);
    const uniquePoints = arrays
        .unique(iteratingPoints.map(cartesianPlane.serializePoint))
        .map(cartesianPlane.deserializePoint);
    const entries = uniquePoints
        .map((point) => {
            const state = getValue(model, point);
            const neighbors = neighborsFns.aliveFromModel(
                model,
                point,
            );
            const newCell = cellFns.iterate(state, neighbors);
            return newCell === stateType.ALIVE
                ? [cartesianPlane.serializePoint(point), newCell]
                : undefined;
        })
        .filter((value) => value !== undefined)
        .map((value) => value as [string, stateType.ALIVE]);

    return {
        value: new Map(entries),
        iteration: model.iteration + 1,
        position: model.position,
    };
}

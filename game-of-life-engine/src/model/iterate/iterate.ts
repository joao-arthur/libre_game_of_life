import { arrays, maps } from "funis";
import { cartesianPlaneFns } from "../../cartesianPlane/mod.js";
import { Model } from "../model.js";
import { neighborsFns } from "../../neighbors/mod.js";
import { cellFns, State } from "../../cell/mod.js";
import { getValue } from "../getValue/mod.js";

export function iterate(
    model: Model,
): Model {
    const iteratingPoints = maps
        .keys(model.value)
        .map(cartesianPlaneFns.deserializePoint)
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
        .unique(iteratingPoints.map(cartesianPlaneFns.serializePoint))
        .map(cartesianPlaneFns.deserializePoint);
    const entries = uniquePoints
        .map((point) => {
            const state = getValue(model, point);
            const neighbors = neighborsFns.aliveFromModel(
                model,
                point,
            );
            const newCell = cellFns.iterate(state, neighbors);
            return newCell === State.ALIVE
                ? [cartesianPlaneFns.serializePoint(point), newCell]
                : undefined;
        })
        .filter((value) => value !== undefined)
        .map((value) => value as [string, State.ALIVE]);

    return {
        value: new Map(entries),
        iteration: model.iteration + 1,
        position: model.position,
    };
}

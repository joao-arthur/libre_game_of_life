import { modelFns, modelType } from "../model/mod.ts";
import { getNeighbors } from "../getNeighbors/mod.ts";
import { cellIteration } from "../cellIteration/mod.ts";
import { stateType } from "../state.ts";

export function modelIteration(
    model: modelType,
): modelType {
    const { mapModel } = modelFns;

    return mapModel(model, (position, state) => {
        const neighbors = getNeighbors(model, position);
        const aliveNeighbors = neighbors
            .filter((neighbor) => neighbor === stateType.ALIVE)
            .length as 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
        const newState = cellIteration(state, aliveNeighbors);
        return newState;
    });
}

import { cellFns } from "../../cell/mod.ts";
import { neighborsFns } from "../../neighbors/mod.ts";
import { map } from "../map/mod.ts";
import { modelType } from "../model.ts";

export function iterate(
    model: modelType,
): modelType {
    return map(model, (position, state) => {
        const getNumberOfAliveNeighbors = neighborsFns
            .aliveFromModel(
                model,
                position,
            );
        const newState = cellFns.iterate(
            state,
            getNumberOfAliveNeighbors,
        );
        return newState;
    });
}

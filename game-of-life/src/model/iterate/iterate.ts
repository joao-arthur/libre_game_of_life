import { pipe } from "funis";
import { cellFns } from "../../cell/mod.js";
import { neighborsFns } from "../../neighbors/mod.js";
import { map } from "../map/mod.js";
import { modelType } from "../model.js";

export function iterate(
    model: modelType,
): modelType {
    return pipe(
        () =>
            map(
                model,
                (position, state) =>
                    pipe(
                        () =>
                            neighborsFns.aliveFromModel(
                                model,
                                position,
                            ),
                        (neighbors) =>
                            cellFns.iterate(state, neighbors),
                    )(undefined),
            ),
        (newModel) => ({
            ...newModel,
            iteration: newModel.iteration + 1,
        }),
    )(undefined);
}

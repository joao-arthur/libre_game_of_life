import { pipe } from "https://deno.land/x/funis@v1.0.2/mod.ts";
import { cellFns } from "../../cell/mod.ts";
import { neighborsFns } from "../../neighbors/mod.ts";
import { map } from "../map/mod.ts";
import { modelType } from "../model.ts";

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

import { fromString } from "../fromString/mod.js";
import { modelType } from "../model.js";

export function zoomIn(model: modelType, amount: number): modelType {
    if (amount * 2 >= model.size) {
        return fromString([""]);
    }
    return {
        size: model.size - amount * 2,
        value: model.value
            .slice(amount, model.size - amount)
            .map((row) =>
                row.filter((_, index) =>
                    index >= amount && model.size - index > amount,
                ),
            ),
        iteration: model.iteration,
    };
}

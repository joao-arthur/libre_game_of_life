import { stateType } from "../../cell/mod.js";
import { modelType } from "../model.js";

export function zoomOut(model: modelType, amount: number): modelType {
    return {
        size: model.size + amount * 2,
        value: [
            ...Array(amount).fill(
                Array(model.size + amount * 2).fill(stateType.DEAD),
            ),
            ...Array(model.size).fill(undefined)
                .map((_, index) => [
                    ...Array(amount).fill(stateType.DEAD),
                    ...model.value[index],
                    ...Array(amount).fill(stateType.DEAD),
                ]),
            ...Array(amount).fill(
                Array(model.size + amount * 2).fill(stateType.DEAD),
            ),
        ],
        iteration: model.iteration,
    };
}

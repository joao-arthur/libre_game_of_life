import { modelFns } from "game-of-life-engine";
import { systemModelType } from "./systemModel";

export function buildModel(
    drawContext: systemModelType["drawContext"],
    dimension: systemModelType["dimension"],
): systemModelType {
    return {
        model: modelFns.fromString(["⬛"]),
        dimension,
        gap: 1,
        fps: 4,
        status: "paused",
        drawContext,
    };
}

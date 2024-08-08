import { Model } from "../model.js";

export function getLength(model: Model): number {
    const deltaX = model.position.x2 - model.position.x1;

    return deltaX + 1;
}

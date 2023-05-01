import { getLength } from "../getLength/mod.js";
import { Model } from "../model.js";

export function getCellSize(model: Model, totalSize: number): number {
    return totalSize / getLength(model);
}

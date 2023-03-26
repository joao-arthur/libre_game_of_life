import { modelType } from "../model.ts";
import { zoomOut } from "../zoomOut/mod.ts";
import { zoomIn } from "../zoomIn/mod.ts";

export function zoom(model: modelType, newSize: number): modelType {
    if (newSize > model.size) {
        return zoomOut(model, (newSize - model.size) / 2);
    }
    if (newSize < model.size) {
        return zoomIn(model, (model.size - newSize) / 2);
    }
    return model;
}

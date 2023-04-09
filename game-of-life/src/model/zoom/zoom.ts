import { modelType } from "../model.js";

export function zoom(model: modelType, newSize: number): modelType {
    const halfX = (model.position.x1 + model.position.x2) / 2;
    const halfY = (model.position.y1 + model.position.y2) / 2;

    const x1Raw = halfX - (newSize / 2);
    const y1Raw = halfY - (newSize / 2);
    const x2Raw = halfX + (newSize / 2);
    const y2Raw = halfY + (newSize / 2);

    const differentQuadrantX = Math.sign(x1Raw) !== Math.sign(x2Raw);
    const differentQuadrantY = Math.sign(y1Raw) !== Math.sign(y2Raw);

    const x1 = Math.ceil(x1Raw);
    const y1 = Math.ceil(y1Raw);
    const x2 = Math.floor(x2Raw);
    const y2 = Math.floor(y2Raw);

    return {
        value: model.value,
        iteration: model.iteration,
        position: { x1, y1, x2, y2 },
    };
}

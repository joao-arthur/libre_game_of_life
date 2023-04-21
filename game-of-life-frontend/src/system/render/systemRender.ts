import { maps } from "funis";
import { cartesianPlane, modelFns } from "game-of-life-engine";
import { getUnitSize } from "../getUnitSize/mod";
import { SystemModel } from "../model/mod";

export class SystemRender {
    private readonly DEAD_COLOR = "#dbdbdb";
    private readonly ALIVE_COLOR = "#2e2e2e";

    public constructor(
        private readonly systemModel: SystemModel,
    ) {}

    public render(): void {
        const model = this.systemModel.getModel();
        const length = modelFns.getSize(model.model);
        const unitSize = getUnitSize(model.model, model.dimension);
        model.drawContext.clear({
            x: 0,
            y: 0,
            size: model.dimension,
        });

        for (let col = 0; col < length; col++) {
            for (let row = 0; row < length; row++) {
                model.drawContext.drawSquare({
                    x: col * unitSize + model.gap,
                    y: row * unitSize + model.gap,
                    size: unitSize - model.gap * 2,
                }, this.DEAD_COLOR);
            }
        }

        const diffX =
            (model.model.position.x1 + model.model.position.x2) / 2;
        const diffY =
            (model.model.position.y1 + model.model.position.y2) / 2;

        const deltaX = diffX * unitSize;
        const deltaY = diffY * unitSize;

        maps.keys(model.model.value)
            .map(cartesianPlane.deserializePoint)
            .forEach((point) => {
                const { col, row } = cartesianPlane.pointToIndex(
                    point,
                    length,
                );
                model.drawContext.drawSquare({
                    x: col * unitSize + model.gap - deltaX,
                    y: row * unitSize + model.gap + deltaY,
                    size: unitSize - model.gap * 2,
                }, this.ALIVE_COLOR);
            });
    }
}

import {
    cartesianPlane,
    modelFns,
    stateType,
} from "game-of-life-engine";
import { getUnitSize } from "../getUnitSize/mod";
import { SystemModel } from "../model/mod";

export class SystemRender {
    public constructor(
        private readonly systemModel: SystemModel,
    ) {}

    public render(): void {
        const model = this.systemModel.getModel();
        const length = model.model.position.x2 -
            model.model.position.x1;
        const unitSize = getUnitSize(model.model, model.dimension);
        model.drawContext.clear({
            x: 0,
            y: 0,
            size: model.dimension,
        });

        modelFns.forEach(
            model.model,
            (point, cellState) => {
                const { col, row } = cartesianPlane.pointToIndex(
                    point,
                    length,
                );

                model.drawContext.drawSquare({
                    x: col * unitSize + model.gap,
                    y: row * unitSize + model.gap,
                    size: unitSize - model.gap * 2,
                }, this.getSquareColor(cellState));
            },
        );
    }

    private getSquareColor(state: stateType): string {
        switch (state) {
            case stateType.DEAD:
                return "#dbdbdb";
            case stateType.ALIVE:
                return "#2e2e2e";
        }
    }
}

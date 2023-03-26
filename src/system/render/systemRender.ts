import { modelFns, stateType } from "../../game/mod.ts";
import { SystemModel } from "../model/mod.ts";

export class SystemRender {
    public constructor(
        private readonly systemModel: SystemModel,
    ) {}

    public render(): void {
        const model = this.systemModel.getModel();
        const unitSize = model.dimension / model.model.size;
        model.drawContext.clear({
            x: 0,
            y: 0,
            size: model.dimension,
        });
        modelFns.forEach(
            model.model,
            ({ column, row }, cellState) => {
                model.drawContext.drawSquare({
                    x: column * unitSize + model.gap,
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

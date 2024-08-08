import { map } from "funis";
import { cartesianPlaneFns, modelFns } from "game_of_life_engine";
import { SystemModel } from "../model/mod";

export class SystemRender {
    private readonly DEAD_COLOR = "#dbdbdb";
    private readonly ALIVE_COLOR = "#2e2e2e";

    public constructor(
        private readonly systemModel: SystemModel,
    ) {}

    public render(): void {
        const model = this.systemModel.getModel();
        const length = modelFns.getLength(model.model);
        const cellSize = modelFns.getCellSize(
            model.model,
            model.dimension,
        );
        const middleCell = modelFns.getMiddleCell(
            model.model,
            model.dimension,
        );

        model.drawContext.drawSquare({
            x: 0,
            y: 0,
            size: model.dimension,
        }, this.DEAD_COLOR);

        map.keys(model.model.value)
            .map(cartesianPlaneFns.deserializePoint)
            .forEach((point) => {
                const { col, row } = cartesianPlaneFns.pointToIndex(
                    point,
                    length,
                );
                model.drawContext.drawSquare({
                    x: col * cellSize + model.gap - middleCell.x,
                    y: row * cellSize + model.gap + middleCell.y,
                    size: cellSize - model.gap * 2,
                }, this.ALIVE_COLOR);
            });
    }
}

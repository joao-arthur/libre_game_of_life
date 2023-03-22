import { stateType } from "../../game/cell/mod.ts";
import { modelFns } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";
import { GameModel } from "../gameModel/mod.ts";

export class GameRender {
    public constructor(
        private readonly gameModel: GameModel,
        private readonly drawContext: drawContextType,
    ) {}

    public render(): void {
        const state = this.gameModel.getModel();
        const modelSize = Math.min(
            state.model.width,
            state.model.height,
        );
        const size = state.dimension / modelSize;
        this.drawContext.clear({
            x: 0,
            y: 0,
            size: state.dimension,
        });
        modelFns.forEach(
            state.model,
            ({ column, row }, cellState) => {
                this.drawContext.drawSquare({
                    x: column * size + state.gap,
                    y: row * size + state.gap,
                    size: size - state.gap * 2,
                }, this.getSquareColor(cellState));
            },
        );
    }

    private getSquareColor(state: stateType): string {
        switch (state) {
            case stateType.DEAD:
                return "#2e2e2e";
            case stateType.ALIVE:
                return "#dbdbdb";
        }
    }
}

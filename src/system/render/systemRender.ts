import { stateType } from "../../game/cell/mod.ts";
import { modelFns } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";
import { SystemModel } from "../model/mod.ts";

export class SystemRender {
    public constructor(
        private readonly systemModel: SystemModel,
        private readonly drawContext: drawContextType,
    ) {}

    public render(): void {
        const state = this.systemModel.getModel();
        const size = state.dimension / state.model.size;
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
                return "#dbdbdb";
            case stateType.ALIVE:
                return "#2e2e2e";
        }
    }
}

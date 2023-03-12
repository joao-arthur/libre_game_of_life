import { stateType } from "../../game/cell/mod.ts";
import { modelFns } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";
import { GameModelProxy } from "../gameModel/GameModelProxy.ts";

export class GameRender {
    timeoutId = 0;

    public constructor(
        private readonly gameModel: GameModelProxy,
        private readonly drawContext: drawContextType,
    ) {
        this.setup();
    }

    private setup(): void {
        this.gameModel.addOnSettingsChangeListener(() =>
            this.setupLoop()
        );
    }

    private setupLoop(): void {
        if (this.timeoutId) {
            globalThis.clearInterval(this.timeoutId);
        }
        const state = this.gameModel.getModel();
        if (state.status === "paused") return;
        this.timeoutId = globalThis.setInterval(
            () => this.loop(),
            1000 / state.fps,
        );
    }

    private loop(): void {
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
        this.gameModel.iterate();
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

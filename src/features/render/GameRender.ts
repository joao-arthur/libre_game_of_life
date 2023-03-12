import { stateType } from "../../game/cell/mod.ts";
import { modelFns } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";
import { GameStoreProxy } from "../gameStore/GameStoreProxy.ts";

export class GameRender {
    timeoutId = 0;

    public constructor(
        private readonly gameStore: GameStoreProxy,
        private readonly drawContext: drawContextType,
    ) {
        this.setup();
    }

    private setup(): void {
        this.gameStore.addOnSettingsChangeListener(() =>
            this.setupLoop()
        );
    }

    private setupLoop(): void {
        if (this.timeoutId) {
            globalThis.clearInterval(this.timeoutId);
        }
        const state = this.gameStore.getState();
        if (state.status === "paused") return;
        this.timeoutId = globalThis.setInterval(
            () => this.loop(),
            1000 / state.fps,
        );
    }

    private loop(): void {
        const state = this.gameStore.getState();

        const dimensionSize = Math.min(
            state.dimensions.width,
            state.dimensions.height,
        );
        const modelSize = Math.min(
            state.model.width,
            state.model.height,
        );
        const size = dimensionSize / modelSize;

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
        this.gameStore.iterate();
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

import { GameModelProxy } from "../gameModel/mod.ts";
import { GameController } from "../gameController/mod.ts";
import { GameRender } from "../gameRender/mod.ts";

export class GameManager {
    private timeoutId = 0;

    constructor(
        private readonly gameModelProxy: GameModelProxy,
        private readonly gameController: GameController,
        private readonly gameRender: GameRender,
    ) {
        this.setup();
    }

    private setup(): void {
        this.gameModelProxy.addOnChangeListener((prop) => {
            switch (prop) {
                case "gap":
                case "tiles":
                case "fps":
                case "status":
                case "dimension":
                    this.setupLoop();
            }
        });
    }

    private setupLoop(): void {
        if (this.timeoutId) {
            globalThis.clearInterval(this.timeoutId);
        }
        const model = this.gameModelProxy.getModel();
        switch (model.status) {
            case "initial":
                if (model.dimension !== undefined) {
                    this.initialLoop();
                    this.gameController.pause();
                }
                break;
            case "resumed":
                this.timeoutId = globalThis.setInterval(
                    () => this.loop(),
                    1000 / model.fps,
                );
        }
    }

    private initialLoop(): void {
        this.gameRender.render();
    }

    private loop(): void {
        this.gameRender.render();
        this.gameController.iterate();
    }
}

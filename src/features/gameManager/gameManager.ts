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
        const model = this.gameModelProxy
            .getModel();
        if (
            model.status === "initial" &&
            model.dimension !== undefined
        ) {
            this.initialRender();
            this.gameController.pause();
        }
        if (model.status === "paused") return;
        this.timeoutId = globalThis.setInterval(
            () => this.loop(),
            1000 / model.fps,
        );
    }

    private loop(): void {
        this.gameRender.render();
        this.gameController.iterate();
    }

    private initialRender(): void {
        this.gameRender.render();
    }
}

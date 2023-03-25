import { SystemModel } from "../model/mod.ts";
import { SystemController } from "../controller/mod.ts";
import { SystemRender } from "../render/mod.ts";
import { fpsToMilis } from "../../features/mod.ts";

export class SystemManager {
    private timeoutId = 0;

    constructor(
        private readonly systemModel: SystemModel,
        private readonly systemController: SystemController,
        private readonly systemRender: SystemRender,
    ) {
        this.setup();
    }

    private setup(): void {
        this.systemModel.addOnChangeListener((prop) => {
            switch (prop) {
                case "gap":
                case "tiles":
                case "fps":
                case "status":
                case "dimension":
                    this.setupLoop();
            }
        });
        this.setupLoop();
    }

    private setupLoop(): void {
        if (this.timeoutId) {
            globalThis.clearInterval(this.timeoutId);
        }
        const model = this.systemModel.getModel();
        switch (model.status) {
            case "initial":
                this.render();
                this.systemController.pause();
                break;
            case "resumed":
                this.timeoutId = globalThis.setInterval(
                    () => this.loop(),
                    fpsToMilis(model.fps),
                );
        }
    }

    private render(): void {
        this.systemRender.render();
    }

    private loop(): void {
        this.systemController.iterate();
        this.systemRender.render();
    }
}

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
            const model = this.systemModel.getModel();
            switch (model.status) {
                case "resumed":
                    switch (prop) {
                        case "status":
                            globalThis.clearInterval(this.timeoutId);
                            this.timeoutId = globalThis.setInterval(
                                () => this.loop(),
                                fpsToMilis(model.fps),
                            );
                            break;
                        case "fps":
                            globalThis.clearInterval(this.timeoutId);
                            this.timeoutId = globalThis.setInterval(
                                () => this.loop(),
                                fpsToMilis(model.fps),
                            );
                    }
                    break;
                case "paused":
                    switch (prop) {
                        case "gap":
                        case "tiles":
                        case "dimension":
                        case "model":
                            this.systemRender.render();
                            break;
                        case "status":
                            globalThis.clearInterval(this.timeoutId);
                    }
            }
        });
        this.systemRender.render();
        this.systemController.pause();
    }

    private loop(): void {
        this.systemController.iterate();
        this.systemRender.render();
    }
}

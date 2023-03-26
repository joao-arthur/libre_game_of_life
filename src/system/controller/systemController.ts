import { modelFns } from "../../game/model/mod.ts";
import { SystemModel, systemModelType } from "../model/mod.ts";

export class SystemController {
    constructor(private readonly systemModel: SystemModel) {}

    public pause(): void {
        this.systemModel.setStatus("paused");
    }

    public resume(): void {
        this.systemModel.setStatus("resumed");
    }

    public singleIteration(): void {
        this.systemModel.setStatus("paused");
        this.systemModel.setModel(
            modelFns.iterate(this.systemModel.getModel().model),
        );
    }

    public iterate(): void {
        this.systemModel.setModel(
            modelFns.iterate(this.systemModel.getModel().model),
        );
    }

    public setDimension(
        dimension: systemModelType["dimension"],
    ): void {
        this.systemModel.setDimension(dimension);
    }

    public setGap(gap: systemModelType["gap"]): void {
        this.systemModel.setGap(gap);
    }

    public setTiles(tiles: systemModelType["tiles"]): void {
        this.systemModel.setTiles(tiles);
    }

    public setFps(fps: systemModelType["fps"]): void {
        this.systemModel.setFps(fps);
    }
}

import { buildPresetsItems } from "../buildPresetsItems";
import { CartesianPoint, modelFns } from "game-of-life-engine";
import { SystemModel, systemModelType } from "../model/mod";

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

    public toggleCell(
        point: CartesianPoint,
    ): void {
        this.systemModel.setModel(
            modelFns.toggle(
                this.systemModel.getModel().model,
                point,
            ),
        );
    }

    public setPreset(
        preset: string,
    ): void {
        const selectedPreset = buildPresetsItems().find(({ id }) =>
            id === preset
        );
        if (selectedPreset === undefined) {
            return;
        }
        this.systemModel.setModel(selectedPreset.model);
    }

    public setDimension(
        dimension: systemModelType["dimension"],
    ): void {
        this.systemModel.setDimension(dimension);
    }

    public setGap(gap: systemModelType["gap"]): void {
        this.systemModel.setGap(gap);
    }

    public setSize(newSize: number): void {
        this.systemModel.setModel(
            modelFns.zoom(
                this.systemModel.getModel().model,
                newSize,
            ),
        );
    }

    public move(delta: CartesianPoint): void {
        this.systemModel.setModel(
            modelFns.move(
                this.systemModel.getModel().model,
                delta,
            ),
        );
    }

    public setFps(fps: systemModelType["fps"]): void {
        this.systemModel.setFps(fps);
    }
}

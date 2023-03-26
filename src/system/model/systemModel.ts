import { modelType } from "../../game/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";

type cbType = (param: keyof systemModelType) => void;

export type systemModelType = {
    readonly model: modelType;
    readonly gap: number;
    readonly tiles: number;
    readonly fps: number;
    readonly status: "resumed" | "paused";
    readonly dimension: number;
    readonly drawContext: drawContextType;
};

export class SystemModel {
    private readonly onChangeListeners: (cbType)[] = [];

    private model: modelType;
    private gap: number;
    private tiles: number;
    private fps: number;
    private status: "resumed" | "paused";
    private dimension: number;
    private drawContext: drawContextType;

    constructor(systemModel: systemModelType) {
        this.model = systemModel.model;
        this.gap = systemModel.gap;
        this.tiles = systemModel.tiles;
        this.fps = systemModel.fps;
        this.status = systemModel.status;
        this.dimension = systemModel.dimension;
        this.drawContext = systemModel.drawContext;
    }

    public setModel(model: systemModelType["model"]): void {
        this.model = model;
        this.onChange("model");
    }

    public setGap(gap: systemModelType["gap"]): void {
        this.gap = gap;
        this.onChange("gap");
    }

    public setTiles(tiles: systemModelType["tiles"]): void {
        this.tiles = tiles;
        this.onChange("tiles");
    }

    public setFps(fps: systemModelType["fps"]): void {
        this.fps = fps;
        this.onChange("fps");
    }

    public setStatus(status: systemModelType["status"]): void {
        this.status = status;
        this.onChange("status");
    }

    public setDimension(
        dimension: systemModelType["dimension"],
    ): void {
        this.dimension = dimension;
        this.onChange("dimension");
    }

    public setDrawContext(
        drawContext: systemModelType["drawContext"],
    ): void {
        this.drawContext = drawContext;
        this.onChange("drawContext");
    }

    public getModel(): systemModelType {
        return {
            model: this.model,
            gap: this.gap,
            tiles: this.tiles,
            fps: this.fps,
            status: this.status,
            dimension: this.dimension,
            drawContext: this.drawContext,
        };
    }

    public addOnChangeListener(cb: cbType): void {
        this.onChangeListeners.push(cb);
    }

    private onChange(param: keyof systemModelType): void {
        this.onChangeListeners.forEach((cb) => cb(param));
    }
}

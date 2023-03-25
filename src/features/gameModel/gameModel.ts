import { modelType } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";

type cbType = (param: keyof gameModelType) => void;

export type gameModelType = {
    readonly model: modelType;
    readonly gap: number;
    readonly tiles: number;
    readonly fps: number;
    readonly status: "initial" | "resumed" | "paused";
    readonly dimension: number;
    readonly drawContext: drawContextType;
};

export class GameModel {
    private readonly onChangeListeners: (cbType)[] = [];

    private model: modelType;
    private gap: number;
    private tiles: number;
    private fps: number;
    private status: "initial" | "resumed" | "paused";
    private dimension: number;
    private drawContext: drawContextType;

    constructor(gameModel: gameModelType) {
        this.model = gameModel.model;
        this.gap = gameModel.gap;
        this.tiles = gameModel.tiles;
        this.fps = gameModel.fps;
        this.status = gameModel.status;
        this.dimension = gameModel.dimension;
        this.drawContext = gameModel.drawContext;
    }

    public setModel(model: gameModelType["model"]): void {
        this.model = model;
        this.onChange("model");
    }

    public setGap(gap: gameModelType["gap"]): void {
        this.gap = gap;
        this.onChange("gap");
    }

    public setTiles(tiles: gameModelType["tiles"]): void {
        this.tiles = tiles;
        this.onChange("tiles");
    }

    public setFps(fps: gameModelType["fps"]): void {
        this.fps = fps;
        this.onChange("fps");
    }

    public setStatus(status: gameModelType["status"]): void {
        this.status = status;
        this.onChange("status");
    }

    public setDimension(dimension: gameModelType["dimension"]): void {
        this.dimension = dimension;
        this.onChange("dimension");
    }

    public setDrawContext(
        drawContext: gameModelType["drawContext"],
    ): void {
        this.drawContext = drawContext;
        this.onChange("drawContext");
    }

    public getModel(): gameModelType {
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

    private onChange(param: keyof gameModelType): void {
        this.onChangeListeners.forEach((cb) => cb(param));
    }
}

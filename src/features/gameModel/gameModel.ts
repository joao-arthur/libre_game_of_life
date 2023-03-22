import { modelType } from "../../game/model/mod.ts";
import { drawContextType } from "../../ports/drawContext.ts";

export type gameModelType = {
    readonly model: modelType;
    readonly gap: number;
    readonly tiles: number;
    readonly fps: number;
    readonly status: "initial" | "resumed" | "paused";
    readonly dimension: number | undefined;
    readonly drawContext: drawContextType | undefined;
};

export class GameModel {
    private model: modelType;
    private gap: number;
    private tiles: number;
    private fps: number;
    private status: "initial" | "resumed" | "paused";
    private dimension: number | undefined;
    private drawContext: drawContextType | undefined;

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
    }

    public setGap(gap: gameModelType["gap"]): void {
        this.gap = gap;
    }

    public setTiles(tiles: gameModelType["tiles"]): void {
        this.tiles = tiles;
    }

    public setFps(fps: gameModelType["fps"]): void {
        this.fps = fps;
    }

    public setStatus(status: gameModelType["status"]): void {
        this.status = status;
    }

    public setDimension(dimension: gameModelType["dimension"]): void {
        this.dimension = dimension;
    }

    public setDrawContext(
        drawContext: gameModelType["drawContext"],
    ): void {
        this.drawContext = drawContext;
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
}

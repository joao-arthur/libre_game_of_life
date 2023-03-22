import { GameModel, gameModelType } from "./gameModel.ts";

type cbType = (param: keyof gameModelType) => void;

export class GameModelProxy {
    private readonly onChangeListeners: (cbType)[] = [];

    public constructor(private readonly gameModel: GameModel) {}

    public setModel(model: gameModelType["model"]): void {
        this.gameModel.setModel(model);
        this.onChange("model");
    }

    public setGap(gap: gameModelType["gap"]): void {
        this.gameModel.setGap(gap);
        this.onChange("gap");
    }

    public setTiles(tiles: gameModelType["tiles"]): void {
        this.gameModel.setTiles(tiles);
        this.onChange("tiles");
    }

    public setFps(fps: gameModelType["fps"]): void {
        this.gameModel.setFps(fps);
        this.onChange("fps");
    }

    public setStatus(status: gameModelType["status"]): void {
        this.gameModel.setStatus(status);
        this.onChange("status");
    }

    public setDimension(dimension: gameModelType["dimension"]): void {
        this.gameModel.setDimension(dimension);
        this.onChange("dimension");
    }

    public getModel(): gameModelType {
        return this.gameModel.getModel();
    }

    public addOnChangeListener(cb: cbType): void {
        this.onChangeListeners.push(cb);
    }

    public removeOnChangeListener(cb: cbType): void {
        this.onChangeListeners.splice(
            this.onChangeListeners.indexOf(cb),
            1,
        );
    }

    private onChange(param: keyof gameModelType): void {
        this.onChangeListeners.forEach((cb) => cb(param));
    }
}

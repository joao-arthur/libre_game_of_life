import { GameModel, gameModelType } from "./GameModel.ts";

type cbType = () => void;

export class GameModelProxy {
    private readonly onSettingsChangeListeners: (cbType)[] = [];
    private readonly onChangeListeners: (cbType)[] = [];

    public constructor(private readonly gameModel: GameModel) {}

    public pause(): void {
        this.gameModel.pause();
        this.onSettingsChange();
        this.onChange();
    }

    public resume(): void {
        this.gameModel.resume();
        this.onSettingsChange();
        this.onChange();
    }

    public singleIteration(): void {
        this.gameModel.singleIteration();
        this.onSettingsChange();
        this.onChange();
    }

    public iterate(): void {
        this.gameModel.iterate();
        this.onChange();
    }

    public setDimension(dimension: number): void {
        this.gameModel.setDimension(dimension);
        this.onChange();
    }

    public setGap(gap: number): void {
        this.gameModel.setGap(gap);
        this.onChange();
    }

    public setSize(size: number): void {
        this.gameModel.setSize(size);
        this.onSettingsChange();
        this.onChange();
    }

    public setFps(fps: number): void {
        this.gameModel.setFps(fps);
        this.onSettingsChange();
        this.onChange();
    }

    public getModel(): gameModelType {
        return this.gameModel.getModel();
    }

    public addOnSettingsChangeListener(cb: cbType): void {
        this.onSettingsChangeListeners.push(cb);
    }

    public removeOnSettingsChangeListener(cb: cbType): void {
        this.onSettingsChangeListeners.splice(
            this.onSettingsChangeListeners.indexOf(cb),
            1,
        );
    }

    private onSettingsChange(): void {
        this.onSettingsChangeListeners.forEach((cb) => cb());
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

    private onChange(): void {
        this.onChangeListeners.forEach((cb) => cb());
    }
}

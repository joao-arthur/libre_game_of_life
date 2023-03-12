import { dimensionType } from "../../core/dimension.ts";
import { GameModel, gameModelType } from "./GameModel.ts";

type cbType = () => void;

export class GameModelProxy {
    private onSettingsChangeListeners: (cbType)[] = [];

    public constructor(private readonly gameModel: GameModel) {}

    public pause(): void {
        this.gameModel.pause();
        this.onSettingsChange();
    }

    public resume(): void {
        this.gameModel.resume();
        this.onSettingsChange();
    }

    public singleIteration(): void {
        this.gameModel.singleIteration();
        this.onSettingsChange();
    }

    public iterate(): void {
        this.gameModel.iterate();
    }

    public setDimensions(dimensions: dimensionType): void {
        this.gameModel.setDimensions(dimensions);
    }

    public setGap(gap: number): void {
        this.gameModel.setGap(gap);
    }

    public setSize(size: number): void {
        this.gameModel.setSize(size);
        this.onSettingsChange();
    }

    public setFps(fps: number): void {
        this.gameModel.setFps(fps);
        this.onSettingsChange();
    }

    private onSettingsChange(): void {
        this.onSettingsChangeListeners.forEach((cb) => cb());
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
}

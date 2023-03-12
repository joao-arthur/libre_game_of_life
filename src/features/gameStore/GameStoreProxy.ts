import { dimensionType } from "../../core/dimension.ts";
import { gameStateType, GameStore } from "./GameStore.ts";

type cbType = () => void;

export class GameStoreProxy {
    private onSettingsChangeListeners: (cbType)[] = [];

    public constructor(private readonly gameStore: GameStore) {}

    public pause(): void {
        this.gameStore.pause();
        this.onSettingsChange();
    }

    public resume(): void {
        this.gameStore.resume();
        this.onSettingsChange();
    }

    public singleIteration(): void {
        this.gameStore.singleIteration();
    }

    public iterate(): void {
        this.gameStore.iterate();
    }

    public setDimensions(dimensions: dimensionType): void {
        this.gameStore.setDimensions(dimensions);
    }

    public setGap(gap: number): void {
        this.gameStore.setGap(gap);
    }

    public setSize(size: number): void {
        this.gameStore.setSize(size);
        this.onSettingsChange();
    }

    public setFps(fps: number): void {
        this.gameStore.setFps(fps);
        this.onSettingsChange();
    }

    private onSettingsChange(): void {
        this.onSettingsChangeListeners.forEach((cb) => cb());
    }

    public getState(): gameStateType {
        return this.gameStore.getState();
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

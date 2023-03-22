import { modelFns } from "../../game/model/mod.ts";
import { GameModelProxy, gameModelType } from "../gameModel/mod.ts";

export class GameController {
    constructor(private readonly gameModelProxy: GameModelProxy) {}

    public pause(): void {
        this.gameModelProxy.setStatus("paused");
    }

    public resume(): void {
        this.gameModelProxy.setStatus("resumed");
    }

    public singleIteration(): void {
        this.gameModelProxy.setModel(
            modelFns.iterate(this.gameModelProxy.getModel().model),
        );
        this.gameModelProxy.setStatus("paused");
    }

    public iterate(): void {
        this.gameModelProxy.setModel(
            modelFns.iterate(this.gameModelProxy.getModel().model),
        );
    }

    public setDimension(dimension: gameModelType["dimension"]): void {
        this.gameModelProxy.setDimension(dimension);
    }

    public setGap(gap: gameModelType["gap"]): void {
        this.gameModelProxy.setGap(gap);
    }

    public setTiles(tiles: gameModelType["tiles"]): void {
        this.gameModelProxy.setTiles(tiles);
    }

    public setFps(fps: gameModelType["fps"]): void {
        this.gameModelProxy.setFps(fps);
    }
}

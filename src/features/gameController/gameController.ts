import { modelFns } from "../../game/model/mod.ts";
import { GameModel, gameModelType } from "../gameModel/mod.ts";

export class GameController {
    constructor(private readonly gameModel: GameModel) {}

    public pause(): void {
        this.gameModel.setStatus("paused");
    }

    public resume(): void {
        this.gameModel.setStatus("resumed");
    }

    public singleIteration(): void {
        this.gameModel.setModel(
            modelFns.iterate(this.gameModel.getModel().model),
        );
        this.gameModel.setStatus("initial");
    }

    public iterate(): void {
        this.gameModel.setModel(
            modelFns.iterate(this.gameModel.getModel().model),
        );
    }

    public setDimension(dimension: gameModelType["dimension"]): void {
        this.gameModel.setDimension(dimension);
    }

    public setGap(gap: gameModelType["gap"]): void {
        this.gameModel.setGap(gap);
    }

    public setTiles(tiles: gameModelType["tiles"]): void {
        this.gameModel.setTiles(tiles);
    }

    public setFps(fps: gameModelType["fps"]): void {
        this.gameModel.setFps(fps);
    }
}

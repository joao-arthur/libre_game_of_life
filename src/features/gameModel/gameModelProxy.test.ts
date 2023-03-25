import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../../game/model/mod.ts";
import { GameModel, gameModelType } from "./gameModel.ts";
import { GameModelProxy } from "./gameModelProxy.ts";

const defaultModel: gameModelType = {
    model: fromString([""]),
    gap: 10,
    tiles: 50,
    fps: 1,
    status: "initial",
    dimension: 100,
    drawContext: {
        clear: () => {},
        drawSquare: () => {},
    },
};

Deno.test("Should call the listener for each changed value", () => {
    const gameModel = new GameModel(defaultModel);
    const gameModelProxy = new GameModelProxy(gameModel);
    const changedProps: (keyof gameModelType)[] = [];
    gameModelProxy.addOnChangeListener((prop) => {
        changedProps.push(prop);
    });
    gameModelProxy.setModel(fromString(["⬛"]));
    gameModelProxy.setGap(0);
    gameModelProxy.setTiles(0);
    gameModelProxy.setFps(0);
    gameModelProxy.setStatus("resumed");
    gameModelProxy.setDimension(0);
    gameModelProxy.setDrawContext({
        clear: () => {},
        drawSquare: () => {},
    });
    assertEquals(changedProps, [
        "model",
        "gap",
        "tiles",
        "fps",
        "status",
        "dimension",
        "drawContext",
    ]);
});

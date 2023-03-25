import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../../game/model/mod.ts";
import { GameModel, gameModelType } from "./gameModel.ts";

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

const alternativeDrawContext: gameModelType["drawContext"] = {
    clear: () => {
        return "clear";
    },
    drawSquare: () => {
        return "drawSquare";
    },
};

Deno.test("Should create with passed arguments", () => {
    const gameModel = new GameModel(defaultModel);
    assertEquals(gameModel.getModel(), defaultModel);
});

Deno.test("Setters should", () => {
    const gameModel = new GameModel(defaultModel);
    gameModel.setModel(fromString(["⬛"]));
    gameModel.setGap(0);
    gameModel.setTiles(0);
    gameModel.setFps(0);
    gameModel.setStatus("resumed");
    gameModel.setDimension(0);
    gameModel.setDrawContext(alternativeDrawContext);
    assertEquals(gameModel.getModel(), {
        model: fromString(["⬛"]),
        gap: 0,
        tiles: 0,
        fps: 0,
        status: "resumed",
        dimension: 0,
        drawContext: alternativeDrawContext,
    });
});

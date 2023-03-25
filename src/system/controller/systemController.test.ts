import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../../game/model/mod.ts";
import { GameModel, gameModelType } from "../gameModel/mod.ts";
import { GameController } from "./gameController.ts";

const defaultModel: gameModelType = {
    model: fromString([""]),
    gap: 10,
    tiles: 50,
    fps: 1,
    status: "paused",
    dimension: 100,
    drawContext: {
        clear: () => {},
        drawSquare: () => {},
    },
};

Deno.test("pause", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.pause();
    assertEquals(gameModel.getModel().status, "paused");
});

Deno.test("resume", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.resume();
    assertEquals(gameModel.getModel().status, "resumed");
});

Deno.test("singleIteration", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.singleIteration();
    assertEquals(gameModel.getModel().model.iteration, 1);
    assertEquals(gameModel.getModel().status, "initial");
});

Deno.test("iterate", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.iterate();
    assertEquals(gameModel.getModel().model.iteration, 1);
    assertEquals(gameModel.getModel().status, "paused");
});

Deno.test("setDimension", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.setDimension(9);
    assertEquals(gameModel.getModel().dimension, 9);
});

Deno.test("setGap", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.setGap(9);
    assertEquals(gameModel.getModel().gap, 9);
});

Deno.test("setTiles", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.setTiles(9);
    assertEquals(gameModel.getModel().tiles, 9);
});

Deno.test("setFps", () => {
    const gameModel = new GameModel(defaultModel);
    const gameController = new GameController(gameModel);
    gameController.setFps(9);
    assertEquals(gameModel.getModel().fps, 9);
});

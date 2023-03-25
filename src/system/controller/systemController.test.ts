import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { fromString } from "../../game/model/mod.ts";
import { SystemModel, systemModelType } from "../model/mod.ts";
import { SystemController } from "./systemController.ts";

const defaultModel: systemModelType = {
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
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.pause();
    assertEquals(systemModel.getModel().status, "paused");
});

Deno.test("resume", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.resume();
    assertEquals(systemModel.getModel().status, "resumed");
});

Deno.test("singleIteration", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.singleIteration();
    assertEquals(systemModel.getModel().model.iteration, 1);
    assertEquals(systemModel.getModel().status, "paused");
});

Deno.test("iterate", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.iterate();
    assertEquals(systemModel.getModel().model.iteration, 1);
    assertEquals(systemModel.getModel().status, "paused");
});

Deno.test("setDimension", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setDimension(9);
    assertEquals(systemModel.getModel().dimension, 9);
});

Deno.test("setGap", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setGap(9);
    assertEquals(systemModel.getModel().gap, 9);
});

Deno.test("setTiles", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setTiles(9);
    assertEquals(systemModel.getModel().tiles, 9);
});

Deno.test("setFps", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setFps(9);
    assertEquals(systemModel.getModel().fps, 9);
});

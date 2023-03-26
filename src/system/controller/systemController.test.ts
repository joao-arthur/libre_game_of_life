import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { modelFns } from "../../game/mod.ts";
import { SystemModel, systemModelType } from "../model/mod.ts";
import { SystemController } from "./systemController.ts";

const defaultModel: systemModelType = {
    model: modelFns.fromString(["⬛"]),
    gap: 10,
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

Deno.test("toggleCell", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.toggleCell({ column: 0, row: 0 });
    assertEquals(
        systemModel.getModel().model,
        modelFns.fromString(["⬜"]),
    );
});

Deno.test("setDimension", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setDimension(9);
    assertEquals(systemModel.getModel().dimension, 9);
});

Deno.test("setSize", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setSize(5);
    assertEquals(systemModel.getModel().model.size, 5);
});

Deno.test("setGap", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setGap(9);
    assertEquals(systemModel.getModel().gap, 9);
});

Deno.test("setFps", () => {
    const systemModel = new SystemModel(defaultModel);
    const systemController = new SystemController(systemModel);
    systemController.setFps(9);
    assertEquals(systemModel.getModel().fps, 9);
});

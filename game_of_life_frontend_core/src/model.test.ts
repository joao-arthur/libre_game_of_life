import type { DrawContext, SystemModel } from "./model.js";
import { assert, test } from "vitest";
import { buildPresets, fpsToMilis, SystemController, SystemModelProxy } from "./model.js";
import { modelFromString } from "game_of_life_engine";

const defaultModelDead: SystemModel = {
    model: modelFromString([""]),
    gap: 10,
    fps: 1,
    status: "paused",
    dimension: 100,
    drawContext: {
        clear: () => {},
        drawSquare: () => {},
    },
};

const defaultModelOneDead: SystemModel = {
    model: modelFromString(["⬛"]),
    gap: 10,
    fps: 1,
    status: "paused",
    dimension: 100,
    drawContext: {
        clear: () => {},
        drawSquare: () => {},
    },
};

const alternativeDrawContext: DrawContext = {
    clear: () => {
        return "clear";
    },
    drawSquare: () => {
        return "drawSquare";
    },
};

test("fpsToMilis", () => {
    assert.deepStrictEqual(fpsToMilis(0), Infinity);
    assert.deepStrictEqual(fpsToMilis(1), 1000);
    assert.deepStrictEqual(fpsToMilis(5), 200);
    assert.deepStrictEqual(fpsToMilis(10), 100);
    assert.deepStrictEqual(fpsToMilis(50), 20);
    assert.deepStrictEqual(fpsToMilis(100), 10);
    assert.deepStrictEqual(fpsToMilis(500), 2);
    assert.deepStrictEqual(fpsToMilis(1000), 1);
});

test("SystemModel Should create with passed arguments", () => {
    const systemModel = new SystemModelProxy(defaultModelDead);
    assert.deepStrictEqual(systemModel.getModel(), defaultModelDead);
});

test("SystemModel Setters should", () => {
    const systemModel = new SystemModelProxy(defaultModelDead);
    systemModel.setModel(modelFromString(["⬛"]));
    systemModel.setGap(0);
    systemModel.setFps(0);
    systemModel.setStatus("resumed");
    systemModel.setDimension(0);
    systemModel.setDrawContext(alternativeDrawContext);
    assert.deepStrictEqual(
        systemModel.getModel(),
        {
            model: modelFromString(["⬛"]),
            gap: 0,
            fps: 0,
            status: "resumed",
            dimension: 0,
            drawContext: alternativeDrawContext,
        },
    );
});

test("SystemModel Should call the listener for each changed value", () => {
    const systemModel = new SystemModelProxy(defaultModelDead);
    const changedProps: (keyof SystemModel)[] = [];
    systemModel.addOnChangeListener((prop) => {
        changedProps.push(prop);
    });
    systemModel.setModel(modelFromString(["⬛"]));
    systemModel.setGap(0);
    systemModel.setFps(0);
    systemModel.setStatus("resumed");
    systemModel.setDimension(0);
    systemModel.setDrawContext({ clear: () => {}, drawSquare: () => {} });
    assert.deepStrictEqual(
        changedProps,
        ["model", "gap", "fps", "status", "dimension", "drawContext"],
    );
});

test("SystemModelProxy pause", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.pause();
    assert.deepStrictEqual(systemModel.getModel().status, "paused");
});

test("SystemModelProxy resume", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.resume();
    assert.deepStrictEqual(systemModel.getModel().status, "resumed");
});

test("SystemModelProxy singleIteration", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.singleIteration();
    assert.deepStrictEqual(systemModel.getModel().model.iter, 1);
    assert.deepStrictEqual(systemModel.getModel().status, "paused");
});

test("SystemModelProxy iterate", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.iterate();
    assert.deepStrictEqual(systemModel.getModel().model.iter, 1);
    assert.deepStrictEqual(systemModel.getModel().status, "paused");
});

test("SystemModelProxy toggleCell", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.toggleCell({ x: 0, y: 0 });
    assert.deepStrictEqual(systemModel.getModel().model, modelFromString(["⬜"]));
});

test("SystemModelProxy setDimension", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.setDimension(9);
    assert.deepStrictEqual(systemModel.getModel().dimension, 9);
});

test("SystemModelProxy setPreset", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.setPreset("blinker");
    const blinker = buildPresets().find(({ id }) => id === "blinker")!;
    assert.deepStrictEqual(systemModel.getModel().model.value, blinker.model.value);
});

test("SystemModelProxy setSize", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.setSize(5);
    const pos = systemModel.getModel().model.pos;
    const length = pos.x2 - pos.x1;
    assert.deepStrictEqual(length, 4);
});

test("SystemModelProxy move", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    const oldX1 = systemModel.getModel().model.pos.x1;
    systemController.move({ x: 1, y: 0 });
    const newX1 = systemModel.getModel().model.pos.x1;
    assert.deepStrictEqual(newX1 - 1, oldX1);
});

test("SystemModelProxy setGap", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.setGap(9);
    assert.deepStrictEqual(systemModel.getModel().gap, 9);
});

test("SystemModelProxy setFps", () => {
    const systemModel = new SystemModelProxy(defaultModelOneDead);
    const systemController = new SystemController(systemModel);
    systemController.setFps(9);
    assert.deepStrictEqual(systemModel.getModel().fps, 9);
});

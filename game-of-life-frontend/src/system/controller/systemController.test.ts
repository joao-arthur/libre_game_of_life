import { describe, expect, it } from "vitest";
import { modelFns } from "game-of-life-engine";
import { SystemModel, systemModelType } from "../model/mod";
import { buildPresetsItems } from "../buildPresetsItems";
import { SystemController } from "./systemController";

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

describe("describe", () => {
    it("pause", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.pause();
        expect(systemModel.getModel().status).toBe("paused");
    });

    it("resume", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.resume();
        expect(systemModel.getModel().status).toBe("resumed");
    });

    it("singleIteration", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.singleIteration();
        expect(systemModel.getModel().model.iteration).toBe(1);
        expect(systemModel.getModel().status).toBe("paused");
    });

    it("iterate", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.iterate();
        expect(systemModel.getModel().model.iteration).toBe(1);
        expect(systemModel.getModel().status).toBe("paused");
    });

    it("toggleCell", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.toggleCell({ x: 0, y: 0 });
        expect(systemModel.getModel().model).toEqual(
            modelFns.fromString(["⬜"]),
        );
    });

    it("setDimension", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.setDimension(9);
        expect(systemModel.getModel().dimension).toBe(9);
    });

    it("setPreset", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.setPreset("blinker");
        const blinker = buildPresetsItems().find(({ id }) =>
            id === "blinker"
        )!;
        expect(
            systemModel.getModel().model.value,
        ).toEqual(
            blinker.model.value,
        );
    });

    it("setSize", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.setSize(5);
        const { x2, x1 } = systemModel.getModel().model.position;
        const length = x2 - x1;
        expect(length).toBe(4);
    });

    it("move", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        const oldX1 = systemModel.getModel().model.position.x1;
        systemController.move({ x: 1, y: 0 });
        const newX1 = systemModel.getModel().model.position.x1;
        expect(newX1 - 1).toBe(oldX1);
    });

    it("setGap", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.setGap(9);
        expect(systemModel.getModel().gap).toBe(9);
    });

    it("setFps", () => {
        const systemModel = new SystemModel(defaultModel);
        const systemController = new SystemController(systemModel);
        systemController.setFps(9);
        expect(systemModel.getModel().fps).toBe(9);
    });
});

import { describe, expect, it } from "vitest";
import { modelFns } from "game_of_life_engine";
import { SystemModel, systemModelType } from "./systemModel";

const defaultModel: systemModelType = {
    model: modelFns.fromString([""]),
    gap: 10,
    fps: 1,
    status: "paused",
    dimension: 100,
    drawContext: {
        clear: () => {},
        drawSquare: () => {},
    },
};

const alternativeDrawContext: systemModelType["drawContext"] = {
    clear: () => {
        return "clear";
    },
    drawSquare: () => {
        return "drawSquare";
    },
};

describe("SystemModel", () => {
    it("Should create with passed arguments", () => {
        const systemModel = new SystemModel(defaultModel);
        expect(systemModel.getModel()).toEqual(defaultModel);
    });

    it("Setters should", () => {
        const systemModel = new SystemModel(defaultModel);
        systemModel.setModel(modelFns.fromString(["⬛"]));
        systemModel.setGap(0);
        systemModel.setFps(0);
        systemModel.setStatus("resumed");
        systemModel.setDimension(0);
        systemModel.setDrawContext(alternativeDrawContext);
        expect(systemModel.getModel()).toEqual({
            model: modelFns.fromString(["⬛"]),
            gap: 0,
            fps: 0,
            status: "resumed",
            dimension: 0,
            drawContext: alternativeDrawContext,
        });
    });

    it("Should call the listener for each changed value", () => {
        const systemModel = new SystemModel(defaultModel);
        const changedProps: (keyof systemModelType)[] = [];
        systemModel.addOnChangeListener((prop) => {
            changedProps.push(prop);
        });
        systemModel.setModel(modelFns.fromString(["⬛"]));
        systemModel.setGap(0);
        systemModel.setFps(0);
        systemModel.setStatus("resumed");
        systemModel.setDimension(0);
        systemModel.setDrawContext({
            clear: () => {},
            drawSquare: () => {},
        });
        expect(changedProps).toEqual([
            "model",
            "gap",
            "fps",
            "status",
            "dimension",
            "drawContext",
        ]);
    });
});

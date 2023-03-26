import { assertEquals } from "https://deno.land/std@0.171.0/testing/asserts.ts";
import { modelFns } from "../../game/mod.ts";
import { SystemModel, systemModelType } from "./systemModel.ts";

const defaultModel: systemModelType = {
    model: modelFns.fromString([""]),
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

const alternativeDrawContext: systemModelType["drawContext"] = {
    clear: () => {
        return "clear";
    },
    drawSquare: () => {
        return "drawSquare";
    },
};

Deno.test("Should create with passed arguments", () => {
    const systemModel = new SystemModel(defaultModel);
    assertEquals(systemModel.getModel(), defaultModel);
});

Deno.test("Setters should", () => {
    const systemModel = new SystemModel(defaultModel);
    systemModel.setModel(modelFns.fromString(["⬛"]));
    systemModel.setGap(0);
    systemModel.setTiles(0);
    systemModel.setFps(0);
    systemModel.setStatus("resumed");
    systemModel.setDimension(0);
    systemModel.setDrawContext(alternativeDrawContext);
    assertEquals(systemModel.getModel(), {
        model: modelFns.fromString(["⬛"]),
        gap: 0,
        tiles: 0,
        fps: 0,
        status: "resumed",
        dimension: 0,
        drawContext: alternativeDrawContext,
    });
});

Deno.test("Should call the listener for each changed value", () => {
    const systemModel = new SystemModel(defaultModel);
    const changedProps: (keyof systemModelType)[] = [];
    systemModel.addOnChangeListener((prop) => {
        changedProps.push(prop);
    });
    systemModel.setModel(modelFns.fromString(["⬛"]));
    systemModel.setGap(0);
    systemModel.setTiles(0);
    systemModel.setFps(0);
    systemModel.setStatus("resumed");
    systemModel.setDimension(0);
    systemModel.setDrawContext({
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

import { fromString } from "../src/game/model/fromString/fromString.ts";
import { dimensionType } from "../src/core/dimension.ts";
import { modelType } from "../src/game/model/mod.ts";
import { modelFns } from "../src/game/model/modelFns.ts";

type gameStoreType = {
    model: modelType;
    dimensions: dimensionType;
    gap: number;
    size: number;
    speed: number;
    status: "resumed" | "paused";
};

const defaultState: gameStoreType = {
    model: fromString([
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬜⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬜⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
        "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
    ]),
    dimensions: { width: 0, height: 0 },
    gap: 1,
    size: 20,
    speed: 20,
    status: "paused",
};

const actions = {
    pause: (state: gameStoreType): gameStoreType => ({
        ...state,
        status: "paused",
    }),
    resume: (state: gameStoreType): gameStoreType => ({
        ...state,
        status: "resumed",
    }),
    iterate: (state: gameStoreType): gameStoreType => ({
        ...state,
        model: modelFns.iterate(state.model),
        status: "paused",
    }),
    setGap: (
        state: gameStoreType,
        action: { gap: gameStoreType["gap"] },
    ): gameStoreType => ({
        ...state,
        gap: action.gap,
    }),
    setSize: (
        state: gameStoreType,
        action: { size: gameStoreType["size"] },
    ): gameStoreType => ({
        ...state,
        size: action.size,
    }),
    setSpeed: (
        state: gameStoreType,
        action: { speed: gameStoreType["speed"] },
    ): gameStoreType => ({
        ...state,
        speed: action.speed,
    }),
};

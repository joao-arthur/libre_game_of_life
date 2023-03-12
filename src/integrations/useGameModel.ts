import { useState } from "preact/hooks";
import { dimensionType } from "../core/dimension.ts";
import {
    GameModel,
    GameModelProxy,
    gameModelType,
} from "../features/gameModel/mod.ts";

type returnType = {
    readonly model: gameModelType;
    readonly actions: {
        readonly pause: () => void;
        readonly resume: () => void;
        readonly singleIteration: () => void;
        readonly iterate: () => void;
        readonly setDimensions: (dimensions: dimensionType) => void;
        readonly setGap: (gap: number) => void;
        readonly setSize: (size: number) => void;
        readonly setFps: (fps: number) => void;
    };
    readonly gameModelProxy: GameModelProxy;
};

const gameModel = new GameModel();
const gameModelProxy = new GameModelProxy(gameModel);

export function useGameModel(): returnType {
    const [model, setModel] = useState(
        gameModelProxy.getModel(),
    );

    function pause(): void {
        gameModelProxy.pause();
        setModel(gameModelProxy.getModel());
    }

    function resume(): void {
        gameModelProxy.resume();
        setModel(gameModelProxy.getModel());
    }

    function singleIteration(): void {
        gameModelProxy.singleIteration();
        setModel(gameModelProxy.getModel());
    }

    function iterate(): void {
        gameModelProxy.iterate();
        setModel(gameModelProxy.getModel());
    }

    function setDimensions(dimensions: dimensionType): void {
        gameModelProxy.setDimensions(dimensions);
        setModel(gameModelProxy.getModel());
    }

    function setGap(gap: number): void {
        gameModelProxy.setGap(gap);
        setModel(gameModelProxy.getModel());
    }

    function setSize(size: number): void {
        gameModelProxy.setSize(size);
        setModel(gameModelProxy.getModel());
    }

    function setFps(fps: number): void {
        gameModelProxy.setFps(fps);
        setModel(gameModelProxy.getModel());
    }

    return {
        model,
        actions: {
            pause,
            resume,
            singleIteration,
            iterate,
            setDimensions,
            setGap,
            setSize,
            setFps,
        },
        gameModelProxy,
    };
}

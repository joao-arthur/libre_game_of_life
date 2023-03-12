import { useState } from "preact/hooks";
import { dimensionType } from "../src/core/dimension.ts";
import {
    gameStateType,
    GameStore,
    GameStoreProxy,
} from "../src/features/gameStore/mod.ts";

type returnType = {
    readonly state: gameStateType;
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
    readonly store: GameStoreProxy;
};

const store = new GameStore();
const storeProxy = new GameStoreProxy(store);

export function useGameStore(): returnType {
    const [gameState, setGameState] = useState(storeProxy.getState());

    function pause(): void {
        storeProxy.pause();
        setGameState(storeProxy.getState());
    }

    function resume(): void {
        storeProxy.resume();
        setGameState(storeProxy.getState());
    }

    function singleIteration(): void {
        storeProxy.singleIteration();
        setGameState(storeProxy.getState());
    }

    function iterate(): void {
        storeProxy.iterate();
        setGameState(storeProxy.getState());
    }

    function setDimensions(dimensions: dimensionType): void {
        storeProxy.setDimensions(dimensions);
        setGameState(storeProxy.getState());
    }

    function setGap(gap: number): void {
        storeProxy.setGap(gap);
        setGameState(storeProxy.getState());
    }

    function setSize(size: number): void {
        storeProxy.setSize(size);
        setGameState(storeProxy.getState());
    }

    function setFps(fps: number): void {
        storeProxy.setFps(fps);
        setGameState(storeProxy.getState());
    }

    return {
        state: gameState,
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
        store: storeProxy,
    };
}

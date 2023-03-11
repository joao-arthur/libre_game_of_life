import { useState } from "preact/hooks";
import { actions, defaultState, gameStoreType } from "./gameStore.ts";

type returnType = {
    readonly store: gameStoreType;
    readonly actions: {
        readonly pause: () => void;
        readonly resume: () => void;
        readonly iterate: () => void;
        readonly setGap: (gap: number) => void;
        readonly setSize: (size: number) => void;
        readonly setFps: (fps: number) => void;
    };
};

export function useGameStore(): returnType {
    const [store, setStore] = useState(defaultState);

    function pause(): void {
        setStore(actions.pause(store));
    }

    function resume(): void {
        setStore(actions.resume(store));
    }

    function iterate(): void {
        setStore(actions.iterate(store));
    }

    function setGap(gap: number): void {
        setStore(actions.setGap(store, { gap }));
    }

    function setSize(size: number): void {
        setStore(actions.setSize(store, { size }));
    }

    function setFps(fps: number): void {
        setStore(actions.setFps(store, { fps }));
    }

    return {
        store,
        actions: {
            pause,
            resume,
            iterate,
            setGap,
            setSize,
            setFps,
        },
    };
}

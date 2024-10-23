import { useEffect, useState } from "react";
import {
    engineAddOnChangeListener,
    engineGetSettings,
    EngineInfo,
    engineInit,
    engineSetDimension,
} from "game_of_life_engine";
import { useWindowDimension } from "./useWindowDimension";

type GameOfLife = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: EngineInfo | undefined;
};

export function useGameOfLife(): GameOfLife {
    const [hasInited, setInit] = useState(false);
    const [model, setModel] = useState<EngineInfo | undefined>(undefined);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (hasInited) {
            engineSetDimension(dimension);
        }
    }, [dimension, hasInited]);

    function init(canvasElement: HTMLCanvasElement) {
        const context = canvasElement.getContext("2d");
        if (!context) {
            return;
        }

        engineInit(context);

        engineAddOnChangeListener(() => {
            let obj = engineGetSettings();
            setModel({
                size: obj.size,
                fps: obj.fps,
                gap: obj.gap,
                preset: obj.preset,
                iter: obj.iter,
                status: obj.status,
            } as any);
        });
        setInit(true);
    }

    return { init, model };
}

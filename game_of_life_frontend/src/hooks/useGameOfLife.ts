import { useEffect, useState } from "react";
import {
    engineGetSettings,
    engineInit,
    engineSetDimension,
    EngineSettings,
} from "game_of_life_engine";
import { useWindowDimension } from "./useWindowDimension";

type GameOfLife = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: EngineSettings | undefined;
};

export function useGameOfLife(): GameOfLife {
    const [model, setModel] = useState<EngineSettings | undefined>(undefined);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (model) {
            engineSetDimension(dimension);
        }
    }, [dimension, model]);

    function init(canvasElement: HTMLCanvasElement) {
        const context = canvasElement.getContext("2d");
        if (!context) {
            return;
        }

        engineInit(context);
        let obj = engineGetSettings();
        //engineAddOnChangeListener(() => {
        //    console.log(engineGetSettings());
        //});
        setModel({
            dim: obj.dim,
            fps: obj.fps,
            gap: obj.gap,
            preset: obj.preset,
            status: obj.status,
        } as any);
    }

    return { init, model };
}

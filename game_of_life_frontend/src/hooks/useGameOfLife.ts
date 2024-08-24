import { useCallback, useEffect, useRef, useState } from "react";
import {
    buildModel,
    CanvasDrawContext,
    manage,
    SystemController,
    SystemModel,
    SystemModelProxy,
} from "game_of_life_frontend_core";
import { useWindowDimension } from "./useWindowDimension";

type GameOfLife = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: SystemModel | undefined;
    readonly controller: SystemController | undefined;
};

export function useGameOfLife(): GameOfLife {
    const systemControllerRef = useRef<SystemController | undefined>(undefined);
    const [model, setModel] = useState<SystemModel | undefined>(undefined);
    const dimension = useWindowDimension();

    useEffect(() => {
        systemControllerRef.current?.setDimension(dimension);
    }, [dimension]);

    const init = useCallback(
        (canvasElement: HTMLCanvasElement) => {
            const context = canvasElement.getContext("2d");
            if (!context) {
                return;
            }
            const canvasDrawContext = new CanvasDrawContext(context);
            const systemModel = new SystemModelProxy(buildModel(canvasDrawContext, dimension));
            const systemController = new SystemController(systemModel);
            manage(systemModel, systemController);
            systemModel.addOnChangeListener(() =>{console.log('wfheuiuhe'); setModel(systemModel.getModel())});
            setModel(systemModel.getModel());
            systemControllerRef.current = systemController;
        },
        [],
    );

    return {
        init,
        model,
        controller: systemControllerRef.current,
    };
}

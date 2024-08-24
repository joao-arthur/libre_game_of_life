import { useCallback, useEffect, useRef, useState } from "react";
import { 
    CanvasDrawContext
    buildModel,
    SystemController,
    SystemManager,
    SystemModel,
    systemModelType,
    SystemRender,
 } from "game_of_life_frontend_core";
import { useWindowDimension } from "./useWindowDimension";

type GameOfLife = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: systemModelType | undefined;
    readonly controller: SystemController | undefined;
};

export function useGameOfLife(): GameOfLife {
    const systemControllerRef = useRef<SystemController | undefined>(undefined,);
    const systemManagerRef = useRef<SystemManager | undefined>(undefined,);
    const [model, setModel] = useState<systemModelType | undefined>(undefined,);
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
            const systemModel = new SystemModel(
                buildModel(canvasDrawContext, dimension),
            );
            const systemRender = new SystemRender(systemModel);
            const systemController = new SystemController(
                systemModel,
            );
            systemManagerRef.current = new SystemManager(
                systemModel,
                systemController,
                systemRender,
            );
            systemModel.addOnChangeListener(() =>
                setModel(systemModel.getModel())
            );
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

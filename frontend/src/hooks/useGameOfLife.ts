import {
    useCallback,
    useEffect,
    useRef,
    useState,
} from "react";
import { useWindowDimension } from "./useWindowDimension";
import { CanvasDrawContext } from "../adapters/canvasDrawContext";
import {
    buildModel,
    SystemController,
    SystemManager,
    SystemModel,
    systemModelType,
    SystemRender,
} from "../system/mod";

type gameOfLifeType = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: systemModelType | undefined;
    readonly controller: SystemController | undefined;
};

export function useGameOfLife(): gameOfLifeType {
    const systemControllerRef = useRef<SystemController | undefined>(
        undefined,
    );
    const [model, setModel] = useState<systemModelType | undefined>(
        undefined,
    );
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
            const _systemManager = new SystemManager(
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

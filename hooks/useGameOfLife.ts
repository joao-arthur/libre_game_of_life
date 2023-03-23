import {
    useCallback,
    useEffect,
    useRef,
    useState,
} from "preact/hooks";
import { useWindowDimension } from "../hooks/useWindowDimension.ts";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { buildModel } from "../src/features/gameModel/buildModel.ts";
import {
    GameController,
    GameManager,
    GameModel,
    GameModelProxy,
    gameModelType,
    GameRender,
} from "../src/features/mod.ts";

type gameOfLifeType = {
    init: (canvasElement: HTMLCanvasElement) => void;
    model: gameModelType | undefined;
    controller: GameController | undefined;
};

export function useGameOfLife(): gameOfLifeType {
    const gameModelProxyRef = useRef<GameModelProxy | undefined>(
        undefined,
    );
    const gameControllerRef = useRef<GameController | undefined>(
        undefined,
    );
    const dimension = useWindowDimension();

    const [model, setModel] = useState<gameModelType | undefined>(
        undefined,
    );

    useEffect(() => {
        gameControllerRef.current?.setDimension(dimension);
    }, [dimension]);

    const init = useCallback(
        (canvasElement: HTMLCanvasElement) => {
            const context = canvasElement.getContext("2d");
            if (!context) {
                return;
            }
            const drawContext = new CanvasDrawContext(context);
            const gameModel = new GameModel(
                buildModel(drawContext, dimension),
            );
            const gameModelProxy = new GameModelProxy(gameModel);
            const gameRender = new GameRender(gameModel, drawContext);
            const gameController = new GameController(gameModelProxy);
            const _gameManager = new GameManager(
                gameModelProxy,
                gameController,
                gameRender,
            );
            gameModelProxy.addOnChangeListener(() =>
                setModel(gameModelProxy.getModel())
            );
            setModel(gameModelProxy.getModel());
            gameModelProxyRef.current = gameModelProxy;
            gameControllerRef.current = gameController;
        },
        [],
    );

    return {
        init,
        model,
        controller: gameControllerRef.current,
    };
}
